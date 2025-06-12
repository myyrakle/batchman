use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTaskDefinitionBody {
    pub name: String,              // task name
    pub image: String,             // docker image
    pub command: Option<String>,   // docker run command
    pub args: Option<String>,      // docker run arguments
    pub env: Option<String>,       // environment variables
    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

#[derive(Debug, Clone)]
pub struct CreateDefinitionRequest {
    pub request_body: CreateTaskDefinitionBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PatchTaskDefinitionBody {
    pub image: Option<String>,     // docker image
    pub command: Option<String>,   // docker run command
    pub args: Option<String>,      // docker run arguments
    pub env: Option<String>,       // environment variables
    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

#[derive(Debug, Clone)]
pub struct PatchDefinitionRequest {
    pub task_definition_id: i64,
    pub request: PatchTaskDefinitionBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListTaskDefinitionsQuery {
    pub task_definition_id: Option<i64>,
    pub contains_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListTaskDefinitionsRequest {
    pub query: ListTaskDefinitionsQuery,
}

#[derive(Serialize)]
pub struct ListTaskDefinitionsItem {
    pub id: i64,      // primary key
    pub name: String, // task name
    pub version: i64, // task version

    pub image: String,           // docker image
    pub command: Option<String>, // docker run command
    pub args: Option<String>,    // docker run arguments
    pub env: Option<String>,     // environment variables

    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

#[derive(Serialize)]
pub struct ListTaskDefinitionsResponse {
    pub task_definitions: Vec<ListTaskDefinitionsItem>,
}

#[derive(Debug, Clone)]
pub struct DeleteDefinitionRequest {
    pub task_definition_id: i64,
}
