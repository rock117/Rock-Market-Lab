use async_trait::async_trait;
use tracing::{error, warn, info};
use entity::sea_orm::{DatabaseConnection, Set, TransactionTrait};
use entity::stock;
use crate::task::Task;
use ext_api::tushare;
use entity::stock::Model as Stock;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use common::db::get_entity_update_columns;


pub struct FetchStockListTask(DatabaseConnection);

impl FetchStockListTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchStockListTask(connection)
    }
}

#[async_trait]
impl Task for FetchStockListTask {
    fn get_schedule(&self) -> String {
        // "0 5 23 * * *".to_string() // every day at 23:00
        "*/10 * * * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let mut stocks = tushare::stock_basic().await?;
        let tx = self.0.begin().await?;
        let total = stocks.len();
        let mut curr = 0;
        for mut stock in stocks {
           let active_model = entity::stock::ActiveModel { ..stock.clone().into() };
                let pks = [
                    stock::Column::TsCode,
                ];
                let update_columns = get_entity_update_columns::<stock::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();
                if let Err(e) = entity::stock::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert stock failed, ts_code: {},  error: {:?}", stock.ts_code, e);
                }
        }
        tx.commit().await?;
        info!("fetch stock list task complete...");
        Ok(())
    }
}