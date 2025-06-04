use dao::{
    InspectContainerParams, InspectContainerResult, KillContainerParams, RunContainerParams,
    RunContainerResult, StopContainerParams,
};

use crate::errors;

pub mod dao;
pub mod repository;

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
}
