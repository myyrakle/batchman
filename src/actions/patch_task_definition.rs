use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect,
};

use crate::{
    context::SharedContext,
    db::entities::{self, task_definition},
    repositories::ListTaskDefinitionsParams,
    routes::task_definitions::PatchTaskDefinitionBody,
};

#[derive(Debug, Clone)]
pub struct PatchDefinitionRequest<'a> {
    pub connection: &'a DatabaseConnection,
    pub task_definition_id: i64,
    pub request: PatchTaskDefinitionBody,
}

pub async fn patch_task_definition(
    context: SharedContext,
    params: PatchDefinitionRequest<'_>,
) -> anyhow::Result<()> {
    // version이 없다면 동일한 이름의 task definition이 있는지 확인

    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![params.task_definition_id],
            limit: Some(1),
            ..Default::default()
        })
        .await?;

    if task_definitions.is_empty() {
        return Err(anyhow::anyhow!("Task definition not found"));
    }

    let task_definition = task_definitions[0].clone();

    let mut task_definition = task_definition.into_active_model();

    if let Some(args) = params.request.args {
        task_definition.args = Set(Some(args));
    }

    if let Some(command) = params.request.command {
        task_definition.command = Set(Some(command));
    }

    if let Some(cpu_limit) = params.request.cpu_limit {
        task_definition.cpu_limit = Set(Some(cpu_limit));
    }

    if let Some(memory_limit) = params.request.memory_limit {
        task_definition.memory_limit = Set(Some(memory_limit));
    }

    if let Some(env) = params.request.env {
        task_definition.env = Set(Some(env));
    }

    if let Some(image) = params.request.image {
        task_definition.image = Set(image);
    }

    task_definition.update(params.connection).await?;

    Ok(())
}
