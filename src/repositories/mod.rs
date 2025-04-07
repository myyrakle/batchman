use sea_orm::prelude::async_trait::async_trait;

use crate::db::entities;

pub mod job;
pub mod schedule;
pub mod task_definition;

#[derive(Debug)]
pub struct ListTaskDefinitionsParams {
    pub name: Option<String>,
    pub limit: Option<u64>,
    pub order_by_desc: Option<entities::task_definition::Column>,
}

#[async_trait::async_trait]
pub trait TaskDefinitionRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>>;
}

pub trait JobRepository {}

pub trait ScheduleRepository {}
