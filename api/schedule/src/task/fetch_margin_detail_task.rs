use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use entity::{margin_detail, stock};
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMarginDetailTask(DatabaseConnection);

impl FetchMarginDetailTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}
#[async_trait]
impl Task for FetchMarginDetailTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow!("invalid date"))?;
        let end = Local::now().date_naive();
        let stocks:Vec<stock::Model> = entity::stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let margin_details = ext_api::tushare::margin_detail(&stock.ts_code, &start, &end).await?;
            let tx = self.0.begin().await?;
            for margin_detail in &margin_details {
                let active_model = entity::margin_detail::ActiveModel { ..margin_detail.clone().into() };
                let pks = [
                    margin_detail::Column::TsCode,
                    margin_detail::Column::TradeDate
                ];
                let update_columns = get_entity_update_columns::<entity::margin_detail::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::margin_detail::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert margin_detail failed, exchangeId: {}, trade date: {}, error: {:?}", margin_detail.ts_code, margin_detail.trade_date, e);
                }
            }
            curr += 1;
            info!("Insert margin_detail data for tscode: {}, progress {}/{}", stock.ts_code, curr, stocks.len());
            tx.commit().await?;
        }
        info!("fetch margin_detail detail complete");
        Ok(())
    }
}