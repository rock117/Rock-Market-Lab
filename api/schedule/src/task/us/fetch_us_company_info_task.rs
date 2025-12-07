use crate::task::Task;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::us_daily::{Model as UsDaily, Model};
use entity::{us_stock, us_company_info};
use ext_api::mstar;
use tracing::{error, info, warn};

use common::db::get_entity_update_columns;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::sea_query::OnConflict;
use entity::sea_orm::EntityOrSelect;
use std::sync::Arc;

pub struct FetchUsCompanyInfoTask(DatabaseConnection);

impl FetchUsCompanyInfoTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsCompanyInfoTask(connection)
    }
 
}

#[async_trait]
impl Task for FetchUsCompanyInfoTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks = us_stock::Entity::find()
            .all(&self.0)
            .await?;
        for stock in &stocks {
            let company = mstar::company::get_company_general_info(stock.exchange_id.as_str(), stock.symbol.as_str()).await?;
        
        }
        Ok(())
    }
}