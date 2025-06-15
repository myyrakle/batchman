pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod routes;
pub mod service;

use dao::*;
use dto::{
    CreateDefinitionRequest, DeleteDefinitionRequest, ListTaskDefinitionsRequest,
    PatchDefinitionRequest,
};

use crate::{domain::task_definition::dto::ListTaskDefinitionsResponse, errors};

#[async_trait::async_trait]
pub trait TaskDefinitionRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> errors::Result<Vec<entities::task_definition::Model>>;

    async fn count_task_definitions(
        &self,
        params: CountTaskDefinitionsParams,
    ) -> errors::Result<u64>;

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> errors::Result<i64>;

    async fn patch_task_definition(&self, params: PatchTaskDefinitionParams) -> errors::Result<()>;

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> errors::Result<()>;
}

#[async_trait::async_trait]
pub trait TaskDefinitionService {
    async fn create_task_definition(&self, request: CreateDefinitionRequest)
    -> errors::Result<i64>;

    async fn patch_task_definition(&self, request: PatchDefinitionRequest) -> errors::Result<()>;

    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsRequest,
    ) -> errors::Result<ListTaskDefinitionsResponse>;

    async fn delete_task_definition(&self, params: DeleteDefinitionRequest) -> errors::Result<()>;
}
