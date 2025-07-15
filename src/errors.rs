use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    TaskDefinitionNotFound,
    JobNotFound,
    JobAlreadyFinished,
    JobAlreadyFailed,
    JobHasNoContainerID,
    ContainerIDNotFound,
    ScheduleNotFound,
    CronExpressionIsInvalid(String),
    ContainerNotFound,
    ContainerFailedToKill(String),
    ContainerFailedToStart(String),
    ContainerFailedToInspect(String),
    ContainerFailedToRemove(String),
    JobLogExpired,
    IO(std::io::Error),
    Seaorm(sea_orm::DbErr),
    SerdeJson(serde_json::Error),
}

impl Error {
    pub fn error_code(&self) -> String {
        match self {
            Error::TaskDefinitionNotFound => "TASK_DEFINITION_NOT_FOUND".to_string(),
            Error::JobNotFound => "JOB_NOT_FOUND".to_string(),
            Error::JobAlreadyFinished => "JOB_ALREADY_FINISHED".to_string(),
            Error::JobAlreadyFailed => "JOB_ALREADY_FAILED".to_string(),
            Error::JobHasNoContainerID => "JOB_HAS_NO_CONTAINER_ID".to_string(),
            Error::ContainerIDNotFound => "CONTAINER_ID_NOT_FOUND".to_string(),
            Error::ScheduleNotFound => "SCHEDULE_NOT_FOUND".to_string(),
            Error::CronExpressionIsInvalid(_) => "INVALID_CRON_EXPRESSION".to_string(),
            Error::ContainerNotFound => "CONTAINER_NOT_FOUND".to_string(),
            Error::ContainerFailedToKill(_) => "FAILED_TO_KILL_CONTAINER".to_string(),
            Error::ContainerFailedToStart(_) => "FAILED_TO_START_CONTAINER".to_string(),
            Error::ContainerFailedToInspect(_) => "FAILED_TO_INSPECT_CONTAINER".to_string(),
            Error::ContainerFailedToRemove(_) => "FAILED_TO_REMOVE_CONTAINER".to_string(),
            Error::JobLogExpired => "JOB_LOG_EXPIRED".to_string(),
            Error::IO(_) => "IO_ERROR".to_string(),
            Error::Seaorm(_) => "DATABASE_ERROR".to_string(),
            Error::SerdeJson(_) => "JSON_SERIALIZATION_ERROR".to_string(),
        }
    }

    pub fn into_json_response(self) -> String {
        let error_response: ErrorResponse = self.into();
        serde_json::to_string(&error_response).unwrap_or("{}".to_string())
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl From<&Error> for String {
    fn from(error: &Error) -> String {
        match error {
            Error::TaskDefinitionNotFound => "Task definition not found".to_string(),
            Error::JobNotFound => "Job not found".to_string(),
            Error::JobAlreadyFinished => "Job is already finished".to_string(),
            Error::JobAlreadyFailed => "Job is already failed".to_string(),
            Error::JobHasNoContainerID => "Job has no container ID".to_string(),
            Error::ContainerIDNotFound => "Container ID not found".to_string(),
            Error::ScheduleNotFound => "Schedule not found".to_string(),
            Error::CronExpressionIsInvalid(expr) => format!("Invalid Cron Expression: {}", expr),
            Error::ContainerNotFound => "Container not found".to_string(),
            Error::ContainerFailedToKill(err) => format!("Failed to kill container: {}", err),
            Error::ContainerFailedToStart(err) => format!("Failed to start container: {}", err),
            Error::ContainerFailedToInspect(err) => format!("Failed to inspect container: {}", err),
            Error::ContainerFailedToRemove(err) => format!("Failed to remove container: {}", err),
            Error::JobLogExpired => "Job log has expired and is no longer available".to_string(),
            Error::IO(err) => format!("I/O error: {}", err),
            Error::Seaorm(err) => format!("Database error: {}", err),
            Error::SerdeJson(err) => {
                format!("JSON serialization/deserialization error: {}", err)
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::Seaorm(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse {
            error_code: error.error_code(),
            message: error.to_string(),
        }
    }
}
