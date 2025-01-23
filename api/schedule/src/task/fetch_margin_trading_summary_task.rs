use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::margin_trading_summary;
use entity::sea_orm::DatabaseConnection;
use ext_api::tushare;
use crate::task::Task;
use entity::margin_trading_summary::*;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMarginTradingSummaryTask(DatabaseConnection);

impl FetchMarginTradingSummaryTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait::async_trait]
impl Task for FetchMarginTradingSummaryTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).ok_or(anyhow::anyhow!("Invalid date"))?;
        let end_date = Local::now().date_naive();
        let exchnages = vec!["SSE", "SZSE", "BSE"];
        for exchange_id in exchnages {
            let margins = tushare::margin(exchange_id, &start_date, &end_date).await;
            if let Err(e) = margins {
                error!("Failed to fetch margin data for exchange_id: {}, error: {:?}", exchange_id, e);
                continue;
            }
            for margin in &margins? {
                let res = margin_trading_summary::ActiveModel { ..margin.clone().into() }.insert(&self.0).await;
                if let Err(e) = res {
                    error!("Failed to insert margin data for exchange_id: {}, error: {:?}, data: {:?}", exchange_id, e, serde_json::to_string(&margin));
                }
            }
        }
        info!("FetchMarginTradingSummaryTask run success");
        Ok(())
    }
}