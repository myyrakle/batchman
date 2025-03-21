use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerInspectResult {
    #[serde(rename = "State")]
    pub state: ContainerState,
}

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
    pub exit_code: i32,
    #[serde(rename = "StartedAt")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: chrono::DateTime<chrono::Utc>,
}

pub fn inspect_container(container_id: &str) -> anyhow::Result<ContainerInspectResult> {
    let mut command = Command::new("docker");

    command.arg("inspect");
    command.arg(container_id);

    let output = command.output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);

        if error.contains("No such object") {
            return Err(anyhow::anyhow!("Container not found"));
        }

        return Err(anyhow::anyhow!(
            "Failed to inspect Docker container: {error}"
        ));
    }

    let inspect_raw_response = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    let inspect_response: Vec<ContainerInspectResult> =
        serde_json::from_str(&inspect_raw_response)?;

    if inspect_response.is_empty() {
        return Err(anyhow::anyhow!("No container found"));
    }

    return Ok(inspect_response[0].clone());
}
