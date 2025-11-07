use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::etf;
use entity::fund_portfolio;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

/// 从tushare获取etf持仓数据
pub struct FetchFundPortfolioTask(DatabaseConnection);

impl FetchFundPortfolioTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchFundPortfolioTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let etfs: Vec<etf::Model> = etf::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for etf in &etfs {
            let res = ext_api::tushare::fund_portfolio(&etf.ts_code).await;
            if let Err(e) = res {
                error!("fetch etf portfolio failed, ts_code: {}, error: {:?}", etf.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let fund_portfolios = res?;
            for fund_portfolio in &fund_portfolios {
                let active_model = entity::fund_portfolio::ActiveModel { ..fund_portfolio.clone().into() };
                let pks = [
                    fund_portfolio::Column::TsCode,
                    fund_portfolio::Column::Symbol,
                    fund_portfolio::Column::AnnDate,
                    fund_portfolio::Column::EndDate
                ];
                let update_columns = get_entity_update_columns::<fund_portfolio::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();
                if let Err(e) = entity::fund_portfolio::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert etf portfolio failed, ts_code: {}, ann_date: {}, error: {:?}", etf.ts_code, fund_portfolio.ann_date, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert etf portfolio complete, ts_code: {}, etf portfolio size: {}, progress: {}/{}", etf.ts_code, fund_portfolios.len(), curr, etfs.len());
        }
        info!("fetch etf portfolio complete");
        Ok(())
    }
}