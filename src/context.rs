use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    background::scheduler::ScheduleCDCEvent,
    domain::{
        self,
        job::JobRepository,
        schedule::ScheduleRepository,
        task_definition::{TaskDefinitionRepository, TaskDefinitionService},
    },
};

pub struct Context {
    pub connection: DatabaseConnection,

    pub schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,

    pub task_definition_repository: Arc<dyn TaskDefinitionRepository + Send + Sync>,
    pub job_repository: Arc<dyn JobRepository + Send + Sync>,
    pub schedule_repository: Arc<dyn ScheduleRepository + Send + Sync>,

    pub task_definition_service: Box<dyn TaskDefinitionService + Send + Sync>,
    pub schedule_service: Box<dyn domain::schedule::ScheduleService + Send + Sync>,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub fn new(
        connection: DatabaseConnection,
        schedule_cdc_sender: tokio::sync::mpsc::Sender<ScheduleCDCEvent>,
    ) -> Self {
        let task_definition_repository = Arc::new(
            domain::task_definition::repository::TaskDefinitionSeaOrmRepository::new(
                connection.clone(),
            ),
        );

        let job_repository = Arc::new(domain::job::repository::JobSeaOrmRepository::new(
            connection.clone(),
        ));

        let schedule_repository = Arc::new(
            domain::schedule::repository::ScheduleSeaOrmRepository::new(connection.clone()),
        );

        Self {
            connection: connection.clone(),
            schedule_cdc_sender,
            task_definition_repository: task_definition_repository.clone(),
            job_repository,
            schedule_repository: schedule_repository.clone(),
            task_definition_service: Box::new(
                domain::task_definition::service::TaskDefinitionServiceImpl::new(
                    task_definition_repository.clone(),
                ),
            ),
            schedule_service: Box::new(domain::schedule::service::ScheduleServiceImpl::new(
                schedule_repository,
                task_definition_repository,
            )),
        }
    }
}
