use anyhow::anyhow;
use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::fund;

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
        let funds: Vec<fund::Model> = fund::Entity::find().all(&self.0).await?;
        let start_date = NaiveDate::from_ymd_opt(2025, 3, 1).ok_or(anyhow!("date none"))?;
        let end_date = chrono::Local::now().date_naive();
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
                let res = entity::fund_daily::ActiveModel { ..fund_daily.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                      error!("insert fund daily failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", fund.ts_code, end_date, err, fund_daily);
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