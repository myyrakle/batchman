use crate::{
    context::SharedContext,
    domain::task_definition::{dao::DeleteTaskDefinitionParams, dto::DeleteDefinitionRequest},
};

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
