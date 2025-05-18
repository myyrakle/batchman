use sea_orm::EntityTrait;

use crate::{
    context::SharedContext, db::entities, repositories::CreateJobParams,
    routes::jobs::SubmitJobBody,
};

#[derive(Debug, Clone)]
pub struct SubmitJobRequest {
    pub request_body: SubmitJobBody,
}

pub async fn submit_job(context: SharedContext, params: SubmitJobRequest) -> anyhow::Result<i64> {
    let task_definition =
        entities::task_definition::Entity::find_by_id(params.request_body.task_definition_id)
            .one(&context.connection)
            .await?;

    let Some(_) = task_definition else {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    let new_job_id = context
        .job_repository
        .create_job(CreateJobParams {
            name: params.request_body.job_name.clone(),
            task_definition_id: params.request_body.task_definition_id,
            status: entities::job::JobStatus::Pending,
            submited_at: Some(chrono::Utc::now()),
            ..Default::default()
        })
        .await?;

    Ok(new_job_id)
}
