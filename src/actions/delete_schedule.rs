use crate::context::SharedContext;

pub async fn delete_schedule(context: SharedContext, schedule_id: i64) -> anyhow::Result<()> {
    context
        .schedule_repository
        .delete_schedule(schedule_id)
        .await?;

    Ok(())
}
