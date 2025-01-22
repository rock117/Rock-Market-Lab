use async_trait::async_trait;
use tracing::{error, warn, info};
use entity::sea_orm::{DatabaseConnection, Set, TransactionTrait};
use entity::{stock, trade_calendar};
use crate::task::Task;
use ext_api::tushare;
use entity::stock::Model as Stock;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::trade_calendar::Model;
use crate::task::fetch_stock_list_task::FetchStockListTask;

pub struct FetchTradeCalendarTask(DatabaseConnection);

impl FetchTradeCalendarTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchTradeCalendarTask(connection)
    }
}

#[async_trait]
impl Task for FetchTradeCalendarTask {
    fn get_schedule(&self) -> String {
        // "0 5 23 * * *".to_string() // every day at 23:00
        "*/10 * * * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let trade_calendars:Vec<Model> = tushare::trade_cal().await?;
        let tx = self.0.begin().await?;
        let total = trade_calendars.len();
        let mut curr = 0;
        for trade_calendar_m in trade_calendars {
            let exchange = trade_calendar_m.exchange.clone();
            let cal_date = trade_calendar_m.cal_date.clone();
            let res = trade_calendar::ActiveModel { ..trade_calendar_m.clone().into() }.insert(&self.0).await;
            if res.is_err() {
                error!("insert trade_calendar failed, exchange: {}, cal_date: {}, data: {:?}, error: {:?}", exchange, cal_date, trade_calendar_m, res);
            }
            curr += 1;
            info!("insert trade_calendar complete: {}, {}/{}", exchange,  curr, total);
        }
        tx.commit().await?;
        info!("fetch trade_calendar task run...");
        Ok(())
    }
}