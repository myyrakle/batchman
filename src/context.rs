use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    background::scheduler::ScheduleCDCEvent,
    domain::{
        self, job::JobRepository, schedule::ScheduleRepository,
        task_definition::TaskDefinitionRepository,
    },
};

pub struct Context {
    pub connection: DatabaseConnection,

    pub schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,

    pub task_definition_repository: Box<dyn TaskDefinitionRepository + Send + Sync>,
    pub job_repository: Box<dyn JobRepository + Send + Sync>,
    pub schedule_repository: Box<dyn ScheduleRepository + Send + Sync>,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub fn new(
        connection: DatabaseConnection,
        schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,
    ) -> Self {
        Self {
            connection: connection.clone(),
            schedule_cdc_sender,
            task_definition_repository: Box::new(
                domain::task_definition::repository::TaskDefinitionSeaOrmRepository::new(
                    connection.clone(),
                ),
            ),
            job_repository: Box::new(domain::job::repository::JobSeaOrmRepository::new(
                connection.clone(),
            )),
            schedule_repository: Box::new(
                domain::schedule::repository::ScheduleSeaOrmRepository::new(connection),
            ),
        }
    }
}
