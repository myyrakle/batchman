use crate::{
    context::SharedContext,
    repositories::{CreateScheduleParams, ListTaskDefinitionsParams},
    routes::schedules::CreateScheduleBody,
    types::cron::CronExpression,
};

#[derive(Debug, Clone)]
pub struct CreateSchduleRequest {
    pub request_body: CreateScheduleBody,
}

pub async fn create_schdule(
    context: SharedContext,
    params: CreateSchduleRequest,
) -> anyhow::Result<i64> {
    if let Err(error) = CronExpression::parse(params.request_body.cron_expression.as_str()) {
        return Err(anyhow::anyhow!("Invalid Cron Expression: {}", error));
    }

    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![params.request_body.task_definition_id],
            ..Default::default()
        })
        .await?;

    if task_definitions.is_empty() {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    let new_job_id = context
        .schedule_repository
        .create_schedule(CreateScheduleParams {
            name: params.request_body.name,
            job_name: params.request_body.job_name,
            cron_expression: params.request_body.cron_expression,
            task_definition_id: params.request_body.task_definition_id,
            command: params.request_body.command,
            timezone: params.request_body.timezone,
            timezone_offset: params.request_body.timezone_offset,
            enabled: params.request_body.enabled,
        })
        .await?;

    Ok(new_job_id)
}
