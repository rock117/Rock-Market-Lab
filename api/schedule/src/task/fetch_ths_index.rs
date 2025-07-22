use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait, Set, EntityTrait, InsertResult};
use crate::task::Task;
use entity::ths_index;
use tracing::{info};
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
        for index in indexes {
            let active_model = ths_index::ActiveModel {
                ts_code: Set(index.ts_code.clone()),
                name: Set(index.name.clone()),
                count: Set(index.count),
                exchange: Set(index.exchange.clone()),
                list_date: Set(index.list_date.clone()),
                r#type: Set(index.r#type.clone()),
            };
            let on_conflict = OnConflict::columns([ths_index::Column::TsCode, ths_index::Column::Exchange, ths_index::Column::Type, ths_index::Column::ListDate])
                .update_columns([ths_index::Column::Name, ths_index::Column::Count])
                .to_owned();
            
            ths_index::Entity::insert(active_model)
                .on_conflict(on_conflict)
                .exec(&tx)
                .await?;
            curr += 1;
        }
        tx.commit().await?;
        info!("fetch ths index task complete");
        Ok(())
    }
}