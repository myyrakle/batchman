use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::db::entities;

#[derive(Debug, Clone)]
pub struct ListTaskDefinitionsParams<'a> {
    pub connection: &'a DatabaseConnection,
    pub task_definition_id: Option<i64>,
    pub contains_name: Option<String>,
    pub name: Option<String>,
}

pub async fn list_task_definitions(
    params: ListTaskDefinitionsParams<'_>,
) -> anyhow::Result<Vec<entities::task_definition::Model>> {
    let mut find_query = entities::task_definition::Entity::find();

    if let Some(task_definition_id) = params.task_definition_id {
        find_query =
            find_query.filter(entities::task_definition::Column::Id.eq(task_definition_id));
    }

    if let Some(contains_name) = &params.contains_name {
        find_query = find_query
            .filter(entities::task_definition::Column::Name.contains(contains_name.to_string()));
    }

    if let Some(name) = &params.name {
        find_query =
            find_query.filter(entities::task_definition::Column::Name.eq(name.to_string()));
    }

    let task_definitions = find_query.all(params.connection).await?;

    Ok(task_definitions)
}
