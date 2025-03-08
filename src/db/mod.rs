pub(crate) mod entities;

use std::time::Duration;

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema,
    sea_query::TableCreateStatement,
};

pub async fn create_database_connection() -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new("sqlite://./db.sqlite?mode=rwc");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;

    Ok(db)
}

pub async fn setup_schema(db: &DatabaseConnection) {
    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    // Derive from Entity
    let create_table_statement: TableCreateStatement = schema
        .create_table_from_entity(entities::task_definition::Entity)
        .into();

    let database_backend = db.get_database_backend();

    // Execute create table statement
    db.execute(database_backend.build(&create_table_statement))
        .await
        .expect("Failed to create table");
}
