use rocket::{get, post, State};
use tracing::info;

use schedule::TaskManager;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/tasks")]
pub async fn list_tasks(task_manager: &State<TaskManager>) -> Result<WebResponse<Vec<schedule::TaskListItem>>> {
    let items = task_manager.list().await?;
    WebResponse::new(items).into_result()
}

#[post("/api/tasks/<task_name>/run")]
pub async fn run_task(task_name: &str, task_manager: &State<TaskManager>) -> Result<WebResponse<String>> {
    info!("run task: {}", task_name);
    task_manager.run_now(task_name).await?;
    WebResponse::new("ok".to_string()).into_result()
}

#[post("/api/tasks/<task_name>/pause")]
pub async fn pause_task(task_name: &str, task_manager: &State<TaskManager>) -> Result<WebResponse<String>> {
    info!("pause task: {}", task_name);
    task_manager.pause(task_name).await?;
    WebResponse::new("ok".to_string()).into_result()
}

#[post("/api/tasks/<task_name>/resume")]
pub async fn resume_task(task_name: &str, task_manager: &State<TaskManager>) -> Result<WebResponse<String>> {
    info!("resume task: {}", task_name);
    task_manager.resume(task_name).await?;
    WebResponse::new("ok".to_string()).into_result()
}

#[post("/api/tasks/<task_name>/stop")]
pub async fn stop_task(task_name: &str, task_manager: &State<TaskManager>) -> Result<WebResponse<String>> {
    info!("stop task: {}", task_name);
    task_manager.stop(task_name).await?;
    WebResponse::new("ok".to_string()).into_result()
}
