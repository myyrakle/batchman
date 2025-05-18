use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    IntoActiveModel,
};

use crate::db::entities;

use super::{CreateJobParams, JobRepository};

pub struct JobSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

impl JobSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl JobRepository for JobSeaOrmRepository {
    async fn create_job(&self, params: CreateJobParams) -> anyhow::Result<i64> {
        let new_job = entities::job::ActiveModel {
            id: NotSet,
            name: Set(params.name),
            task_definition_id: Set(params.task_definition_id),
            status: Set(params.status),
            submited_at: Set(params.submited_at),
            started_at: Set(params.started_at),
            finished_at: Set(params.finished_at),
            container_id: Set(params.container_id),
            exit_code: Set(params.exit_code),
            error_message: Set(params.error_message),
        };

        let model = new_job.into_active_model().insert(&self.connection).await?;

        Ok(model.id)
    }
}
