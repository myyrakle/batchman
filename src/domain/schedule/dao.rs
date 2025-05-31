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
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
}
