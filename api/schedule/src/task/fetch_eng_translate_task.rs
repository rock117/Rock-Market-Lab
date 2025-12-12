use async_trait::async_trait;
use serde::de::IntoDeserializer;
use tracing::{error, info};
use common::db::get_entity_update_columns;
use common::llm;
use common::task_runner::run_with_limit;
use entity::{us_company_info};
use entity::sea_orm::{ActiveModelTrait, Condition, DatabaseConnection, Set};
use ext_api::tushare::dc_member;
use crate::task::fetch_dc_member_task::FetchDcMemberTask;
use crate::task::Task;
use entity::sea_orm::EntityTrait;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FetchEngTranslateTask(DatabaseConnection);

impl FetchEngTranslateTask {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

#[async_trait]
impl Task for FetchEngTranslateTask {
    fn get_schedule(&self) -> String {
        "0 0 0 1 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {

        // load all unique ts_code from dc_index using SQL DISTINCT
        let companies = us_company_info::Entity::find()
            .all(&self.0)
            .await?;

        let companies: Vec<us_company_info::Model> = companies
            .into_iter()
            .filter(|c| c.sector_name.is_some() && c.sector_name_cn.is_none())
            .collect();
        let total = companies.len();
        let completed = Arc::new(AtomicUsize::new(0));
        let db = Arc::new(self.0.clone());

        run_with_limit(
            30,
            companies,
            {
                let db = db.clone();
                move |company: us_company_info::Model| {
                    let db = db.clone();
                    async move {
                        // let desc = company.business_description.clone().unwrap();
                        // let translate = llm::translate_finance_eng(&desc).await?;
                        // let am = us_company_info::ActiveModel {
                        //     exchange_id: Set(company.exchange_id.clone()),
                        //     symbol: Set(company.symbol.clone()),
                        //     business_description_cn: Set(Some(translate)),
                        //     ..Default::default()
                        // };

                        let sector_name_cn = match company.sector_name {
                            Some(sector_name) => Some(llm::translate_finance_eng(&sector_name).await?),
                            None => None,
                        };
                        let industry_name_cn = match company.industry_name {
                            Some(industry_name) => Some(llm::translate_finance_eng(&industry_name).await?),
                            None => None,
                        };
                        let am = us_company_info::ActiveModel {
                            exchange_id: Set(company.exchange_id.clone()),
                            symbol: Set(company.symbol.clone()),
                            sector_name_cn: Set(sector_name_cn),
                            industry_name_cn: Set(industry_name_cn),
                            ..Default::default()
                        };

                        let res = am.update(&*db).await;
                        if let Err(e) = res {
                            error!("failed to update, {:?}", e);
                        }
                        Ok::<(), anyhow::Error>(())
                    }
                }
            },
            {
                let completed = completed.clone();
                move |company: us_company_info::Model, result: anyhow::Result<()>| {
                    let completed = completed.clone();
                    async move {
                        match result {
                            Ok(_) => {
                                let current = completed.fetch_add(1, Ordering::SeqCst) + 1;
                                info!(
                                    "translate us_company_info complete rate: {}/{}, exchange_id: {}, symbol: {}",
                                    current,
                                    total,
                                    company.exchange_id,
                                    company.symbol
                                );
                            }
                            Err(e) => error!(
                                "translate us_company_info failed, exchange_id: {}, symbol: {}, err: {:?}",
                                company.exchange_id,
                                company.symbol,
                                e
                            ),
                        }
                    }
                }
            },
        )
        .await;

        Ok(())
    }
}