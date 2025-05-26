use crate::{
    actions::run_pending_job::run_pending_job,
    context::SharedContext,
    domain::job::{
        dao::{ListJobsParams, PatchJobParams},
        entities::job::JobStatus,
    },
};

pub async fn start_runner_loop(context: SharedContext) {
    let _ = tokio::spawn(async move {
        loop {
            let pending_jobs_result = context
                .job_repository
                .list_jobs(ListJobsParams {
                    statuses: vec![JobStatus::Pending],
                    limit: Some(5), // TODO: config 설정 가능한 값으로 빼기
                    ..Default::default()
                })
                .await;

            let pending_jobs = match pending_jobs_result {
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

                    let patch_result = context
                        .job_repository
                        .patch_job(PatchJobParams {
                            job_id: pending_job.id,
                            status: Some(JobStatus::Failed),
                            ..Default::default()
                        })
                        .await;

                    if let Err(error) = patch_result {
                        println!("Error updating job status: {:?}", error);
                    }
                }
            }
        }
    })
    .await;
}
