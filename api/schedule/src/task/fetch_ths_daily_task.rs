use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait, Set, EntityTrait, InsertResult};
use crate::task::Task;
use entity::ths_daily;
use entity::ths_index;
use tracing::{info, error};
use entity::sea_orm::sea_query::OnConflict;
use chrono::{Days, Local};

pub struct FetchThsDailyTask(DatabaseConnection);

impl FetchThsDailyTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

}

#[async_trait]
impl Task for FetchThsDailyTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let indexes = ths_index::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        let end_date = Local::now().date_naive();
        let start_date = end_date.checked_sub_days(Days::new(1000)).unwrap();
        let start_date = start_date.format("%Y%m%d").to_string();
        let end_date = end_date.format("%Y%m%d").to_string();

        info!("fetch ths daily count: {}", indexes.len());
        for index in &indexes {
            let tx = self.0.begin().await?;
            let daily = ext_api::tushare::ths_daily(Some(&index.ts_code), None, Some(&start_date), Some(&end_date)).await?;
            for day in &daily {
                let active_model = ths_daily::ActiveModel { ..day.clone().into() };
                let on_conflict = OnConflict::columns([ths_daily::Column::TsCode, ths_daily::Column::TradeDate])
                    .update_columns([ths_daily::Column::Vol])
                    .to_owned();

                if let Err(e) = ths_daily::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert ths_daily failed, ts_code: {}, error: {:?}", day.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert ths_index complete: {}, daily num: {}, {}/{}", index.ts_code, daily.len(), curr, indexes.len());
        }

        info!("fetch ths daily task complete");
        Ok(())
    }
}