use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};

use crate::{
    db::entities::{self, job::JobStatus},
    docker::run_container,
};

pub async fn run_pending_job(
    database_connection: &DatabaseConnection,
    pending_job: &entities::job::Model,
) -> anyhow::Result<()> {
    // TODO: 리소스 제한이나 실행 제한 등에 걸리지 않는지 확인 (차후 개발)

    // job 상태를 START로 변경
    let mut job_active_model = pending_job.clone().into_active_model();
    job_active_model.status = Set(JobStatus::Starting);
    job_active_model.started_at = Set(Some(chrono::Utc::now()));
    let pending_job = job_active_model.clone().update(database_connection).await?;

    let Ok(Some(task_definition)) = entities::task_definition::Entity::find()
        .filter(entities::task_definition::Column::Id.eq(pending_job.task_definition_id))
        .one(database_connection)
        .await
    else {
        return Err(anyhow::anyhow!("Error fetching task definition"));
    };

    // 컨테이너 실행
    let container_id = run_container(task_definition)?;

    // 컨테이너 정보를 job에 업데이트, job 상태를 RUNNING으로 변경
    let mut job_active_model = pending_job.into_active_model();
    job_active_model.container_id = Set(Some(container_id));
    job_active_model.status = Set(JobStatus::Running);
    job_active_model.update(database_connection).await?;

    Ok(())
}
