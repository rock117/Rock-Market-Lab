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
use entity::{us_company_info, us_stock, us_main_indicator};
use ext_api::{mstar, dongcai};
use tracing::{error, info, warn};

use common::db::get_entity_update_columns;
use common::task_runner::run_with_limit;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::sea_query::OnConflict;
use entity::sea_orm::EntityOrSelect;
use std::collections::HashSet;
use std::sync::Arc;
use ext_api::dongcai::usf10_data_mainindicator::RptUsf10DataMainindicatorResp;

pub struct FetchUsMainIndicatorTask(DatabaseConnection);

impl FetchUsMainIndicatorTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchUsMainIndicatorTask(connection)
    }

    fn create_completion_handler(
        completed_count: Arc<std::sync::atomic::AtomicUsize>,
        db_conn: Arc<DatabaseConnection>,
        total_count: usize,
    ) -> impl Fn(us_stock::Model,  (us_stock::Model, anyhow::Result<RptUsf10DataMainindicatorResp>)) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync {
        move |stock, (st, usf10_data_mainindicator_result)| {
            let completed_count = completed_count.clone();
            let db_conn = db_conn.clone();
            Box::pin(async move {
                match usf10_data_mainindicator_result {
                    Ok(indicator) => {
                        let res = Self::save_main_indictor(&stock.exchange_id, &indicator, &*db_conn).await;
                        if let Err(e) = res {
                            error!("save_main_indictor failed, indicator: {:?}, err: {:?}", indicator, e);
                        }
                    }
                    Err(e) => {
                         error!("save_main_indictor failed, exchange_id: {}, symbol: {}, err: {:?}", stock.exchange_id, stock.symbol, e);
                    }
                }
            })
        }
    }

    async fn save_main_indictor(exchange_id: &str, resp: &RptUsf10DataMainindicatorResp,  db_conn: &DatabaseConnection,) -> anyhow::Result<()> {
        let records = resp.result.clone().ok_or_else(|| anyhow!("save_main_indictor failed"))?.data;
        let Some(record) = records.first() else {
            return Ok(());
        };
        
        let us_main_indicator = us_main_indicator::Model {
            symbol: record.secucode.clone().replace(".O", ""),
            secucode: record.security_code.clone(),
            exchange_id: exchange_id.to_string(),
            total_market_cap: record.total_market_cap.clone().map(|v| Decimal::try_from(v).ok()).flatten(),
            currency: Some(record.currency.clone()),
            pe_ttm: record.pe_ttm.clone().map(|v| Decimal::try_from(v).ok()).flatten(),
            sale_gpr: record.sale_gpr.clone().map(|v| Decimal::try_from(v).ok()).flatten(),
            pb:record.pb.clone().map(|v| Decimal::try_from(v).ok()).flatten(),
            dividend_rate: record.dividend_rate.clone().map(|v| Decimal::try_from(v).ok()).flatten(),
            std_report_date: record.std_report_date.clone(),
            report_date: record.report_date.clone(),
        };
        let active_model = entity::us_main_indicator::ActiveModel { ..us_main_indicator.clone().into() };
        let pks = [
                us_main_indicator::Column::ExchangeId,
                us_main_indicator::Column::Symbol,
                us_main_indicator::Column::ReportDate,
        ];
        let update_columns = get_entity_update_columns::<entity::us_main_indicator::Entity>(&pks);
        let on_conflict = entity::sea_orm::sea_query::OnConflict::columns(pks)
            .update_columns(update_columns)
            .to_owned();
        let tx = db_conn.begin().await?;
        entity::us_main_indicator::Entity::insert(active_model)
            .on_conflict(on_conflict)
            .exec(&tx)
            .await?; 
        tx.commit().await?;
        Ok(())
    }

}

#[async_trait]
impl Task for FetchUsMainIndicatorTask {
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
           // .filter(|s| !existing_keys.contains(&(s.exchange_id.clone(), s.symbol.clone())))
        //   .filter(|s| s.symbol == "AAPL")
            .collect();

        let total_count = stocks.len();
        let completed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let db_conn = Arc::new(self.0.clone());

        let handler = Self::create_completion_handler(completed_count, db_conn, total_count);
        
        run_with_limit(
            5, // 并发数为5
            stocks,
            |stock| async move {
                let mainindicator_result = dongcai::usf10_data_mainindicator::rpt_usf10_data_mainindicator(stock.symbol.as_str()).await;
                (stock.clone(), mainindicator_result)
            },
            handler,
        ).await;
        info!("save company info complete");
        Ok(())
    }
}