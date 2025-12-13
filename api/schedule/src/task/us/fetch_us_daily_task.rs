use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, Months, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, QuerySelect, Set, TransactionTrait};
use entity::{us_basic, us_daily, us_stock};
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
use common::db::get_entity_update_columns;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::sea_query::OnConflict;

pub struct FetchUsDailyTask(DatabaseConnection);

impl FetchUsDailyTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsDailyTask(connection)
    }
}

#[async_trait]
impl Task for FetchUsDailyTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let us_stocks = us_stock::Entity::find()
            // .select_only()
            // .column(us_stock::Column::Symbol)
            .all(&self.0)
            .await?;
        info!("us stock size: {}", us_stocks.len());

        let end_date = Local::now().naive_local();
        let start_date = end_date.checked_sub_months(Months::new(3)).unwrap().format("%Y%m%d").to_string();
        let end_date = end_date.format("%Y%m%d").to_string();
        let mut curr = 0;
        for stock in &us_stocks {
            curr += 1;
            let datas = match tushare::us_daily(&stock.symbol, &start_date, &end_date).await {
                Ok(data) => data,
                Err(e) => {
                    error!("fetch us_daily failed, stock: {:?}, err: {:?}", stock.symbol, e);
                    continue;
                }
            };
            let tx = self.0.begin().await?;
            for data in &datas {
                let pks = [
                    us_daily::Column::TsCode,
                    us_daily::Column::TradeDate,
                ];
                let update_columns = get_entity_update_columns::<us_daily::Entity>(&pks);
                let on_conflict = OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();
                let am = us_daily::ActiveModel { ..data.clone().into() };
                if let Err(e) = us_daily::Entity::insert(am)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert us_daily failed err: {:?}",  e);
                }
            }
            tx.commit().await?;
            info!("insert us_daily complete, stock: {:?}, total: {}", stock.symbol, datas.len());
        }

        Ok(())
    }
}