pub mod dao;
pub mod dto;
pub mod entities;
use dao::*;

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
