use dao::{
    ContainerInspectParams, ContainerInspectResult, ContainerRunParams, ContainerRunResult,
    StopContainerParams,
};

use crate::errors;

pub mod dao;
pub mod repository;

#[async_trait::async_trait]
pub trait ContainerRepository {
    async fn inspect_container(
        &self,
        params: ContainerInspectParams,
    ) -> errors::Result<ContainerInspectResult>;

    async fn run_container(
        &self,
        task_definition: ContainerRunParams,
    ) -> errors::Result<ContainerRunResult>;

    async fn kill_container(&self, container_id: String) -> errors::Result<()>;

    async fn stop_container(&self, params: StopContainerParams) -> errors::Result<()>;
}
