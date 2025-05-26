use crate::{
    context::SharedContext,
    domain::{
        job::{dao::CreateJobParams, dto::SubmitJobRequest, entities},
        task_definition::dao::ListTaskDefinitionsParams,
    },
};

pub async fn submit_job(context: SharedContext, params: SubmitJobRequest) -> anyhow::Result<i64> {
    let task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![params.request_body.task_definition_id],
            ..Default::default()
        })
        .await?;

    if task_definitions.is_empty() {
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
