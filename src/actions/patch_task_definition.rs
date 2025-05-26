use crate::{
    context::SharedContext,
    domain::task_definition::{
        dao::{ListTaskDefinitionsParams, PatchTaskDefinitionParams},
        dto::PatchTaskDefinitionBody,
    },
};

#[derive(Debug, Clone)]
pub struct PatchDefinitionRequest {
    pub task_definition_id: i64,
    pub request: PatchTaskDefinitionBody,
}

pub async fn patch_task_definition(
    context: SharedContext,
    request: PatchDefinitionRequest,
) -> anyhow::Result<()> {
    // version이 없다면 동일한 이름의 task definition이 있는지 확인

    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![request.task_definition_id],
            limit: Some(1),
            ..Default::default()
        })
        .await?;

    if task_definitions.is_empty() {
        return Err(anyhow::anyhow!("Task definition not found"));
    }

    let _ = context
        .task_definition_repository
        .patch_task_definition(PatchTaskDefinitionParams {
            task_definition_id: request.task_definition_id,
            image: request.request.image,
            command: request.request.command,
            args: request.request.args,
            env: request.request.env,
            memory_limit: request.request.memory_limit,
            cpu_limit: request.request.cpu_limit,
            ..Default::default()
        })
        .await;

    Ok(())
}
