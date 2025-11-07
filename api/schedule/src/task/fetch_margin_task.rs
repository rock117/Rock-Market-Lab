use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::margin;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMarginTask(DatabaseConnection);

impl FetchMarginTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchMarginTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow!("invalid date"))?;
        let end = Local::now().date_naive();
        let exchanges = vec!["SSE", "SZSE", "BSE"];
        for exchange_id in exchanges {
            let margins = ext_api::tushare::margin(exchange_id, &start, &end).await?;
            let tx = self.0.begin().await?;
            for margin in &margins {
                let active_model = entity::margin::ActiveModel { ..margin.clone().into() };
                let pks = [
                    margin::Column::TradeDate,
                    margin::Column::ExchangeId
                ];
                let update_columns = get_entity_update_columns::<entity::margin::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::margin::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert margin failed, exchangeId: {}, trade date: {}, error: {:?}", margin.exchange_id, margin.trade_date, e);
                }
                info!("Insert margin data for exchange_id: {}, trade_date: {}", exchange_id, margin.trade_date);
            }
            tx.commit().await?;
        }
        info!("fetch margin complete");
        Ok(())
    }

}