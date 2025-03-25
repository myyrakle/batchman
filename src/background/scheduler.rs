use sea_orm::{DatabaseConnection, EntityTrait};

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

pub type ScheduleCDCSender = tokio::sync::mpsc::Sender<ScheduleCDCEvent>;
pub type ScheduleCDCReceiver = tokio::sync::mpsc::Receiver<ScheduleCDCEvent>;

pub async fn list_schedules(
    database_connection: DatabaseConnection,
) -> anyhow::Result<Vec<entities::schedule::Model>> {
    let schedules = entities::schedule::Entity::find()
        .all(&database_connection)
        .await?;

    Ok(schedules)
}

pub async fn start_scheduler_loop(
    _database_connection: DatabaseConnection,
    _receiver: ScheduleCDCReceiver,
) {
    let _ = tokio::spawn(async move {
        let schedules = list_schedules(_database_connection)
            .await
            .expect("Failed to load schedules");

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("Scheduler loop");
        }
    })
    .await;
}
