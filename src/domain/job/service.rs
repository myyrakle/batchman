use std::sync::Arc;

use crate::{
    docker::{self, run_container},
    domain::task_definition::{TaskDefinitionRepository, dao::ListTaskDefinitionsParams},
};

use super::{
    JobRepository, JobService,
    dao::{CreateJobParams, ListJobsParams, PatchJobParams},
    dto::{StopJobRequest, SubmitJobRequest},
    entities::{self, job::JobStatus},
};

pub struct JobServiceImpl {
    pub job_repository: Arc<dyn JobRepository + Send + Sync>,
    pub task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
}

impl JobServiceImpl {
    pub fn new(
        job_repository: Arc<dyn JobRepository + Send + Sync>,
        task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
    ) -> Self {
        Self {
            job_repository,
            task_definition_repository,
        }
    }
}

#[async_trait::async_trait]
impl JobService for JobServiceImpl {
    async fn submit_job(&self, params: SubmitJobRequest) -> anyhow::Result<i64> {
        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: vec![params.request_body.task_definition_id],
                ..Default::default()
            })
            .await?;

        if task_definitions.is_empty() {
            return Err(anyhow::anyhow!("Task definition not found"));
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

        Ok(new_job_id)
    }

    async fn stop_job(&self, params: StopJobRequest) -> anyhow::Result<()> {
        let job_id = params.request_body.job_id;

        let mut jobs = self
            .job_repository
            .list_jobs(ListJobsParams {
                job_ids: vec![job_id],
                ..Default::default()
            })
            .await?;

        let Some(job) = jobs.pop() else {
            return Err(anyhow::anyhow!("Job not found: {}", job_id));
        };

        if job.status == JobStatus::Finished {
            return Err(anyhow::anyhow!("Job already finished"));
        }

        if job.status == JobStatus::Failed {
            return Err(anyhow::anyhow!("Job already failed"));
        }

        let Some(container_id) = job.container_id.as_ref() else {
            return Err(anyhow::anyhow!("Job has no container ID"));
        };

        docker::stop::stop_container(container_id, 3)?;

        Ok(())
    }

    async fn run_pending_job(&self, pending_job: &entities::job::Model) -> anyhow::Result<()> {
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
            return Err(anyhow::anyhow!("Task definition not found"));
        };

        // 3. 컨테이너 실행
        let container_id = run_container(task_definition)?;

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
}
