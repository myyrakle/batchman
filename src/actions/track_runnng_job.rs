use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use crate::{db::entities, docker};

pub async fn track_runnng_job(
    database_connection: &DatabaseConnection,
    job: &entities::job::Model,
) -> anyhow::Result<()> {
    let Some(container_id) = &job.container_id else {
        return Err(anyhow::anyhow!("Container ID not found"));
    };

    let inspect_result = docker::inspect_container(&container_id)?;

    // 1. 컨테이너가 종료되었을 경우 종료 처리
    if let Some(finished_at) = inspect_result.state.finished_at {
        let mut job_active_model = job.clone().into_active_model();
        job_active_model.status = Set(entities::job::JobStatus::Finished);
        job_active_model.finished_at = Set(Some(finished_at));
        job_active_model.exit_code = Set(inspect_result.state.exit_code);
        job_active_model.update(database_connection).await?;

        return Ok(());
    }

    // 2. 컨테이너가 모종의 이유로 조기 종료(실패)했을 경우 처리
    if inspect_result.state.dead {
        let mut job_active_model = job.clone().into_active_model();
        job_active_model.status = Set(entities::job::JobStatus::Failed);
        job_active_model.error_message = Set(Some(format!(
            "Container is dead: {}",
            inspect_result.state.error.unwrap_or_default()
        )));
        job_active_model.update(database_connection).await?;

        return Ok(());
    }

    Ok(())
}
