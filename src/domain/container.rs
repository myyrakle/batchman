use dao::{ContainerInspectParams, ContainerInspectResult};

use crate::errors;

pub mod dao;
pub mod repository;

#[async_trait::async_trait]
pub trait ContainerRepository {
    async fn inspect_container(
        &self,
        params: ContainerInspectParams,
    ) -> errors::Result<ContainerInspectResult>;
}
