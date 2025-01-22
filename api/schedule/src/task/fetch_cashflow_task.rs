use super::Task;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use ext_api::tushare;
use ext_api::tushare::cashflow;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchCashflowTask(DatabaseConnection);

impl FetchCashflowTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchCashflowTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::parse_from_str("20200101", "%Y%m%d")?;
        let end_date = Local::now().date_naive();
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let cashflow = cashflow(&stock.ts_code, "1", &start_date, &end_date).await;
            if let Err(e) = cashflow {
                error!("fetch cashflow failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            for cash in cashflow? {
                let res = entity::cashflow::ActiveModel { ..cash.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert cashflow failed, ts_code: {}, end_date: {}, error: {:?}", stock.ts_code, end_date, err);
                    continue;
                }
            }
            curr += 1;
            info!("insert cashflow complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch cashflow task complete");
        Ok(())
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

 
