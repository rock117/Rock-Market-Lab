use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use tracing::{error, warn, info};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::{ths_member, us_basic, us_daily, us_stock};
use crate::task::Task;
use ext_api::mstar;
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

pub struct FetchUsStockTask(DatabaseConnection);

impl FetchUsStockTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsStockTask(connection)
    }

    async fn save_stocks(&self, stocks: &[us_stock::Model]) -> anyhow::Result<()> {
        let tx = self.0.begin().await?;
        for stock in stocks {
            let pks = [
                us_stock::Column::ExchangeId,
                us_stock::Column::Symbol,
            ];
            let update_columns = get_entity_update_columns::<us_stock::Entity>(&pks);
            let on_conflict = OnConflict::columns(pks)
                .update_columns(update_columns)
                .to_owned();
            let am = us_stock::ActiveModel { ..stock.clone().into() };
            if let Err(e) = us_stock::Entity::insert(am)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await {
                error!("insert us_stock failed, stock: {:?}, err: {:?}", stock, e);
            }
        }
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchUsStockTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let exchanges = &[
            "ARCX",
            "ASE",
            "BATS",
            "GREY",
            "NAS",
            "NYS",
            "OTC",
            "PINX"
        ];
     
        for exchange_id in exchanges {
            let resp = mstar::equity::get_stock_list(*exchange_id).await;
            if resp.is_err() {
                error!("get_stock_list failed, exchange_id: {}, err: {:?}", exchange_id, resp.err());
                continue;
            }
            let resp = resp?;
            let stocks: Vec<us_stock::Model> = resp
                .full_stock_exchange_security_entity_list
                .into_iter()
                .filter(|s| s.symbol.is_some() && s.exchange_id.is_some())
                .map(|s| us_stock::Model {
                    symbol: s.symbol.unwrap(),
                    exchange_id: s.exchange_id.unwrap(),
                    name: s.company_name,
                })
                .collect();
            info!("Fetched {} stocks from {}", stocks.len(), exchange_id);

            self.save_stocks(&stocks).await?;
            info!("Saved {} stocks from {} to database", stocks.len(), exchange_id);
        }
        Ok(())
    }
}