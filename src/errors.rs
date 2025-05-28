#[derive(Debug)]
pub enum Error {
    TaskDefinitionNotFound,
    JobNotFound,
    JobAlreadyFinished,
    JobAlreadyFailed,
    JobHasNoContainerID,
    ContainerIDNotFound,
    SeaormError(sea_orm::DbErr),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::TaskDefinitionNotFound => "Task definition not found".to_string(),
            Error::JobNotFound => "Job not found".to_string(),
            Error::JobAlreadyFinished => "Job is already finished".to_string(),
            Error::JobAlreadyFailed => "Job is already failed".to_string(),
            Error::JobHasNoContainerID => "Job has no container ID".to_string(),
            Error::ContainerIDNotFound => "Container ID not found".to_string(),
            Error::SeaormError(err) => format!("Database error: {}", err),
        }
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::SeaormError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
