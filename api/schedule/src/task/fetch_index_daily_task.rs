use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::index;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchIndexDailyTask(DatabaseConnection);

impl FetchIndexDailyTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchIndexDailyTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let indexes: Vec<index::Model> = index::Entity::find().all(&self.0).await?;
        let (start_date, end_date) = super::get_start_end_date_from_default()?;
        let mut curr = 0;
        for index in &indexes {
            let res = ext_api::tushare::index_daily(&index.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch index daily failed, ts_code: {}, error: {:?}", index.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let index_dailys = res?;
            for index_daily in index_dailys {
                let res = entity::index_daily::ActiveModel { ..index_daily.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                  //  error!("insert index daily failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", index.ts_code, end_date, err, index_daily);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert index daily complete, ts_code: {}, progress: {}/{}", index.ts_code, curr, indexes.len());
        }
        info!("fetch index daily complete");
        Ok(())
    }
}