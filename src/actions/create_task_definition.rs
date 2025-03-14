use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, QuerySelect,
};

use crate::{db::entities, routes::task_definitions::CreateTaskDefinitionBody};

#[derive(Debug, Clone)]
pub struct CreateDefinitionParams<'a> {
    pub connection: &'a DatabaseConnection,
    pub request: CreateTaskDefinitionBody,
}

pub async fn create_task_definition(params: CreateDefinitionParams<'_>) -> anyhow::Result<u64> {
    // version이 없다면 동일한 이름의 task definition이 있는지 확인

    let mut version = 1;

    {
        let task_definitions = entities::task_definition::Entity::find()
            .filter(entities::task_definition::Column::Name.eq(&params.request.name))
            .order_by_desc(entities::task_definition::Column::Version)
            .limit(1)
            .all(params.connection)
            .await?;

        if task_definitions.len() > 0 {
            version = task_definitions[0].version + 1;
        }
    }

    let new_definition = entities::task_definition::Model {
        id: 0,
        name: params.request.name,
        version: version,
        image: params.request.image,
        command: params.request.command,
        args: params.request.args,
        env: params.request.env,
        memory_limit: params.request.memory_limit,
        cpu_limit: params.request.cpu_limit,
    };

    let saved = new_definition
        .into_active_model()
        .save(params.connection)
        .await?;

    Ok(saved.id.unwrap())
}
