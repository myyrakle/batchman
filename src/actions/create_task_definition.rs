use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    context::SharedContext, db::entities, repositories::ListTaskDefinitionsParams,
    routes::task_definitions::CreateTaskDefinitionBody,
};

#[derive(Debug, Clone)]
pub struct CreateDefinitionRequest<'a> {
    pub connection: &'a DatabaseConnection,
    pub request: CreateTaskDefinitionBody,
}

pub async fn create_task_definition(
    context: SharedContext,
    params: CreateDefinitionRequest<'_>,
) -> anyhow::Result<i64> {
    // version이 없다면 동일한 이름의 task definition이 있는지 확인

    let mut version = 1;

    {
        let task_definitions = context
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                name: Some(params.request.name.clone()),
                limit: Some(1),
                order_by_desc: Some(entities::task_definition::Column::Version),
            })
            .await?;

        if !task_definitions.is_empty() {
            version = task_definitions[0].version + 1;
        }
    }

    let new_definition = entities::task_definition::ActiveModel {
        id: NotSet,
        name: Set(params.request.name),
        version: Set(version),
        image: Set(params.request.image),
        command: Set(params
            .request
            .command
            .map(|command| serde_json::to_string(&command).unwrap_or_default())),
        args: Set(params.request.args),
        env: Set(params.request.env),
        memory_limit: Set(params.request.memory_limit),
        cpu_limit: Set(params.request.cpu_limit),
    };

    let saved = new_definition.insert(params.connection).await?;

    Ok(saved.id)
}
