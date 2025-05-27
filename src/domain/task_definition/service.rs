use std::sync::Arc;

use super::{
    TaskDefinitionRepository,
    dao::{
        CreateTaskDefinitionParams, DeleteTaskDefinitionParams, ListTaskDefinitionsParams,
        PatchTaskDefinitionParams,
    },
    dto::{
        CreateDefinitionRequest, DeleteDefinitionRequest, ListTaskDefinitionsRequest,
        PatchDefinitionRequest,
    },
    entities,
};

pub struct TaskDefinitionServiceImpl {
    pub task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
}

impl TaskDefinitionServiceImpl {
    pub fn new(
        task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
    ) -> Self {
        Self {
            task_definition_repository,
        }
    }
}

#[async_trait::async_trait]
impl super::TaskDefinitionService for TaskDefinitionServiceImpl {
    async fn create_task_definition(
        &self,
        request: CreateDefinitionRequest,
    ) -> anyhow::Result<i64> {
        // version이 없다면 동일한 이름의 task definition이 있는지 확인

        let mut version = 1;

        {
            let task_definitions = self
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

        let task_definition_id = self
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

    async fn patch_task_definition(&self, request: PatchDefinitionRequest) -> anyhow::Result<()> {
        // version이 없다면 동일한 이름의 task definition이 있는지 확인

        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: vec![request.task_definition_id],
                limit: Some(1),
                ..Default::default()
            })
            .await?;

        if task_definitions.is_empty() {
            return Err(anyhow::anyhow!("Task definition not found"));
        }

        let _ = self
            .task_definition_repository
            .patch_task_definition(PatchTaskDefinitionParams {
                task_definition_id: request.task_definition_id,
                image: request.request.image,
                command: request.request.command,
                args: request.request.args,
                env: request.request.env,
                memory_limit: request.request.memory_limit,
                cpu_limit: request.request.cpu_limit,
                ..Default::default()
            })
            .await;

        Ok(())
    }

    async fn delete_task_definition(&self, params: DeleteDefinitionRequest) -> anyhow::Result<()> {
        self.task_definition_repository
            .delete_task_definition(DeleteTaskDefinitionParams {
                task_definition_id: params.task_definition_id,
            })
            .await?;

        Ok(())
    }

    async fn list_task_definitions(
        &self,
        params: ListTaskDefinitionsRequest,
    ) -> anyhow::Result<Vec<entities::task_definition::Model>> {
        let task_definitions = self
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
}
