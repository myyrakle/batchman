use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};

use crate::entities::{self};

#[derive(Debug, Clone)]
pub struct SubmitJobParams<'a> {
    connection: &'a DatabaseConnection,
    task_definition_id: u64,
    job_name: String,
}

pub async fn submit_job(params: SubmitJobParams<'_>) -> anyhow::Result<()> {
    let task_definition = entities::task_definition::Entity::find_by_id(params.task_definition_id)
        .one(params.connection)
        .await?;

    let Some(_) = task_definition else {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    let new_job = entities::job::Model {
        id: 0,
        name: params.job_name,
        task_definition_id: params.task_definition_id,
        status: entities::job::JobStatus::Pending,
        submited_at: Some(chrono::Utc::now().naive_utc()),
        started_at: None,
        finished_at: None,
        container_id: None,
    };

    new_job.into_active_model().save(params.connection).await?;

    Ok(())
}
