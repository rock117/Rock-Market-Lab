use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_execution_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub execution_id: String,
    pub log_level: String,
    pub message: String,
    pub timestamp: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::task_execution::Entity",
        from = "Column::ExecutionId",
        to = "super::task_execution::Column::ExecutionId"
    )]
    TaskExecution,
}

impl Related<super::task_execution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskExecution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl From<LogLevel> for String {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::Debug => "DEBUG".to_string(),
        }
    }
}

impl From<String> for LogLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "DEBUG" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
}
