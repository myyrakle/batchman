pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod service;

use dao::*;
use dto::{StopJobRequest, SubmitJobRequest};

#[async_trait::async_trait]
pub trait JobRepository {
    async fn list_jobs(&self, params: ListJobsParams) -> anyhow::Result<Vec<entities::job::Model>>;
    async fn create_job(&self, params: CreateJobParams) -> anyhow::Result<i64>;
    async fn patch_job(&self, params: PatchJobParams) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait JobService {
    async fn submit_job(&self, params: SubmitJobRequest) -> anyhow::Result<i64>;
    async fn stop_job(&self, params: StopJobRequest) -> anyhow::Result<()>;
    async fn run_pending_job(&self, pending_job: &entities::job::Model) -> anyhow::Result<()>;
    async fn track_runnng_job(&self, job: &entities::job::Model) -> anyhow::Result<()>;
}
