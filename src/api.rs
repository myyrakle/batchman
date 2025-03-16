pub(crate) mod actions;
pub(crate) mod db;
pub(crate) mod docker;
pub(crate) mod routes;

use axum::{
    Extension, Router,
    routing::{get, patch, post},
};
use db::setup_schema;
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();
    setup_schema(&connection).await;

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/database-check", get(database_check))
        .route(
            "/task-definitions",
            get(routes::task_definitions::list_task_definitions),
        )
        .route(
            "/task-definitions",
            post(routes::task_definitions::create_task_definition),
        )
        .route(
            "/task-definitions/{task_definition_id}",
            patch(routes::task_definitions::patch_task_definition),
        )
        .layer(Extension(connection));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:13939")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn database_check(Extension(state): Extension<DatabaseConnection>) -> &'static str {
    state.ping().await.unwrap();

    "OK"
}
