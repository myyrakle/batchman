use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{errors, types::cron::CronExpression};

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

impl TryFrom<Model> for ScheduleWithStates {
    type Error = errors::Error;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        let cron_expression = CronExpression::parse(&model.cron_expression)?;

        Ok(ScheduleWithStates {
            model,
            cron_expression,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ScheduleWithStates {
    pub model: Model,
    pub cron_expression: CronExpression,
}

impl ScheduleWithStates {
    pub fn is_time_to_trigger(&self, _now: &chrono::DateTime<Utc>) -> bool {
        return false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
