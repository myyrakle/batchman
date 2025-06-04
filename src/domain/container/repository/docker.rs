use std::process::Command;

use crate::{
    domain::container::{
        ContainerRepository,
        dao::{ContainerInspectParams, ContainerInspectResult},
    },
    errors,
};

#[derive(Debug, Clone)]
pub struct ContainerDockerRepository {}

impl ContainerDockerRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ContainerRepository for ContainerDockerRepository {
    async fn inspect_container(
        &self,
        params: ContainerInspectParams,
    ) -> errors::Result<ContainerInspectResult> {
        let mut command = Command::new("docker");

        command.arg("inspect");
        command.arg(params.container_id);

        let output = command.output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);

            if error.contains("No such object") {
                return Err(errors::Error::ContainerNotFound);
            }

            return Err(errors::Error::ContainerFailedToInspect(error.to_string()));
        }

        let inspect_raw_response = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        let inspect_response: Vec<ContainerInspectResult> =
            serde_json::from_str(&inspect_raw_response)?;

        if inspect_response.is_empty() {
            return Err(errors::Error::ContainerNotFound);
        }

        Ok(inspect_response[0].clone())
    }
}
