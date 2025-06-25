use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QuerySelect,
};

use crate::{
    domain::job::{
        JobRepository,
        dao::{CreateJobParams, ListJobsParams, PatchJobParams},
        entities,
    },
    errors,
};

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
    async fn list_jobs(&self, params: ListJobsParams) -> errors::Result<Vec<entities::job::Model>> {
        let mut find_job_query = entities::job::Entity::find();

        if !params.job_ids.is_empty() {
            find_job_query = find_job_query.filter(entities::job::Column::Id.is_in(params.job_ids));
        }

        if !params.statuses.is_empty() {
            find_job_query =
                find_job_query.filter(entities::job::Column::Status.is_in(params.statuses));
        }

        if let Some(contains_name) = &params.contains_name {
            find_job_query =
                find_job_query.filter(entities::job::Column::Name.contains(contains_name));
        }

        if let Some(limit) = params.limit {
            find_job_query = find_job_query.limit(limit);
        }

        if let Some(offset) = params.offset {
            find_job_query = find_job_query.offset(offset);
        }

        let jobs = find_job_query.all(&self.connection).await?;

        Ok(jobs)
    }

    async fn create_job(&self, params: CreateJobParams) -> errors::Result<i64> {
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
            created_at: Set(chrono::Utc::now()),
        };

        let model = new_job.into_active_model().insert(&self.connection).await?;

        Ok(model.id)
    }

    async fn patch_job(&self, params: PatchJobParams) -> errors::Result<()> {
        let job = entities::job::Entity::find_by_id(params.job_id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| errors::Error::JobNotFound)?;

        let mut model = job.into_active_model();

        if let Some(name) = params.name {
            model.name = Set(name);
        }

        if let Some(task_definition_id) = params.task_definition_id {
            model.task_definition_id = Set(task_definition_id);
        }

        if let Some(status) = params.status {
            model.status = Set(status);
        }

        if let Some(submited_at) = params.submited_at {
            model.submited_at = Set(Some(submited_at));
        }

        if let Some(started_at) = params.started_at {
            model.started_at = Set(Some(started_at));
        }

        if let Some(finished_at) = params.finished_at {
            model.finished_at = Set(Some(finished_at));
        }

        if let Some(container_id) = params.container_id {
            model.container_id = Set(Some(container_id));
        }

        if let Some(exit_code) = params.exit_code {
            model.exit_code = Set(Some(exit_code));
        }

        if let Some(error_message) = params.error_message {
            model.error_message = Set(Some(error_message));
        }

        model.update(&self.connection).await?;

        Ok(())
    }

    async fn count_jobs(&self, params: ListJobsParams) -> errors::Result<u64> {
        let mut count_job_query = entities::job::Entity::find();

        if !params.job_ids.is_empty() {
            count_job_query =
                count_job_query.filter(entities::job::Column::Id.is_in(params.job_ids));
        }

        if !params.statuses.is_empty() {
            count_job_query =
                count_job_query.filter(entities::job::Column::Status.is_in(params.statuses));
        }

        if let Some(contains_name) = &params.contains_name {
            count_job_query =
                count_job_query.filter(entities::job::Column::Name.contains(contains_name));
        }

        let count = count_job_query.count(&self.connection).await?;

        Ok(count)
    }
}
