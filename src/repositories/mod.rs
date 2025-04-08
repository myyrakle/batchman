use crate::db::entities;

pub mod job;
pub mod schedule;
pub mod task_definition;

#[derive(Debug, Default)]
pub struct ListTaskDefinitionsParams {
    pub task_definition_ids: Vec<i64>,
    pub name: Option<String>,
    pub contains_name: Option<String>,
    pub limit: Option<u64>,
    pub order_by_desc: Option<entities::task_definition::Column>,
}

#[derive(Debug)]
pub struct CreateTaskDefinitionParams {
    pub name: String, // task name
    pub version: i64, // task version

    pub image: String,           // docker image
    pub command: Option<String>, // docker run command
    pub args: Option<String>,    // docker run arguments
    pub env: Option<String>,     // environment variables

    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

#[derive(Debug, Default)]
pub struct PatchTaskDefinitionParams {
    pub task_definition_id: i64,
    pub name: Option<String>,
    pub version: Option<i64>,
    pub image: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,
    pub memory_limit: Option<u32>,
    pub cpu_limit: Option<u32>,
}

#[derive(Debug)]
pub struct DeleteTaskDefinitionParams {
    pub task_definition_id: i64,
}

#[async_trait::async_trait]
pub trait TaskDefinitionRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>>;

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> anyhow::Result<i64>;

    async fn patch_task_definition(&self, params: PatchTaskDefinitionParams) -> anyhow::Result<()>;

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> anyhow::Result<()>;
}

pub trait JobRepository {}

pub trait ScheduleRepository {}
