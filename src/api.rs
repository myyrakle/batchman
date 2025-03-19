pub(crate) mod actions;
pub(crate) mod background;
pub(crate) mod db;
pub(crate) mod docker;
pub(crate) mod routes;

use axum::{
    Extension, Router,
    routing::{delete, get, patch, post},
};
use db::setup_schema;
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();
    setup_schema(&connection).await;

    background::start_background_loop(connection.clone()).expect("Failed to start background loop");

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
        .route(
            "/task-definitions/{task_definition_id}",
            delete(routes::task_definitions::delete_task_definition),
        )
        .route("/jobs/submit", post(routes::jobs::submit_job))
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
