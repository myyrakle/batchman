use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::db::entities;

use super::{
    CreateTaskDefinitionParams, DeleteTaskDefinitionParams, ListTaskDefinitionsParams,
    TaskDefinitionRepository,
};

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

        if !params.task_definition_ids.is_empty() {
            find_query = find_query
                .filter(entities::task_definition::Column::Id.is_in(params.task_definition_ids));
        }

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

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> anyhow::Result<i64> {
        let new_definition = entities::task_definition::ActiveModel {
            id: NotSet,
            name: Set(params.name),
            version: Set(params.version),
            image: Set(params.image),
            command: Set(params
                .command
                .map(|command| serde_json::to_string(&command).unwrap_or_default())),
            args: Set(params.args),
            env: Set(params.env),
            memory_limit: Set(params.memory_limit),
            cpu_limit: Set(params.cpu_limit),
        };

        let saved = new_definition.insert(&self.connection).await?;

        Ok(saved.id)
    }

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> anyhow::Result<()> {
        let _ = entities::task_definition::Entity::delete_by_id(params.task_definition_id)
            .exec(&self.connection)
            .await?;

        Ok(())
    }
}

impl TaskDefinitionSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
