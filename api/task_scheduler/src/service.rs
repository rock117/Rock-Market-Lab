use crate::types::*;
use chrono::{DateTime, Utc};
use ::entity::{scheduled_task, task_execution};
use sea_orm::*;
use serde_json::Value;

#[derive(Clone)]
pub struct TaskService {
    db: DatabaseConnection,
}

impl TaskService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, task: CreateTaskRequest) -> anyhow::Result<i64> {
        let now = Utc::now().into();
        
        let task_model = scheduled_task::ActiveModel {
            name: Set(task.name),
            description: Set(task.description),
            task_type: Set(task.task_type),
            schedule_type: Set(String::from(task.schedule_type)),
            schedule_config: Set(serde_json::to_string(&task.schedule_config)?),
            task_config: Set(serde_json::to_string(&task.task_config)?),
            status: Set(String::from(TaskStatus::Enabled)),
            max_concurrent: Set(task.max_concurrent.unwrap_or(1)),
            timeout_seconds: Set(task.timeout_seconds.unwrap_or(300)),
            retry_count: Set(task.retry_count.unwrap_or(0)),
            retry_interval_seconds: Set(task.retry_interval_seconds.unwrap_or(60)),
            created_at: Set(now),
            updated_at: Set(now),
            created_by: Set(task.created_by),
            next_run_time: Set(None),
            ..Default::default()
        };

        let result = scheduled_task::Entity::insert(task_model)
            .exec(&self.db)
            .await?;

        Ok(result.last_insert_id)
    }

    pub async fn update_task(&self, task_id: i64, task: UpdateTaskRequest) -> anyhow::Result<()> {
        let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Task not found"))?
            .into();

        if let Some(name) = task.name {
            task_model.name = Set(name);
        }
        if let Some(description) = task.description {
            task_model.description = Set(description);
        }
        if let Some(schedule_config) = task.schedule_config {
            task_model.schedule_config = Set(serde_json::to_string(&schedule_config)?);
        }
        if let Some(task_config) = task.task_config {
            task_model.task_config = Set(serde_json::to_string(&task_config)?);
        }
        if let Some(status) = task.status {
            task_model.status = Set(String::from(status));
        }
        if let Some(max_concurrent) = task.max_concurrent {
            task_model.max_concurrent = Set(max_concurrent);
        }
        if let Some(timeout_seconds) = task.timeout_seconds {
            task_model.timeout_seconds = Set(timeout_seconds);
        }
        if let Some(retry_count) = task.retry_count {
            task_model.retry_count = Set(retry_count);
        }
        if let Some(retry_interval_seconds) = task.retry_interval_seconds {
            task_model.retry_interval_seconds = Set(retry_interval_seconds);
        }

        task_model.updated_at = Set(Utc::now().into());

        scheduled_task::Entity::update(task_model)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_task(&self, task_id: i64) -> anyhow::Result<()> {
        let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Task not found"))?
            .into();

        task_model.status = Set(String::from(TaskStatus::Deleted));
        task_model.updated_at = Set(Utc::now().into());

        scheduled_task::Entity::update(task_model)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_task_by_id(&self, task_id: i64) -> anyhow::Result<Option<TaskConfig>> {
        let task = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?;

        match task {
            Some(task) => Ok(Some(self.model_to_config(task)?)),
            None => Ok(None),
        }
    }

    pub async fn get_tasks(&self, params: GetTasksRequest) -> anyhow::Result<TaskListResponse> {
        let mut query = scheduled_task::Entity::find()
            .filter(scheduled_task::Column::Status.ne("deleted"));

        if let Some(task_type) = &params.task_type {
            query = query.filter(scheduled_task::Column::TaskType.eq(task_type));
        }

        if let Some(status) = &params.status {
            query = query.filter(scheduled_task::Column::Status.eq(String::from(status.clone())));
        }

        let total = query.clone().count(&self.db).await?;

        let tasks = query
            .order_by_desc(scheduled_task::Column::CreatedAt)
            .paginate(&self.db, params.page_size.unwrap_or(20))
            .fetch_page(params.page.unwrap_or(0))
            .await?;

        let task_configs: Result<Vec<TaskConfig>, _> = tasks
            .into_iter()
            .map(|task| self.model_to_config(task))
            .collect();

        Ok(TaskListResponse {
            tasks: task_configs?,
            total: total as i32,
            page: params.page.unwrap_or(0),
            page_size: params.page_size.unwrap_or(20),
        })
    }

    pub async fn get_enabled_tasks(&self) -> anyhow::Result<Vec<TaskConfig>> {
        let tasks = scheduled_task::Entity::find()
            .filter(scheduled_task::Column::Status.eq("enabled"))
            .all(&self.db)
            .await?;

        let task_configs: Result<Vec<TaskConfig>, _> = tasks
            .into_iter()
            .map(|task| self.model_to_config(task))
            .collect();

        task_configs
    }

    pub async fn create_execution_record(
        &self,
        task_id: i64,
        execution_id: &str,
        status: ExecutionStatus,
        started_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let execution_model = task_execution::ActiveModel {
            task_id: Set(task_id),
            execution_id: Set(execution_id.to_string()),
            status: Set(String::from(status)),
            started_at: Set(started_at.into()),
            retry_attempt: Set(0),
            ..Default::default()
        };

        task_execution::Entity::insert(execution_model)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn update_execution_record(
        &self,
        execution_id: &str,
        status: ExecutionStatus,
        finished_at: Option<DateTime<Utc>>,
        duration_ms: Option<i32>,
        error_message: Option<String>,
        output_summary: Option<String>,
    ) -> anyhow::Result<()> {
        let mut execution_model: task_execution::ActiveModel = task_execution::Entity::find()
            .filter(task_execution::Column::ExecutionId.eq(execution_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Execution record not found"))?
            .into();

        execution_model.status = Set(String::from(status));
        if let Some(finished_at) = finished_at {
            execution_model.finished_at = Set(Some(finished_at.into()));
        }
        if let Some(duration_ms) = duration_ms {
            execution_model.duration_ms = Set(Some(duration_ms));
        }
        if let Some(error_message) = error_message {
            execution_model.error_message = Set(Some(error_message));
        }
        if let Some(output_summary) = output_summary {
            execution_model.output_summary = Set(Some(output_summary));
        }

        task_execution::Entity::update(execution_model)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_execution_records(&self, params: GetExecutionRecordsRequest) -> anyhow::Result<ExecutionRecordListResponse> {
        let mut query = task_execution::Entity::find();

        if let Some(task_id) = params.task_id {
            query = query.filter(task_execution::Column::TaskId.eq(task_id));
        }

        if let Some(status) = &params.status {
            query = query.filter(task_execution::Column::Status.eq(String::from(status.clone())));
        };

        let total = query.clone().count(&self.db).await?;

        let executions = query
            .order_by_desc(task_execution::Column::StartedAt)
            .paginate(&self.db, params.page_size.unwrap_or(20))
            .fetch_page(params.page.unwrap_or(0))
            .await?;

        let execution_records: Vec<TaskExecutionRecord> = executions
            .into_iter()
            .map(|exec| self.execution_model_to_record(exec))
            .collect();

        Ok(ExecutionRecordListResponse {
            executions: execution_records,
            total: total as i32,
            page: params.page.unwrap_or(0),
            page_size: params.page_size.unwrap_or(20),
        })
    }

    fn model_to_config(&self, task: scheduled_task::Model) -> anyhow::Result<TaskConfig> {
        let schedule_config: ScheduleConfig = serde_json::from_str(&task.schedule_config)?;
        let task_config: Value = serde_json::from_str(&task.task_config)?;

        Ok(TaskConfig {
            id: task.id,
            name: task.name,
            description: task.description,
            task_type: task.task_type,
            schedule_type: ScheduleType::from(task.schedule_type),
            schedule_config,
            task_config,
            status: TaskStatus::from(task.status),
            max_concurrent: task.max_concurrent,
            timeout_seconds: task.timeout_seconds,
            retry_count: task.retry_count,
            retry_interval_seconds: task.retry_interval_seconds,
            next_run_time: task.next_run_time.map(|dt| dt.into()),
        })
    }

    fn execution_model_to_record(&self, exec: task_execution::Model) -> TaskExecutionRecord {
        TaskExecutionRecord {
            id: exec.id,
            task_id: exec.task_id,
            execution_id: exec.execution_id,
            status: ExecutionStatus::from(exec.status),
            started_at: exec.started_at.into(),
            finished_at: exec.finished_at.map(|dt| dt.into()),
            duration_ms: exec.duration_ms,
            error_message: exec.error_message,
            output_summary: exec.output_summary,
            retry_attempt: exec.retry_attempt,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: ScheduleType,
    pub schedule_config: ScheduleConfig,
    pub task_config: Value,
    pub max_concurrent: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub retry_count: Option<i32>,
    pub retry_interval_seconds: Option<i32>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub schedule_config: Option<ScheduleConfig>,
    pub task_config: Option<Value>,
    pub status: Option<TaskStatus>,
    pub max_concurrent: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub retry_count: Option<i32>,
    pub retry_interval_seconds: Option<i32>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetTasksRequest {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub task_type: Option<String>,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskConfig>,
    pub total: i32,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetExecutionRecordsRequest {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub task_id: Option<i64>,
    pub status: Option<ExecutionStatus>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionRecordListResponse {
    pub executions: Vec<TaskExecutionRecord>,
    pub total: i32,
    pub page: u64,
    pub page_size: u64,
}
