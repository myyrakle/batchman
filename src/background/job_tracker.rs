use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::{
    actions::track_runnng_job::track_runnng_job,
    context::SharedContext,
    db::entities::{self, job::JobStatus},
};

pub async fn start_status_tracker_loop(context: SharedContext) {
    let _ = tokio::spawn(async move {
        loop {
            let mut find_query = entities::job::Entity::find();

            find_query = find_query.filter(entities::job::Column::Status.eq(JobStatus::Running));

            let running_jobs = match find_query.all(&context.connection).await {
                Ok(jobs) => jobs,
                Err(error) => {
                    println!("Error fetching running jobs: {:?}", error);
                    // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue;
                }
            };

            if running_jobs.is_empty() {
                // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }

            for running_job in running_jobs {
                if let Err(error) = track_runnng_job(context.clone(), &running_job).await {
                    println!("Error processing job: {:?}", error);
                    let mut job_active_model = running_job.into_active_model();
                    job_active_model.status = Set(JobStatus::Failed);

                    if let Err(error) = job_active_model.update(&context.connection).await {
                        println!("Error updating job status: {:?}", error);
                    }
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    })
    .await;
}
