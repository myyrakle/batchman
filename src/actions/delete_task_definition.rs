use crate::{context::SharedContext, domain::task_definition::dao::DeleteTaskDefinitionParams};

#[derive(Debug, Clone)]
pub struct DeleteDefinitionRequest {
    pub task_definition_id: i64,
}

pub async fn delete_task_definition(
    context: SharedContext,
    params: DeleteDefinitionRequest,
) -> anyhow::Result<()> {
    context
        .task_definition_repository
        .delete_task_definition(DeleteTaskDefinitionParams {
            task_definition_id: params.task_definition_id,
        })
        .await?;

    Ok(())
}
