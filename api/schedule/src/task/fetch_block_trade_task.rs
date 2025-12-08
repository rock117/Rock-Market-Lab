use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info, warn};
use entity::sea_orm::DatabaseConnection;
use entity::{stock, block_trade};
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use common::db::get_entity_update_columns;
use entity::sea_orm::TransactionTrait;

pub struct FetchBlockTradeTask(DatabaseConnection);

impl FetchBlockTradeTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchBlockTradeTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let block_trades = ext_api::tushare::block_trade(&stock.ts_code).await;
            if let Err(e) = block_trades {
                warn!("failed to fetch block_trade for {}, {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for block_trade in block_trades? {
                let active_model = entity::block_trade::ActiveModel { ..block_trade.clone().into() };
                // ts_code  ann_date f_ann_date  end_date report_type comp_type
                let pks = [
                    entity::block_trade::Column::TsCode,
                    entity::block_trade::Column::TradeDate,
                ];
                let update_columns = get_entity_update_columns::<entity::block_trade::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();

                if let Err(e) = entity::block_trade::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert block_trade failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert block_trade complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch block_trade task complete");
        Ok(())
    }
}