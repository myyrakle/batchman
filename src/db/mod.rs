use std::time::Duration;

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema, sea_query,
};

use crate::{domain, errors};

pub async fn create_database_connection() -> errors::Result<DatabaseConnection> {
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

        use domain::task_definition::entities;
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

    // job table generate
    {
        use domain::job::entities;
        let mut create_table_statement = schema.create_table_from_entity(entities::job::Entity);

        create_table_statement.if_not_exists();

        let database_backend = db.get_database_backend();

        // Execute create table statement
        db.execute(database_backend.build(&create_table_statement))
            .await
            .expect("Failed to create table");

        // add columes
        {
            // use sea_orm::sea_query::ColumnDef;
            // use sea_orm::sea_query::TableAlterStatement;

            // let alter_table_statement = TableAlterStatement::new()
            //     .table(entities::job::Entity)
            //     .add_column_if_not_exists(
            //         ColumnDef::new(entities::job::Column::ContainerType)
            //             .default("Docker")
            //             .not_null(),
            //     )
            //     .to_owned();

            // db.execute(database_backend.build(&alter_table_statement))
            //     .await
            //     .expect("Failed to alter table");
        }
    }

    // schedule table generate
    {
        use domain::schedule::entities;
        let mut create_table_statement =
            schema.create_table_from_entity(entities::schedule::Entity);

        create_table_statement.if_not_exists();

        let database_backend = db.get_database_backend();

        // Execute create table statement
        db.execute(database_backend.build(&create_table_statement))
            .await
            .expect("Failed to create table");
    }
}
