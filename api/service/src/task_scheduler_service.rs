use chrono::{DateTime, Utc};
use entity::{scheduled_task, task_execution, task_execution_log};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct TaskSchedulerService {
    db: DatabaseConnection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: String,
    pub schedule_config: Value,
    pub task_config: Value,
    pub max_concurrent: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub retry_count: Option<i32>,
    pub retry_interval_seconds: Option<i32>,
    pub created_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub schedule_config: Option<Value>,
    pub task_config: Option<Value>,
    pub status: Option<String>,
    pub max_concurrent: Option<i32>,
    pub timeout_seconds: Option<i32>,
    pub retry_count: Option<i32>,
    pub retry_interval_seconds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: String,
    pub schedule_config: Value,
    pub task_config: Value,
    pub status: String,
    pub max_concurrent: i32,
    pub timeout_seconds: i32,
    pub retry_count: i32,
    pub retry_interval_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub next_run_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskResponse>,
    pub total: i64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResponse {
    pub id: i64,
    pub task_id: i64,
    pub execution_id: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i32>,
    pub error_message: Option<String>,
    pub output_summary: Option<String>,
    pub retry_attempt: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionListResponse {
    pub executions: Vec<ExecutionResponse>,
    pub total: i64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionLogResponse {
    pub id: i64,
    pub execution_id: String,
    pub log_level: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl TaskSchedulerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, req: CreateTaskRequest) -> anyhow::Result<TaskResponse> {
        let now = Utc::now();

        let task_model = scheduled_task::ActiveModel {
            name: Set(req.name),
            description: Set(req.description),
            task_type: Set(req.task_type),
            schedule_type: Set(req.schedule_type),
            schedule_config: Set(serde_json::to_string(&req.schedule_config)?),
            task_config: Set(serde_json::to_string(&req.task_config)?),
            status: Set("enabled".to_string()),
            max_concurrent: Set(req.max_concurrent.unwrap_or(1)),
            timeout_seconds: Set(req.timeout_seconds.unwrap_or(300)),
            retry_count: Set(req.retry_count.unwrap_or(0)),
            retry_interval_seconds: Set(req.retry_interval_seconds.unwrap_or(60)),
            created_by: Set(req.created_by),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        let result = task_model.insert(&self.db).await?;
        self.model_to_response(result)
    }

    pub async fn get_task(&self, task_id: i64) -> anyhow::Result<Option<TaskResponse>> {
        let task = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?;

        match task {
            Some(t) => Ok(Some(self.model_to_response(t)?)),
            None => Ok(None),
        }
    }

    pub async fn get_tasks(
        &self,
        page: u64,
        page_size: u64,
        task_type: Option<String>,
        status: Option<String>,
    ) -> anyhow::Result<TaskListResponse> {
        let mut query =
            scheduled_task::Entity::find().filter(scheduled_task::Column::Status.ne("deleted"));

        if let Some(t) = task_type {
            query = query.filter(scheduled_task::Column::TaskType.eq(t));
        }

        if let Some(s) = status {
            query = query.filter(scheduled_task::Column::Status.eq(s));
        }

        let total = query.clone().count(&self.db).await?;

        let tasks = query
            .order_by_desc(scheduled_task::Column::CreatedAt)
            .paginate(&self.db, page_size)
            .fetch_page(page)
            .await?;

        let task_responses: Result<Vec<TaskResponse>, _> = tasks
            .into_iter()
            .map(|t| self.model_to_response(t))
            .collect();

        Ok(TaskListResponse {
            tasks: task_responses?,
            total: total as i64,
            page,
            page_size,
        })
    }

    pub async fn update_task(
        &self,
        task_id: i64,
        req: UpdateTaskRequest,
    ) -> anyhow::Result<Option<TaskResponse>> {
        let task = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?;

        match task {
            Some(t) => {
                let mut active_model: scheduled_task::ActiveModel = t.into();

                if let Some(name) = req.name {
                    active_model.name = Set(name);
                }
                if let Some(desc) = req.description {
                    active_model.description = Set(desc);
                }
                if let Some(config) = req.schedule_config {
                    active_model.schedule_config = Set(serde_json::to_string(&config)?);
                }
                if let Some(config) = req.task_config {
                    active_model.task_config = Set(serde_json::to_string(&config)?);
                }
                if let Some(status) = req.status {
                    active_model.status = Set(status);
                }
                if let Some(max_concurrent) = req.max_concurrent {
                    active_model.max_concurrent = Set(max_concurrent);
                }
                if let Some(timeout) = req.timeout_seconds {
                    active_model.timeout_seconds = Set(timeout);
                }
                if let Some(retry) = req.retry_count {
                    active_model.retry_count = Set(retry);
                }
                if let Some(interval) = req.retry_interval_seconds {
                    active_model.retry_interval_seconds = Set(interval);
                }

                active_model.updated_at = Set(Utc::now().into());

                let updated = active_model.update(&self.db).await?;
                Ok(Some(self.model_to_response(updated)?))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_task(&self, task_id: i64) -> anyhow::Result<bool> {
        let task = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?;

        match task {
            Some(t) => {
                let mut active_model: scheduled_task::ActiveModel = t.into();
                active_model.status = Set("deleted".to_string());
                active_model.updated_at = Set(Utc::now().into());
                active_model.update(&self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn enable_task(&self, task_id: i64) -> anyhow::Result<Option<TaskResponse>> {
        self.update_task_status(task_id, "enabled").await
    }

    pub async fn disable_task(&self, task_id: i64) -> anyhow::Result<Option<TaskResponse>> {
        self.update_task_status(task_id, "disabled").await
    }

    pub async fn pause_task(&self, task_id: i64) -> anyhow::Result<Option<TaskResponse>> {
        self.update_task_status(task_id, "paused").await
    }

    async fn update_task_status(
        &self,
        task_id: i64,
        status: &str,
    ) -> anyhow::Result<Option<TaskResponse>> {
        let task = scheduled_task::Entity::find_by_id(task_id)
            .one(&self.db)
            .await?;

        match task {
            Some(t) => {
                let mut active_model: scheduled_task::ActiveModel = t.into();
                active_model.status = Set(status.to_string());
                active_model.updated_at = Set(Utc::now().into());
                let updated = active_model.update(&self.db).await?;
                Ok(Some(self.model_to_response(updated)?))
            }
            None => Ok(None),
        }
    }

    pub async fn get_executions(
        &self,
        task_id: i64,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<ExecutionListResponse> {
        let query =
            task_execution::Entity::find().filter(task_execution::Column::TaskId.eq(task_id));

        let total = query.clone().count(&self.db).await?;

        let executions = query
            .order_by_desc(task_execution::Column::StartedAt)
            .paginate(&self.db, page_size)
            .fetch_page(page)
            .await?;

        let execution_responses: Vec<ExecutionResponse> = executions
            .into_iter()
            .map(|e| self.execution_model_to_response(e))
            .collect();

        Ok(ExecutionListResponse {
            executions: execution_responses,
            total: total as i64,
            page,
            page_size,
        })
    }

    pub async fn get_execution_logs(
        &self,
        execution_id: &str,
    ) -> anyhow::Result<Vec<ExecutionLogResponse>> {
        let logs = task_execution_log::Entity::find()
            .filter(task_execution_log::Column::ExecutionId.eq(execution_id))
            .order_by_asc(task_execution_log::Column::Timestamp)
            .all(&self.db)
            .await?;

        let log_responses: Vec<ExecutionLogResponse> = logs
            .into_iter()
            .map(|l| ExecutionLogResponse {
                id: l.id,
                execution_id: l.execution_id,
                log_level: l.log_level,
                message: l.message,
                timestamp: l.timestamp.into(),
            })
            .collect();

        Ok(log_responses)
    }

    fn model_to_response(&self, model: scheduled_task::Model) -> anyhow::Result<TaskResponse> {
        Ok(TaskResponse {
            id: model.id,
            name: model.name,
            description: model.description,
            task_type: model.task_type,
            schedule_type: model.schedule_type,
            schedule_config: serde_json::from_str(&model.schedule_config)?,
            task_config: serde_json::from_str(&model.task_config)?,
            status: model.status,
            max_concurrent: model.max_concurrent,
            timeout_seconds: model.timeout_seconds,
            retry_count: model.retry_count,
            retry_interval_seconds: model.retry_interval_seconds,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            created_by: model.created_by,
            next_run_time: model.next_run_time.map(|t| t.into()),
        })
    }

    fn execution_model_to_response(&self, model: task_execution::Model) -> ExecutionResponse {
        ExecutionResponse {
            id: model.id,
            task_id: model.task_id,
            execution_id: model.execution_id,
            status: model.status,
            started_at: model.started_at.into(),
            finished_at: model.finished_at.map(|t| t.into()),
            duration_ms: model.duration_ms,
            error_message: model.error_message,
            output_summary: model.output_summary,
            retry_attempt: model.retry_attempt,
        }
    }
}
