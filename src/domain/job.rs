pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod routes;
pub mod service;

use dao::*;
use dto::{ListJobsRequest, StopJobRequest, SubmitJobRequest};

use crate::{
    domain::job::dto::{
        CountJobLogsRequest, CountJobLogsResponse, ListJobLogsRequest, SubmitJobResponse,
    },
    errors,
};

#[async_trait::async_trait]
pub trait JobRepository {
    async fn list_jobs(&self, params: ListJobsParams) -> errors::Result<Vec<entities::job::Model>>;
    async fn count_jobs(&self, params: ListJobsParams) -> errors::Result<u64>;
    async fn create_job(&self, params: CreateJobParams) -> errors::Result<i64>;
    async fn patch_job(&self, params: PatchJobParams) -> errors::Result<()>;
}

#[async_trait::async_trait]
pub trait JobService {
    async fn submit_job(&self, params: SubmitJobRequest) -> errors::Result<SubmitJobResponse>;
    async fn stop_job(&self, params: StopJobRequest) -> errors::Result<()>;
    async fn list_jobs(&self, params: ListJobsRequest) -> errors::Result<dto::ListJobsResponse>;
    async fn run_pending_job(&self, pending_job: &entities::job::Model) -> errors::Result<()>;
    async fn track_running_job(&self, job: &entities::job::Model) -> errors::Result<()>;
    async fn list_job_logs(
        &self,
        request: ListJobLogsRequest,
    ) -> errors::Result<dto::ListJobLogsResponse>;
    async fn count_job_logs(
        &self,
        job_id: CountJobLogsRequest,
    ) -> errors::Result<CountJobLogsResponse>;
}
