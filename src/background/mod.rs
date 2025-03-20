pub mod runner;
pub mod scheduler;

use sea_orm::DatabaseConnection;

pub async fn start_background_loop(database_connection: DatabaseConnection) {
    tokio::join!(
        runner::start_runner_loop(database_connection.clone()),
        scheduler::start_scheduler_loop(database_connection.clone()),
    );
}
