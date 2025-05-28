#[derive(Debug)]
pub enum Error {
    TaskDefinitionNotFound,
    SeaormError(sea_orm::DbErr),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::TaskDefinitionNotFound => "Task definition not found".to_string(),
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
