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
use entity::{stock, cn_security_info};
use ext_api::dongcai;
use tracing::{error, info, warn};

use common::db::get_entity_update_columns;
use common::task_runner::run_with_limit;
use entity::sea_orm::prelude::Decimal;
use entity::sea_orm::sea_query::OnConflict;
use entity::sea_orm::EntityOrSelect;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FetchBasicOrgInfoTask(DatabaseConnection);

impl FetchBasicOrgInfoTask {
    pub fn new(connection: DatabaseConnection) -> Self {
        FetchBasicOrgInfoTask(connection)
    }

    fn create_completion_handler(
        completed_count: Arc<std::sync::atomic::AtomicUsize>,
        db_conn: Arc<DatabaseConnection>,
        total_count: usize,
    ) -> impl Fn(stock::Model, (stock::Model, anyhow::Result<dongcai::BasicOrgInfoResponse>, anyhow::Result<dongcai::ConceptsResponse>)) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync {
        move |_original_stock, (stock, res, res2)| {
            let completed_count = completed_count.clone();
            let db_conn = db_conn.clone();
            Box::pin(async move {
                match (res, res2) {
                    (Ok(info), Ok(concept)) => {
                        if let Err(e) = Self::save_company_info(&*db_conn, &stock, &info, &concept).await {
                            error!("cn security info failed, stock: {:?}, err: {:?}", stock, e);
                        } else {
                            let current = completed_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                            info!("fetch cn security info rate: {}/{}", current, total_count);
                        }
                    }
                    (Err(e), _) => {
                        error!("get cn security info failed,  symbol: {}, err: {:?}", stock.ts_code, e);
                    }
                    (_, Err(e)) => {
                        error!("get cn security concepts failed,  symbol: {}, err: {:?}", stock.ts_code, e);
                    }
                }
            })
        }
    }


    fn get_concepts(resp: &dongcai::ConceptsResponse) -> anyhow::Result<Vec<String>>{
        if !resp.success {
           return  Ok(vec![])
        }
        let Some(result) = &resp.result else {
            return  Ok(vec![])
        };
        let mut concepts = Vec::new();
        for item in  result.data.iter() {
            concepts.push(item.board_name.clone());
        }
        Ok(concepts)
    }


    async fn save_company_info(
        db_conn: &DatabaseConnection,
        stock: &stock::Model,
        org_resp: &dongcai::BasicOrgInfoResponse,
        concept_resp: &dongcai::ConceptsResponse,
    ) -> anyhow::Result<()> {
        // 检查响应是否成功
        if !org_resp.success {
            return Err(anyhow!("API response not successful: {:?}", org_resp.message));
        }
        let Some(result) = &org_resp.result else {
            return Err(anyhow!("No data found in response"));
        };
        // 获取第一个数据元素
        let basic_info =  result.data.first()
            .ok_or_else(|| anyhow!("No data found in response"))?;
        if basic_info.secucode.is_none() {
            return Err(anyhow!("API response not successful, secucode is empty: {:?}", org_resp.message));
        }
        let concepts = Self::get_concepts(concept_resp)?.join(",");
        // 转换为 cn_security_info::Model
        let model = cn_security_info::Model {
            secucode: basic_info.secucode.clone().unwrap_or_default(),
            security_code: basic_info.security_code.clone().unwrap_or_default(),
            security_name_abbr: basic_info.security_name_abbr.clone().unwrap_or_default(),
            security_type: basic_info.security_type.clone(),
            security_type_code: basic_info.security_type_code.clone(),
            org_code: basic_info.org_code.clone(),
            org_name: basic_info.org_name.clone().unwrap_or_default(),
            org_name_en: basic_info.org_name_en.clone(),
            formername: basic_info.formername.clone(),
            str_codea: basic_info.str_codea.clone(),
            str_namea: basic_info.str_namea.clone(),
            str_codeb: basic_info.str_codeb.clone(),
            str_nameb: basic_info.str_nameb.clone(),
            str_codeh: basic_info.str_codeh.clone(),
            str_nameh: basic_info.str_nameh.clone(),
            em2016: basic_info.em2016.clone(),
            trade_market: basic_info.trade_market.clone(),
            trade_markett: basic_info.trade_markett.clone(),
            trade_market_code: basic_info.trade_market_code.clone(),
            industrycsrc1: basic_info.industrycsrc1.clone(),
            board_name_level: basic_info.board_name_level.clone(),
            currency: basic_info.currency.clone(),
            president: basic_info.president.clone(),
            legal_person: basic_info.legal_person.clone(),
            secretary: basic_info.secretary.clone(),
            chairman: basic_info.chairman.clone(),
            secpresent: basic_info.secpresent.clone(),
            indedirectors: basic_info.indedirectors.clone(),
            org_tel: basic_info.org_tel.clone(),
            org_email: basic_info.org_email.clone(),
            org_fax: basic_info.org_fax.clone(),
            org_web: basic_info.org_web.clone(),
            address: basic_info.address.clone(),
            reg_address: basic_info.reg_address.clone(),
            province: basic_info.province.clone(),
            address_postcode: basic_info.address_postcode.clone(),
            reg_capital: basic_info.reg_capital.map(|v| Decimal::from_f64_retain(v).unwrap_or_default()),
            reg_num: basic_info.reg_num.clone(),
            emp_num: basic_info.emp_num,
            tatolnumber: basic_info.tatolnumber,
            law_firm: basic_info.law_firm.clone(),
            accountfirm_name: basic_info.accountfirm_name.clone(),
            org_profile: basic_info.org_profile.clone(),
            business_scope: basic_info.business_scope.clone(),
            main_business: basic_info.main_business.clone(),
            listing_date: basic_info.listing_date.clone(),
            found_date: basic_info.found_date.clone(),
            marketing_start_date: basic_info.marketing_start_date.clone(),
            expand_name_abbrn: basic_info.expand_name_abbrn.clone(),
            expand_name_pinyin: basic_info.expand_name_pinyin.clone(),
            expand_name_abbr: basic_info.expand_name_abbr.clone(),
            host_broker: basic_info.host_broker.clone(),
            transfer_way: basic_info.transfer_way.clone(),
            actual_holder: basic_info.actual_holder.clone(),
            market_maker: basic_info.market_maker.clone(),
            trade_market_type: basic_info.trade_market_type.clone(),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            concepts: Some(concepts),
        };
        let tx = db_conn.begin().await?;
        let pks = [
            cn_security_info::Column::SecurityCode,
        ];
        let update_columns = get_entity_update_columns::<cn_security_info::Entity>(&pks);
        let on_conflict = OnConflict::columns(pks)
            .update_columns(update_columns)
            .to_owned();
        let am = cn_security_info::ActiveModel { ..model.clone().into() };
        if let Err(e) = cn_security_info::Entity::insert(am)
            .on_conflict(on_conflict)
            .exec(&tx)
            .await {
            error!("insert cn_security_info failed, stock: {:?}, err: {:?}", stock, e);
        }
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl Task for FetchBasicOrgInfoTask {
    fn get_schedule(&self) -> String {
        "0 5 23 * * *".to_string()
    }

    async fn run(&self) -> anyhow::Result<()> {
        // existing company infos, build key set (exchange_id, symbol)
        let existing_keys: HashSet<String> = cn_security_info::Entity::find()
            .select_only()
            .column(cn_security_info::Column::Secucode)
            .into_tuple::<String>()
            .all(&self.0)
            .await?
            .into_iter()
            .collect();

        // only keep stocks that don't have company info yet
        let all_stocks = stock::Entity::find()
            .all(&self.0)
            .await?;
        let stocks: Vec<stock::Model> = all_stocks
            .into_iter()
            .filter(|s| !existing_keys.contains(&s.ts_code))
            .collect();

        info!("total stocks: {}", stocks.len());
        let total_count = stocks.len();
        let completed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let db_conn = Arc::new(self.0.clone());

        let handler = Self::create_completion_handler(completed_count, db_conn, total_count);

        run_with_limit(
            3,
            stocks,
            |stock| async move {
                let profile = dongcai::rpt_f10_basic_orginfo(stock.ts_code.as_str()).await;
                let concept = dongcai::rpt_f10_coretheme_boardtype(stock.ts_code.as_str()).await;
                (stock, profile, concept)
            },
            handler,
        ).await;
        info!("save company info complete");
        Ok(())
    }
}