use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::domain::container::ContainerType;

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Default)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum JobStatus {
    #[sea_orm(string_value = "Pending")]
    #[default]
    Pending,
    #[sea_orm(string_value = "Starting")]
    Starting,
    #[sea_orm(string_value = "Running")]
    Running,
    #[sea_orm(string_value = "Finished")]
    Finished,
    #[sea_orm(string_value = "Failed")]
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "job")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // primary key
    pub name: String, // job name

    pub task_definition_id: i64, // task definition id

    pub status: JobStatus, // job status

    pub submited_at: Option<chrono::DateTime<Utc>>, // job submited time
    pub started_at: Option<chrono::DateTime<Utc>>,  // job started time
    pub finished_at: Option<chrono::DateTime<Utc>>, // job finished time

    pub container_type: ContainerType,
    pub container_id: Option<String>, // batch container id (docker container id)
    pub exit_code: Option<i32>,       // batch exit code
    pub error_message: Option<String>, // batch error message

    pub log_expire_after: Option<chrono::DateTime<Utc>>, // log expire time
    pub log_expired: bool,                               // log expired

    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
