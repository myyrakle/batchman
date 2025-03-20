use sea_orm::DatabaseConnection;

pub async fn start_scheduler_loop(_database_connection: DatabaseConnection) {
    let _ = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("Background loop");
        }
    })
    .await;
}
