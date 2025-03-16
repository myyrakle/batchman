use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::Response,
    response::{self, IntoResponse},
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::actions;

#[derive(Deserialize, Debug, Clone)]
pub struct ListTaskDefinitionsQuery {
    pub task_definition_id: Option<i64>,
    pub contains_name: Option<String>,
    pub name: Option<String>,
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
    let task_definitions = actions::list_task_definition::list_task_definitions(
        actions::list_task_definition::ListTaskDefinitionsParams {
            connection: &connection,
            query,
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

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTaskDefinitionBody {
    pub name: String,              // task name
    pub image: String,             // docker image
    pub command: Option<String>,   // docker run command
    pub args: Option<String>,      // docker run arguments
    pub env: Option<String>,       // environment variables
    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

pub async fn create_task_definition(
    Extension(connection): Extension<DatabaseConnection>,
    Json(query): Json<CreateTaskDefinitionBody>,
) -> response::Response {
    let task_definition = actions::create_task_definition::create_task_definition(
        actions::create_task_definition::CreateDefinitionParams {
            connection: &connection,
            request: query,
        },
    )
    .await;

    match task_definition {
        Ok(task_definition) => Json(task_definition).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PatchTaskDefinitionBody {
    pub image: Option<String>,     // docker image
    pub command: Option<String>,   // docker run command
    pub args: Option<String>,      // docker run arguments
    pub env: Option<String>,       // environment variables
    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

pub async fn patch_task_definition(
    Path(task_definition_id): Path<i64>,
    Extension(connection): Extension<DatabaseConnection>,
    Json(query): Json<PatchTaskDefinitionBody>,
) -> response::Response {
    let task_definition = actions::patch_task_definition::patch_task_definition(
        actions::patch_task_definition::PatchDefinitionParams {
            connection: &connection,
            task_definition_id,
            request: query,
        },
    )
    .await;

    match task_definition {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
