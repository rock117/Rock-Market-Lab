use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_execution")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_id: i64,
    pub execution_id: String,
    pub status: String,
    pub started_at: DateTimeWithTimeZone,
    pub finished_at: Option<DateTimeWithTimeZone>,
    pub duration_ms: Option<i32>,
    pub error_message: Option<String>,
    pub output_summary: Option<String>,
    pub retry_attempt: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scheduled_task::Entity",
        from = "Column::TaskId",
        to = "super::scheduled_task::Column::Id"
    )]
    ScheduledTask,
    #[sea_orm(has_many = "super::task_execution_log::Entity")]
    TaskExecutionLogs,
}

impl Related<super::scheduled_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduledTask.def()
    }
}

impl Related<super::task_execution_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskExecutionLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Running,
    Success,
    Failed,
    Timeout,
    Cancelled,
}

impl From<ExecutionStatus> for String {
    fn from(status: ExecutionStatus) -> Self {
        match status {
            ExecutionStatus::Running => "running".to_string(),
            ExecutionStatus::Success => "success".to_string(),
            ExecutionStatus::Failed => "failed".to_string(),
            ExecutionStatus::Timeout => "timeout".to_string(),
            ExecutionStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl From<String> for ExecutionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "running" => ExecutionStatus::Running,
            "success" => ExecutionStatus::Success,
            "failed" => ExecutionStatus::Failed,
            "timeout" => ExecutionStatus::Timeout,
            "cancelled" => ExecutionStatus::Cancelled,
            _ => ExecutionStatus::Failed,
        }
    }
}
