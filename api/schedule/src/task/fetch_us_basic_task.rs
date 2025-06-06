use async_trait::async_trait;
use tracing::{error, info};
use entity::us_basic;
use entity::sea_orm::DatabaseConnection;
use ext_api::tushare::us_basic;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchUsBasicTask(DatabaseConnection);

impl FetchUsBasicTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    async fn get_us_stocks(&self, offset: usize, limit: usize) -> anyhow::Result<usize> {
        let mut indexes = us_basic(offset, limit).await?;
        info!("offset: {}, limit: {}, us stock num: {}", offset, limit, indexes.len());
        for mut index in &indexes  {
            let res = us_basic::ActiveModel { ..index.clone().into() }.insert(&self.0).await;
            if let Err(err) = res {
                //  error!("insert index failed, error: {:?}", err);
            }
        }
        info!("fetch us basic complete");
        Ok(indexes.len())
    }
}

#[async_trait]
impl Task for FetchUsBasicTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let mut offset = 0;
        let limit = 6000;
        loop {
            let num = self.get_us_stocks(offset, limit).await?;
            if(num == 0) {
                break;
            }
            offset += limit;
        }
        info!("fetch us basic complete");
        Ok(())
    }


}