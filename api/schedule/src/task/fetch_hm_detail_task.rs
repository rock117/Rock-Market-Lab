use anyhow::anyhow;
use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stock;
use chrono::{Local, Months, NaiveDate};
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use ext_api::tushare;
use crate::task::fetch_balancesheet_task::FetchBalancesheetTask;

pub struct FetchHmDetailTask(DatabaseConnection);
impl FetchHmDetailTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}
#[async_trait]
impl Task for FetchHmDetailTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let end_date = Local::now().naive_local().date();
        let start_date = end_date.checked_sub_months(Months::new(3)).ok_or_else(|| anyhow!("date overflow"))?;
        let mut curr = 0;
        for stock in &stocks {
            let hms = ext_api::tushare::get_hm_detail(&stock.ts_code, &start_date, &end_date).await;
            if let Err(e) = hms {
                error!("fetch hm_detail failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for hm_detail in hms? {
                let active_model = entity::hm_detail::ActiveModel {
                    ..hm_detail.clone().into()
                };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::hm_detail::Column::TsCode,
                    entity::hm_detail::Column::HmName,
                    entity::hm_detail::Column::TradeDate,
                ];
                let update_columns = get_entity_update_columns::<entity::hm_detail::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::hm_detail::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await
                {
                    error!(
                    "insert hm_detail failed, hm_detail: {:?}, error: {:?}",
                    hm_detail, e
                );
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert hm_detail complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch hm_detail task complete");
        Ok(())
    }
}