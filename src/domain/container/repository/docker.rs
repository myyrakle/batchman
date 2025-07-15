use std::process::Command;

use crate::{
    domain::container::{
        ContainerRepository,
        dao::{
            InspectContainerParams, InspectContainerResult, KillContainerParams,
            RemoveContainerParams, RunContainerParams, RunContainerResult, StopContainerParams,
        },
    },
    errors,
};

const DOCKER_PATH: &str = "docker";

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
        params: InspectContainerParams,
    ) -> errors::Result<InspectContainerResult> {
        let mut command = Command::new(DOCKER_PATH);

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

        let inspect_response: Vec<InspectContainerResult> =
            serde_json::from_str(&inspect_raw_response)?;

        if inspect_response.is_empty() {
            return Err(errors::Error::ContainerNotFound);
        }

        Ok(inspect_response[0].clone())
    }

    async fn run_container(
        &self,
        params: RunContainerParams,
    ) -> errors::Result<RunContainerResult> {
        let task_definition = params.task_definition;

        // Docker 컨테이너 실행
        let image_name = &task_definition.image;

        let mut command = Command::new(DOCKER_PATH);

        command.arg("run");
        command.arg("-d");

        // log 드라이버 설정
        // 참조: https://docs.docker.com/engine/logging/configure/
        command.arg("--log-driver");
        command.arg("json-file");

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
            let env_vars = env_vars.trim();
            if !env_vars.is_empty() {
                for env_var in env_vars.split(',') {
                    let env_var = env_var.trim();
                    if !env_var.is_empty() {
                        command.arg("-e");
                        command.arg(env_var);
                    }
                }
            }
        }

        command.arg(image_name);

        // CMD 설정
        if let Some(cmd) = &task_definition.command {
            let cmd = cmd.trim();
            if !cmd.is_empty() {
                // split 로직 고도화 필요
                let command_list = cmd
                    .split(' ')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                for cmd in command_list {
                    command.arg(cmd);
                }
            }
        }

        // Arguments 전달
        if let Some(args) = &task_definition.args {
            let args = args.trim();
            if !args.is_empty() {
                for arg in args.split(',') {
                    let arg = arg.trim();
                    if !arg.is_empty() {
                        command.arg(arg);
                    }
                }
            }
        }

        let output = command.output()?;

        if !output.status.success() {
            return Err(errors::Error::ContainerFailedToStart(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        Ok(RunContainerResult { container_id })
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
    async fn kill_container(&self, params: KillContainerParams) -> errors::Result<()> {
        let mut command = Command::new(DOCKER_PATH);

        command.arg("kill");
        command.arg(&params.container_id);

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
        let mut command = Command::new(DOCKER_PATH);

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
        self.kill_container(KillContainerParams {
            container_id: params.container_id,
        })
        .await?;

        Ok(())
    }

    async fn remove_container(&self, params: RemoveContainerParams) -> errors::Result<()> {
        let mut command = Command::new(DOCKER_PATH);

        command.arg("rm");

        if params.force {
            command.arg("--force");
        }

        if params.remove_volumes {
            command.arg("--volumes");
        }

        if params.remove_links {
            command.arg("--link");
        }

        command.arg(&params.container_id);

        let output = command.output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);

            if error.contains("No such container") {
                return Err(errors::Error::ContainerNotFound);
            }

            return Err(errors::Error::ContainerFailedToRemove(error.to_string()));
        }

        Ok(())
    }
}
