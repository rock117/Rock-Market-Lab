use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use entity::etf;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use common::db::get_entity_update_columns;
use entity::sea_orm::TransactionTrait;

pub struct FetchEtfTask(DatabaseConnection);

impl FetchEtfTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchEtfTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::parse_from_str("20200101", "%Y%m%d")?;
        let end_date = Local::now().date_naive();
        let etfs  =  ext_api::tushare::etf_basic().await?;
        let mut curr = 0;
        for etf in &etfs {
            let tx = self.0.begin().await?;
            let active_model = entity::etf::ActiveModel { ..etf.clone().into() };
            let pks = [
                 etf::Column::TsCode,
            ];
            let update_columns = get_entity_update_columns::<entity::etf::Entity>(&pks);
            let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                .update_columns(update_columns)
                .to_owned();

            if let Err(e) = entity::etf::Entity::insert(active_model)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await {
                error!("insert etf failed, ts_code: {}, end_date: {}, error: {:?}", etf.ts_code, end_date, e);
            }
            tx.commit().await?;
            curr += 1;
            info!("insert etf complete, ts_code: {}, progress: {}/{}", etf.ts_code, curr, etfs.len());
        }
        info!("fetch etf task complete");
        Ok(())
    }
}