use common::http;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RptUsf10DataMainindicatorResp {
    pub version: Option<String>,
    pub result: Option<RptUsf10DataMainindicatorResult>,
    pub success: bool,
    pub message: String,
    pub code: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RptUsf10DataMainindicatorResult {
    pub pages: i64,
    pub data: Vec<RptUsf10DataMainindicatorRecord>,
    pub count: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RptUsf10DataMainindicatorRecord {
    #[serde(rename = "SECUCODE")]
    pub secucode: String,
    #[serde(rename = "SECURITY_CODE")]
    pub security_code: String,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    pub security_name_abbr: String,
    #[serde(rename = "REPORT_DATE")]
    pub report_date: String,
    #[serde(rename = "CURRENCY")]
    pub currency: String,

    #[serde(rename = "PE_TTM")]
    pub pe_ttm: Option<f64>,
    #[serde(rename = "RATIO_EPS_TTM")]
    pub ratio_eps_ttm: Option<f64>,
    #[serde(rename = "DPS_USD")]
    pub dps_usd: Option<f64>,
    #[serde(rename = "SALE_GPR")]
    pub sale_gpr: Option<f64>,
    #[serde(rename = "TURNOVER")]
    pub turnover: Option<f64>,
    #[serde(rename = "HOLDER_PROFIT")]
    pub holder_profit: Option<f64>,
    #[serde(rename = "ISSUED_COMMON_SHARES")]
    pub issued_common_shares: Option<f64>,
    #[serde(rename = "PB")]
    pub pb: Option<f64>,
    #[serde(rename = "BVPS")]
    pub bvps: Option<f64>,
    #[serde(rename = "DIVIDEND_RATE")]
    pub dividend_rate: Option<f64>,
    #[serde(rename = "SALE_NPR")]
    pub sale_npr: Option<f64>,
    #[serde(rename = "TURNOVER_YOY")]
    pub turnover_yoy: Option<f64>,
    #[serde(rename = "HOLDER_PROFIT_YOY")]
    pub holder_profit_yoy: Option<f64>,
    #[serde(rename = "TOTAL_MARKET_CAP")]
    pub total_market_cap: Option<f64>,
    #[serde(rename = "SECURITY_INNER_CODE")]
    pub security_inner_code: Option<String>,

    #[serde(rename = "NET_INTEREST_INCOME")]
    pub net_interest_income: Option<f64>,
    #[serde(rename = "LOAN_LOSS_PROVISION")]
    pub loan_loss_provision: Option<f64>,
    #[serde(rename = "REINSURE_INCOME")]
    pub reinsure_income: Option<f64>,
    #[serde(rename = "COMPENSATE_EXPENSE")]
    pub compensate_expense: Option<f64>,

    #[serde(rename = "ORG_TYPE")]
    pub org_type: Option<String>,
    #[serde(rename = "SECURITY_TYPE")]
    pub security_type: Option<String>,
    #[serde(rename = "CURRENCY_ABBR")]
    pub currency_abbr: Option<String>,

    #[serde(rename = "EPS_TTM_CNY")]
    pub eps_ttm_cny: Option<f64>,
    #[serde(rename = "EPS_TTM_USD")]
    pub eps_ttm_usd: Option<f64>,
    #[serde(rename = "EPS_TTM_HKD")]
    pub eps_ttm_hkd: Option<f64>,

    #[serde(rename = "STD_REPORT_DATE")]
    pub std_report_date: Option<String>,

    #[serde(rename = "SALE_GPR_SOURCE")]
    pub sale_gpr_source: Option<String>,
    #[serde(rename = "TURNOVER_SOURCE")]
    pub turnover_source: Option<String>,
    #[serde(rename = "TURNOVER_YOY_SOURCE")]
    pub turnover_yoy_source: Option<String>,
    #[serde(rename = "HOLDER_PROFIT_SOURCE")]
    pub holder_profit_source: Option<String>,
    #[serde(rename = "HOLDER_PROFIT_YOYSOURCE")]
    pub holder_profit_yoysource: Option<String>,
    #[serde(rename = "PB_SOURCE")]
    pub pb_source: Option<String>,
    #[serde(rename = "BVPS_SOURCE")]
    pub bvps_source: Option<String>,
    #[serde(rename = "SALE_NPR_SOURCE")]
    pub sale_npr_source: Option<String>,
    #[serde(rename = "ISSUED_COMMON_SHARESSOURCE")]
    pub issued_common_share_source: Option<String>,

    #[serde(rename = "PE_TTM_EXPLAIN")]
    pub pe_ttm_explain: Option<String>,
    #[serde(rename = "RATIO_EPS_TTMEXPLAIN")]
    pub ratio_eps_ttm_explain: Option<String>,
    #[serde(rename = "DPS_USD_EXPLAIN")]
    pub dps_usd_explain: Option<String>,
    #[serde(rename = "DIVIDEND_RATE_EXPLAIN")]
    pub dividend_rate_explain: Option<String>,
    #[serde(rename = "TOTAL_MARKET_CAPEXPLAIN")]
    pub total_market_cap_explain: Option<String>,

    #[serde(rename = "NET_INTEREST_INCOMESOURCE")]
    pub net_interest_income_source: Option<String>,
    #[serde(rename = "LOAN_LOSS_PROVISIONSOURCE")]
    pub loan_loss_provision_source: Option<String>,
    #[serde(rename = "REINSURE_INCOME_SOURCE")]
    pub reinsure_income_source: Option<String>,
    #[serde(rename = "COMPENSATE_EXPENSE_SOURCE")]
    pub compensate_expense_source: Option<String>,
}

/// 获取美股股票指标基本数据 包含 市盈率，上市时间, 市值，毛利率等
pub async fn rpt_usf10_data_mainindicator(tscode: &str) -> anyhow::Result<RptUsf10DataMainindicatorResp> {
    let tscode = format!("{}.O", tscode);
    let url = format!(r#"https://datacenter.eastmoney.com/securities/api/data/v1/get?reportName=RPT_USF10_DATA_MAININDICATOR&columns=ALL&quoteColumns&filter=(SECUCODE="{}")&pageNumber=1&pageSize=200&sortTypes=-1&sortColumns=REPORT_DATE&source=INTLSECURITIES&client=PC"#, tscode); // AAPL.O
    let resp = http::get(&url, None).await?;
    let response = http::to_string(resp).await?;
    let parsed: RptUsf10DataMainindicatorResp = serde_json::from_str(&response)?;
    Ok(parsed)
}

mod tests {
    use super::rpt_usf10_data_mainindicator;

    #[tokio::test]
    async fn test() {
        let response = rpt_usf10_data_mainindicator("GOOG").await;
        println!("{:?}", response);
    }
}