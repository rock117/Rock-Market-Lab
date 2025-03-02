use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMarginTask(DatabaseConnection);

impl FetchMarginTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchMarginTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow!("invalid date"))?;
        let end = Local::now().date_naive();
        let exchanges = vec!["SSE", "SZSE", "BSE"];
        for exchange_id in exchanges {
            let margins = ext_api::tushare::margin(exchange_id, &start, &end).await?;
            for margin in &margins {
                let res = entity::margin::ActiveModel { ..margin.clone().into() }.insert(&self.0).await;
                if let Err(e) = res {
                    error!("Failed to insert margin data for exchange_id: {}, error: {:?}, data: {:?}", exchange_id, e, serde_json::to_string(&margin));
                }
                info!("Insert margin data for exchange_id: {}, trade_date: {}", exchange_id, margin.trade_date);
            }
        }
        info!("fetch margin complete");
        Ok(())
    }

}