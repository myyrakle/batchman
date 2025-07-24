pub mod dao;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod routes;
pub mod service;

use dao::*;
use dto::{CreateSchduleRequest, ListSchedulesRequest, PatchScheduleRequest};

use crate::{domain::schedule::dto::ListSchedulesResponse, errors};

#[async_trait::async_trait]
pub trait ScheduleRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> errors::Result<Vec<entities::schedule::Model>>;

    async fn count_schedules(&self, params: ListSchedulesParams) -> errors::Result<i64>;

    async fn create_schedule(&self, params: CreateScheduleParams) -> errors::Result<i64>;

    async fn patch_schedule(&self, params: PatchScheduleParams) -> errors::Result<()>;

    async fn delete_schedule(&self, schedule_id: i64) -> errors::Result<()>;
}

#[async_trait::async_trait]
pub trait ScheduleService {
    async fn create_schdule(&self, request: CreateSchduleRequest) -> errors::Result<i64>;
    async fn patch_schedule(&self, request: PatchScheduleRequest) -> errors::Result<()>;
    async fn delete_schedule(&self, schedule_id: i64) -> errors::Result<()>;
    async fn list_schedules(
        &self,
        request: ListSchedulesRequest,
    ) -> errors::Result<ListSchedulesResponse>;
}
