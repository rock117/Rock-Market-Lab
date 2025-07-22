use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use entity::sea_orm::query::UpdateOnConflict;
use crate::task::Task;
use entity::ths_index;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::sea_query::OnConflict;

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
        let mut curr = 0;
        let indexes = ext_api::tushare::ths_index(None, Some("A"), None).await?;
        let tx = self.0.begin().await?;
        info!("fetch ths index count: {}", indexes.len());
        for index in indexes {

            curr += 1;
        }
        tx.commit().await?;
        info!("fetch ths index task complete");
        Ok(())
    }
}