use crate::{
    actions::track_runnng_job::track_runnng_job,
    context::SharedContext,
    db::entities::job::JobStatus,
    repositories::{ListJobsParams, PatchJobParams},
};

pub async fn start_status_tracker_loop(context: SharedContext) {
    let _ = tokio::spawn(async move {
        loop {
            let running_jobs_result = context
                .job_repository
                .list_jobs(ListJobsParams {
                    statuses: vec![JobStatus::Running],
                    ..Default::default()
                })
                .await;

            let running_jobs = match running_jobs_result {
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

                    let patch_result = context
                        .job_repository
                        .patch_job(PatchJobParams {
                            job_id: running_job.id,
                            status: Some(JobStatus::Failed),
                            ..Default::default()
                        })
                        .await;

                    if let Err(error) = patch_result {
                        println!("Error updating job status: {:?}", error);
                    }
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    })
    .await;
}
