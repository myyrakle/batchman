use crate::{
    context::SharedContext, db::entities::job::JobStatus, docker, domain::job::dao::ListJobsParams,
    routes::jobs::StopJobBody,
};

#[derive(Debug, Clone)]
pub struct StopJobRequest {
    pub request_body: StopJobBody,
}

pub async fn stop_job(context: SharedContext, params: StopJobRequest) -> anyhow::Result<()> {
    let job_id = params.request_body.job_id;

    let mut jobs = context
        .job_repository
        .list_jobs(ListJobsParams {
            job_ids: vec![job_id],
            ..Default::default()
        })
        .await?;

    let Some(job) = jobs.pop() else {
        return Err(anyhow::anyhow!("Job not found: {}", job_id));
    };

    if job.status == JobStatus::Finished {
        return Err(anyhow::anyhow!("Job already finished"));
    }

    if job.status == JobStatus::Failed {
        return Err(anyhow::anyhow!("Job already failed"));
    }

    let Some(container_id) = job.container_id.as_ref() else {
        return Err(anyhow::anyhow!("Job has no container ID"));
    };

    docker::stop::stop_container(container_id, 3)?;

    Ok(())
}
