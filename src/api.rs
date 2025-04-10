pub(crate) mod actions;
pub(crate) mod background;
pub(crate) mod context;
pub(crate) mod db;
pub(crate) mod docker;
pub(crate) mod repositories;
pub(crate) mod routes;

use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{delete, get, patch, post},
};
use background::scheduler::ScheduleCDCEvent;
use context::SharedContext;
use db::setup_schema;

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();
    setup_schema(&connection).await;

    let (schedule_cdc_sender, schedule_cdc_receiver) =
        tokio::sync::mpsc::channel::<ScheduleCDCEvent>(8);

    let context = Arc::new(context::Context::new(connection, schedule_cdc_sender));

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
        .route("/jobs/stop", post(routes::jobs::stop_job))
        .layer(Extension(context.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:13939")
        .await
        .unwrap();

    let (server, _) = tokio::join!(
        axum::serve(listener, app),
        background::start_background_loop(context, schedule_cdc_receiver),
    );

    server.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn database_check(Extension(state): Extension<SharedContext>) -> &'static str {
    state.connection.ping().await.unwrap();

    "OK"
}
