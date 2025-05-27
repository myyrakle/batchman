use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    actions::{self},
    context::SharedContext,
    domain::schedule::dto::{
        CreateSchduleRequest, CreateScheduleBody, ListSchedulesItem, ListSchedulesQuery,
        ListSchedulesRequest, ListSchedulesResponse, PatchScheduleBody, PatchScheduleRequest,
    },
};

pub async fn create_schedule(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<CreateScheduleBody>,
) -> impl IntoResponse {
    let result = state
        .schedule_service
        .create_schdule(CreateSchduleRequest { request_body: body })
        .await;

    match result {
        Ok(_) => Json(()).into_response(),
        Err(error) => {
            if error.to_string().starts_with("Invalid Cron Expression") {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::new(error.to_string()))
                    .unwrap();
            }

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::new(error.to_string()))
                .unwrap()
        }
    }
}

pub async fn patch_schedule(
    Path(schedule_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
    Json(body): Json<PatchScheduleBody>,
) -> impl IntoResponse {
    let result = context
        .schedule_service
        .patch_schedule(PatchScheduleRequest { schedule_id, body })
        .await;

    match result {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap(),
        Err(error) => {
            // Check if the error message indicates "Schedule not found"
            if error.to_string().starts_with("Invalid Cron Expression") {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::new(error.to_string()))
                    .unwrap();
            } else if error.to_string().starts_with("Schedule not found") {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::new(error.to_string()))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::new(error.to_string()))
                    .unwrap()
            }
        }
    }
}

pub async fn delete_schedule(
    Path(schedule_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
) -> impl IntoResponse {
    let result = actions::delete_schedule::delete_schedule(context, schedule_id).await;

    match result {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap(),
        Err(error) => {
            // Check if the error message indicates "Schedule not found"
            if error.to_string().starts_with("Schedule not found") {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::new(error.to_string()))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::new(error.to_string()))
                    .unwrap()
            }
        }
    }
}

pub async fn list_schedules(
    Query(query): Query<ListSchedulesQuery>,
    Extension(context): Extension<SharedContext>,
) -> impl IntoResponse {
    let schedules =
        actions::list_schedules::list_schedules(context, ListSchedulesRequest { query }).await;

    match schedules {
        Ok(schedules) => {
            let response = ListSchedulesResponse {
                schedules: schedules.into_iter().map(ListSchedulesItem::from).collect(),
            };
            Json(response).into_response()
        }
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
