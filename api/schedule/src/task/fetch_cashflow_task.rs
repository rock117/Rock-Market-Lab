use super::Task;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use entity::stock;
use ext_api::tushare;
use ext_api::tushare::cashflow;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchCashflowTask(DatabaseConnection);

impl FetchCashflowTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchCashflowTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let cashflow = cashflow(&stock.ts_code).await;
            if let Err(e) = cashflow {
                error!("fetch cashflow failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for cash in cashflow? {
                let active_model = entity::cashflow::ActiveModel { ..cash.clone().into() };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::cashflow::Column::TsCode,
                    entity::cashflow::Column::ReportType,
                    entity::cashflow::Column::AnnDate,
                    entity::cashflow::Column::FAnnDate,
                ];
                let update_columns = get_entity_update_columns::<entity::cashflow::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::cashflow::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert cashflow failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert cashflow complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch cashflow task complete");
        Ok(())
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

 
