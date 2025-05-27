pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod service;

use dao::*;
use dto::{CreateSchduleRequest, ListSchedulesRequest, PatchScheduleRequest};

#[async_trait::async_trait]
pub trait ScheduleRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> anyhow::Result<Vec<entities::schedule::Model>>;

    async fn create_schedule(&self, params: CreateScheduleParams) -> anyhow::Result<i64>;

    async fn patch_schedule(&self, params: PatchScheduleParams) -> anyhow::Result<()>;

    async fn delete_schedule(&self, schedule_id: i64) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait ScheduleService {
    async fn create_schdule(&self, request: CreateSchduleRequest) -> anyhow::Result<i64>;
    async fn patch_schedule(&self, request: PatchScheduleRequest) -> anyhow::Result<()>;
    async fn delete_schedule(&self, schedule_id: i64) -> anyhow::Result<()>;
    async fn list_schedules(
        &self,
        request: ListSchedulesRequest,
    ) -> anyhow::Result<Vec<entities::schedule::Model>>;
}
