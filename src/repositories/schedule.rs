use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set,
};

use crate::{background::scheduler::ScheduleCDCEvent, db::entities};

use super::{ListSchedulesParams, PatchScheduleParams, ScheduleRepository};

pub struct ScheduleSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
    pub schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,
}

#[async_trait::async_trait]
impl ScheduleRepository for ScheduleSeaOrmRepository {
    async fn list_schedules(
        &self,
        params: ListSchedulesParams,
    ) -> anyhow::Result<Vec<entities::schedule::Model>> {
        let mut query = entities::schedule::Entity::find();

        if !params.schedule_ids.is_empty() {
            query = query.filter(entities::schedule::Column::Id.is_in(params.schedule_ids));
        }

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }

        let schedules = query.all(&self.connection).await?;

        Ok(schedules)
    }

    async fn create_schedule(&self, params: super::CreateScheduleParams) -> anyhow::Result<i64> {
        let schedule = entities::schedule::ActiveModel {
            name: sea_orm::Set(params.name),
            job_name: sea_orm::Set(params.job_name),
            cron_expression: sea_orm::Set(params.cron_expression),
            task_definition_id: sea_orm::Set(params.task_definition_id),
            command: sea_orm::Set(params.command),
            timezone: sea_orm::Set(params.timezone),
            timezone_offset: sea_orm::Set(params.timezone_offset),
            created_at: sea_orm::Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let schedule = schedule.insert(&self.connection).await?;

        Ok(schedule.id)
    }

    async fn patch_schedule(&self, params: PatchScheduleParams) -> anyhow::Result<()> {
        let schedule_model = entities::schedule::Entity::find_by_id(params.schedule_id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Schedule not found with id {}", params.schedule_id))?;

        let mut schedule_active_model = schedule_model.into_active_model();

        let mut time_fields_changed = false;

        if let Some(name) = params.name {
            schedule_active_model.name = Set(name);
        }
        if let Some(job_name) = params.job_name {
            schedule_active_model.job_name = Set(job_name);
        }
        if let Some(cron_expression) = params.cron_expression {
            if schedule_active_model.cron_expression.as_ref() != &cron_expression {
                schedule_active_model.cron_expression = Set(cron_expression);
                time_fields_changed = true;
            }
        }
        if let Some(task_definition_id) = params.task_definition_id {
            schedule_active_model.task_definition_id = Set(task_definition_id);
        }
        if let Some(command) = params.command {
            schedule_active_model.command = Set(command);
        }
        if let Some(timezone) = params.timezone {
            if schedule_active_model.timezone.as_ref().as_ref() != Some(&timezone) {
                schedule_active_model.timezone = Set(Some(timezone));
                time_fields_changed = true;
            }
        } else if params.timezone.is_some() && schedule_active_model.timezone.as_ref().is_some() { // Handles explicit None
             schedule_active_model.timezone = Set(None);
             time_fields_changed = true;
        }

        if let Some(timezone_offset) = params.timezone_offset {
            if schedule_active_model.timezone_offset.as_ref().as_ref() != Some(&timezone_offset) {
                schedule_active_model.timezone_offset = Set(Some(timezone_offset));
                time_fields_changed = true;
            }
        } else if params.timezone_offset.is_some() && schedule_active_model.timezone_offset.as_ref().is_some() { // Handles explicit None
            schedule_active_model.timezone_offset = Set(None);
            time_fields_changed = true;
        }
        
        schedule_active_model.updated_at = Set(Some(chrono::Utc::now()));
        schedule_active_model.update(&self.connection).await?;

        if time_fields_changed {
            self.schedule_cdc_sender
                .send(ScheduleCDCEvent::Updated {
                    schedule_id: params.schedule_id,
                })
                .await
                .map_err(|e| anyhow::anyhow!("Failed to send ScheduleCDCEvent: {}", e))?;
        }

        Ok(())
    }
}

impl ScheduleSeaOrmRepository {
    pub fn new(
        connection: sea_orm::DatabaseConnection,
        schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,
    ) -> Self {
        Self {
            connection,
            schedule_cdc_sender,
        }
    }
}
