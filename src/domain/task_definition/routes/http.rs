use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, Query},
    http::Response,
    response::{self, IntoResponse},
};

use crate::{
    context::SharedContext,
    domain::task_definition::{
        dto::{
            CreateDefinitionRequest, CreateTaskDefinitionBody, DeleteDefinitionRequest,
            ListTaskDefinitionsItem, ListTaskDefinitionsQuery, ListTaskDefinitionsRequest,
            PatchDefinitionRequest, PatchTaskDefinitionBody,
        },
        entities,
    },
    errors,
};

impl From<entities::task_definition::Model> for ListTaskDefinitionsItem {
    fn from(model: entities::task_definition::Model) -> Self {
        ListTaskDefinitionsItem {
            id: model.id,
            name: model.name,
            version: model.version,
            description: model.description,
            image: model.image,
            command: model.command,
            args: model.args,
            env: model.env,
            memory_limit: model.memory_limit,
            cpu_limit: model.cpu_limit,
            created_at: model.created_at,
            enabled: model.enabled,
            is_latest: model.is_latest,
        }
    }
}

pub async fn list_task_definitions(
    Query(query): Query<ListTaskDefinitionsQuery>,
    Extension(context): Extension<SharedContext>,
) -> response::Response {
    let response = context
        .task_definition_service
        .list_task_definitions(ListTaskDefinitionsRequest { query })
        .await;

    match response {
        Ok(response) => Json(response).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn create_task_definition(
    Extension(context): Extension<SharedContext>,
    Json(body): Json<CreateTaskDefinitionBody>,
) -> response::Response {
    let task_definition_id = context
        .task_definition_service
        .create_task_definition(CreateDefinitionRequest { request_body: body })
        .await;

    match task_definition_id {
        Ok(task_definition_id) => Json(task_definition_id).into_response(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn patch_task_definition(
    Path(task_definition_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
    Json(query): Json<PatchTaskDefinitionBody>,
) -> response::Response {
    let result = context
        .task_definition_service
        .patch_task_definition(PatchDefinitionRequest {
            task_definition_id,
            request: query,
        })
        .await;

    match result {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(errors::Error::TaskDefinitionNotFound) => Response::builder()
            .status(404)
            .body(Body::new(
                errors::Error::TaskDefinitionNotFound.into_json_response(),
            ))
            .unwrap(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}

pub async fn delete_task_definition(
    Path(task_definition_id): Path<i64>,
    Extension(context): Extension<SharedContext>,
) -> response::Response {
    let result = context
        .task_definition_service
        .delete_task_definition(DeleteDefinitionRequest { task_definition_id })
        .await;

    match result {
        Ok(_) => Response::builder().status(200).body(Body::empty()).unwrap(),
        Err(error) => Response::builder()
            .status(500)
            .body(Body::new(error.into_json_response()))
            .unwrap(),
    }
}
