use dao::{
    InspectContainerParams, InspectContainerResult, KillContainerParams, RemoveContainerParams,
    RunContainerParams, RunContainerResult, StopContainerParams,
};
use serde::Serialize;

use crate::errors;
use sea_orm::entity::prelude::*;

pub mod dao;
pub mod repository;

#[derive(Serialize, EnumIter, DeriveActiveEnum, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(50))")]
pub enum ContainerType {
    #[sea_orm(string_value = "Docker")]
    #[default]
    Docker,
}

#[async_trait::async_trait]
pub trait ContainerRepository {
    async fn inspect_container(
        &self,
        params: InspectContainerParams,
    ) -> errors::Result<InspectContainerResult>;

    async fn run_container(
        &self,
        task_definition: RunContainerParams,
    ) -> errors::Result<RunContainerResult>;

    async fn kill_container(&self, params: KillContainerParams) -> errors::Result<()>;

    async fn stop_container(&self, params: StopContainerParams) -> errors::Result<()>;

    async fn remove_container(&self, params: RemoveContainerParams) -> errors::Result<()>;
}
