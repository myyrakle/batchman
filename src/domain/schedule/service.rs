use std::sync::Arc;

use crate::{
    domain::task_definition::{TaskDefinitionRepository, dao::ListTaskDefinitionsParams},
    errors,
    types::cron::CronExpression,
};

use super::{
    ScheduleRepository, ScheduleService,
    dao::{CreateScheduleParams, ListSchedulesParams, PatchScheduleParams},
    dto::{CreateSchduleRequest, ListSchedulesRequest, PatchScheduleRequest},
    entities,
};

pub struct ScheduleServiceImpl {
    pub schedule_repository: Arc<dyn ScheduleRepository + Send + Sync>,
    pub task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
}

impl ScheduleServiceImpl {
    pub fn new(
        schedule_repository: Arc<dyn ScheduleRepository + Send + Sync>,
        task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
    ) -> Self {
        Self {
            schedule_repository,
            task_definition_repository,
        }
    }
}

#[async_trait::async_trait]
impl ScheduleService for ScheduleServiceImpl {
    async fn create_schdule(&self, request: CreateSchduleRequest) -> errors::Result<i64> {
        if let Err(error) = CronExpression::parse(request.request_body.cron_expression.as_str()) {
            return Err(errors::Error::CronExpressionIsInvalid(error.to_string()));
        }

        let task_definitions = self
            .task_definition_repository
            .list_task_definitions(ListTaskDefinitionsParams {
                task_definition_ids: vec![request.request_body.task_definition_id],
                ..Default::default()
            })
            .await?;

        if task_definitions.is_empty() {
            return Err(errors::Error::TaskDefinitionNotFound);
        };

        let new_job_id = self
            .schedule_repository
            .create_schedule(CreateScheduleParams {
                name: request.request_body.name,
                job_name: request.request_body.job_name,
                cron_expression: request.request_body.cron_expression,
                task_definition_id: request.request_body.task_definition_id,
                command: request.request_body.command,
                timezone: request.request_body.timezone,
                timezone_offset: request.request_body.timezone_offset,
                enabled: request.request_body.enabled,
            })
            .await?;

        Ok(new_job_id)
    }

    async fn patch_schedule(&self, request: PatchScheduleRequest) -> errors::Result<()> {
        if let Some(cron_expression) = &request.body.cron_expression {
            // Validate the cron expression
            if let Err(error) = CronExpression::parse(cron_expression.as_str()) {
                return Err(errors::Error::CronExpressionIsInvalid(error.to_string()));
            }
        }

        // Check if schedule exists
        let schedules = self
            .schedule_repository
            .list_schedules(ListSchedulesParams {
                schedule_ids: vec![request.schedule_id],
                limit: Some(1),
                ..Default::default()
            })
            .await?;

        if schedules.is_empty() {
            return Err(errors::Error::ScheduleNotFound);
        }

        let params = PatchScheduleParams {
            schedule_id: request.schedule_id,
            name: request.body.name,
            job_name: request.body.job_name,
            cron_expression: request.body.cron_expression,
            task_definition_id: request.body.task_definition_id,
            command: request.body.command,
            timezone: request.body.timezone,
            timezone_offset: request.body.timezone_offset,
            enabled: request.body.enabled,
        };

        self.schedule_repository.patch_schedule(params).await?;

        Ok(())
    }

    async fn delete_schedule(&self, schedule_id: i64) -> errors::Result<()> {
        self.schedule_repository
            .delete_schedule(schedule_id)
            .await?;

        Ok(())
    }

    async fn list_schedules(
        &self,
        request: ListSchedulesRequest,
    ) -> errors::Result<Vec<entities::schedule::Model>> {
        let result = self
            .schedule_repository
            .list_schedules(ListSchedulesParams {
                enabled: request.query.enabled,
                name: request.query.name.clone(),
                contains_name: request.query.contains_name.clone(),
                schedule_ids: match request.query.schedule_id {
                    Some(schedule_id) => vec![schedule_id],
                    None => vec![],
                },
                ..Default::default()
            })
            .await?;

        Ok(result)
    }
}
