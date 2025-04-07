use axum::{
    Extension, Json,
    body::Body,
    http::Response,
    response::{self, IntoResponse},
};
use serde::Deserialize;

use crate::{
    actions::{self, stop_job::StopJobParams, submit_job::SubmitJobParams},
    context::SharedContext,
};

#[derive(Deserialize, Debug, Clone)]
pub struct SubmitJobBody {
    pub task_definition_id: i64,
    pub job_name: String,
}

pub async fn submit_job(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<SubmitJobBody>,
) -> response::Response {
    let job_id = actions::submit_job::submit_job(SubmitJobParams {
        connection: &state.connection,
        request_body: body,
    })
    .await;

    match job_id {
        Ok(job_id) => Json(job_id).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopJobBody {
    pub job_id: i64,
}

pub async fn stop_job(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<StopJobBody>,
) -> response::Response {
    let result = actions::stop_job::stop_job(StopJobParams {
        connection: &state.connection,
        request_body: body,
    })
    .await;

    match result {
        Ok(_) => Json(()).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
