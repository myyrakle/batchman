use crate::{
    context::SharedContext, db::entities, repositories::ListTaskDefinitionsParams,
    routes::task_definitions::ListTaskDefinitionsQuery,
};

#[derive(Debug, Clone)]
pub struct ListTaskDefinitionsRequest {
    pub query: ListTaskDefinitionsQuery,
}

pub async fn list_task_definitions(
    context: SharedContext,
    params: ListTaskDefinitionsRequest,
) -> anyhow::Result<Vec<entities::task_definition::Model>> {
    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: match params.query.task_definition_id {
                Some(task_definition_id) => vec![task_definition_id],
                None => vec![],
            },
            name: params.query.name.clone(),
            contains_name: params.query.contains_name,
            ..Default::default()
        })
        .await?;

    Ok(task_definitions)
}
