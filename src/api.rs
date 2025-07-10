pub(crate) mod background;
pub(crate) mod context;
pub(crate) mod db;
pub(crate) mod domain;
pub(crate) mod errors;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod web;

use std::sync::Arc;

use axum::{
    Extension, Router,
    http::Uri,
    response::Response,
    routing::{delete, get, patch, post},
};
use background::scheduler::ScheduleCDCEvent;
use context::SharedContext;
use db::setup_schema;

pub fn app(context: SharedContext) -> Router {
    let api_router = Router::new()
        .route("/healthz", get(root))
        .route("/database-check", get(database_check))
        .route(
            "/task-definitions",
            get(domain::task_definition::routes::http::list_task_definitions),
        )
        .route(
            "/task-definitions",
            post(domain::task_definition::routes::http::create_task_definition),
        )
        .route(
            "/task-definitions/{task_definition_id}",
            patch(domain::task_definition::routes::http::patch_task_definition),
        )
        .route(
            "/task-definitions/{task_definition_id}",
            delete(domain::task_definition::routes::http::delete_task_definition),
        )
        .route("/jobs/submit", post(domain::job::routes::http::submit_job))
        .route("/jobs/stop", post(domain::job::routes::http::stop_job))
        .route("/jobs", get(domain::job::routes::http::list_jobs))
        .route(
            "/jobs/{job_id}/logs",
            get(domain::job::routes::http::list_job_logs),
        )
        .route(
            "/schedules",
            get(domain::schedule::routes::http::list_schedules),
        )
        .route(
            "/schedules",
            post(domain::schedule::routes::http::create_schedule),
        )
        .route(
            "/schedules/{schedule_id}",
            patch(domain::schedule::routes::http::patch_schedule),
        )
        .route(
            "/schedules/{schedule_id}",
            delete(domain::schedule::routes::http::delete_schedule),
        )
        .layer(Extension(context));

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(web::index_html))
        .route("/bundle.js", get(web::bundle_js))
        .route("/index.css", get(web::index_css))
        .nest("/api", api_router)
        .fallback(fallback)
}

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();
    setup_schema(&connection).await;

    let (schedule_cdc_sender, schedule_cdc_receiver) =
        tokio::sync::mpsc::channel::<ScheduleCDCEvent>(8);

    let context = Arc::new(context::Context::new(
        connection.clone(), // Pass cloned connection for context
        schedule_cdc_sender,
    ));

    let router = app(context.clone()); // Use the app function

    let listener = tokio::net::TcpListener::bind("0.0.0.0:13939")
        .await
        .unwrap();

    let (server, _) = tokio::join!(
        axum::serve(listener, router), // Use the router from app()
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

async fn fallback(uri: Uri) -> Response {
    if uri.path().starts_with("/api") {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("Not Found"))
            .unwrap();
    }

    web::index_html().await
}
