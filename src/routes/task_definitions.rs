use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::Response,
    response::{self, IntoResponse},
};

use crate::{
    actions,
    context::SharedContext,
    domain::task_definition::{
        dto::{
            CreateDefinitionRequest, CreateTaskDefinitionBody, DeleteDefinitionRequest,
            ListTaskDefinitionsItem, ListTaskDefinitionsQuery, ListTaskDefinitionsRequest,
            ListTaskDefinitionsResponse, PatchTaskDefinitionBody,
        },
        entities,
    },
};

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

pub async fn list_task_definitions(
    Query(query): Query<ListTaskDefinitionsQuery>,
    Extension(context): Extension<SharedContext>,
) -> response::Response {
    let task_definitions = actions::list_task_definition::list_task_definitions(
        context,
        ListTaskDefinitionsRequest { query },
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

pub async fn create_task_definition(
    Extension(state): Extension<SharedContext>,
    Json(body): Json<CreateTaskDefinitionBody>,
) -> response::Response {
    let task_definition_id = actions::create_task_definition::create_task_definition(
        state,
        CreateDefinitionRequest { request_body: body },
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

pub async fn patch_task_definition(
    Path(task_definition_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
    Json(query): Json<PatchTaskDefinitionBody>,
) -> response::Response {
    let result = actions::patch_task_definition::patch_task_definition(
        context.clone(),
        actions::patch_task_definition::PatchDefinitionRequest {
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
    Extension(context): Extension<SharedContext>,
) -> response::Response {
    let result = actions::delete_task_definition::delete_task_definition(
        context,
        DeleteDefinitionRequest { task_definition_id },
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
