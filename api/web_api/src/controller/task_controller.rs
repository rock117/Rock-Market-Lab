use anyhow::anyhow;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{State, get, post, put, delete, routes, Route};
use serde_json::Value;
use service::task_scheduler_service::{
    TaskSchedulerService, CreateTaskRequest, UpdateTaskRequest,
    TaskResponse, TaskListResponse, ExecutionListResponse, ExecutionLogResponse,
};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

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
#[get("/api/task-types")]
pub async fn get_task_types() -> Result<WebResponse<Vec<TaskTypeSchema>>> {
    WebResponse::new(vec![
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
    ]).into_result()
}

// 获取任务列表
#[get("/api/tasks?<page>&<page_size>&<task_type>&<status>")]
pub async fn get_tasks(
    service: &State<TaskSchedulerService>,
    page: Option<u64>,
    page_size: Option<u64>,
    task_type: Option<String>,
    status: Option<String>,
) -> Result<WebResponse<TaskListResponse>> {
    let response = service.get_tasks(
        page.unwrap_or(0),
        page_size.unwrap_or(20),
        task_type,
        status,
    ).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(response).into_result()
}

// 创建任务
#[post("/api/tasks", data = "<request>")]
pub async fn create_task(
    service: &State<TaskSchedulerService>,
    request: Json<CreateTaskRequest>,
) -> Result<WebResponse<TaskResponse>> {
    let task = service.create_task(request.into_inner()).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(task).into_result()
}

// 获取任务详情
#[get("/api/tasks/<id>")]
pub async fn get_task(
    service: &State<TaskSchedulerService>,
    id: i64,
) -> Result<WebResponse<Option<TaskResponse>>> {
    let task = service.get_task(id).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(task).into_result()
}

// 更新任务
#[put("/api/tasks/<id>", data = "<request>")]
pub async fn update_task(
    service: &State<TaskSchedulerService>,
    id: i64,
    request: Json<UpdateTaskRequest>,
) -> Result<WebResponse<Option<TaskResponse>>> {
    let task = service.update_task(id, request.into_inner()).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(task).into_result()
}

// 删除任务（软删除）
#[delete("/api/tasks/<id>")]
pub async fn delete_task(
    service: &State<TaskSchedulerService>,
    id: i64,
) -> Result<WebResponse<bool>> {
    let deleted = service.delete_task(id).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(deleted).into_result()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTaskResponse {
    pub success: bool,
    pub message: String,
    pub task_id: i64,
    pub execution_id: String,
}

// 手动执行任务
#[post("/api/tasks/<id>/run")]
pub async fn run_task(
    _service: &State<TaskSchedulerService>,
    id: i64,
) -> Result<WebResponse<RunTaskResponse>> {
    // TODO: 实际实现需要调用调度器
    WebResponse::new(RunTaskResponse {
        success: true,
        message: "Task execution started".to_string(),
        task_id: id,
        execution_id: format!("exec_{}", uuid::Uuid::new_v4()),
    }).into_result()
}

// 暂停任务
#[post("/api/tasks/<id>/pause")]
pub async fn pause_task(
    service: &State<TaskSchedulerService>,
    id: i64,
) -> Result<WebResponse<Option<TaskResponse>>> {
    let task = service.pause_task(id).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(task).into_result()
}

// 恢复任务
#[post("/api/tasks/<id>/resume")]
pub async fn resume_task(
    service: &State<TaskSchedulerService>,
    id: i64,
) -> Result<WebResponse<Option<TaskResponse>>> {
    let task = service.enable_task(id).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(task).into_result()
}

// 获取执行记录
#[get("/api/tasks/<task_id>/executions?<page>&<page_size>")]
pub async fn get_executions(
    service: &State<TaskSchedulerService>,
    task_id: i64,
    page: Option<u64>,
    page_size: Option<u64>,
) -> Result<WebResponse<ExecutionListResponse>> {
    let response = service.get_executions(
        task_id,
        page.unwrap_or(0),
        page_size.unwrap_or(20),
    ).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(response).into_result()
}

// 获取执行日志
#[get("/api/tasks/executions/<execution_id>/logs")]
pub async fn get_execution_logs(
    service: &State<TaskSchedulerService>,
    execution_id: &str,
) -> Result<WebResponse<Vec<ExecutionLogResponse>>> {
    let logs = service.get_execution_logs(execution_id).await.map_err(|e| anyhow!(e))?;
    WebResponse::new(logs).into_result()
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
        get_executions,
        get_execution_logs
    ]
}
