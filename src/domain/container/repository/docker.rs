use std::process::Command;

use crate::{
    domain::container::{
        ContainerRepository,
        dao::{
            ContainerInspectParams, ContainerInspectResult, ContainerRunParams, ContainerRunResult,
            StopContainerParams,
        },
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

    async fn run_container(
        &self,
        params: ContainerRunParams,
    ) -> errors::Result<ContainerRunResult> {
        let task_definition = params.task_definition;

        // Docker 컨테이너 실행
        let image_name = &task_definition.image;

        let mut command = Command::new("docker");

        command.arg("run");
        command.arg("-d");

        // 메모리 제한 설정
        if let Some(memory_limit) = task_definition.memory_limit {
            command.arg("--memory");
            command.arg(format!("{}m", memory_limit));
        }

        // CPU 제한 설정
        if let Some(cpu_limit) = task_definition.cpu_limit {
            command.arg("--cpu-shares");
            command.arg(format!("{}", cpu_limit));
        }

        // 환경 변수 설정
        if let Some(env_vars) = &task_definition.env {
            for env_var in env_vars.split(',') {
                command.arg("-e");
                command.arg(env_var);
            }
        }

        command.arg(image_name);

        // CMD 설정
        if let Some(cmd) = &task_definition.command {
            let command_list = serde_json::from_str::<Vec<String>>(cmd)?;

            for cmd in command_list {
                command.arg(cmd);
            }
        }

        // Arguments 전달
        if let Some(args) = &task_definition.args {
            for arg in args.split(',') {
                command.arg(arg);
            }
        }

        let output = command.output()?;

        if !output.status.success() {
            return Err(errors::Error::ContainerFailedToStart(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        Ok(ContainerRunResult { container_id })
    }

    /// Forcefully stops a Docker container by sending a SIGKILL signal
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container to stop
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the container was successfully stopped
    /// * `Err` with an error message if the operation failed
    ///
    async fn kill_container(&self, container_id: String) -> errors::Result<()> {
        let mut command = Command::new("docker");

        command.arg("kill");
        command.arg(container_id);

        let output = command.output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);

            if error.contains("No such container") {
                return Err(errors::Error::ContainerNotFound);
            }

            return Err(errors::Error::ContainerFailedToKill(error.to_string()));
        }

        Ok(())
    }

    /// Forcefully stops a Docker container with a timeout
    ///
    /// This function first attempts to gracefully stop the container with `docker stop`
    /// using the specified timeout. If that fails, it forcefully kills the container
    /// with `docker kill`.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container to stop
    /// * `timeout_seconds` - The timeout in seconds to wait for graceful shutdown
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the container was successfully stopped
    /// * `Err` with an error message if both stop and kill operations failed
    ///
    async fn stop_container(&self, params: StopContainerParams) -> errors::Result<()> {
        // First try to stop gracefully with timeout
        let mut command = Command::new("docker");

        command.arg("stop");
        command.arg("--time");
        command.arg(params.timeout_seconds.to_string());
        command.arg(&params.container_id);

        let output = command.output()?;

        // If stop succeeded, return success
        if output.status.success() {
            return Ok(());
        }

        // If stop failed for a reason other than "container not found", log the error
        let error = String::from_utf8_lossy(&output.stderr);
        if !error.contains("No such container") {
            eprintln!("Warning: Failed to gracefully stop container: {}", error);
        }

        // Fall back to kill
        self.kill_container(params.container_id).await?;

        Ok(())
    }
}
