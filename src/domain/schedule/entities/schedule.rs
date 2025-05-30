use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "schedule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // primary key
    pub name: String,                                     // schedule name
    pub job_name: String,                                 // job name
    pub cron_expression: String,                          // cron expression
    pub task_definition_id: i64,                          // task definition id
    pub command: Option<String>,                          // docker run command
    pub timezone: Option<String>,                         // timezone text (example: "Asia/Seoul")
    pub timezone_offset: Option<i32>, // timezone offset (in minutes) (example: 540=9:00 for "Asia/Seoul")
    pub enabled: bool,                // schedule enabled status
    pub created_at: Option<chrono::DateTime<Utc>>, // job submited time
    pub last_triggered_at: Option<chrono::DateTime<Utc>>, // last triggered time
}

impl Model {
    pub fn is_time_to_trigger(&self, _now: &chrono::DateTime<Utc>) -> bool {
        // TODO: cron expression을 파싱해서 현재 시간과 비교

        return false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
