use crate::{
    context::SharedContext, repositories::ListSchedulesParams,
    repositories::schedule::ScheduleRepository,
};

pub async fn list_schedules(
    context: SharedContext,
    params: ListSchedulesParams,
) -> anyhow::Result<Vec<crate::db::entities::schedule::Model>> {
    context.repositories.schedule().list_schedules(params).await
}
