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
    IO(std::io::Error),
    Seaorm(sea_orm::DbErr),
    SerdeJson(serde_json::Error),
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
