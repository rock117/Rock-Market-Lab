use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::etf;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchFundDailyTask(DatabaseConnection);

impl FetchFundDailyTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}

#[async_trait]
impl Task for FetchFundDailyTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let funds: Vec<etf::Model> = etf::Entity::find().all(&self.0).await?;
        let (start_date, end_date) = super::get_start_end_date_from_default()?;
        let mut curr = 0;
        for fund in &funds {
            let res = ext_api::tushare::fund_daily(&fund.ts_code, &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch fund daily failed, ts_code: {}, error: {:?}", fund.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let fund_dailys = res?;
            for fund_daily in &fund_dailys {
                let active_model = entity::fund_daily::ActiveModel { ..fund_daily.clone().into() };
                let pks = [
                    entity::fund_daily::Column::TsCode,
                    entity::fund_daily::Column::TradeDate,
                ];
                let update_columns = get_entity_update_columns::<entity::fund_daily::Entity>(&pks);
                let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
                    .update_columns(update_columns)
                    .to_owned();
                if let Err(e) = entity::fund_daily::Entity::insert(active_model)
                    .on_conflict(on_conflict)
                    .exec(&tx)
                    .await {
                    error!("insert fund daily failed, ts_code: {}, trade_date: {}, error: {:?}", fund.ts_code, fund_daily.trade_date, e);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert fund daily complete, ts_code: {}, fund daily size: {}, progress: {}/{}", fund.ts_code, fund_dailys.len(), curr, funds.len());
        }
        info!("fetch fund daily complete");
        Ok(())
    }
}