use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stock;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use crate::task::fetch_balancesheet_task::FetchBalancesheetTask;

pub struct FetchIncomeTask(DatabaseConnection);
impl FetchIncomeTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}
#[async_trait]
impl Task for FetchIncomeTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let incomes = ext_api::tushare::income(&stock.ts_code).await;
            if let Err(e) = incomes {
                error!("fetch income failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for income in incomes? {
                let active_model = entity::income::ActiveModel { ..income.clone().into() };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::income::Column::TsCode,
                    entity::income::Column::ReportType,
                    entity::income::Column::AnnDate,
                    entity::income::Column::FAnnDate,
                ];
                let update_columns = get_entity_update_columns::<entity::income::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::income::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert income failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert income complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch income task complete");
        Ok(())
    }
}