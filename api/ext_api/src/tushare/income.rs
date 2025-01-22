use std::collections::HashMap;
use chrono::NaiveDate;
use serde_json;
use map_macro::hash_map;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;
use entity::income;
use entity::prelude::Income;
use entity::sea_orm::prelude::{BigDecimal, Decimal};

/// 利润表
pub async fn income(ts_code: &str, report_type: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<income::Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let datas =  call_tushare_api_as::<500, IncomeData>(Api::income,
                                                        &hash_map! {"ts_code" => ts_code, "report_type" => report_type, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
                                                        &[
                                                  "ts_code",
                                                  "ann_date",
                                                  "f_ann_date",
                                                  "end_date",
                                                  "report_type",
                                                  "comp_type",
                                                  "end_type",
                                                  "basic_eps",
                                                  "diluted_eps",
                                                  "total_revenue",
                                                  "revenue",
                                                  "int_income",
                                                  "prem_earned",
                                                  "comm_income",
                                                  "n_commis_income",
                                                  "n_oth_income",
                                                  "n_oth_b_income",
                                                  "prem_income",
                                                  "out_prem",
                                                  "une_prem_reser",
                                                  "reins_income",
                                                  "n_sec_tb_income",
                                                  "n_sec_uw_income",
                                                  "n_asset_mg_income",
                                                  "oth_b_income",
                                                  "fv_value_chg_gain",
                                                  "invest_income",
                                                  "ass_invest_income",
                                                  "forex_gain",
                                                  "total_cogs",
                                                  "oper_cost",
                                                  "int_exp",
                                                  "comm_exp",
                                                  "biz_tax_surchg",
                                                  "sell_exp",
                                                  "admin_exp",
                                                  "fin_exp",
                                                  "assets_impair_loss",
                                                  "prem_refund",
                                                  "compens_payout",
                                                  "reser_insur_liab",
                                                  "div_payt",
                                                  "reins_exp",
                                                  "oper_exp",
                                                  "compens_payout_refu",
                                                  "insur_reser_refu",
                                                  "reins_cost_refund",
                                                  "other_bus_cost",
                                                  "operate_profit",
                                                  "non_oper_income",
                                                  "non_oper_exp",
                                                  "nca_disploss",
                                                  "total_profit",
                                                  "income_tax",
                                                  "n_income",
                                                  "n_income_attr_p",
                                                  "minority_gain",
                                                  "oth_compr_income",
                                                  "t_compr_income",
                                                  "compr_inc_attr_p",
                                                  "compr_inc_attr_m_s",
                                                  "ebit",
                                                  "ebitda",
                                                  "insurance_exp",
                                                  "undist_profit",
                                                  "distable_profit",
                                                  "rd_exp",
                                                  "fin_exp_int_exp",
                                                  "fin_exp_int_inc",
                                                  "transfer_surplus_rese",
                                                  "transfer_housing_imprest",
                                                  "transfer_oth",
                                                  "adj_lossgain",
                                                  "withdra_legal_surplus",
                                                  "withdra_legal_pubfund",
                                                  "withdra_biz_devfund",
                                                  "withdra_rese_fund",
                                                  "withdra_oth_ersu",
                                                  "workers_welfare",
                                                  "distr_profit_shrhder",
                                                  "prfshare_payable_dvd",
                                                  "comshare_payable_dvd",
                                                  "capit_comstock_div",
                                                  "net_after_nr_lp_correct",
                                                  "credit_impa_loss",
                                                  "net_expo_hedging_benefits",
                                                  "oth_impair_loss_assets",
                                                  "total_opcost",
                                                  "amodcost_fin_assets",
                                                  "oth_income",
                                                  "asset_disp_income",
                                                  "continued_net_profit",
                                                  "end_net_profit",
                                                  "update_flag"
                                              ]).await?;

    let mut incomes = vec![];
    for data in datas {
        let compr_inc_attr_m_s = data.compr_inc_attr_m_s.clone();
        let mut income = data.extra;
        income.compr_inc_attr_ms = compr_inc_attr_m_s;
        incomes.push(income);
    }
    Ok(incomes)
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomeData {
    compr_inc_attr_m_s: Option<Decimal>, // sea_orm bug, 字段转换bug: compr_inc_attr_m_s -> compr_inc_attr_ms
    #[serde(flatten)]
    extra: income::Model,
}