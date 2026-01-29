use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: ScheduleType,
    pub schedule_config: ScheduleConfig,
    pub task_config: Value,
    pub status: TaskStatus,
    pub max_concurrent: i32,
    pub timeout_seconds: i32,
    pub retry_count: i32,
    pub retry_interval_seconds: i32,
    pub next_run_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleConfig {
    Cron { expression: String },
    Interval { seconds: u64 },
    Once { run_at: DateTime<Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub task_id: i64,
    pub execution_id: String,
    pub logger: ExecutionLogger,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ExecutionLogger {
    pub execution_id: String,
}

impl ExecutionLogger {
    pub fn new(execution_id: String) -> Self {
        Self { execution_id }
    }

    pub async fn info(&self, message: &str) {
        tracing::info!("[{}] {}", self.execution_id, message);
        // TODO: Store to database
    }

    pub async fn warn(&self, message: &str) {
        tracing::warn!("[{}] {}", self.execution_id, message);
        // TODO: Store to database
    }

    pub async fn error(&self, message: &str) {
        tracing::error!("[{}] {}", self.execution_id, message);
        // TODO: Store to database
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

impl TaskResult {
    pub fn success(output: Option<String>) -> Self {
        Self {
            success: true,
            output,
            error: None,
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfigSchema {
    pub name: String,
    pub description: String,
    pub fields: Vec<TaskConfigField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfigField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
    pub default_value: Option<Value>,
    pub options: Option<Vec<String>>,
}

#[async_trait]
pub trait TaskExecutor: Send + Sync {
    async fn execute(&self, config: Value, context: ExecutionContext) -> anyhow::Result<TaskResult>;
    fn validate_config(&self, config: &Value) -> anyhow::Result<()>;
    fn get_schema(&self) -> TaskConfigSchema;
    fn get_type_name(&self) -> &'static str;
}

pub type TaskRegistry = HashMap<String, Box<dyn TaskExecutor>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionRecord {
    pub id: i64,
    pub task_id: i64,
    pub execution_id: String,
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i32>,
    pub error_message: Option<String>,
    pub output_summary: Option<String>,
    pub retry_attempt: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
