use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "schedule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64, // primary key
    pub name: String, // schedule name

    pub cron_expression: String, // cron expression

    pub task_definition_id: u64, // task definition id
    pub command: Option<String>, // docker run command

    pub created_at: Option<chrono::NaiveDateTime>, // job submited time
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
