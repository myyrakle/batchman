use crate::{
    context::SharedContext,
    db::entities::{self, job::JobStatus},
    docker::run_container,
    repositories::{ListTaskDefinitionsParams, PatchJobParams},
};

pub async fn run_pending_job(
    context: SharedContext,
    pending_job: &entities::job::Model,
) -> anyhow::Result<()> {
    // TODO: 리소스 제한이나 실행 제한 등에 걸리지 않는지 확인 (차후 개발)

    // 1. job 상태를 START로 변경
    context
        .job_repository
        .patch_job(PatchJobParams {
            job_id: pending_job.id,
            status: Some(JobStatus::Starting),
            started_at: Some(chrono::Utc::now()),
            ..Default::default()
        })
        .await?;

    // 2. 컨테이너 실행을 위해 task definition을 가져옴
    let mut task_definitions = context
        .task_definition_repository
        .list_task_definitions(ListTaskDefinitionsParams {
            task_definition_ids: vec![pending_job.task_definition_id],
            ..Default::default()
        })
        .await?;

    let Some(task_definition) = task_definitions.pop() else {
        return Err(anyhow::anyhow!("Task definition not found"));
    };

    // 3. 컨테이너 실행
    let container_id = run_container(task_definition)?;

    // 4. 컨테이너 정보를 job에 업데이트, job 상태를 RUNNING으로 변경
    context
        .job_repository
        .patch_job(PatchJobParams {
            job_id: pending_job.id,
            container_id: Some(container_id.clone()),
            status: Some(JobStatus::Running),
            ..Default::default()
        })
        .await?;

    Ok(())
}
