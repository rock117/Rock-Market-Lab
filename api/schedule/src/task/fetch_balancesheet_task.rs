use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchBalancesheetTask(DatabaseConnection);

impl FetchBalancesheetTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchBalancesheetTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start_date = NaiveDate::parse_from_str("20200101", "%Y%m%d")?;
        let end_date = Local::now().date_naive();
        let stocks: Vec<stock::Model> = stock::Entity::find().all(&self.0).await?;
        let mut curr = 0;
        for stock in &stocks {
            let balancesheet = ext_api::tushare::balancesheet(&stock.ts_code, "1", &start_date, &end_date).await;
            if let Err(e) = balancesheet {
                error!("fetch balancesheet failed, ts_code: {}, error: {:?}", stock.ts_code, e);
                continue;
            }
            for balance in balancesheet? {
                let res = entity::balancesheet::ActiveModel { ..balance.clone().into() }.insert(&self.0).await;
                info!("ts_code: {}, end_date: {}, ann_date: {:?}, f_ann_date: {:?}, report_type: {:?}, comp_type: {:?}, end_type: {:?}", balance.ts_code, balance.end_date, balance.ann_date, balance.f_ann_date, balance.report_type, balance.comp_type, balance.end_type);
                if let Err(err) = res {
                    error!("insert balancesheet failed, ts_code: {}, end_date: {}, error: {:?}", stock.ts_code, end_date, err);
                    continue;
                }
            }
            curr += 1;
            info!("insert balancesheet complete, ts_code: {}, progress: {}/{}", stock.ts_code, curr, stocks.len());
        }
        info!("fetch balancesheet task complete");
        Ok(())
    }
}