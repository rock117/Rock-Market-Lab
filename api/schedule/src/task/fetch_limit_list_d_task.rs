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

pub struct FetchLimitListDTask(DatabaseConnection);
impl FetchLimitListDTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}
#[async_trait]
impl Task for FetchLimitListDTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let end_date = Local::now().naive_local().date();
        let start_date = end_date.checked_sub_months(Months::new(3)).ok_or_else(|| anyhow!("date overflow"))?;
        let mut curr = 0;
        for stock in &stocks {
            let hms = ext_api::tushare::limit_list_d(&stock.ts_code, &start_date, &end_date).await;
            if let Err(e) = hms {
                error!("fetch limit_list_d failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for limit_list_d in hms? {
                let active_model = entity::limit_list_d::ActiveModel {
                    ..limit_list_d.clone().into()
                };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::limit_list_d::Column::TsCode,
                    entity::limit_list_d::Column::TradeDate,
                ];
                let update_columns = get_entity_update_columns::<entity::limit_list_d::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::limit_list_d::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await
                {
                    error!(
                    "insert limit_list_d failed, limit_list_d: {:?}, error: {:?}",
                    limit_list_d, e
                );
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert limit_list_d complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch limit_list_d task complete");
        Ok(())
    }
}