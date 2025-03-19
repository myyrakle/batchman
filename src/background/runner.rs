use sea_orm::DatabaseConnection;

pub fn start_runner_loop(database_connection: DatabaseConnection) -> anyhow::Result<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("Background loop");
        }
    });

    Ok(())
}
