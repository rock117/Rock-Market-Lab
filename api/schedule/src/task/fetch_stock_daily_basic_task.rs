use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, Set, TransactionTrait};
use entity::{stock_daily_basic, trade_calendar};
use crate::task::Task;
use ext_api::tushare;
use entity::stock_daily_basic::{Model as StockDailyBasic, Model};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::ColumnTrait;

use entity::trade_calendar::{Entity as TradeCalendar};
use entity::sea_orm::EntityOrSelect;
use tokio::sync::{mpsc, Semaphore};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

pub struct FetchStockDailyBasicTask(DatabaseConnection);

impl FetchStockDailyBasicTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchStockDailyBasicTask(connection)
    }
    fn fetch_data(dates: &[NaiveDate]) -> Receiver<anyhow::Result<Vec<Model>>> {
        let max_concurrent_requests = 1; // TODO 太大 tushare 会拒接连接
        let (tx, rx) = mpsc::channel(max_concurrent_requests);
        for date in dates {
            let date = date.clone();
            let tx_clone = tx.clone();
            let _ = tokio::spawn(async move {
                let stock_dailys = tushare::daily_basic(&date, &date).await;
                let _ = tx_clone.send(stock_dailys).await;
            });
        }
        rx
    }

    async fn fetch_data_by_date(&self, date: &NaiveDate) -> anyhow::Result<()> {
        let stock_dailys = tushare::daily_basic(date, date).await?;

        let tx = self.0.begin().await?;
        let total = stock_dailys.len();
        let mut curr = 0;
        for stock_daily_data in stock_dailys {
            let ts_code = stock_daily_data.ts_code.clone();
            let trade_date = stock_daily_data.trade_date.clone();
            let res = stock_daily_basic::ActiveModel { ..stock_daily_data.clone().into() }.insert(&self.0).await;
            if res.is_err() {
                //  error!("insert stock_daily failed, ts_code: {}, trade_date: {}, data: {:?}, error: {:?}", ts_code, trade_date, stock_daily_data, res);
            }
            curr += 1;
            info!("insert stock_daily_basic complete, ts_code: {}, trade_date: {}, {}/{}", ts_code, trade_date,  curr, total);
        }
        info!("insert stock_daily_basic complete, trade_date: {}, total: {}", date, total);
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchStockDailyBasicTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }


    async fn run(&self) -> anyhow::Result<()> {
        let dates = super::get_calendar_dates(30, &self.0).await?;
        for date in &dates {
            let res = self.fetch_data_by_date(date).await;
            if let Err(e) = res {
                error!("fetch stock_daily failed, trade-date: {}, error: {:?}", date, e);
            }
        }
        info!("fetch all stock_daily tasks run..., start = {}, end = {}", dates[0], dates[dates.len() - 1]);
        Ok(())
    }


}