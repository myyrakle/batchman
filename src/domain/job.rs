pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod routes;
pub mod service;

use dao::*;
use dto::{StopJobRequest, SubmitJobRequest};

use crate::errors;

#[async_trait::async_trait]
pub trait JobRepository {
    async fn list_jobs(&self, params: ListJobsParams) -> errors::Result<Vec<entities::job::Model>>;
    async fn create_job(&self, params: CreateJobParams) -> errors::Result<i64>;
    async fn patch_job(&self, params: PatchJobParams) -> errors::Result<()>;
}

#[async_trait::async_trait]
pub trait JobService {
    async fn submit_job(&self, params: SubmitJobRequest) -> errors::Result<i64>;
    async fn stop_job(&self, params: StopJobRequest) -> errors::Result<()>;
    async fn run_pending_job(&self, pending_job: &entities::job::Model) -> errors::Result<()>;
    async fn track_running_job(&self, job: &entities::job::Model) -> errors::Result<()>;
}
