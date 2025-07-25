use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::domain::container::ContainerType;

#[derive(Serialize, Debug, Clone, Default)]
pub enum JobStatusDto {
    #[default]
    Pending,
    Starting,
    Running,
    Finished,
    Failed,
}

impl From<super::entities::job::JobStatus> for JobStatusDto {
    fn from(status: super::entities::job::JobStatus) -> Self {
        match status {
            super::entities::job::JobStatus::Pending => JobStatusDto::Pending,
            super::entities::job::JobStatus::Starting => JobStatusDto::Starting,
            super::entities::job::JobStatus::Running => JobStatusDto::Running,
            super::entities::job::JobStatus::Finished => JobStatusDto::Finished,
            super::entities::job::JobStatus::Failed => JobStatusDto::Failed,
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct JobDto {
    pub id: i64,
    pub name: String,
    pub task_definition_id: i64,
    pub task_definition_name: Option<String>,
    pub status: JobStatusDto,
    pub submited_at: Option<chrono::DateTime<Utc>>,
    pub started_at: Option<chrono::DateTime<Utc>>,
    pub finished_at: Option<chrono::DateTime<Utc>>,
    pub container_type: ContainerType,
    pub container_id: Option<String>,
    pub exit_code: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub log_expire_after: Option<chrono::DateTime<Utc>>,
    pub log_expired: bool,
}

impl From<super::entities::job::Model> for JobDto {
    fn from(model: super::entities::job::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            task_definition_id: model.task_definition_id,
            status: model.status.into(),
            submited_at: model.submited_at,
            started_at: model.started_at,
            finished_at: model.finished_at,
            container_id: model.container_id,
            exit_code: model.exit_code,
            error_message: model.error_message,
            created_at: model.created_at,
            container_type: model.container_type,
            log_expire_after: model.log_expire_after,
            log_expired: model.log_expired,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SubmitJobBody {
    pub task_definition_id: i64,
    pub job_name: String,
    pub log_expire_after: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SubmitJobRequest {
    pub request_body: SubmitJobBody,
}

#[derive(Serialize, Debug, Clone)]
pub struct SubmitJobResponse {
    pub job_id: i64, // primary key of the job
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopJobBody {
    pub job_id: i64,
}

#[derive(Debug, Clone)]
pub struct StopJobRequest {
    pub request_body: StopJobBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListJobsQuery {
    pub page_number: Option<u64>,
    pub page_size: Option<u64>,
    pub job_id: Option<i64>,
    pub status: Option<String>,
    pub contains_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListJobsRequest {
    pub request_query: ListJobsQuery,
}

#[derive(Serialize, Debug, Clone)]
pub struct ListJobsResponse {
    pub jobs: Vec<JobDto>,
    pub total_count: u64,
}

#[derive(Debug, Clone)]
pub struct ListJobLogsRequest {
    pub job_id: i64,
    pub query: ListJobLogsQuery,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListJobLogsQuery {
    pub offset: usize,
    pub limit: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct JobLogDto {
    pub index: usize,
    pub time: chrono::DateTime<Utc>,
    pub message: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ListJobLogsResponse {
    pub logs: Vec<JobLogDto>,
}

#[derive(Debug, Clone)]
pub struct CountJobLogsRequest {
    pub job_id: i64,
}

#[derive(Serialize, Debug, Clone)]
pub struct CountJobLogsResponse {
    pub count: usize,
}
