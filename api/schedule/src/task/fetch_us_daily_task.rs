use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::{us_basic, us_daily};
use crate::task::Task;
use ext_api::tushare;
use entity::us_daily::{Model as UsDaily, Model};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::ColumnTrait;

use entity::sea_orm::EntityOrSelect;
use std::sync::Arc;
use entity::sea_orm::prelude::Decimal;

pub struct FetchUsDailyTask(DatabaseConnection);

impl FetchUsDailyTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsDailyTask(connection)
    }
    async fn fetch_data_by_date(&self, date: &NaiveDate) -> anyhow::Result<()> {
        let us_dailys = tushare::us_daily(date).await?;

        let tx = self.0.begin().await?;
        let total = us_dailys.len();
        let mut curr = 0;
        for us_daily_data in us_dailys {
            let ts_code = us_daily_data.ts_code.clone();
            let trade_date = us_daily_data.trade_date.clone();
            let res = us_daily::ActiveModel { ..us_daily_data.clone().into() }.insert(&self.0).await;
            if res.is_err() {
                //  error!("insert us_daily failed, ts_code: {}, trade_date: {}, data: {:?}, error: {:?}", ts_code, trade_date, us_daily_data, res);
            }
            curr += 1;
            //info!("insert us_daily complete, ts_code: {}, trade_date: {}, {}/{}", ts_code, trade_date,  curr, total);
        }
        info!("insert us_daily complete, trade_date: {}, total: {}", date, total);
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchUsDailyTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
        let end_date = Local::now().date_naive();
        let mut curr_date = start_date.clone();
        while curr_date <= end_date {
            let res = self.fetch_data_by_date(&curr_date).await;
            if let Err(e) = res {
                error!("fetch us_daily failed, trade-date: {}, error: {:?}", curr_date, e);
            }
            curr_date.checked_add_days(Days::new(1)).unwrap();
        }
        info!("fetch all us_daily tasks run..., start = {}, end = {}", start_date, end_date);
        Ok(())
    }
}