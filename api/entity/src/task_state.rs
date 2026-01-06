//! `SeaORM` Entity

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "task_state"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub task_name: String,
    pub status: String,
    pub paused: bool,
    pub stopped: bool,
    pub last_started_at: Option<String>,
    pub last_ended_at: Option<String>,
    pub last_success_count: i32,
    pub last_fail_count: i32,
    pub updated_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TaskName,
    Status,
    Paused,
    Stopped,
    LastStartedAt,
    LastEndedAt,
    LastSuccessCount,
    LastFailCount,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TaskName,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::TaskName => ColumnType::String(StringLen::N(100u32)).def(),
            Self::Status => ColumnType::String(StringLen::N(30u32)).def(),
            Self::Paused => ColumnType::Boolean.def(),
            Self::Stopped => ColumnType::Boolean.def(),
            Self::LastStartedAt => ColumnType::String(StringLen::N(30u32)).def().null(),
            Self::LastEndedAt => ColumnType::String(StringLen::N(30u32)).def().null(),
            Self::LastSuccessCount => ColumnType::Integer.def(),
            Self::LastFailCount => ColumnType::Integer.def(),
            Self::UpdatedAt => ColumnType::String(StringLen::N(30u32)).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
