use sea_orm::DatabaseConnection;

use crate::db::entities;

#[derive(Debug)]
pub enum ScheduleCDCEvent {
    New(NewSchedule),
    Update(UpdateSchedule),
    Delete(DeleteSchedule),
}

#[derive(Debug)]
pub struct NewSchedule {
    pub id: i64,
    pub model: entities::schedule::Model,
}

#[derive(Debug)]
pub struct UpdateSchedule {
    pub id: i64,
    pub model: entities::schedule::Model,
}

#[derive(Debug)]
pub struct DeleteSchedule {
    pub id: i64,
}

type ScheduleCDCSender = tokio::sync::mpsc::Sender<ScheduleCDCEvent>;
type ScheduleCDCReceiver = tokio::sync::mpsc::Receiver<ScheduleCDCEvent>;

pub async fn start_scheduler_loop(_database_connection: DatabaseConnection) {
    let _ = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("Scheduler loop");
        }
    })
    .await;
}
