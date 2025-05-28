use std::process::Command;

use serde::Deserialize;

use crate::errors;

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerInspectResult {
    #[serde(rename = "State")]
    pub state: ContainerState,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ContainerState {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Running")]
    pub running: bool,
    #[serde(rename = "Paused")]
    pub paused: bool,
    #[serde(rename = "Restarting")]
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    #[serde(rename = "Dead")]
    pub dead: bool,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
}

pub fn inspect_container(container_id: &str) -> errors::Result<ContainerInspectResult> {
    let mut command = Command::new("docker");

    command.arg("inspect");
    command.arg(container_id);

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

    return Ok(inspect_response[0].clone());
}
