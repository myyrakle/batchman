use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::{
    actions::{
        self, create_schdule::CreateSchduleRequest, list_schedules::ListSchedulesRequest,
        patch_schedule::PatchScheduleRequest,
    },
    context::SharedContext,
    db::entities,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ListSchedulesQuery {
    pub schedule_id: Option<i64>,      // schedule id
    pub contains_name: Option<String>, // name contains text
    pub name: Option<String>,          // exact name
    pub enabled: Option<bool>,         // enabled status
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateScheduleBody {
    pub name: String,                 // schedule name
    pub job_name: String,             // job name
    pub cron_expression: String,      // cron expression
    pub task_definition_id: i64,      // task definition id
    pub command: Option<String>,      // docker run command
    pub timezone: Option<String>,     // timezone text (example: "Asia/Seoul")
    pub timezone_offset: Option<i32>, // timezone offset (in minutes) (example: 540=9:00 for "Asia/Seoul")
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PatchScheduleBody {
    pub name: Option<String>,
    pub job_name: Option<String>,
    pub cron_expression: Option<String>,
    pub task_definition_id: Option<i64>,
    pub command: Option<String>,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i32>,
    pub enabled: Option<bool>,
}

pub async fn create_schedule(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<CreateScheduleBody>,
) -> impl IntoResponse {
    let result =
        actions::create_schdule::create_schdule(state, CreateSchduleRequest { request_body: body })
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
    let request = PatchScheduleRequest { schedule_id, body };

    let result = actions::patch_schedule::patch_schedule(context, request).await;

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

#[derive(Serialize)]
pub struct ListSchedulesItem {
    pub id: i64,
    pub name: String,
    pub job_name: String,
    pub cron_expression: String,
    pub task_definition_id: i64,
    pub command: Option<String>,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i32>,
    pub enabled: bool,
}

impl From<entities::schedule::Model> for ListSchedulesItem {
    fn from(schedule: entities::schedule::Model) -> Self {
        ListSchedulesItem {
            id: schedule.id,
            name: schedule.name,
            job_name: schedule.job_name,
            cron_expression: schedule.cron_expression,
            task_definition_id: schedule.task_definition_id,
            command: schedule.command,
            timezone: schedule.timezone,
            timezone_offset: schedule.timezone_offset,
            enabled: schedule.enabled,
        }
    }
}

#[derive(Serialize)]
pub struct ListSchedulesResponse {
    schedules: Vec<ListSchedulesItem>,
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
