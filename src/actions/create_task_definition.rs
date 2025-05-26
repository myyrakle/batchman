use crate::{
    context::SharedContext,
    db::entities,
    domain::task_definition::dao::{CreateTaskDefinitionParams, ListTaskDefinitionsParams},
    routes::task_definitions::CreateTaskDefinitionBody,
};

#[derive(Debug, Clone)]
pub struct CreateDefinitionRequest {
    pub request_body: CreateTaskDefinitionBody,
}

pub async fn create_task_definition(
    context: SharedContext,
    request: CreateDefinitionRequest,
) -> anyhow::Result<i64> {
    // version이 없다면 동일한 이름의 task definition이 있는지 확인

    let mut version = 1;

    {
        let task_definitions = context
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                name: Some(request.request_body.name.clone()),
                limit: Some(1),
                order_by_desc: Some(entities::task_definition::Column::Version),
                ..Default::default()
            })
            .await?;

        if !task_definitions.is_empty() {
            version = task_definitions[0].version + 1;
        }
    }

    let task_definition_id = context
        .task_definition_repository
        .create_task_definition(CreateTaskDefinitionParams {
            name: request.request_body.name,
            version,
            image: request.request_body.image,
            command: request
                .request_body
                .command
                .map(|command| serde_json::to_string(&command).unwrap_or_default()),
            args: request.request_body.args,
            env: request.request_body.env,
            memory_limit: request.request_body.memory_limit,
            cpu_limit: request.request_body.cpu_limit,
        })
        .await?;

    Ok(task_definition_id)
}
