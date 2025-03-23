use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    db::entities::{self, job::JobStatus},
    docker,
    routes::jobs::StopJobBody,
};

#[derive(Debug, Clone)]
pub struct StopJobParams<'a> {
    pub connection: &'a DatabaseConnection,
    pub request_body: StopJobBody,
}

pub async fn stop_job(params: StopJobParams<'_>) -> anyhow::Result<()> {
    let job_id = params.request_body.job_id;

    let mut find_job_query = entities::job::Entity::find();

    find_job_query = find_job_query.filter(entities::job::Column::Id.eq(job_id));

    let Some(job) = find_job_query.one(params.connection).await? else {
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
