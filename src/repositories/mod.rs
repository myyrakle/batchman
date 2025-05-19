use chrono::Utc;

use crate::db::entities::{self, job::JobStatus};

pub mod job;
pub mod schedule;
pub mod task_definition;

#[derive(Debug, Default)]
pub struct ListTaskDefinitionsParams {
    pub task_definition_ids: Vec<i64>,
    pub name: Option<String>,
    pub contains_name: Option<String>,
    pub limit: Option<u64>,
    pub order_by_desc: Option<entities::task_definition::Column>,
}

#[derive(Debug)]
pub struct CreateTaskDefinitionParams {
    pub name: String, // task name
    pub version: i64, // task version

    pub image: String,           // docker image
    pub command: Option<String>, // docker run command
    pub args: Option<String>,    // docker run arguments
    pub env: Option<String>,     // environment variables

    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

#[derive(Debug, Default)]
pub struct PatchTaskDefinitionParams {
    pub task_definition_id: i64,
    pub name: Option<String>,
    pub version: Option<i64>,
    pub image: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,
    pub memory_limit: Option<u32>,
    pub cpu_limit: Option<u32>,
}

#[derive(Debug)]
pub struct DeleteTaskDefinitionParams {
    pub task_definition_id: i64,
}

#[async_trait::async_trait]
pub trait TaskDefinitionRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>>;

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> anyhow::Result<i64>;

    async fn patch_task_definition(&self, params: PatchTaskDefinitionParams) -> anyhow::Result<()>;

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
pub struct CreateJobParams {
    pub name: String,                               // job name
    pub task_definition_id: i64,                    // task definition id
    pub status: JobStatus,                          // job status
    pub submited_at: Option<chrono::DateTime<Utc>>, // job submited time
    pub started_at: Option<chrono::DateTime<Utc>>,  // job started time
    pub finished_at: Option<chrono::DateTime<Utc>>, // job finished time
    pub container_id: Option<String>,               // batch container id (docker container id)
    pub exit_code: Option<i32>,                     // batch exit code
    pub error_message: Option<String>,              // batch error message
}

#[derive(Debug, Default)]
pub struct PatchJobParams {
    pub job_id: i64,                                // job id
    pub name: Option<String>,                       // job name
    pub task_definition_id: Option<i64>,            // task definition id
    pub status: Option<JobStatus>,                  // job status
    pub submited_at: Option<chrono::DateTime<Utc>>, // job submited time
    pub started_at: Option<chrono::DateTime<Utc>>,  // job started time
    pub finished_at: Option<chrono::DateTime<Utc>>, // job finished time
    pub container_id: Option<String>,               // batch container id (docker container id)
    pub exit_code: Option<i32>,                     // batch exit code
    pub error_message: Option<String>,              // batch error message
}

#[derive(Debug, Default)]
pub struct ListJobsParams {
    pub job_ids: Vec<i64>,
    pub statuses: Vec<JobStatus>,
    pub limit: Option<u64>,
}

#[async_trait::async_trait]
pub trait JobRepository {
    async fn list_jobs(&self, params: ListJobsParams) -> anyhow::Result<Vec<entities::job::Model>>;
    async fn create_job(&self, params: CreateJobParams) -> anyhow::Result<i64>;
    async fn patch_job(&self, params: PatchJobParams) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
pub struct ListSchedulesParams {
    pub schedule_ids: Vec<i64>,
    pub limit: Option<u64>,
}

#[async_trait::async_trait]
pub trait ScheduleRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> anyhow::Result<Vec<entities::schedule::Model>>;
}
