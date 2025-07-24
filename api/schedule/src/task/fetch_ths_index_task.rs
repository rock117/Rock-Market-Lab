use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait, Set, EntityTrait, InsertResult};
use crate::task::Task;
use entity::{ths_index, ths_member};
use tracing::{info, error};
use common::db::get_entity_update_columns;
use entity::sea_orm::sea_query::OnConflict;


pub struct FetchThsIndexTask(DatabaseConnection);

impl FetchThsIndexTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchThsIndexTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let mut curr = 0;
        let indexes = ext_api::tushare::ths_index(None, Some("A"), None).await?;
        let tx = self.0.begin().await?;
        info!("fetch ths index count: {}", indexes.len());
        for index in &indexes {
            let active_model = ths_index::ActiveModel { ..index.clone().into() };
            let pks = [
                ths_index::Column::TsCode, ths_index::Column::Exchange, ths_index::Column::Type, ths_index::Column::ListDate
            ];
            let update_columns = get_entity_update_columns(&pks);
            let on_conflict = OnConflict::columns(&pks)
                .update_columns(update_columns)
                .to_owned();
            
            let ts_code = index.ts_code.clone();
            if let Err(e) = ths_index::Entity::insert(active_model)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await {
                error!("insert ths_index failed, ts_code: {}, error: {:?}", ts_code, e);
            }
            curr += 1;
            info!("insert ths_index complete: {}, {}/{}", ts_code, curr, indexes.len());
        }
        tx.commit().await?;
        info!("fetch ths index task complete");
        Ok(())
    }
}