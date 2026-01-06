use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::{margin, stock, stock_daily, trade_calendar};
use crate::task::Task;
use ext_api::tushare;
use entity::stock_daily::{Model as StockDaily, Model};
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
use common::db::get_entity_update_columns;
use entity::sea_orm::prelude::Decimal;

const DAYS_AGO: u64 = 250;

pub struct FetchStockDailyTask(DatabaseConnection);

impl FetchStockDailyTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchStockDailyTask(connection)
    }
    async fn fetch_price_from_listdate(&self) -> anyhow::Result<()> {
        let date = Local::now().date_naive();
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let tx = self.0.begin().await?;
            if let Some(list_date) = &stock.list_date {
                let list_date = NaiveDate::parse_from_str(list_date, "%Y%m%d")?;
                let dailys = tushare::daily(Some(&stock.ts_code), &list_date, &date).await?;
                for daily in &dailys {
                     let active_model = entity::stock_daily::ActiveModel { ..daily.clone().into() };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                    let pks = [
                        entity::stock_daily::Column::TsCode,
                        entity::stock_daily::Column::TradeDate,
                    ];
                    let update_columns = get_entity_update_columns::<entity::stock_daily::Entity>(&pks);
                    let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                        .update_columns(update_columns)
                        .to_owned();

                    if let Err(e) = entity::stock_daily::Entity::insert(active_model)
                        .on_conflict(on_conflict)
                        .exec(&tx)
                        .await {
                        error!("insert stock_daily failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                    }
                }
            }
            curr += 1;
            tx.commit().await?;
            info!("fetch stock_daily complete, ts_code: {}, list date: {:?}, progress: {}/{}", stock.ts_code, stock.list_date, curr, stocks.len());
        }
        Ok(())
    }
    async fn fetch_data_by_date(&self, date: &NaiveDate) -> anyhow::Result<()> {
        let stock_dailys = tushare::daily(None, date, date).await?;

        let tx = self.0.begin().await?;
        let total = stock_dailys.len();
        let mut curr = 0;
        for stock_daily_data in stock_dailys {
            let active_model = entity::stock_daily::ActiveModel { ..stock_daily_data.clone().into() };
            let pks = [
                stock_daily::Column::TsCode,
                stock_daily::Column::TradeDate
            ];
            let update_columns = get_entity_update_columns::<entity::stock_daily::Entity>(&pks);
            let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                .update_columns(update_columns)
                .to_owned();

            if let Err(e) = entity::stock_daily::Entity::insert(active_model)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await {
                error!("insert stock_daily failed, ts code: {}, trade date: {}, error: {:?}", stock_daily_data.ts_code, stock_daily_data.trade_date, e);
            }
        }
        info!("insert stock_daily complete, trade_date: {}, total: {}", date, total);
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchStockDailyTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let dates = super::get_calendar_dates(DAYS_AGO, &self.0).await?;
        info!("fetch    all s   tock_daily tasks run..., start = {}, end = {}", dates[0], dates[dates.len() - 1]);
        for date in &dates {
            let res = self.fetch_data_by_date(date).await;
            if let Err(e) = res {
                error!("fetch stock_daily failed, trade-date: {}, error: {:?}", date, e);
            }
        }

        info!("fetch all stock_daily tasks run..., start = {}, end = {}", dates[0], dates[dates.len() - 1]);
        // self.fetch_price_from_listdate().await?;
        // info!("fetch all stock_daily tasks run...");
        Ok(())
    }
}