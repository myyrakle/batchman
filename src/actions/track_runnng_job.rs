use crate::{
    context::SharedContext,
    docker,
    domain::job::{dao::PatchJobParams, entities},
};

pub async fn track_runnng_job(
    context: SharedContext,
    job: &entities::job::Model,
) -> anyhow::Result<()> {
    let Some(container_id) = &job.container_id else {
        return Err(anyhow::anyhow!("Container ID not found"));
    };

    let inspect_result = docker::inspect_container(&container_id)?;

    // 1. 컨테이너가 종료되었을 경우 종료 처리
    if let Some(finished_at) = inspect_result.state.finished_at {
        context
            .job_repository
            .patch_job(PatchJobParams {
                job_id: job.id,
                status: Some(entities::job::JobStatus::Finished),
                finished_at: Some(finished_at),
                exit_code: inspect_result.state.exit_code,
                ..Default::default()
            })
            .await?;

        return Ok(());
    }

    // 2. 컨테이너가 모종의 이유로 조기 종료(실패)했을 경우 처리
    if inspect_result.state.dead {
        context
            .job_repository
            .patch_job(PatchJobParams {
                job_id: job.id,
                status: Some(entities::job::JobStatus::Failed),
                error_message: Some(format!(
                    "Container is dead: {}",
                    inspect_result.state.error.unwrap_or_default()
                )),
                ..Default::default()
            })
            .await?;

        return Ok(());
    }

    Ok(())
}
