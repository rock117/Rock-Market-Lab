use async_trait::async_trait;
use tracing::{error, info};
use entity::index;
use entity::sea_orm::DatabaseConnection;
use ext_api::tushare::index_basic;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchIndexTask(DatabaseConnection);

impl FetchIndexTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchIndexTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let mut indexes = index_basic().await?;
        for mut index in indexes  {
            index.name_py = index.name.as_ref().map(|name| common::get_security_pinyin(&name));
            let res = index::ActiveModel { ..index.clone().into() }.insert(&self.0).await;
            if let Err(err) = res {
              //  error!("insert index failed, error: {:?}", err);
            }
        }
        info!("fetch index complete");
        Ok(())
    }
}