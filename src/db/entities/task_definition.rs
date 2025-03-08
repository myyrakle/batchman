use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task_definition")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub version: String,

    pub image: String,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,

    pub memory_limit: Option<u32>,
    pub cpu_limit: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
