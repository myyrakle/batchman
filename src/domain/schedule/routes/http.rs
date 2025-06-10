use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    context::SharedContext,
    domain::schedule::dto::{
        CreateSchduleRequest, CreateScheduleBody, ListSchedulesItem, ListSchedulesQuery,
        ListSchedulesRequest, ListSchedulesResponse, PatchScheduleBody, PatchScheduleRequest,
    },
    errors,
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
        Err(errors::Error::CronExpressionIsInvalid(message)) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::new(
                errors::Error::CronExpressionIsInvalid(message).into_json_response(),
            ))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
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
        Err(errors::Error::ScheduleNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::new(
                errors::Error::ScheduleNotFound.into_json_response(),
            ))
            .unwrap(),
        Err(errors::Error::CronExpressionIsInvalid(message)) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::new(
                errors::Error::CronExpressionIsInvalid(message).into_json_response(),
            ))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn delete_schedule(
    Path(schedule_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
) -> impl IntoResponse {
    let result = context.schedule_service.delete_schedule(schedule_id).await;

    match result {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap(),
        Err(errors::Error::ScheduleNotFound) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::new(
                errors::Error::ScheduleNotFound.into_json_response(),
            ))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn list_schedules(
    Query(query): Query<ListSchedulesQuery>,
    Extension(context): Extension<SharedContext>,
) -> impl IntoResponse {
    let schedules = context
        .schedule_service
        .list_schedules(ListSchedulesRequest { query })
        .await;

    match schedules {
        Ok(schedules) => {
            let response = ListSchedulesResponse {
                schedules: schedules.into_iter().map(ListSchedulesItem::from).collect(),
            };
            Json(response).into_response()
        }
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}
