use crate::task::Task;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::{ActiveModelTrait, QuerySelect};
use entity::sea_orm::{Condition, DatabaseConnection, InsertResult, Set, TransactionTrait};
use entity::us_daily::{Model as UsDaily, Model};
use entity::{us_company_info, us_stock};
use ext_api::mstar;
use tracing::{error, info, warn};

use common::db::get_entity_update_columns;
use common::task_runner::run_with_limit;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::sea_query::OnConflict;
use entity::sea_orm::EntityOrSelect;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FetchUsCompanyInfoTask(DatabaseConnection);

impl FetchUsCompanyInfoTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsCompanyInfoTask(connection)
    }

    fn create_completion_handler(
        completed_count: Arc<std::sync::atomic::AtomicUsize>,
        db_conn: Arc<DatabaseConnection>,
        total_count: usize,
    ) -> impl Fn(us_stock::Model, (us_stock::Model, anyhow::Result<mstar::company::CompanyGeneralInfoResp>, anyhow::Result<mstar::company::CompanyBusinessDescriptionResp>)) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync {
        move |original_stock, (stock, company_result, desc_result)| {
            let completed_count = completed_count.clone();
            let db_conn = db_conn.clone();
            Box::pin(async move {
                match (company_result, desc_result) {
                    (Ok(company), Ok(desc)) => {
                        if let Err(e) = Self::save_company_info(&*db_conn, &stock, &company, &desc).await {
                            error!("save_company_info failed, stock: {:?}, err: {:?}", stock, e);
                        } else {
                            let current = completed_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                            info!("fetch us company info complete rate: {}/{}", current, total_count);
                        }
                    }
                    (Err(e), _) => {
                        error!("get_company_general_info failed, exchange_id: {}, symbol: {}, err: {:?}", stock.exchange_id, stock.symbol, e);
                    }
                    (_, Err(e)) => {
                        error!("get_company_business_description failed, exchange_id: {}, symbol: {}, err: {:?}", stock.exchange_id, stock.symbol, e);
                    }
                }
            })
        }
    }

    async fn save_company_info(
        db_conn: &DatabaseConnection,
        stock: &us_stock::Model,
        company: &mstar::company::CompanyGeneralInfoResp,
        desc: &mstar::company::CompanyBusinessDescriptionResp,
    ) -> anyhow::Result<()> {
        let info = &company.company_info_entity;
        let model = us_company_info::Model {
            symbol: stock.symbol.clone(),
            exchange_id: stock.exchange_id.clone(),
            web_address: info.web_address.clone(),
            local_name: info.local_name.clone(),
            local_name_language_code: info.local_name_language_code.clone(),
            short_name: info.short_name.clone(),
            business_country: info.business_country.clone(),
            domicile_country: info.domicile_country.clone(),
            place_of_incorporation: info.place_of_in_corporation.clone(),
            year_established: info.year_established.map(|v| v as i32),
            industry_name: info.industry_name.clone(),
            industry_group_name: info.industry_group_name.clone(),
            sector_name: info.sector_name.clone(),
            report_style_name: info.report_style_name.clone(),
            industry_template_name: info.industry_template_name.clone(),
            country: info.country.clone(),
            business_description: desc.business_description_entity.long_description.clone(),
        };
        let tx = db_conn.begin().await?;
        let pks = [
            us_company_info::Column::ExchangeId,
            us_company_info::Column::Symbol,
        ];
        let update_columns = get_entity_update_columns::<us_company_info::Entity>(&pks);
        let on_conflict = OnConflict::columns(pks)
            .update_columns(update_columns)
            .to_owned();
        let am = us_company_info::ActiveModel { ..model.clone().into() };
        if let Err(e) = us_company_info::Entity::insert(am)
            .on_conflict(on_conflict)
            .exec(&tx)
            .await {
            error!("insert us_company_info failed, stock: {:?}, err: {:?}", stock, e);
        }
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchUsCompanyInfoTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        // existing company infos, build key set (exchange_id, symbol)
        let existing_keys: HashSet<(String, String)> = us_company_info::Entity::find()
            .select_only()
            .column(us_company_info::Column::ExchangeId)
            .column(us_company_info::Column::Symbol)
            .into_tuple::<(String, String)>()
            .all(&self.0)
            .await?
            .into_iter()
            .collect();

        // only keep stocks that don't have company info yet
        let all_stocks = us_stock::Entity::find()
            .all(&self.0)
            .await?;
        let stocks: Vec<us_stock::Model> = all_stocks
            .into_iter()
            .filter(|s| !existing_keys.contains(&(s.exchange_id.clone(), s.symbol.clone())))
            .collect();

        let total_count = stocks.len();
        let completed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let db_conn = Arc::new(self.0.clone());

        let handler = Self::create_completion_handler(completed_count, db_conn, total_count);
        
        run_with_limit(
            5, // 并发数为5
            stocks,
            |stock| async move {
                // 并发执行两个API调用
                let (company_result, desc_result) = tokio::join!(
                    mstar::company::get_company_general_info(stock.exchange_id.as_str(), stock.symbol.as_str()),
                    mstar::company::get_company_business_description(stock.exchange_id.as_str(), stock.symbol.as_str())
                );
                (stock, company_result, desc_result)
            },
            handler,
        ).await;
        info!("save company info complete");
        Ok(())
    }
}