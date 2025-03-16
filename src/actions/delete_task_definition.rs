use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect};

use crate::db::entities;

#[derive(Debug, Clone)]
pub struct DeleteDefinitionParams<'a> {
    pub connection: &'a DatabaseConnection,
    pub task_definition_id: i64,
}

pub async fn delete_task_definition(params: DeleteDefinitionParams<'_>) -> anyhow::Result<()> {
    let task_definition = entities::task_definition::Entity::find()
        .filter(entities::task_definition::Column::Id.eq(params.task_definition_id))
        .limit(1)
        .one(params.connection)
        .await?;

    let Some(task_definition) = task_definition else {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    task_definition.delete(params.connection).await?;

    Ok(())
}
