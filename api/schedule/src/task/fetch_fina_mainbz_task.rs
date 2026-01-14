use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, Set, TransactionTrait};
use entity::{margin, stock, stock_daily_basic, trade_calendar};
use crate::task::Task;
use ext_api::tushare;
use entity::finance_main_business::Model;
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

const DAYS_AGO: u64 = 250;

pub struct FetchFinaMainbzTask(DatabaseConnection);

impl FetchFinaMainbzTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchFinaMainbzTask(connection)
    }
    
}

#[async_trait]
impl Task for FetchFinaMainbzTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }


    async fn run(&self) -> anyhow::Result<()> {
        let end_date = Local::now().date_naive();
        let start_date = NaiveDate::from_ymd_opt(2020,1,1).ok_or_else(|| anyhow!("invalid date"))?;
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        info!("stocks num: {}", stocks.len());

        let types = ["P", "D", "I"];
        let stock_pairs = stocks.iter()
            .flat_map(|stock| types.iter().map(move |type_| (stock, type_)))
            .collect::<Vec<_>>();

        info!("stock_pairs num: {}", stock_pairs.len());
        let mut curr = 0;
        for (stock, type_) in &stock_pairs {
            let finance_main_businesses = ext_api::tushare::fina_mainbz(&stock.ts_code, type_, &start_date, &end_date).await;
            if let Err(e) = finance_main_businesses {
                error!("fetch finance_main_business failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for finance_main_business in finance_main_businesses? {
                let mut active_model = entity::finance_main_business::ActiveModel { ..finance_main_business.clone().into() };
                active_model.r#type = Set(Some(type_.to_string()));
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::finance_main_business::Column::TsCode,
                    entity::finance_main_business::Column::EndDate,
                    entity::finance_main_business::Column::Type,
                    entity::finance_main_business::Column::BzItem,
                ];
                let update_columns = get_entity_update_columns::<entity::finance_main_business::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::finance_main_business::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert finance_main_business failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert finance_main_business complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stock_pairs.len());
        }
        info!("fetch finance_main_business task complete");
        Ok(())
    }


}