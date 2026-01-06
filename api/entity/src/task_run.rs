//! `SeaORM` Entity

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "task_run"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub task_name: String,
    pub status: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub success_count: i32,
    pub fail_count: i32,
    pub error: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TaskName,
    Status,
    StartedAt,
    EndedAt,
    SuccessCount,
    FailCount,
    Error,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::TaskName => ColumnType::String(StringLen::N(100u32)).def(),
            Self::Status => ColumnType::String(StringLen::N(30u32)).def(),
            Self::StartedAt => ColumnType::String(StringLen::N(30u32)).def(),
            Self::EndedAt => ColumnType::String(StringLen::N(30u32)).def().null(),
            Self::SuccessCount => ColumnType::Integer.def(),
            Self::FailCount => ColumnType::Integer.def(),
            Self::Error => ColumnType::String(StringLen::N(500u32)).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
