use crate::{
    context::SharedContext,
    repositories::{ListSchedulesParams, PatchScheduleParams},
    routes::schedules::PatchScheduleBody,
    types::cron::CronExpression,
};

#[derive(Debug, Clone)]
pub struct PatchScheduleRequest {
    pub schedule_id: i64,
    pub body: PatchScheduleBody,
}

pub async fn patch_schedule(
    context: SharedContext,
    request: PatchScheduleRequest,
) -> anyhow::Result<()> {
    if let Some(cron_expression) = &request.body.cron_expression {
        // Validate the cron expression
        if let Err(error) = CronExpression::parse(cron_expression.as_str()) {
            return Err(anyhow::anyhow!("Invalid Cron Expression: {}", error));
        }
    }

    // Check if schedule exists
    let schedules = context
        .schedule_repository
        .list_schedules(ListSchedulesParams {
            schedule_ids: vec![request.schedule_id],
            limit: Some(1),
            ..Default::default()
        })
        .await?;

    if schedules.is_empty() {
        return Err(anyhow::anyhow!(
            "Schedule not found with id {}",
            request.schedule_id
        ));
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

    context.schedule_repository.patch_schedule(params).await?;

    Ok(())
}
