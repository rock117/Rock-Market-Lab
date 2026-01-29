use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{State, get, post, put, delete, routes, Route};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait, ColumnTrait};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,
    pub schedule_type: String, // "cron", "interval", "once"
    pub schedule_config: Value, // JSON config
    pub task_config: Value,     // JSON config
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
    pub total: i32,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionRecordResponse {
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
    pub executions: Vec<ExecutionRecordResponse>,
    pub total: i32,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskTypeSchema {
    pub name: String,
    pub description: String,
    pub fields: Vec<TaskConfigField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfigField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
    pub default_value: Option<Value>,
    pub options: Option<Vec<String>>,
}

// 获取任务类型列表
#[get("/task-types")]
pub async fn get_task_types() -> Json<Vec<TaskTypeSchema>> {
    Json(vec![
        TaskTypeSchema {
            name: "HTTP Request".to_string(),
            description: "Make HTTP requests to external APIs".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "url".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: "Target URL".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "method".to_string(),
                    field_type: "select".to_string(),
                    required: false,
                    description: "HTTP method".to_string(),
                    default_value: Some(Value::String("GET".to_string())),
                    options: Some(vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]),
                },
                TaskConfigField {
                    name: "headers".to_string(),
                    field_type: "object".to_string(),
                    required: false,
                    description: "HTTP headers".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "body".to_string(),
                    field_type: "text".to_string(),
                    required: false,
                    description: "Request body".to_string(),
                    default_value: None,
                    options: None,
                },
            ],
        },
        TaskTypeSchema {
            name: "Shell Command".to_string(),
            description: "Execute shell commands".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "command".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: "Command to execute".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "working_dir".to_string(),
                    field_type: "string".to_string(),
                    required: false,
                    description: "Working directory".to_string(),
                    default_value: None,
                    options: None,
                },
            ],
        },
    ])
}

// 获取任务列表
#[get("/tasks?<page>&<page_size>&<task_type>&<status>")]
pub async fn get_tasks(
    db: &State<DatabaseConnection>,
    page: Option<u64>,
    page_size: Option<u64>,
    task_type: Option<String>,
    status: Option<String>,
) -> Json<TaskListResponse> {
    use entity::scheduled_task;
    use sea_orm::*;

    let mut query = scheduled_task::Entity::find()
        .filter(scheduled_task::Column::Status.ne("deleted"));

    if let Some(task_type) = task_type {
        query = query.filter(scheduled_task::Column::TaskType.eq(task_type));
    }

    if let Some(status) = status {
        query = query.filter(scheduled_task::Column::Status.eq(status));
    }

    let total = query.clone().count(db.inner()).await.unwrap_or(0);

    let tasks = query
        .order_by_desc(scheduled_task::Column::CreatedAt)
        .paginate(db.inner(), page_size.unwrap_or(20))
        .fetch_page(page.unwrap_or(0))
        .await
        .unwrap_or_default();

    let task_responses: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse {
            id: task.id,
            name: task.name,
            description: task.description,
            task_type: task.task_type,
            schedule_type: task.schedule_type,
            schedule_config: serde_json::from_str(&task.schedule_config).unwrap_or(Value::Null),
            task_config: serde_json::from_str(&task.task_config).unwrap_or(Value::Null),
            status: task.status,
            max_concurrent: task.max_concurrent,
            timeout_seconds: task.timeout_seconds,
            retry_count: task.retry_count,
            retry_interval_seconds: task.retry_interval_seconds,
            created_at: task.created_at.into(),
            updated_at: task.updated_at.into(),
            created_by: task.created_by,
            next_run_time: task.next_run_time.map(|dt| dt.into()),
        })
        .collect();

    Json(TaskListResponse {
        tasks: task_responses,
        total: total as i32,
        page: page.unwrap_or(0),
        page_size: page_size.unwrap_or(20),
    })
}

// 创建任务
#[post("/tasks", data = "<request>")]
pub async fn create_task(
    db: &State<DatabaseConnection>,
    request: Json<CreateTaskRequest>,
) -> Json<TaskResponse> {
    use entity::scheduled_task;
    use sea_orm::*;
    use chrono::Utc;

    let now = Utc::now().into();
    
    let task_model = scheduled_task::ActiveModel {
        name: Set(request.name.clone()),
        description: Set(request.description.clone()),
        task_type: Set(request.task_type.clone()),
        schedule_type: Set(request.schedule_type.clone()),
        schedule_config: Set(serde_json::to_string(&request.schedule_config).unwrap_or_default()),
        task_config: Set(serde_json::to_string(&request.task_config).unwrap_or_default()),
        status: Set("enabled".to_string()),
        max_concurrent: Set(request.max_concurrent.unwrap_or(1)),
        timeout_seconds: Set(request.timeout_seconds.unwrap_or(300)),
        retry_count: Set(request.retry_count.unwrap_or(0)),
        retry_interval_seconds: Set(request.retry_interval_seconds.unwrap_or(60)),
        created_at: Set(now),
        updated_at: Set(now),
        created_by: Set(request.created_by.clone()),
        next_run_time: Set(None),
        ..Default::default()
    };

    let result = scheduled_task::Entity::insert(task_model)
        .exec(db.inner())
        .await
        .expect("Failed to create task");

    // 返回创建的任务
    let created_task = scheduled_task::Entity::find_by_id(result.last_insert_id)
        .one(db.inner())
        .await
        .expect("Failed to fetch created task")
        .expect("Task not found");

    Json(TaskResponse {
        id: created_task.id,
        name: created_task.name,
        description: created_task.description,
        task_type: created_task.task_type,
        schedule_type: created_task.schedule_type,
        schedule_config: serde_json::from_str(&created_task.schedule_config).unwrap_or(Value::Null),
        task_config: serde_json::from_str(&created_task.task_config).unwrap_or(Value::Null),
        status: created_task.status,
        max_concurrent: created_task.max_concurrent,
        timeout_seconds: created_task.timeout_seconds,
        retry_count: created_task.retry_count,
        retry_interval_seconds: created_task.retry_interval_seconds,
        created_at: created_task.created_at.into(),
        updated_at: created_task.updated_at.into(),
        created_by: created_task.created_by,
        next_run_time: created_task.next_run_time.map(|dt| dt.into()),
    })
}

// 获取任务详情
#[get("/tasks/<id>")]
pub async fn get_task(
    db: &State<DatabaseConnection>,
    id: i64,
) -> Option<Json<TaskResponse>> {
    use entity::scheduled_task;
    use sea_orm::*;

    let task = scheduled_task::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .ok()??;

    Some(Json(TaskResponse {
        id: task.id,
        name: task.name,
        description: task.description,
        task_type: task.task_type,
        schedule_type: task.schedule_type,
        schedule_config: serde_json::from_str(&task.schedule_config).unwrap_or(Value::Null),
        task_config: serde_json::from_str(&task.task_config).unwrap_or(Value::Null),
        status: task.status,
        max_concurrent: task.max_concurrent,
        timeout_seconds: task.timeout_seconds,
        retry_count: task.retry_count,
        retry_interval_seconds: task.retry_interval_seconds,
        created_at: task.created_at.into(),
        updated_at: task.updated_at.into(),
        created_by: task.created_by,
        next_run_time: task.next_run_time.map(|dt| dt.into()),
    }))
}

// 更新任务
#[put("/tasks/<id>", data = "<request>")]
pub async fn update_task(
    db: &State<DatabaseConnection>,
    id: i64,
    request: Json<UpdateTaskRequest>,
) -> Option<Json<TaskResponse>> {
    use entity::scheduled_task;
    use sea_orm::*;
    use chrono::Utc;

    let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .ok()??
        .into();

    if let Some(name) = &request.name {
        task_model.name = Set(name.clone());
    }
    if let Some(description) = &request.description {
        task_model.description = Set(description.clone());
    }
    if let Some(schedule_config) = &request.schedule_config {
        task_model.schedule_config = Set(serde_json::to_string(schedule_config).unwrap_or_default());
    }
    if let Some(task_config) = &request.task_config {
        task_model.task_config = Set(serde_json::to_string(task_config).unwrap_or_default());
    }
    if let Some(status) = &request.status {
        task_model.status = Set(status.clone());
    }
    if let Some(max_concurrent) = request.max_concurrent {
        task_model.max_concurrent = Set(max_concurrent);
    }
    if let Some(timeout_seconds) = request.timeout_seconds {
        task_model.timeout_seconds = Set(timeout_seconds);
    }
    if let Some(retry_count) = request.retry_count {
        task_model.retry_count = Set(retry_count);
    }
    if let Some(retry_interval_seconds) = request.retry_interval_seconds {
        task_model.retry_interval_seconds = Set(retry_interval_seconds);
    }

    task_model.updated_at = Set(Utc::now().into());

    let updated_task = task_model.update(db.inner())
        .await
        .ok()?;

    Some(Json(TaskResponse {
        id: updated_task.id,
        name: updated_task.name,
        description: updated_task.description,
        task_type: updated_task.task_type,
        schedule_type: updated_task.schedule_type,
        schedule_config: serde_json::from_str(&updated_task.schedule_config).unwrap_or(Value::Null),
        task_config: serde_json::from_str(&updated_task.task_config).unwrap_or(Value::Null),
        status: updated_task.status,
        max_concurrent: updated_task.max_concurrent,
        timeout_seconds: updated_task.timeout_seconds,
        retry_count: updated_task.retry_count,
        retry_interval_seconds: updated_task.retry_interval_seconds,
        created_at: updated_task.created_at.into(),
        updated_at: updated_task.updated_at.into(),
        created_by: updated_task.created_by,
        next_run_time: updated_task.next_run_time.map(|dt| dt.into()),
    }))
}

// 删除任务（软删除）
#[delete("/tasks/<id>")]
pub async fn delete_task(
    db: &State<DatabaseConnection>,
    id: i64,
) -> Option<Json<serde_json::Value>> {
    use entity::scheduled_task;
    use sea_orm::*;
    use chrono::Utc;

    let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .ok()??
        .into();

    task_model.status = Set("deleted".to_string());
    task_model.updated_at = Set(Utc::now().into());

    task_model.update(db.inner())
        .await
        .ok()?;

    Some(Json(serde_json::json!({ "success": true })))
}

// 手动执行任务
#[post("/tasks/<id>/run")]
pub async fn run_task(
    db: &State<DatabaseConnection>,
    id: i64,
) -> Json<serde_json::Value> {
    // 这里暂时返回成功，实际实现需要调用调度器
    Json(serde_json::json!({
        "success": true,
        "message": "Task execution started",
        "execution_id": format!("exec_{}", uuid::Uuid::new_v4())
    }))
}

// 暂停任务
#[post("/tasks/<id>/pause")]
pub async fn pause_task(
    db: &State<DatabaseConnection>,
    id: i64,
) -> Option<Json<serde_json::Value>> {
    use entity::scheduled_task;
    use sea_orm::*;
    use chrono::Utc;

    let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .ok()??
        .into();

    task_model.status = Set("paused".to_string());
    task_model.updated_at = Set(Utc::now().into());

    task_model.update(db.inner())
        .await
        .ok()?;

    Some(Json(serde_json::json!({ "success": true })))
}

// 恢复任务
#[post("/tasks/<id>/resume")]
pub async fn resume_task(
    db: &State<DatabaseConnection>,
    id: i64,
) -> Option<Json<serde_json::Value>> {
    use entity::scheduled_task;
    use sea_orm::*;
    use chrono::Utc;

    let mut task_model: scheduled_task::ActiveModel = scheduled_task::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .ok()??
        .into();

    task_model.status = Set("enabled".to_string());
    task_model.updated_at = Set(Utc::now().into());

    task_model.update(db.inner())
        .await
        .ok()?;

    Some(Json(serde_json::json!({ "success": true })))
}

// 获取执行记录
#[get("/task-executions?<page>&<page_size>&<task_id>&<status>")]
pub async fn get_executions(
    db: &State<DatabaseConnection>,
    page: Option<u64>,
    page_size: Option<u64>,
    task_id: Option<i64>,
    status: Option<String>,
) -> Json<ExecutionListResponse> {
    use entity::task_execution;
    use sea_orm::*;

    let mut query = task_execution::Entity::find();

    if let Some(task_id) = task_id {
        query = query.filter(task_execution::Column::TaskId.eq(task_id));
    }

    if let Some(status) = status {
        query = query.filter(task_execution::Column::Status.eq(status));
    }

    let total = query.clone().count(db.inner()).await.unwrap_or(0);

    let executions = query
        .order_by_desc(task_execution::Column::StartedAt)
        .paginate(db.inner(), page_size.unwrap_or(20))
        .fetch_page(page.unwrap_or(0))
        .await
        .unwrap_or_default();

    let execution_responses: Vec<ExecutionRecordResponse> = executions
        .into_iter()
        .map(|exec| ExecutionRecordResponse {
            id: exec.id,
            task_id: exec.task_id,
            execution_id: exec.execution_id,
            status: exec.status,
            started_at: exec.started_at.into(),
            finished_at: exec.finished_at.map(|dt| dt.into()),
            duration_ms: exec.duration_ms,
            error_message: exec.error_message,
            output_summary: exec.output_summary,
            retry_attempt: exec.retry_attempt,
        })
        .collect();

    Json(ExecutionListResponse {
        executions: execution_responses,
        total: total as i32,
        page: page.unwrap_or(0),
        page_size: page_size.unwrap_or(20),
    })
}

pub fn routes() -> Vec<Route> {
    routes![
        get_task_types,
        get_tasks,
        create_task,
        get_task,
        update_task,
        delete_task,
        run_task,
        pause_task,
        resume_task,
        get_executions
    ]
}
