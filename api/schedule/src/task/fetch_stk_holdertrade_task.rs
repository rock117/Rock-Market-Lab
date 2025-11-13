use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stk_holdertrade;
use entity::stock;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

/// 从tushare获取etf持仓数据
pub struct FetchStkHoldertradeTask(DatabaseConnection);

impl FetchStkHoldertradeTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchStkHoldertradeTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let res = ext_api::tushare::stk_holdertrade(&stock.ts_code).await;
            if let Err(e) = res {
                error!("fetch stk_holdertrade failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let stk_holdertrades = res?;
            for stk_holdertrade in &stk_holdertrades {
                let active_model = entity::stk_holdertrade::ActiveModel { ..stk_holdertrade.clone().into() };
                let pks = [
                    stk_holdertrade::Column::TsCode,
                    stk_holdertrade::Column::AnnDate,
                    stk_holdertrade::Column::HolderName,
                    stk_holdertrade::Column::InDe
                ];
                let update_columns = get_entity_update_columns::<stk_holdertrade::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();
                if let Err(e) = entity::stk_holdertrade::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert etf stk_holdertrade failed, ts_code: {}, ann_date: {}, error: {:?}", stk_holdertrade.ts_code, stk_holdertrade.ann_date, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert stk_holdertrade complete, ts_code: {}, stocks size: {}, progress: {}/{}", stock.ts_code, stk_holdertrades.len(), curr, stocks.len());
        }
        info!("fetch stk_holdertrade complete");
        Ok(())
    }
}