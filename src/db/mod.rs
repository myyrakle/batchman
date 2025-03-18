pub(crate) mod entities;

use std::time::Duration;

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema, sea_query,
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

    // task_definition table generate
    {
        let mut create_table_statement =
            schema.create_table_from_entity(entities::task_definition::Entity);

        create_table_statement.if_not_exists();

        let database_backend = db.get_database_backend();

        // Execute create table statement
        db.execute(database_backend.build(&create_table_statement))
            .await
            .expect("Failed to create table");

        // create index

        let create_unique_index_query = sea_query::Index::create()
            .unique()
            .if_not_exists()
            .name("task_definition_name_unique")
            .table(entities::task_definition::Entity)
            .col(entities::task_definition::Column::Name)
            .col(entities::task_definition::Column::Version)
            .to_owned();

        db.execute(database_backend.build(&create_unique_index_query))
            .await
            .expect("Failed to create index");
    }
}
