use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info, warn};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use common::db::get_entity_update_columns;
use entity::sea_orm::TransactionTrait;

pub struct FetchBalancesheetTask(DatabaseConnection);

impl FetchBalancesheetTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchBalancesheetTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let balancesheet = ext_api::tushare::balancesheet(&stock.ts_code).await;
            if let Err(e) = balancesheet {
                warn!("failed to fetch balancesheet for {}, {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for balance in balancesheet? {
                let active_model = entity::balancesheet::ActiveModel { ..balance.clone().into() };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::balancesheet::Column::TsCode,
                    entity::balancesheet::Column::ReportType,
                    entity::balancesheet::Column::AnnDate,
                    entity::balancesheet::Column::FAnnDate,
                ];
                let update_columns = get_entity_update_columns::<entity::balancesheet::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::balancesheet::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert balancesheet failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert balancesheet complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch balancesheet task complete");
        Ok(())
    }
}