use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{background::scheduler::ScheduleCDCEvent, repositories};

pub struct Context {
    pub connection: DatabaseConnection,

    pub schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,

    pub task_definition_repository: Box<dyn repositories::TaskDefinitionRepository + Send + Sync>,
    pub job_repository: Box<dyn repositories::JobRepository + Send + Sync>,
    pub schedule_repository: Box<dyn repositories::ScheduleRepository + Send + Sync>,
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
                repositories::task_definition::TaskDefinitionSeaOrmRepository::new(
                    connection.clone(),
                ),
            ),
            job_repository: Box::new(repositories::job::JobSeaOrmRepository::new(
                connection.clone(),
            )),
            schedule_repository: Box::new(repositories::schedule::ScheduleSeaOrmRepository::new(
                connection,
            )),
        }
    }
}
