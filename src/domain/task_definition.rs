pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod service;

use dao::*;
use dto::CreateDefinitionRequest;

#[async_trait::async_trait]
pub trait TaskDefinitionRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>>;

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> anyhow::Result<i64>;

    async fn patch_task_definition(&self, params: PatchTaskDefinitionParams) -> anyhow::Result<()>;

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait TaskDefinitionService {
    async fn create_task_definition(&self, request: CreateDefinitionRequest)
    -> anyhow::Result<i64>;
}
