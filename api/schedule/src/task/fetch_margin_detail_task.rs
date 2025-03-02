use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Local, NaiveDate};
use tracing::{error, info};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use crate::task::Task;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;

pub struct FetchMarginDetailTask(DatabaseConnection);

impl FetchMarginDetailTask {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }
}
#[async_trait]
impl Task for FetchMarginDetailTask {
    fn get_schedule(&self) -> String {
        todo!()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).ok_or(anyhow!("invalid date"))?;
        let end = Local::now().date_naive();
        let stocks:Vec<stock::Model> = entity::stock::Entity::find().all(&self.0).await?;
        for stock in &stocks {
            let margin_details = ext_api::tushare::margin_detail(&stock.ts_code, &start, &end).await?;
            for margin_detail in &margin_details {
                let res = entity::margin_detail::ActiveModel { ..margin_detail.clone().into() }.insert(&self.0).await;
                if let Err(e) = res {
                    error!("Failed to insert margin detail data for ts_code: {}, error: {:?}, data: {:?}", stock.ts_code, e, serde_json::to_string(&margin_detail));
                }
                info!("Insert margin detail data for ts_code: {}, trade_date: {}", stock.ts_code, margin_detail.trade_date);
            }
        }
        info!("fetch margin detail complete");
        Ok(())
    }
}