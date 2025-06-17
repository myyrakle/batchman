use std::sync::Arc;

use crate::{
    domain::task_definition::{
        dao::CountTaskDefinitionsParams,
        dto::{ListTaskDefinitionsItem, ListTaskDefinitionsResponse},
    },
    errors,
};

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
    ) -> errors::Result<i64> {
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

                // 기존 작업정의의 is_latest 를 false 로 변경 (TODO: Transaction 처리 필요)
                self.task_definition_repository
                    .patch_task_definition(PatchTaskDefinitionParams {
                        task_definition_id: task_definitions[0].id,
                        is_latest: Some(false),
                        ..Default::default()
                    })
                    .await?;
            }
        }

        let task_definition_id = self
            .task_definition_repository
            .create_task_definition(CreateTaskDefinitionParams {
                name: request.request_body.name,
                description: request.request_body.description,
                version,
                image: request.request_body.image,
                command: request.request_body.command,
                args: request.request_body.args,
                env: request.request_body.env,
                memory_limit: request.request_body.memory_limit,
                cpu_limit: request.request_body.cpu_limit,
            })
            .await?;

        Ok(task_definition_id)
    }

    async fn patch_task_definition(&self, request: PatchDefinitionRequest) -> errors::Result<()> {
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
            return Err(errors::Error::TaskDefinitionNotFound);
        }

        let _ = self
            .task_definition_repository
            .patch_task_definition(PatchTaskDefinitionParams {
                task_definition_id: request.task_definition_id,
                description: request.request.description,
                image: request.request.image,
                command: request.request.command,
                args: request.request.args,
                env: request.request.env,
                memory_limit: request.request.memory_limit,
                cpu_limit: request.request.cpu_limit,
                enabled: request.request.enabled,
                ..Default::default()
            })
            .await;

        Ok(())
    }

    async fn delete_task_definition(&self, params: DeleteDefinitionRequest) -> errors::Result<()> {
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
    ) -> errors::Result<ListTaskDefinitionsResponse> {
        let limit = params.query.page_size;
        let offset = (params.query.page_number - 1) * params.query.page_size;

        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: match params.query.task_definition_id {
                    Some(task_definition_id) => vec![task_definition_id],
                    None => vec![],
                },
                name: params.query.name.clone(),
                contains_name: params.query.contains_name.clone(),
                is_latest: params.query.is_latest_only,
                limit: Some(limit),
                offset: Some(offset),
                ..Default::default()
            })
            .await?;

        let total_count = self
            .task_definition_repository
            .count_task_definitions(CountTaskDefinitionsParams {
                name: params.query.name,
                contains_name: params.query.contains_name,
            })
            .await?;

        let response = ListTaskDefinitionsResponse {
            task_definitions: task_definitions
                .into_iter()
                .map(ListTaskDefinitionsItem::from)
                .collect(),
            total_count,
        };

        Ok(response)
    }
}
