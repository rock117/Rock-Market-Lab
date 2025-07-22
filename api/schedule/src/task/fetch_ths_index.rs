use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::ths_index;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchThsIndexTask(DatabaseConnection);
impl FetchThsIndexTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}
#[async_trait]
impl Task for FetchThsIndexTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let ths_indexes: Vec<ths_index::Model> = ths_index::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for ths_index in &ths_indexes {
            let res = ext_api::tushare::ths_index(None, Some("A"), None).await;
            if let Err(e) = res {
                error!("fetch ths index failed, ts_code: {}, error: {:?}", ths_index.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let indexes = res?;
            for index in indexes {
                let res = entity::ths_index::ActiveModel { ..index.clone().into() }.save(&self.0).await;
                if let Err(err) = res {
                    error!("insert ths index failed, ts_code: {}, error: {:?}, data: {:?}", ths_index.ts_code, err, index);
                    error!("ths index json: {}", serde_json::to_string(&index).unwrap());
                    continue;
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert ths index complete, ts_code: {}, progress: {}/{}", ths_index.ts_code, curr, ths_indexes.len());
        }
        info!("fetch ths index task complete");
        Ok(())
    }
}