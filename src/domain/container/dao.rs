use serde::Deserialize;

use crate::domain;

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerInspectParams {
    pub container_id: String,
}

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

#[derive(Debug, Clone)]
pub struct ContainerRunParams {
    pub task_definition: domain::task_definition::entities::task_definition::Model,
}

#[derive(Debug, Clone)]
pub struct ContainerRunResult {
    pub container_id: String,
}

#[derive(Debug, Clone)]
pub struct StopContainerParams {
    pub container_id: String,
    pub timeout_seconds: u32,
}
