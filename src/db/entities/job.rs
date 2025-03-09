use sea_orm::entity::prelude::*;

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum JobStatus {
    #[sea_orm(string_value = "Pending")]
    Pending,
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
    pub id: u64, // primary key
    pub name: String, // job name

    pub task_definition_id: u64, // task definition id

    pub status: JobStatus, // job status

    pub submited_at: Option<chrono::NaiveDateTime>, // job submited time
    pub started_at: Option<chrono::NaiveDateTime>,  // job started time
    pub finished_at: Option<chrono::NaiveDateTime>, // job finished time

    pub container_id: Option<String>, // batch container id (docker container id)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
