pub(crate) mod actions;
pub(crate) mod db;
pub(crate) mod docker;

use axum::{Router, routing::get};
use db::{entities, setup_schema};

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();

    setup_schema(&connection).await;

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:13939")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
