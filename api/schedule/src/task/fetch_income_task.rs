use async_trait::async_trait;
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::stock;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use crate::task::fetch_balancesheet_task::FetchBalancesheetTask;

pub struct FetchIncomeTask(DatabaseConnection);
impl FetchIncomeTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}
#[async_trait]
impl Task for FetchIncomeTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::parse_from_str("20200101", "%Y%m%d")?;
        let end_date = Local::now().date_naive();
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let res = ext_api::tushare::income(&stock.ts_code, "1", &start_date, &end_date).await;
            if let Err(e) = res {
                error!("fetch income failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            let tx = self.0.begin().await?;
            let incomes = res?;
            for income in incomes {
                let res = entity::income::ActiveModel { ..income.clone().into() }.insert(&self.0).await;
                if let Err(err) = res {
                    error!("insert income failed, ts_code: {}, end_date: {}, error: {:?}, data: {:?}", stock.ts_code, end_date, err, income);
                    error!("income json: {}", serde_json::to_string(&income).unwrap());
                    panic!("exit");
                    continue;
                }
            }
            tx.commit().await?;
            curr += 1;
            info!("insert finance_indicator complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch income task complete");
        Ok(())
    }
}