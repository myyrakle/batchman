use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};

use crate::{
    domain::schedule::{
        ScheduleRepository,
        dao::{CreateScheduleParams, ListSchedulesParams, PatchScheduleParams},
        entities,
    },
    errors,
};

pub struct ScheduleSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl ScheduleRepository for ScheduleSeaOrmRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> errors::Result<Vec<entities::schedule::Model>> {
        let mut query = entities::schedule::Entity::find();

        if !params.schedule_ids.is_empty() {
            query = query.filter(entities::schedule::Column::Id.is_in(params.schedule_ids));
        }

        if let Some(enabled) = params.enabled {
            query = query.filter(entities::schedule::Column::Enabled.eq(enabled));
        }

        if let Some(name) = params.name {
            query = query.filter(entities::schedule::Column::Name.eq(name));
        }

        if let Some(contains_name) = params.contains_name {
            query = query.filter(entities::schedule::Column::Name.contains(contains_name));
        }

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }

        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        let schedules = query.all(&self.connection).await?;

        Ok(schedules)
    }

    async fn count_schedules(&self, params: ListSchedulesParams) -> errors::Result<i64> {
        let mut query = entities::schedule::Entity::find();

        if !params.schedule_ids.is_empty() {
            query = query.filter(entities::schedule::Column::Id.is_in(params.schedule_ids));
        }

        if let Some(enabled) = params.enabled {
            query = query.filter(entities::schedule::Column::Enabled.eq(enabled));
        }

        if let Some(name) = params.name {
            query = query.filter(entities::schedule::Column::Name.eq(name));
        }

        if let Some(contains_name) = params.contains_name {
            query = query.filter(entities::schedule::Column::Name.contains(contains_name));
        }

        let count = query.count(&self.connection).await?;

        Ok(count as i64)
    }

    async fn create_schedule(&self, params: CreateScheduleParams) -> errors::Result<i64> {
        let schedule = entities::schedule::ActiveModel {
            name: sea_orm::Set(params.name),
            job_name: sea_orm::Set(params.job_name),
            cron_expression: sea_orm::Set(params.cron_expression),
            task_definition_id: sea_orm::Set(params.task_definition_id),
            command: sea_orm::Set(params.command),
            timezone: sea_orm::Set(params.timezone),
            timezone_offset: sea_orm::Set(params.timezone_offset),
            created_at: sea_orm::Set(chrono::Utc::now()),
            enabled: sea_orm::Set(params.enabled),
            ..Default::default()
        };

        let schedule = schedule.insert(&self.connection).await?;

        Ok(schedule.id)
    }

    async fn patch_schedule(&self, params: PatchScheduleParams) -> errors::Result<()> {
        let schedule_model = entities::schedule::Entity::find_by_id(params.schedule_id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| errors::Error::ScheduleNotFound)?;

        let mut schedule_active_model = schedule_model.into_active_model();

        if let Some(name) = params.name {
            schedule_active_model.name = Set(name);
        }

        if let Some(job_name) = params.job_name {
            schedule_active_model.job_name = Set(job_name);
        }

        if let Some(cron_expression) = params.cron_expression {
            schedule_active_model.cron_expression = Set(cron_expression);
        }

        if let Some(task_definition_id) = params.task_definition_id {
            schedule_active_model.task_definition_id = Set(task_definition_id);
        }

        if let Some(command) = params.command {
            schedule_active_model.command = Set(Some(command));
        }

        if let Some(timezone) = params.timezone {
            schedule_active_model.timezone = Set(Some(timezone));
        }

        if let Some(timezone_offset) = params.timezone_offset {
            schedule_active_model.timezone_offset = Set(Some(timezone_offset));
        }

        if let Some(last_triggered_at) = params.last_triggered_at {
            schedule_active_model.last_triggered_at = Set(Some(last_triggered_at));
        }

        schedule_active_model.update(&self.connection).await?;

        Ok(())
    }

    async fn delete_schedule(&self, schedule_id: i64) -> errors::Result<()> {
        let schedule = entities::schedule::Entity::find_by_id(schedule_id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| errors::Error::ScheduleNotFound)?;

        schedule.delete(&self.connection).await?;

        Ok(())
    }
}

impl ScheduleSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
