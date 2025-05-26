use crate::{
    context::SharedContext,
    domain::{
        schedule::dao::CreateScheduleParams, task_definition::dao::ListTaskDefinitionsParams,
    },
    routes::schedules::CreateScheduleBody,
    types::cron::CronExpression,
};

#[derive(Debug, Clone)]
pub struct CreateSchduleRequest {
    pub request_body: CreateScheduleBody,
}

pub async fn create_schdule(
    context: SharedContext,
    request: CreateSchduleRequest,
) -> anyhow::Result<i64> {
    if let Err(error) = CronExpression::parse(request.request_body.cron_expression.as_str()) {
        return Err(anyhow::anyhow!("Invalid Cron Expression: {}", error));
    }

    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![request.request_body.task_definition_id],
            ..Default::default()
        })
        .await?;

    if task_definitions.is_empty() {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    let new_job_id = context
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
