use async_trait::async_trait;
use tracing::{error, warn, info};
use entity::sea_orm::{DatabaseConnection, Set, TransactionTrait};
use entity::stock;
use crate::task::Task;
use ext_api::tushare;
use entity::stock::Model as Stock;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;

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
        for mut stock_m in stocks {
            let ts_code = stock_m.ts_code.clone();
            let old_res = stock::Entity::find_by_id(&ts_code).one(&self.0).await;
            if let Some(ref name) = stock_m.name {
                stock_m.name_py = Some(common::get_security_pinyin(name));
            }
            info!("name_py = {:?}, name = {:?}", stock_m.name_py, stock_m.name_py);
            if let Ok(None) = old_res {
                let res = stock::ActiveModel { ..stock_m.clone().into() }.insert(&self.0).await;
                if res.is_err() {
                    error!("insert stock failed, ts_code: {}, data: {:?}, error: {:?}", ts_code, stock_m, res);
                }
            } else if let Err(e) = old_res {
                error!("find stock by  ts_code failed, ts_code: {}, error: {:?}", ts_code, e);
            } else {
                let res = stock::ActiveModel { ..stock_m.clone().into() }.update(&self.0).await;
                if res.is_err() {
                    error!("update stock failed ts_code: {}, data: {:?}, error: {:?}", ts_code, stock_m, res);
                }
            }
            curr += 1;
            info!("insert stock complete: {}, {}/{}", ts_code, curr, total);
        }
        tx.commit().await?;
        info!("fetch stock list task complete...");
        Ok(())
    }
}