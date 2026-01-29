use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "scheduled_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: String,
    pub schedule_config: String, // JSON
    pub task_config: String,     // JSON
    pub status: String,
    pub max_concurrent: i32,
    pub timeout_seconds: i32,
    pub retry_count: i32,
    pub retry_interval_seconds: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub created_by: Option<String>,
    pub next_run_time: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task_execution::Entity")]
    TaskExecutions,
}

impl Related<super::task_execution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskExecutions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Enabled,
    Paused,
    Deleted,
}

impl From<TaskStatus> for String {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::Enabled => "enabled".to_string(),
            TaskStatus::Paused => "paused".to_string(),
            TaskStatus::Deleted => "deleted".to_string(),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "enabled" => TaskStatus::Enabled,
            "paused" => TaskStatus::Paused,
            "deleted" => TaskStatus::Deleted,
            _ => TaskStatus::Paused,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleType {
    Cron,
    Interval,
    Once,
}

impl From<ScheduleType> for String {
    fn from(schedule_type: ScheduleType) -> Self {
        match schedule_type {
            ScheduleType::Cron => "cron".to_string(),
            ScheduleType::Interval => "interval".to_string(),
            ScheduleType::Once => "once".to_string(),
        }
    }
}

impl From<String> for ScheduleType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "cron" => ScheduleType::Cron,
            "interval" => ScheduleType::Interval,
            "once" => ScheduleType::Once,
            _ => ScheduleType::Cron,
        }
    }
}
