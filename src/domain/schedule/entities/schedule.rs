use chrono::{Datelike, Timelike, Utc};
use sea_orm::entity::prelude::*;

use crate::{
    errors,
    types::cron::{CronExpression, CronExpressionField},
};

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
    pub fn is_time_to_trigger(&self, now: &chrono::DateTime<Utc>) -> bool {
        // 1. Check if the schedule is enabled
        if !self.model.enabled {
            return false;
        }

        // 2. Year Check
        match &self.cron_expression.year {
            None => {
                // OK
            }
            Some(CronExpressionField::All) => {
                // OK
            }
            Some(CronExpressionField::Elements(years)) => {
                let mut passed = false;

                for year in years {
                    if year.contains(now.year() as u32) {
                        // OK
                        passed = true;
                        break;
                    }
                }

                if !passed {
                    return false; // Not matched
                }
            }
        }

        // 3. Day of Week Check
        match &self.cron_expression.day_of_week {
            CronExpressionField::All => {
                // OK
            }
            CronExpressionField::Elements(days) => {
                let mut passed = false;

                for day in days {
                    if day.contains(now.weekday().num_days_from_sunday() as u32) {
                        // OK
                        passed = true;
                        break;
                    }
                }

                if !passed {
                    return false; // Not matched
                }
            }
        }

        // 4. Month Check
        match &self.cron_expression.month {
            CronExpressionField::All => {
                // OK
            }
            CronExpressionField::Elements(months) => {
                let mut passed = false;

                for month in months {
                    if month.contains(now.month() as u32) {
                        // OK
                        passed = true;
                        break;
                    }
                }

                if !passed {
                    return false; // Not matched
                }
            }
        }

        // 5. Day of Month Check
        match &self.cron_expression.day_of_month {
            CronExpressionField::All => {
                // OK
            }
            CronExpressionField::Elements(days) => {
                let mut passed = false;

                for day in days {
                    if day.contains(now.day() as u32) {
                        // OK
                        passed = true;
                        break;
                    }
                }

                if !passed {
                    return false; // Not matched
                }
            }
        }

        // 6. Hour Check
        match &self.cron_expression.hours {
            CronExpressionField::All => {
                // OK
            }
            CronExpressionField::Elements(hours) => {
                let mut passed = false;

                for hour in hours {
                    if hour.contains(now.hour()) {
                        // OK
                        passed = true;
                        break;
                    }
                }

                if !passed {
                    return false; // Not matched
                }
            }
        }

        match self.model.last_triggered_at {
            Some(_last_triggered_at) => {}
            None => {}
        }

        return false;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {}
