pub mod runner;
pub mod scheduler;

use sea_orm::DatabaseConnection;

pub fn start_background_loop(database_connection: DatabaseConnection) -> anyhow::Result<()> {
    runner::start_runner_loop(database_connection.clone())?;
    scheduler::start_scheduler_loop(database_connection.clone())?;

    Ok(())
}
