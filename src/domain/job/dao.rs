use chrono::Utc;

use super::entities::job::JobStatus;

#[derive(Debug, Default)]
pub struct CreateJobParams {
    pub name: String,                                    // job name
    pub task_definition_id: i64,                         // task definition id
    pub status: JobStatus,                               // job status
    pub submited_at: Option<chrono::DateTime<Utc>>,      // job submited time
    pub started_at: Option<chrono::DateTime<Utc>>,       // job started time
    pub finished_at: Option<chrono::DateTime<Utc>>,      // job finished time
    pub container_id: Option<String>,                    // batch container id (docker container id)
    pub exit_code: Option<i32>,                          // batch exit code
    pub error_message: Option<String>,                   // batch error message
    pub log_expire_after: Option<chrono::DateTime<Utc>>, // log expire time
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
    pub offset: Option<u64>,
    pub contains_name: Option<String>,
}
