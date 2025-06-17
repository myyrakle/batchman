use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    domain::task_definition::{
        TaskDefinitionRepository,
        dao::{
            CountTaskDefinitionsParams, CreateTaskDefinitionParams, DeleteTaskDefinitionParams,
            ListTaskDefinitionsParams, PatchTaskDefinitionParams,
        },
        entities,
    },
    errors,
};

pub struct TaskDefinitionSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl TaskDefinitionRepository for TaskDefinitionSeaOrmRepository {
    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsParams,
    ) -> crate::errors::Result<Vec<entities::task_definition::Model>> {
        let mut find_query = entities::task_definition::Entity::find();

        if !params.task_definition_ids.is_empty() {
            find_query = find_query
                .filter(entities::task_definition::Column::Id.is_in(params.task_definition_ids));
        }

        if let Some(name) = params.name {
            find_query = find_query.filter(entities::task_definition::Column::Name.eq(name));
        }

        if let Some(contains_name) = params.contains_name {
            find_query =
                find_query.filter(entities::task_definition::Column::Name.contains(contains_name));
        }

        if let Some(order_by_desc) = params.order_by_desc {
            find_query = find_query.order_by_desc(order_by_desc);
        }

        if let Some(limit) = params.limit {
            find_query = find_query.limit(limit);
        }

        if let Some(offset) = params.offset {
            find_query = find_query.offset(offset);
        }

        let task_definitions = find_query.all(&self.connection).await?;

        Ok(task_definitions)
    }

    async fn count_task_definitions(
        &self,
        params: CountTaskDefinitionsParams,
    ) -> errors::Result<u64> {
        let mut find_query = entities::task_definition::Entity::find();

        if let Some(name) = params.name {
            find_query = find_query.filter(entities::task_definition::Column::Name.eq(name));
        }

        if let Some(contains_name) = params.contains_name {
            find_query =
                find_query.filter(entities::task_definition::Column::Name.contains(contains_name));
        }

        let count = find_query.count(&self.connection).await?;

        Ok(count)
    }

    async fn create_task_definition(
        &self,
        params: CreateTaskDefinitionParams,
    ) -> crate::errors::Result<i64> {
        let new_definition = entities::task_definition::ActiveModel {
            id: NotSet,
            name: Set(params.name),
            version: Set(params.version),
            image: Set(params.image),
            command: Set(params.command),
            args: Set(params.args),
            env: Set(params.env),
            memory_limit: Set(params.memory_limit),
            cpu_limit: Set(params.cpu_limit),
            description: Set(params.description),
            created_at: Set(chrono::Utc::now()),
            enabled: Set(true),
            is_latest: Set(true),
        };

        let saved = new_definition.insert(&self.connection).await?;

        Ok(saved.id)
    }

    async fn patch_task_definition(
        &self,
        params: PatchTaskDefinitionParams,
    ) -> crate::errors::Result<()> {
        let task_definition =
            entities::task_definition::Entity::find_by_id(params.task_definition_id)
                .one(&self.connection)
                .await?
                .ok_or_else(|| errors::Error::TaskDefinitionNotFound)?;

        let mut model = task_definition.into_active_model();

        if let Some(name) = params.name {
            model.name = Set(name);
        }

        if let Some(description) = params.description {
            model.description = Set(description);
        }

        if let Some(version) = params.version {
            model.version = Set(version);
        }

        if let Some(image) = params.image {
            model.image = Set(image);
        }

        if let Some(command) = params.command {
            model.command = Set(Some(command));
        }

        if let Some(args) = params.args {
            model.args = Set(Some(args));
        }

        if let Some(env) = params.env {
            model.env = Set(Some(env));
        }

        if let Some(memory_limit) = params.memory_limit {
            model.memory_limit = Set(Some(memory_limit));
        }

        if let Some(cpu_limit) = params.cpu_limit {
            model.cpu_limit = Set(Some(cpu_limit));
        }

        let _ = model.update(&self.connection).await?;

        Ok(())
    }

    async fn delete_task_definition(
        &self,
        params: DeleteTaskDefinitionParams,
    ) -> crate::errors::Result<()> {
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
