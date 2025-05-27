use std::sync::Arc;

use crate::{
    docker,
    domain::task_definition::{TaskDefinitionRepository, dao::ListTaskDefinitionsParams},
};

use super::{
    JobRepository, JobService,
    dao::{CreateJobParams, ListJobsParams},
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
}
