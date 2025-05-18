use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

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
}

impl ScheduleSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
