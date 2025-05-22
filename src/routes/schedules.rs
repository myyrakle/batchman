use axum::{
    Extension, Json,
    body::Body,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    actions::{self, create_schdule::CreateSchduleRequest},
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
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
