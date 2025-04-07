use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::Response,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::{actions, context::SharedContext, db::entities};

#[derive(Deserialize, Debug, Clone)]
pub struct ListTaskDefinitionsQuery {
    pub task_definition_id: Option<i64>,
    pub contains_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct ListTaskDefinitionsItem {
    pub id: i64,      // primary key
    pub name: String, // task name
    pub version: i64, // task version

    pub image: String,                // docker image
    pub command: Option<Vec<String>>, // docker run command
    pub args: Option<String>,         // docker run arguments
    pub env: Option<String>,          // environment variables

    pub memory_limit: Option<u32>, // memory limit in MB
    pub cpu_limit: Option<u32>,    // cpu limit (default 1024)
}

impl From<entities::task_definition::Model> for ListTaskDefinitionsItem {
    fn from(model: entities::task_definition::Model) -> Self {
        ListTaskDefinitionsItem {
            id: model.id,
            name: model.name,
            version: model.version,
            image: model.image,
            command: model
                .command
                .map(|command| serde_json::from_str(&command).unwrap_or_default()),
            args: model.args,
            env: model.env,
            memory_limit: model.memory_limit,
            cpu_limit: model.cpu_limit,
        }
    }
}

#[derive(Serialize)]
pub struct ListTaskDefinitionsResponse {
    task_definitions: Vec<ListTaskDefinitionsItem>,
}

pub async fn list_task_definitions(
    Query(query): Query<ListTaskDefinitionsQuery>,
    Extension(state): Extension<SharedContext>,
) -> response::Response {
    let task_definitions = actions::list_task_definition::list_task_definitions(
        actions::list_task_definition::ListTaskDefinitionsRequest {
            connection: &state.connection,
            query,
        },
    )
    .await;

    match task_definitions {
        Ok(task_definitions) => {
            let response = ListTaskDefinitionsResponse {
                task_definitions: task_definitions
                    .into_iter()
                    .map(ListTaskDefinitionsItem::from)
                    .collect(),
            };
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
    pub name: String,                 // task name
    pub image: String,                // docker image
    pub command: Option<Vec<String>>, // docker run command
    pub args: Option<String>,         // docker run arguments
    pub env: Option<String>,          // environment variables
    pub memory_limit: Option<u32>,    // memory limit in MB
    pub cpu_limit: Option<u32>,       // cpu limit (default 1024)
}

pub async fn create_task_definition(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<CreateTaskDefinitionBody>,
) -> response::Response {
    let task_definition_id = actions::create_task_definition::create_task_definition(
        actions::create_task_definition::CreateDefinitionRequest {
            connection: &state.connection,
            request: body,
        },
    )
    .await;

    match task_definition_id {
        Ok(task_definition_id) => Json(task_definition_id).into_response(),
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
    Extension(state): Extension<SharedContext>,
    Json(query): Json<PatchTaskDefinitionBody>,
) -> response::Response {
    let result = actions::patch_task_definition::patch_task_definition(
        actions::patch_task_definition::PatchDefinitionRequest {
            connection: &state.connection,
            task_definition_id,
            request: query,
        },
    )
    .await;

    match result {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}

pub async fn delete_task_definition(
    Path(task_definition_id): Path<i64>,
    Extension(state): Extension<SharedContext>,
) -> response::Response {
    let result = actions::delete_task_definition::delete_task_definition(
        actions::delete_task_definition::DeleteDefinitionRequest {
            connection: &state.connection,
            task_definition_id,
        },
    )
    .await;

    match result {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.to_string()))
            .unwrap(),
    }
}
