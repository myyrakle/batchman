use axum::{
    Extension, Json,
    body::Body,
    extract::Query,
    http::Response,
    response::{self, IntoResponse},
};

use crate::{
    context::SharedContext,
    domain::job::dto::{
        ListJobsQuery, ListJobsRequest, StopJobBody, StopJobRequest, SubmitJobBody,
        SubmitJobRequest,
    },
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
            .body(Body::new(error.into_json_response()))
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
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn list_jobs(
    Extension(context): Extension<SharedContext>,
    Query(query): Query<ListJobsQuery>,
) -> response::Response {
    let result = context
        .job_service
        .list_jobs(ListJobsRequest {
            request_query: query,
        })
        .await;

    match result {
        Ok(response) => Json(response).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}
