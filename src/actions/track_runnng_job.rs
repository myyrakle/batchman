use sea_orm::DatabaseConnection;

use crate::db::entities;

pub async fn track_runnng_job(
    database_connection: &DatabaseConnection,
    job: &entities::job::Model,
) -> anyhow::Result<()> {
    Ok(())
}
