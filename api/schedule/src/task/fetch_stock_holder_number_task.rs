use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use entity::stock::Model as Stock;
use entity::stock;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::stock_holder_number::Model as StockHolderNumber;
use entity::stock_holder_number;
use ext_api::tushare;
use crate::task::Task;

pub struct FetchStockHolderNumberTask(DatabaseConnection);
impl FetchStockHolderNumberTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchStockHolderNumberTask(connection)
    }
}

#[async_trait]
impl Task for FetchStockHolderNumberTask {
    fn get_schedule(&self) -> String {
        "*/10 * * * * *".to_string()
    }
    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow::anyhow!("invalid date"))?;
        let end_date = Local::now().naive_local().date();
        let stocks: Vec<Stock> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let holder_numbers = tushare::stk_holdernumber(&stock.ts_code, &start_date, &end_date).await;
             if let Err(err) = holder_numbers  {
                error!("fetch stock holder number failed, ts_code: {}, error: {:?}", stock.ts_code, err);
                continue;
            };
            let holder_numbers = holder_numbers?;
            let tx = self.0.begin().await?;
            for holder_number in &holder_numbers {
                let res = stock_holder_number::ActiveModel { ..holder_number.clone().into() }.insert(&self.0).await;
                if res.is_err() {
                    error!("insert stock holder number failed, ts_code: {}, data: {:?}, error: {:?}", stock.ts_code, holder_number, res);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("fetch stock holder number success, ts_code: {}, holder_num: {}, progress: {}/{}", stock.ts_code, holder_numbers.len(), curr + 1, stocks.len());
        }
        Ok(())
    }
}