use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::index;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchIndexMonthlyTask(DatabaseConnection);

impl FetchIndexMonthlyTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchIndexMonthlyTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let indexes: Vec<index::Model> = index::Entity::find().all(&self.0).await?;
        let start_date = NaiveDate::from_ymd_opt(2025, 3, 1).ok_or(anyhow!("date none"))?;
        let end_date = chrono::Local::now().date_naive();
        let mut curr = 0;
        for index in &indexes {
            let res = ext_api::tushare::index_monthly(&index.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch index monthly failed, ts_code: {}, error: {:?}", index.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let index_monthlys = res?;
            for index_monthly in index_monthlys {
                let res = entity::index_monthly::ActiveModel { ..index_monthly.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert index monthly failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", index.ts_code, end_date, err, index_monthly);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert index monthly complete, ts_code: {}, progress: {}/{}", index.ts_code, curr, indexes.len());
        }
        info!("fetch index monthly complete");
        Ok(())
    }
}