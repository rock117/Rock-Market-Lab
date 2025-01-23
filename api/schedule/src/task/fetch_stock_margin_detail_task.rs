use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stock;
use entity::stock_margin_detail;
use ext_api::tushare;
use ext_api::tushare::margin_detail;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchStockMarginDetailTask(DatabaseConnection);
impl FetchStockMarginDetailTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchStockMarginDetailTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).ok_or(anyhow::anyhow!("Invalid date"))?;
        let end_date = Local::now().date_naive();
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let res = tushare::margin_detail(&stock.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("Failed to fetch margin detail for stock: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for margin_detail in &res? {
                let res = stock_margin_detail::ActiveModel { ..margin_detail.clone().into() }.insert(&self.0).await;
                if let Err(e) = res {
                    error!("Failed to insert margin detail for stock: {}, error: {:?}, data: {:?}", stock.ts_code, e, serde_json::to_string(&margin_detail));
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("Finished fetch margin detail for stock: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        Ok(())
    }
}