use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect,
};

use crate::{
    actions::run_pending_job::run_pending_job,
    context::SharedContext,
    db::entities::{self, job::JobStatus},
};

pub async fn start_runner_loop(context: SharedContext) {
    let _ = tokio::spawn(async move {
        loop {
            let mut find_query = entities::job::Entity::find();

            find_query = find_query.filter(entities::job::Column::Status.eq(JobStatus::Pending));
            find_query = find_query.limit(5); // TODO: config 설정 가능한 값으로 빼기

            let pending_jobs = match find_query.all(&context.connection).await {
                Ok(pending_jobs) => pending_jobs,
                Err(error) => {
                    println!("Error fetching pending jobs: {:?}", error);
                    // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue;
                }
            };

            if pending_jobs.is_empty() {
                // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }

            for pending_job in pending_jobs {
                if let Err(error) = run_pending_job(context.clone(), &pending_job).await {
                    println!("Error processing job: {:?}", error);
                    let mut job_active_model = pending_job.into_active_model();
                    job_active_model.status = Set(JobStatus::Failed);

                    if let Err(error) = job_active_model.update(&context.connection).await {
                        println!("Error updating job status: {:?}", error);
                    }
                }
            }
        }
    })
    .await;
}
