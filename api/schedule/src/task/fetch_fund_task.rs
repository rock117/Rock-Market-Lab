use anyhow::bail;
use async_trait::async_trait;
use tracing::{error, info};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};
use crate::task::Task;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::{fund, stock};
use ext_api::tushare::FundMarket;

pub struct FetchFundTask(DatabaseConnection);

impl FetchFundTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchFundTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        let markets = vec![FundMarket::O, FundMarket::E];
        for market in markets {
            let res = ext_api::tushare::fund_basic(market).await;
            if let Err(e) = res {
                error!("fetch fund failed, error: {:?}", e);
                continue;
            }

            let mut funds = res?;
            let tx = self.0.begin().await?;
            for mut fund in funds {
                fund.name_py = fund.name.as_ref().map(|name| common::get_security_pinyin(&name));
                let old_res = fund::Entity::find_by_id(&fund.ts_code).one(&self.0).await;
                if let Ok(None) = old_res {
                    let res = entity::fund::ActiveModel { ..fund.clone().into() }.insert(&self.0).await;
                    if let Err(err) = res {
                        error!("insert fund failed, error: {:?}", err);
                    }
                } else if let Ok(_) = old_res {
                    let res = entity::fund::ActiveModel { ..fund.clone().into() }.update(&self.0).await;
                    if let Err(err) = res{
                        error!("update fund failed data: {:?}, error: {:?}", fund, err);
                    }
                }
            }
            tx.commit().await?;
            info!("fetch fund of market: {} complete", market);
        }
        info!("fetch fund complete");
        Ok(())
    }
}