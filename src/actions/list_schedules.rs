use crate::{
    context::SharedContext,
    domain::schedule::{dao::ListSchedulesParams, dto::ListSchedulesRequest, entities},
};

pub async fn list_schedules(
    context: SharedContext,
    request: ListSchedulesRequest,
) -> anyhow::Result<Vec<entities::schedule::Model>> {
    let result = context
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
