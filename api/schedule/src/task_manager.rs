use anyhow::{anyhow, Context};
use chrono::Local;
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use entity::{task_run, task_state};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use entity::sea_orm::sea_query::Expr;

use crate::task::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub name: String,
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStateView {
    pub task_name: String,
    pub status: String,
    pub paused: bool,
    pub stopped: bool,
    pub last_started_at: Option<String>,
    pub last_ended_at: Option<String>,
    pub last_success_count: i32,
    pub last_fail_count: i32,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListItem {
    pub info: TaskInfo,
    pub state: TaskStateView,
}

#[derive(Clone)]
pub struct TaskManager {
    conn: DatabaseConnection,
    tasks: Arc<RwLock<HashMap<String, Arc<dyn Task>>>>,
}

impl TaskManager {
    pub async fn new(conn: DatabaseConnection, tasks: Vec<Arc<dyn Task>>) -> anyhow::Result<Self> {
        let mut map: HashMap<String, Arc<dyn Task>> = HashMap::new();
        for t in tasks {
            let name = task_name(&t);
            map.insert(name, t);
        }
        let mgr = Self {
            conn,
            tasks: Arc::new(RwLock::new(map)),
        };

        mgr.ensure_states_exist().await?;
        Ok(mgr)
    }

    pub async fn ensure_states_exist(&self) -> anyhow::Result<()> {
        let tasks = self.tasks.read().await;
        for name in tasks.keys() {
            self.ensure_state_row(name).await?;
        }
        Ok(())
    }

    async fn ensure_state_row(&self, task_name: &str) -> anyhow::Result<()> {
        let existing = task_state::Entity::find_by_id(task_name.to_string())
            .one(&self.conn)
            .await?;
        if existing.is_some() {
            return Ok(());
        }

        let now = now_str();
        let model = task_state::ActiveModel {
            task_name: Set(task_name.to_string()),
            status: Set("idle".to_string()),
            paused: Set(false),
            stopped: Set(false),
            last_started_at: Set(None),
            last_ended_at: Set(None),
            last_success_count: Set(0),
            last_fail_count: Set(0),
            updated_at: Set(Some(now)),
        };
        model.insert(&self.conn).await?;
        Ok(())
    }

    pub async fn list(&self) -> anyhow::Result<Vec<TaskListItem>> {
        let tasks = self.tasks.read().await;
        let mut items = Vec::with_capacity(tasks.len());
        for (name, task) in tasks.iter() {
            let state = self.get_state(name).await?;
            items.push(TaskListItem {
                info: TaskInfo {
                    name: name.clone(),
                    schedule: safe_get_schedule(task),
                },
                state,
            });
        }
        items.sort_by(|a, b| a.info.name.cmp(&b.info.name));
        Ok(items)
    }

    pub async fn run_now(&self, task_name: &str) -> anyhow::Result<()> {
        let task = {
            let tasks = self.tasks.read().await;
            tasks.get(task_name).cloned()
        }
        .ok_or_else(|| anyhow!("task not found: {}", task_name))?;

        let st = self.get_state(task_name).await?;
        if st.stopped {
            return Err(anyhow!("task is stopped: {}", task_name));
        }
        if st.paused {
            return Err(anyhow!("task is paused: {}", task_name));
        }

        self.set_running(task_name).await?;

        let run_id = self.create_run_row(task_name).await?;
        info!("[task] run_now start task={} run_id={}", task_name, run_id);

        let started = now_str();
        let res = task.run().await;
        let ended = now_str();

        let (status, success_count, fail_count, err_msg) = match res {
            Ok(()) => ("success".to_string(), 1, 0, None),
            Err(e) => {
                error!("[task] task failed name={} err={:?}", task_name, e);
                ("error".to_string(), 0, 1, Some(format!("{:?}", e)))
            }
        };

        self.finish_run_row(run_id, &ended, &status, success_count, fail_count, err_msg.as_deref())
            .await?;
        self.update_last_run_state(task_name, &status, &started, &ended, success_count, fail_count)
            .await?;

        Ok(())
    }

    pub async fn pause(&self, task_name: &str) -> anyhow::Result<()> {
        self.ensure_state_row(task_name).await?;
        let now = now_str();
        task_state::Entity::update_many()
            .col_expr(task_state::Column::Paused, Expr::value(true))
            .col_expr(task_state::Column::Status, Expr::value("paused"))
            .col_expr(task_state::Column::UpdatedAt, Expr::value(now))
            .filter(task_state::Column::TaskName.eq(task_name.to_string()))
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn resume(&self, task_name: &str) -> anyhow::Result<()> {
        self.ensure_state_row(task_name).await?;
        let now = now_str();
        task_state::Entity::update_many()
            .col_expr(task_state::Column::Paused, Expr::value(false))
            .col_expr(task_state::Column::Status, Expr::value("idle"))
            .col_expr(task_state::Column::UpdatedAt, Expr::value(now))
            .filter(task_state::Column::TaskName.eq(task_name.to_string()))
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn stop(&self, task_name: &str) -> anyhow::Result<()> {
        self.ensure_state_row(task_name).await?;
        let now = now_str();
        task_state::Entity::update_many()
            .col_expr(task_state::Column::Stopped, Expr::value(true))
            .col_expr(task_state::Column::Status, Expr::value("stopped"))
            .col_expr(task_state::Column::UpdatedAt, Expr::value(now))
            .filter(task_state::Column::TaskName.eq(task_name.to_string()))
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    pub async fn start(&self) {
        info!("task manager started (manual control mode)");
    }

    async fn get_state(&self, task_name: &str) -> anyhow::Result<TaskStateView> {
        self.ensure_state_row(task_name).await?;
        let row = task_state::Entity::find_by_id(task_name.to_string())
            .one(&self.conn)
            .await?
            .context("task_state not found")?;

        Ok(TaskStateView {
            task_name: row.task_name,
            status: row.status,
            paused: row.paused,
            stopped: row.stopped,
            last_started_at: row.last_started_at,
            last_ended_at: row.last_ended_at,
            last_success_count: row.last_success_count,
            last_fail_count: row.last_fail_count,
            updated_at: row.updated_at,
        })
    }

    async fn set_running(&self, task_name: &str) -> anyhow::Result<()> {
        let now = now_str();
        task_state::Entity::update_many()
            .col_expr(task_state::Column::Status, Expr::value("running"))
            .col_expr(task_state::Column::UpdatedAt, Expr::value(now))
            .filter(task_state::Column::TaskName.eq(task_name.to_string()))
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    async fn update_last_run_state(
        &self,
        task_name: &str,
        status: &str,
        started_at: &str,
        ended_at: &str,
        success_count: i32,
        fail_count: i32,
    ) -> anyhow::Result<()> {
        let now = now_str();
        task_state::Entity::update_many()
            .col_expr(task_state::Column::Status, Expr::value(status))
            .col_expr(task_state::Column::LastStartedAt, Expr::value(started_at))
            .col_expr(task_state::Column::LastEndedAt, Expr::value(ended_at))
            .col_expr(task_state::Column::LastSuccessCount, Expr::value(success_count))
            .col_expr(task_state::Column::LastFailCount, Expr::value(fail_count))
            .col_expr(task_state::Column::UpdatedAt, Expr::value(now))
            .filter(task_state::Column::TaskName.eq(task_name.to_string()))
            .exec(&self.conn)
            .await?;
        Ok(())
    }

    async fn create_run_row(&self, task_name: &str) -> anyhow::Result<i64> {
        let started_at = now_str();
        let model = task_run::ActiveModel {
            task_name: Set(task_name.to_string()),
            status: Set("running".to_string()),
            started_at: Set(started_at),
            ended_at: Set(None),
            success_count: Set(0),
            fail_count: Set(0),
            error: Set(None),
            ..Default::default()
        };
        let inserted = model.insert(&self.conn).await?;
        Ok(inserted.id)
    }

    async fn finish_run_row(
        &self,
        run_id: i64,
        ended_at: &str,
        status: &str,
        success_count: i32,
        fail_count: i32,
        error_msg: Option<&str>,
    ) -> anyhow::Result<()> {
        let mut q = task_run::Entity::update_many();
        q = q
            .col_expr(task_run::Column::Status, Expr::value(status))
            .col_expr(task_run::Column::EndedAt, Expr::value(ended_at))
            .col_expr(task_run::Column::SuccessCount, Expr::value(success_count))
            .col_expr(task_run::Column::FailCount, Expr::value(fail_count));

        q = if let Some(msg) = error_msg {
            q.col_expr(task_run::Column::Error, Expr::value(msg))
        } else {
            q.col_expr(task_run::Column::Error, Expr::value(Option::<String>::None))
        };

        q.filter(task_run::Column::Id.eq(run_id)).exec(&self.conn).await?;
        Ok(())
    }
}

fn now_str() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z").to_string()
}

fn safe_get_schedule(_task: &Arc<dyn Task>) -> Option<String> {
    // 当前项目中大量 task.get_schedule() 仍是 todo!()。
    // 为避免管理模块因为 panic 失效，这里先不调用 get_schedule。
    None
}

fn task_name(task: &Arc<dyn Task>) -> String {
    let full = std::any::type_name_of_val(task.as_ref());
    full.rsplit("::").next().unwrap_or(full).to_string()
}
