use axum::{
    Extension, Json,
    body::Body,
    http::Response,
    response::{self, IntoResponse},
};

use crate::{
    context::SharedContext,
    domain::job::dto::{StopJobBody, StopJobRequest, SubmitJobBody, SubmitJobRequest},
};

pub async fn submit_job(
    Extension(context): Extension<SharedContext>,
    Json(body): Json<SubmitJobBody>,
) -> response::Response {
    let job_id = context
        .job_service
        .submit_job(SubmitJobRequest { request_body: body })
        .await;

    match job_id {
        Ok(job_id) => Json(job_id).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}

pub async fn stop_job(
    Extension(context): Extension<SharedContext>,
    Json(body): Json<StopJobBody>,
) -> response::Response {
    let result = context
        .job_service
        .stop_job(StopJobRequest { request_body: body })
        .await;

    match result {
        Ok(_) => Json(()).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
