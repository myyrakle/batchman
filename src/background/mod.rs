pub mod runner;
pub mod scheduler;
pub mod status_tracker;

use sea_orm::DatabaseConnection;

pub async fn start_background_loop(database_connection: DatabaseConnection) {
    tokio::join!(
        runner::start_runner_loop(database_connection.clone()),
        scheduler::start_scheduler_loop(database_connection.clone()),
        status_tracker::start_status_tracker_loop(database_connection.clone()),
    );
}
