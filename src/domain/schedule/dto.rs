use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::domain::schedule::entities;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateScheduleBody {
    pub name: String,                 // schedule name
    pub job_name: String,             // job name
    pub cron_expression: String,      // cron expression
    pub task_definition_id: i64,      // task definition id
    pub command: Option<String>,      // docker run command
    pub timezone: Option<String>,     // timezone text (example: "Asia/Seoul")
    pub timezone_offset: Option<i32>, // timezone offset (in minutes) (example: 540=9:00 for "Asia/Seoul")
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CreateSchduleRequest {
    pub request_body: CreateScheduleBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PatchScheduleBody {
    pub name: Option<String>,
    pub job_name: Option<String>,
    pub cron_expression: Option<String>,
    pub task_definition_id: Option<i64>,
    pub command: Option<String>,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i32>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PatchScheduleRequest {
    pub schedule_id: i64,
    pub body: PatchScheduleBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListSchedulesQuery {
    pub schedule_id: Option<i64>,      // schedule id
    pub contains_name: Option<String>, // name contains text
    pub name: Option<String>,          // exact name
    pub enabled: Option<bool>,         // enabled status
}

#[derive(Debug, Clone)]
pub struct ListSchedulesRequest {
    pub query: ListSchedulesQuery,
}

#[derive(Serialize)]
pub struct ListSchedulesItem {
    pub id: i64,
    pub name: String,
    pub job_name: String,
    pub cron_expression: String,
    pub task_definition_id: i64,
    pub command: Option<String>,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i32>,
    pub enabled: bool,
    pub created_at: chrono::DateTime<Utc>,
}

impl From<entities::schedule::Model> for ListSchedulesItem {
    fn from(schedule: entities::schedule::Model) -> Self {
        ListSchedulesItem {
            id: schedule.id,
            name: schedule.name,
            job_name: schedule.job_name,
            cron_expression: schedule.cron_expression,
            task_definition_id: schedule.task_definition_id,
            command: schedule.command,
            timezone: schedule.timezone,
            timezone_offset: schedule.timezone_offset,
            enabled: schedule.enabled,
            created_at: schedule.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct ListSchedulesResponse {
    pub schedules: Vec<ListSchedulesItem>,
}
