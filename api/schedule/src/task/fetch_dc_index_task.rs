use async_trait::async_trait;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::{dc_index, etf};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use ext_api::tushare::dc_index;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchDcIndexTask(DatabaseConnection);

impl FetchDcIndexTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchDcIndexTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let mut indexes = dc_index().await?;
        let tx = self.0.begin().await?;
        let mut curr = 0;
        for mut index in &indexes  {
            let active_model = entity::dc_index::ActiveModel { ..index.clone().into() };
            let pks = [
                dc_index::Column::TsCode,
            ];
            let update_columns = get_entity_update_columns::<entity::dc_index::Entity>(&pks);
            let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                .update_columns(update_columns)
                .to_owned();

            if let Err(e) = entity::dc_index::Entity::insert(active_model)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await {
                error!("insert dc index failed, ts_code: {}, end_date: {}, error: {:?}", index.ts_code, index.trade_date, e);
            }
            curr += 1;
            info!("insert dc index complete, ts_code: {}, progress: {}/{}", index.ts_code, curr, indexes.len());
        }
        tx.commit().await?;
        info!("fetch dc index complete");
        Ok(())
    }
}