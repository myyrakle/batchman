use sea_orm::DatabaseConnection;

use crate::{background::scheduler::ScheduleCDCEvent, repositories};

pub struct Context {
    pub connection: DatabaseConnection,

    pub schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,

    pub task_definition_repository: Box<dyn repositories::TaskDefinitionRepository + Send + Sync>,
    pub job_repository: Box<dyn repositories::JobRepository + Send + Sync>,
    pub schedule_repository: Box<dyn repositories::ScheduleRepository + Send + Sync>,
}

impl Context {
    pub fn new(
        connection: DatabaseConnection,
        schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,
    ) -> Self {
        Self {
            connection,
            schedule_cdc_sender,
        }
    }
}
