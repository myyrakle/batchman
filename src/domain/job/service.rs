use std::sync::Arc;

use crate::{
    domain::{
        container::{
            ContainerRepository,
            dao::{InspectContainerParams, RunContainerParams, StopContainerParams},
        },
        job::dto::{
            CountJobLogsRequest, CountJobLogsResponse, JobLogDto, ListJobLogsRequest,
            ListJobLogsResponse, SubmitJobResponse,
        },
        task_definition::{TaskDefinitionRepository, dao::ListTaskDefinitionsParams},
    },
    errors,
};

use super::{
    JobRepository, JobService,
    dao::{CreateJobParams, ListJobsParams, PatchJobParams},
    dto::{JobDto, ListJobsRequest, ListJobsResponse, StopJobRequest, SubmitJobRequest},
    entities::{self, job::JobStatus},
};

pub struct JobServiceImpl {
    pub job_repository: Arc<dyn JobRepository + Send + Sync>,
    pub task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
    pub container_repository: Arc<dyn ContainerRepository + Send + Sync>,
}

impl JobServiceImpl {
    pub fn new(
        job_repository: Arc<dyn JobRepository + Send + Sync>,
        task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
        container_repository: Arc<dyn ContainerRepository + Send + Sync>,
    ) -> Self {
        Self {
            job_repository,
            task_definition_repository,
            container_repository,
        }
    }
}

#[async_trait::async_trait]
impl JobService for JobServiceImpl {
    async fn submit_job(&self, params: SubmitJobRequest) -> errors::Result<SubmitJobResponse> {
        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: vec![params.request_body.task_definition_id],
                ..Default::default()
            })
            .await?;

        if task_definitions.is_empty() {
            return Err(errors::Error::TaskDefinitionNotFound);
        };

        let new_job_id = self
            .job_repository
            .create_job(CreateJobParams {
                name: params.request_body.job_name.clone(),
                task_definition_id: params.request_body.task_definition_id,
                status: entities::job::JobStatus::Pending,
                submited_at: Some(chrono::Utc::now()),
                ..Default::default()
            })
            .await?;

        Ok(SubmitJobResponse { job_id: new_job_id })
    }

    async fn stop_job(&self, params: StopJobRequest) -> errors::Result<()> {
        let job_id = params.request_body.job_id;

        let mut jobs = self
            .job_repository
            .list_jobs(ListJobsParams {
                job_ids: vec![job_id],
                ..Default::default()
            })
            .await?;

        let Some(job) = jobs.pop() else {
            return Err(errors::Error::JobNotFound);
        };

        if job.status == JobStatus::Finished {
            return Err(errors::Error::JobAlreadyFinished);
        }

        if job.status == JobStatus::Failed {
            return Err(errors::Error::JobAlreadyFailed);
        }

        let Some(container_id) = job.container_id.as_ref() else {
            return Err(errors::Error::JobHasNoContainerID);
        };

        self.container_repository
            .stop_container(StopContainerParams {
                container_id: container_id.clone(),
                timeout_seconds: 3,
            })
            .await?;

        Ok(())
    }

    async fn run_pending_job(&self, pending_job: &entities::job::Model) -> errors::Result<()> {
        // TODO: 리소스 제한이나 실행 제한 등에 걸리지 않는지 확인 (차후 개발)

        // 1. job 상태를 START로 변경
        self.job_repository
            .patch_job(PatchJobParams {
                job_id: pending_job.id,
                status: Some(JobStatus::Starting),
                started_at: Some(chrono::Utc::now()),
                ..Default::default()
            })
            .await?;

        // 2. 컨테이너 실행을 위해 task definition을 가져옴
        let mut task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: vec![pending_job.task_definition_id],
                ..Default::default()
            })
            .await?;

        let Some(task_definition) = task_definitions.pop() else {
            return Err(errors::Error::JobNotFound);
        };

        // 3. 컨테이너 실행
        let container_id = self
            .container_repository
            .run_container(RunContainerParams {
                task_definition: task_definition.clone(),
            })
            .await?
            .container_id;

        // 4. 컨테이너 정보를 job에 업데이트, job 상태를 RUNNING으로 변경
        self.job_repository
            .patch_job(PatchJobParams {
                job_id: pending_job.id,
                container_id: Some(container_id.clone()),
                status: Some(JobStatus::Running),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    async fn track_running_job(&self, job: &entities::job::Model) -> errors::Result<()> {
        let Some(container_id) = &job.container_id else {
            return Err(errors::Error::ContainerIDNotFound);
        };

        let inspect_result = self
            .container_repository
            .inspect_container(InspectContainerParams {
                container_id: container_id.clone(),
            })
            .await?;

        // 1. 컨테이너가 여전히 실행 중인 경우, 아무 작업도 하지 않음
        if inspect_result.state.running {
            return Ok(());
        }

        // 2. 컨테이너가 모종의 이유로 조기 종료(실패)했을 경우 처리
        if inspect_result.state.dead {
            self.job_repository
                .patch_job(PatchJobParams {
                    job_id: job.id,
                    status: Some(entities::job::JobStatus::Failed),
                    error_message: Some(format!(
                        "Container is dead: {}",
                        inspect_result.state.error.unwrap_or_default()
                    )),
                    ..Default::default()
                })
                .await?;

            return Ok(());
        }

        // 3. 컨테이너가 종료되었을 경우 종료 처리
        if let Some(finished_at) = inspect_result.state.finished_at {
            self.job_repository
                .patch_job(PatchJobParams {
                    job_id: job.id,
                    status: Some(entities::job::JobStatus::Finished),
                    finished_at: Some(finished_at),
                    exit_code: inspect_result.state.exit_code,
                    ..Default::default()
                })
                .await?;

            return Ok(());
        }

        Ok(())
    }

    async fn list_jobs(&self, params: ListJobsRequest) -> errors::Result<ListJobsResponse> {
        let query = &params.request_query;

        // 페이지네이션 계산
        let page_number = query.page_number.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        let offset = (page_number - 1) * page_size;

        // 상태 필터링
        let mut statuses = Vec::new();
        if let Some(status_str) = &query.status {
            match status_str.as_str() {
                "Pending" => statuses.push(JobStatus::Pending),
                "Starting" => statuses.push(JobStatus::Starting),
                "Running" => statuses.push(JobStatus::Running),
                "Finished" => statuses.push(JobStatus::Finished),
                "Failed" => statuses.push(JobStatus::Failed),
                _ => {}
            }
        }

        // job_id 필터링
        let mut job_ids = Vec::new();
        if let Some(job_id) = query.job_id {
            job_ids.push(job_id);
        }

        let list_params = ListJobsParams {
            job_ids: job_ids.clone(),
            statuses: statuses.clone(),
            limit: Some(page_size),
            offset: Some(offset),
            contains_name: query.contains_name.clone(),
        };

        let count_params = ListJobsParams {
            job_ids,
            statuses,
            limit: None,
            offset: None,
            contains_name: query.contains_name.clone(),
        };

        // 목록과 전체 카운트를 각각 조회
        let total_count = self.job_repository.count_jobs(count_params).await?;
        if total_count == 0 {
            return Ok(ListJobsResponse {
                jobs: Vec::new(),
                total_count,
            });
        }

        let jobs = self.job_repository.list_jobs(list_params).await?;

        let task_definition_ids: Vec<i64> = jobs.iter().map(|job| job.task_definition_id).collect();

        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids,
                ..Default::default()
            })
            .await?;

        // Model을 JobDto로 변환
        let mut job_dtos: Vec<JobDto> = jobs.into_iter().map(|job| job.into()).collect();

        for job_dto in &mut job_dtos {
            if let Some(task_definition) = task_definitions
                .iter()
                .find(|td| td.id == job_dto.task_definition_id)
            {
                job_dto.task_definition_name = Some(task_definition.name.clone());
            }
        }

        Ok(ListJobsResponse {
            jobs: job_dtos,
            total_count,
        })
    }

    async fn list_job_logs(
        &self,
        request: ListJobLogsRequest,
    ) -> errors::Result<ListJobLogsResponse> {
        let job_id = request.job_id;

        // job_id로 job 조회
        let mut jobs = self
            .job_repository
            .list_jobs(ListJobsParams {
                job_ids: vec![job_id],
                ..Default::default()
            })
            .await?;

        let Some(job) = jobs.pop() else {
            return Err(errors::Error::JobNotFound);
        };

        // 컨테이너 ID가 없으면 에러
        let Some(container_id) = &job.container_id else {
            return Err(errors::Error::ContainerIDNotFound);
        };

        // 컨테이너 정보 조회
        // TODO: inspect 시점을 컨테이너 생성 후로 조정할 필요 있음
        let container_info = self
            .container_repository
            .inspect_container(InspectContainerParams {
                container_id: container_id.clone(),
            })
            .await?;

        let lines = crate::utils::read_lines_range(
            &container_info.log_path,
            request.query.offset,
            request.query.limit,
        )?;

        let mut logs = vec![];

        for (i, line) in lines.into_iter().enumerate() {
            #[derive(serde::Deserialize)]
            struct LogLine {
                log: String,
                time: chrono::DateTime<chrono::Utc>,
            }

            let line = serde_json::from_str::<LogLine>(&line)?;
            logs.push(JobLogDto {
                index: request.query.offset + i,
                time: line.time,
                message: line.log,
            });
        }

        Ok(ListJobLogsResponse { logs })
    }

    async fn count_job_logs(
        &self,
        job_id: CountJobLogsRequest,
    ) -> errors::Result<CountJobLogsResponse> {
        let mut jobs = self
            .job_repository
            .list_jobs(ListJobsParams {
                job_ids: vec![job_id.job_id],
                ..Default::default()
            })
            .await?;

        let Some(job) = jobs.pop() else {
            return Err(errors::Error::JobNotFound);
        };

        let Some(container_id) = &job.container_id else {
            return Err(errors::Error::ContainerIDNotFound);
        };

        let container_info = self
            .container_repository
            .inspect_container(InspectContainerParams {
                container_id: container_id.clone(),
            })
            .await?;

        let log_count = crate::utils::count_lines(&container_info.log_path)?;

        Ok(CountJobLogsResponse { count: log_count })
    }
}
