use sea_orm::DatabaseConnection;

pub async fn start_status_tracker_loop(_database_connection: DatabaseConnection) {
    let _ = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("Status Tracker loop");
        }
    })
    .await;
}
