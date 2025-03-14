use axum::{
    Extension, Json,
    body::Body,
    extract::Query,
    http::Response,
    response::{self, IntoResponse},
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::actions;

#[derive(Deserialize)]
pub struct ListTaskDefinitionsQuery {
    task_definition_id: Option<u64>,
    contains_name: Option<String>,
    name: Option<String>,
}

pub type ListTaskDefinitionsItem = crate::db::entities::task_definition::Model;

#[derive(Serialize)]
pub struct ListTaskDefinitionsResponse {
    task_definitions: Vec<ListTaskDefinitionsItem>,
}

pub async fn list_task_definitions(
    Query(query): Query<ListTaskDefinitionsQuery>,
    Extension(connection): Extension<DatabaseConnection>,
) -> response::Response {
    let task_definitions = actions::task_definition::list_task_definitions(
        actions::task_definition::ListTaskDefinitionsParams {
            connection: &connection,
            task_definition_id: query.task_definition_id,
            contains_name: query.contains_name,
            name: query.name,
        },
    )
    .await;

    match task_definitions {
        Ok(task_definitions) => {
            let response = ListTaskDefinitionsResponse { task_definitions };
            Json(response).into_response()
        }
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}

#[derive(Deserialize)]
pub struct CreateTaskDefinitionBody {
    pub name: String,            // task name
    pub version: Option<String>, // task version

    pub image: String,           // docker image
    pub command: Option<String>, // docker run command
    pub args: Option<String>,    // docker run arguments
    pub env: Option<String>,     // environment variables

    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

pub async fn create_task_definition(
    Json(query): Json<CreateTaskDefinitionBody>,
    Extension(connection): Extension<DatabaseConnection>,
) -> response::Response {
    unimplemented!();
}
