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

#[derive(Debug, Default)]
pub struct ListSchedulesParams {
    pub schedule_ids: Vec<i64>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub contains_name: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Default)]
pub struct CreateScheduleParams {
    pub name: String,                 // schedule name
    pub job_name: String,             // job name
    pub cron_expression: String,      // cron expression
    pub task_definition_id: i64,      // task definition id
    pub command: Option<String>,      // docker run command
    pub timezone: Option<String>,     // timezone text (example: "Asia/Seoul")
    pub timezone_offset: Option<i32>, // timezone offset (in minutes) (example: 540=9:00 for "Asia/Seoul")
    pub enabled: bool,                // schedule enabled
}

#[derive(Debug, Clone, Default)]
pub struct PatchScheduleParams {
    pub schedule_id: i64,
    pub name: Option<String>,
    pub job_name: Option<String>,
    pub cron_expression: Option<String>,
    pub task_definition_id: Option<i64>,
    pub command: Option<String>,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i32>,
    pub enabled: Option<bool>,
}
