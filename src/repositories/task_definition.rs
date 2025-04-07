use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{db::entities, routes::task_definitions};

use super::{ListTaskDefinitionsParams, TaskDefinitionRepository};

pub struct TaskDefinitionSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl TaskDefinitionRepository for TaskDefinitionSeaOrmRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>> {
        let mut find_query = entities::task_definition::Entity::find();

        if let Some(name) = params.name {
            find_query = find_query.filter(entities::task_definition::Column::Name.eq(name));
        }

        if let Some(order_by_desc) = params.order_by_desc {
            find_query = find_query.order_by_desc(order_by_desc);
        }

        if let Some(limit) = params.limit {
            find_query = find_query.limit(limit);
        }

        let task_definitions = find_query.all(&self.connection).await?;

        Ok(task_definitions)
    }
}

impl TaskDefinitionSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
