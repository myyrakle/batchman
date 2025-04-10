use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel,
};

use crate::{db::entities, routes::jobs::SubmitJobBody};

#[derive(Debug, Clone)]
pub struct SubmitJobRequest<'a> {
    pub connection: &'a DatabaseConnection,
    pub request_body: SubmitJobBody,
}

pub async fn submit_job(params: SubmitJobRequest<'_>) -> anyhow::Result<i64> {
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
        exit_code: Set(None),
        error_message: Set(None),
    };

    let model = new_job
        .into_active_model()
        .insert(params.connection)
        .await?;

    Ok(model.id)
}
