use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tokio::sync::{mpsc, Semaphore};
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, warn};

use entity::{stock, stock_monthly, trade_calendar};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::EntityOrSelect;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;
use entity::trade_calendar::Entity as TradeCalendar;
use ext_api::tushare;

use crate::task::Task;

pub struct FetchStockMonthlyTask(DatabaseConnection);

impl FetchStockMonthlyTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchStockMonthlyTask(connection)
    }
}

#[async_trait]
impl Task for FetchStockMonthlyTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let (start_date, end_date) = super::get_start_end_date_from_default()?;
        let mut curr = 0;
        for stock in &stocks {
            let res = ext_api::tushare::monthly(&stock.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch stock monthly failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let stock_monthlys = res?;
            for stock_monthly in &stock_monthlys {
                let res = entity::stock_monthly::ActiveModel { ..stock_monthly.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert stock monthly failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", stock.ts_code, end_date, err, stock_monthly);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert stock monthly complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch stock monthly complete");
        Ok(())
    }
}