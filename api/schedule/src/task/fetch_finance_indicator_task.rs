use std::time::Duration;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{info, warn};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use entity::stock;
use entity::finance_indicator;
use ext_api::tushare::fina_indicator;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchFinanceIndicatorTask(DatabaseConnection);

impl FetchFinanceIndicatorTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchFinanceIndicatorTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let end_date = Local::now().date_naive();
        let start_date = Local::now().date_naive().checked_sub_days(Days::new(60)).ok_or(anyhow!("no value"))?;
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let indicators = fina_indicator(&stock.ts_code, &start_date, &end_date).await;
            if let Err(e) = indicators {
                warn!("fetch finance_indicator failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }

            let tx = self.0.begin().await?;
            let indicators = indicators?;
            for indicator in indicators {
                let res = finance_indicator::ActiveModel { ..indicator.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    //      warn!("insert finance_indicator failed, ts_code: {}, end_date: {}, data: {:?}, error: {:?}", stock.ts_code, end_date, indicator, err);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert finance_indicator complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch all finance_indicator tasks run..., start = {}, end = {}", start_date, end_date);
        Ok(())
    }
}