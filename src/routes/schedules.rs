use axum::{
    Extension, Json,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    actions::{self, create_schdule::CreateSchduleRequest, patch_schedule::PatchScheduleRequest},
    context::SharedContext,
};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateScheduleBody {
    pub name: String,                 // schedule name
    pub job_name: String,             // job name
    pub cron_expression: String,      // cron expression
    pub task_definition_id: i64,      // task definition id
    pub command: Option<String>,      // docker run command
    pub timezone: Option<String>,     // timezone text (example: "Asia/Seoul")
    pub timezone_offset: Option<i32>, // timezone offset (in minutes) (example: 540=9:00 for "Asia/Seoul")
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
        Err(error) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::new(error.to_string()))
            .unwrap(),
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
