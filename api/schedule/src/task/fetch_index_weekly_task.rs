use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::index;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchIndexWeeklyTask(DatabaseConnection);

impl FetchIndexWeeklyTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchIndexWeeklyTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let indexes: Vec<index::Model> = index::Entity::find().all(&self.0).await?;
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow!("date none"))?;
        let end_date = chrono::Local::now().date_naive();
        let mut curr = 0;
        for index in &indexes {
            let res = ext_api::tushare::index_weekly(&index.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch index weekly failed, ts_code: {}, error: {:?}", index.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let index_weeklys = res?;
            for index_weekly in index_weeklys {
                let res = entity::index_weekly::ActiveModel { ..index_weekly.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert index weekly failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", index.ts_code, end_date, err, index_weekly);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert index weekly complete, ts_code: {}, progress: {}/{}", index.ts_code, curr, indexes.len());
        }
        info!("fetch index weekly complete");
        Ok(())
    }
}