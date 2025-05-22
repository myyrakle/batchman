use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::db::entities;

use super::{ListSchedulesParams, ScheduleRepository};

pub struct ScheduleSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl ScheduleRepository for ScheduleSeaOrmRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> anyhow::Result<Vec<entities::schedule::Model>> {
        let mut query = entities::schedule::Entity::find();

        if !params.schedule_ids.is_empty() {
            query = query.filter(entities::schedule::Column::Id.is_in(params.schedule_ids));
        }

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }

        let schedules = query.all(&self.connection).await?;

        Ok(schedules)
    }

    async fn create_schedule(&self, params: super::CreateScheduleParams) -> anyhow::Result<i64> {
        let schedule = entities::schedule::ActiveModel {
            name: sea_orm::Set(params.name),
            job_name: sea_orm::Set(params.job_name),
            cron_expression: sea_orm::Set(params.cron_expression),
            task_definition_id: sea_orm::Set(params.task_definition_id),
            command: sea_orm::Set(params.command),
            timezone: sea_orm::Set(params.timezone),
            timezone_offset: sea_orm::Set(params.timezone_offset),
            created_at: sea_orm::Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let schedule = schedule.insert(&self.connection).await?;

        Ok(schedule.id)
    }
}

impl ScheduleSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
