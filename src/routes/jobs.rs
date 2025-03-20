use axum::{
    Extension, Json,
    body::Body,
    http::Response,
    response::{self, IntoResponse},
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::actions::{self, submit_job::SubmitJobParams};

#[derive(Deserialize, Debug, Clone)]
pub struct SubmitJobBody {
    pub task_definition_id: i64,
    pub job_name: String,
}

pub async fn submit_job(
    Extension(connection): Extension<DatabaseConnection>,
    Json(body): Json<SubmitJobBody>,
) -> response::Response {
    let result = actions::submit_job::submit_job(SubmitJobParams {
        connection: &connection,
        request_body: body,
    })
    .await;

    match result {
        Ok(id) => Json(id).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
