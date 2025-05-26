use crate::db::entities;

pub mod dao;
pub mod dto;

use dao::*;

#[async_trait::async_trait]
pub trait JobRepository {
    async fn list_jobs(&self, params: ListJobsParams) -> anyhow::Result<Vec<entities::job::Model>>;
    async fn create_job(&self, params: CreateJobParams) -> anyhow::Result<i64>;
    async fn patch_job(&self, params: PatchJobParams) -> anyhow::Result<()>;
}
