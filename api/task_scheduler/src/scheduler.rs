use crate::types::*;
use crate::executor::TaskExecutorRegistry;
use crate::service::TaskService;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use uuid::Uuid;

pub struct TaskScheduler {
    db: DatabaseConnection,
    executor_registry: Arc<TaskExecutorRegistry>,
    task_service: TaskService,
    running_tasks: Arc<RwLock<HashMap<String, RunningTask>>>,
    is_running: Arc<RwLock<bool>>,
}

#[derive(Debug, Clone)]
struct RunningTask {
    task_id: i64,
    execution_id: String,
    started_at: DateTime<Utc>,
    cancel_handle: tokio::task::AbortHandle,
}

impl TaskScheduler {
    pub async fn new(db: DatabaseConnection) -> anyhow::Result<Self> {
        let executor_registry = Arc::new(TaskExecutorRegistry::default());
        let task_service = TaskService::new(db.clone());
        
        Ok(Self {
            db,
            executor_registry,
            task_service,
            running_tasks: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        tracing::info!("Starting task scheduler");
        
        *self.is_running.write().await = true;
        
        // Start the main scheduler loop
        self.start_scheduler_loop().await?;
        
        // Start cleanup task for finished executions
        self.start_cleanup_task().await?;
        
        tracing::info!("Task scheduler started successfully");
        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        tracing::info!("Stopping task scheduler");
        
        *self.is_running.write().await = false;
        
        // Cancel all running tasks
        let running_tasks = self.running_tasks.read().await;
        for (execution_id, task) in running_tasks.iter() {
            tracing::info!("Cancelling running task: {}", execution_id);
            task.cancel_handle.abort();
        }
        drop(running_tasks);
        
        tracing::info!("Task scheduler stopped");
        Ok(())
    }

    async fn start_scheduler_loop(&self) -> anyhow::Result<()> {
        let is_running = self.is_running.clone();
        let task_service = self.task_service.clone();
        let scheduler = self.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Check every minute
            
            loop {
                interval.tick().await;
                
                if !*is_running.read().await {
                    break;
                }
                
                // Load enabled tasks and check if any need to run
                if let Ok(tasks) = task_service.get_enabled_tasks().await {
                    let now = Utc::now();
                    
                    for task in tasks {
                        if scheduler.should_run_task(&task, now).await {
                            if let Err(e) = scheduler.execute_task(task).await {
                                tracing::error!("Failed to execute task: {}", e);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    async fn should_run_task(&self, task: &TaskConfig, now: DateTime<Utc>) -> bool {
        // Simple implementation - in production you'd want proper cron parsing
        match &task.schedule_config {
            ScheduleConfig::Cron { expression: _ } => {
                // For now, run every hour (simplified)
                if let Some(next_run) = task.next_run_time {
                    now >= next_run
                } else {
                    true // First run
                }
            }
            ScheduleConfig::Interval { seconds: _seconds } => {
                if let Some(next_run) = task.next_run_time {
                    now >= next_run
                } else {
                    true // First run
                }
            }
            ScheduleConfig::Once { run_at } => {
                now >= *run_at && task.next_run_time.is_none()
            }
        }
    }

    pub async fn execute_task_now(&self, task_id: i64) -> anyhow::Result<String> {
        let task = self.task_service.get_task_by_id(task_id).await?
            .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;
        
        self.execute_task(task).await
    }

    async fn execute_task(&self, task: TaskConfig) -> anyhow::Result<String> {
        let execution_id = Uuid::new_v4().to_string();
        let task_id = task.id;
        let task_name = task.name.clone();
        
        // Check if task is still enabled
        if !matches!(task.status, TaskStatus::Enabled) {
            tracing::warn!("Skipping disabled task: {}", task_name);
            return Ok(execution_id);
        }
        
        // Check concurrent execution limit
        let running_count = self.count_running_tasks_for_task(task_id).await;
        if running_count >= task.max_concurrent as usize {
            tracing::warn!("Task {} has reached max concurrent limit ({})", task_name, task.max_concurrent);
            return Ok(execution_id);
        }
        
        // Get executor
        let executor = self.executor_registry.get_executor(&task.task_type)
            .ok_or_else(|| anyhow::anyhow!("Unknown task type: {}", task.task_type))?;
        
        // Create execution record
        let started_at = Utc::now();
        self.task_service.create_execution_record(
            task_id,
            &execution_id,
            ExecutionStatus::Running,
            started_at,
        ).await?;
        
        // Create execution context
        let logger = ExecutionLogger::new(execution_id.clone());
        let context = ExecutionContext {
            task_id,
            execution_id: execution_id.clone(),
            logger,
            started_at,
        };
        
        // Spawn execution task
        let task_config = task.task_config.clone();
        let timeout_duration = Duration::from_secs(task.timeout_seconds as u64);
        let task_service = self.task_service.clone();
        let running_tasks = self.running_tasks.clone();
        let execution_id_clone = execution_id.clone();
        let task_name_clone = task_name.clone();
        
        let handle = tokio::spawn(async move {
            let result = tokio::time::timeout(
                timeout_duration,
                executor.execute(task_config, context)
            ).await;
            
            let (exec_status, error_message, output_summary) = match result {
                Ok(Ok(task_result)) => {
                    if task_result.success {
                        (ExecutionStatus::Success, None, task_result.output)
                    } else {
                        (ExecutionStatus::Failed, task_result.error, task_result.output)
                    }
                }
                Ok(Err(e)) => {
                    (ExecutionStatus::Failed, Some(e.to_string()), None)
                }
                Err(_) => {
                    (ExecutionStatus::Timeout, Some("Task execution timed out".to_string()), None)
                }
            };
            
            let finished_at = Utc::now();
            let duration_ms = (finished_at - started_at).num_milliseconds() as i32;
            
            // Update execution record
            if let Err(e) = task_service.update_execution_record(
                &execution_id_clone,
                exec_status.clone(),
                Some(finished_at),
                Some(duration_ms),
                error_message,
                output_summary,
            ).await {
                tracing::error!("Failed to update execution record: {}", e);
            }
            
            // Remove from running tasks
            running_tasks.write().await.remove(&execution_id_clone);
            
            tracing::info!("Task {} execution {} completed with status: {:?}", 
                task_name_clone, execution_id_clone, exec_status);
        });
        
        // Track running task
        let running_task = RunningTask {
            task_id,
            execution_id: execution_id.clone(),
            started_at,
            cancel_handle: handle.abort_handle(),
        };
        
        self.running_tasks.write().await.insert(execution_id.clone(), running_task);
        
        tracing::info!("Started execution {} for task: {}", execution_id, task_name);
        Ok(execution_id)
    }

    async fn count_running_tasks_for_task(&self, task_id: i64) -> usize {
        self.running_tasks
            .read()
            .await
            .values()
            .filter(|task| task.task_id == task_id)
            .count()
    }

    async fn start_cleanup_task(&self) -> anyhow::Result<()> {
        let mut cleanup_interval = interval(Duration::from_secs(300)); // 5 minutes
        let running_tasks = self.running_tasks.clone();
        
        tokio::spawn(async move {
            loop {
                cleanup_interval.tick().await;
                
                let mut tasks_to_remove = Vec::new();
                {
                    let running = running_tasks.read().await;
                    let now = Utc::now();
                    
                    for (execution_id, task) in running.iter() {
                        // Remove tasks that have been running for more than 1 hour
                        if (now - task.started_at).num_hours() > 1 {
                            tasks_to_remove.push(execution_id.clone());
                        }
                    }
                }
                
                if !tasks_to_remove.is_empty() {
                    let mut running = running_tasks.write().await;
                    for execution_id in tasks_to_remove {
                        if let Some(task) = running.remove(&execution_id) {
                            task.cancel_handle.abort();
                            tracing::warn!("Cleaned up long-running task: {}", execution_id);
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    pub async fn get_running_tasks(&self) -> Vec<RunningTask> {
        self.running_tasks.read().await.values().cloned().collect()
    }

    pub async fn cancel_execution(&self, execution_id: &str) -> anyhow::Result<()> {
        if let Some(task) = self.running_tasks.write().await.remove(execution_id) {
            task.cancel_handle.abort();
            
            // Update execution record
            self.task_service.update_execution_record(
                execution_id,
                ExecutionStatus::Cancelled,
                Some(Utc::now()),
                None,
                Some("Cancelled by user".to_string()),
                None,
            ).await?;
            
            tracing::info!("Cancelled task execution: {}", execution_id);
        }
        
        Ok(())
    }
}

impl Clone for TaskScheduler {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            executor_registry: self.executor_registry.clone(),
            task_service: self.task_service.clone(),
            running_tasks: self.running_tasks.clone(),
            is_running: self.is_running.clone(),
        }
    }
}
