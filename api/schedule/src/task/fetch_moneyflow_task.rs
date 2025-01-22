use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stock;
use ext_api::tushare::moneyflow;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMoneyflowTask(DatabaseConnection);

impl FetchMoneyflowTask {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
#[async_trait]
impl Task for FetchMoneyflowTask{
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date= NaiveDate::from_ymd_opt(2010, 1, 1).ok_or(anyhow::anyhow!("Invalid date"))?;
        let end_date = Local::now().date_naive();
        let stocks:Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let stocks = stocks.into_iter().take(1).collect::<Vec<_>>();
        let mut curr = 0;
        for stock in &stocks {
            // 002135.SZ
            let moneyflows = moneyflow(&stock.ts_code, &start_date, &end_date).await;
            let moneyflows = moneyflow("002135.SZ", &start_date, &end_date).await;
            if let Err(e) = moneyflows {
                error!("fetch moneyflow failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            for moneyflow in &moneyflows? {
                let res = entity::moneyflow::ActiveModel { ..moneyflow.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert moneyflow failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", stock.ts_code, end_date, err, moneyflow);
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert moneyflow complete, ts_code: {}, complete: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch moneyflow complete");
        Ok(())
    }
}