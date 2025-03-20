use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel,
};

use crate::{db::entities, routes::jobs::SubmitJobBody};

#[derive(Debug, Clone)]
pub struct SubmitJobParams<'a> {
    pub connection: &'a DatabaseConnection,
    pub request_body: SubmitJobBody,
}

pub async fn submit_job(params: SubmitJobParams<'_>) -> anyhow::Result<()> {
    let task_definition =
        entities::task_definition::Entity::find_by_id(params.request_body.task_definition_id)
            .one(params.connection)
            .await?;

    let Some(_) = task_definition else {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    let new_job = entities::job::ActiveModel {
        id: NotSet,
        name: Set(params.request_body.job_name),
        task_definition_id: Set(params.request_body.task_definition_id),
        status: Set(entities::job::JobStatus::Pending),
        submited_at: Set(Some(chrono::Utc::now())),
        started_at: Set(None),
        finished_at: Set(None),
        container_id: Set(None),
    };

    new_job.into_active_model().save(params.connection).await?;

    Ok(())
}
