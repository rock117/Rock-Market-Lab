use common::http;
use serde::{Deserialize, Serialize};

/// 东财基本信息响应结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct BasicOrgInfoResponse {
    pub version: String,
    pub result: BasicOrgInfoResult,
    pub success: bool,
    pub message: String,
    pub code: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicOrgInfoResult {
    pub pages: i32,
    pub data: Vec<BasicOrgInfo>,
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicOrgInfo {
    #[serde(rename = "SECUCODE")]
    pub secucode: Option<String>,
    #[serde(rename = "SECURITY_CODE")]
    pub security_code: Option<String>,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    pub security_name_abbr: Option<String>,
    #[serde(rename = "ORG_CODE")]
    pub org_code: Option<String>,
    #[serde(rename = "ORG_NAME")]
    pub org_name: Option<String>,
    #[serde(rename = "ORG_NAME_EN")]
    pub org_name_en: Option<String>,
    #[serde(rename = "FORMERNAME")]
    pub formername: Option<String>,
    #[serde(rename = "STR_CODEA")]
    pub str_codea: Option<String>,
    #[serde(rename = "STR_NAMEA")]
    pub str_namea: Option<String>,
    #[serde(rename = "STR_CODEB")]
    pub str_codeb: Option<String>,
    #[serde(rename = "STR_NAMEB")]
    pub str_nameb: Option<String>,
    #[serde(rename = "STR_CODEH")]
    pub str_codeh: Option<String>,
    #[serde(rename = "STR_NAMEH")]
    pub str_nameh: Option<String>,
    #[serde(rename = "SECURITY_TYPE")]
    pub security_type: Option<String>,
    #[serde(rename = "EM2016")]
    pub em2016: Option<String>,
    #[serde(rename = "TRADE_MARKET")]
    pub trade_market: Option<String>,
    #[serde(rename = "INDUSTRYCSRC1")]
    pub industrycsrc1: Option<String>,
    #[serde(rename = "PRESIDENT")]
    pub president: Option<String>,
    #[serde(rename = "LEGAL_PERSON")]
    pub legal_person: Option<String>,
    #[serde(rename = "SECRETARY")]
    pub secretary: Option<String>,
    #[serde(rename = "CHAIRMAN")]
    pub chairman: Option<String>,
    #[serde(rename = "SECPRESENT")]
    pub secpresent: Option<String>,
    #[serde(rename = "INDEDIRECTORS")]
    pub indedirectors: Option<String>,
    #[serde(rename = "ORG_TEL")]
    pub org_tel: Option<String>,
    #[serde(rename = "ORG_EMAIL")]
    pub org_email: Option<String>,
    #[serde(rename = "ORG_FAX")]
    pub org_fax: Option<String>,
    #[serde(rename = "ORG_WEB")]
    pub org_web: Option<String>,
    #[serde(rename = "ADDRESS")]
    pub address: Option<String>,
    #[serde(rename = "REG_ADDRESS")]
    pub reg_address: Option<String>,
    #[serde(rename = "PROVINCE")]
    pub province: Option<String>,
    #[serde(rename = "ADDRESS_POSTCODE")]
    pub address_postcode: Option<String>,
    #[serde(rename = "REG_CAPITAL")]
    pub reg_capital: Option<f64>,
    #[serde(rename = "REG_NUM")]
    pub reg_num: Option<String>,
    #[serde(rename = "EMP_NUM")]
    pub emp_num: Option<i32>,
    #[serde(rename = "TATOLNUMBER")]
    pub tatolnumber: Option<i32>,
    #[serde(rename = "LAW_FIRM")]
    pub law_firm: Option<String>,
    #[serde(rename = "ACCOUNTFIRM_NAME")]
    pub accountfirm_name: Option<String>,
    #[serde(rename = "ORG_PROFILE")]
    pub org_profile: Option<String>,
    #[serde(rename = "BUSINESS_SCOPE")]
    pub business_scope: Option<String>,
    #[serde(rename = "TRADE_MARKETT")]
    pub trade_markett: Option<String>,
    #[serde(rename = "TRADE_MARKET_CODE")]
    pub trade_market_code: Option<String>,
    #[serde(rename = "SECURITY_TYPEE")]
    pub security_typee: Option<String>,
    #[serde(rename = "SECURITY_TYPE_CODE")]
    pub security_type_code: Option<String>,
    #[serde(rename = "EXPAND_NAME_ABBRN")]
    pub expand_name_abbrn: Option<String>,
    #[serde(rename = "EXPAND_NAME_PINYIN")]
    pub expand_name_pinyin: Option<String>,
    #[serde(rename = "EXPAND_NAME_ABBR")]
    pub expand_name_abbr: Option<String>,
    #[serde(rename = "LISTING_DATE")]
    pub listing_date: Option<String>,
    #[serde(rename = "FOUND_DATE")]
    pub found_date: Option<String>,
    #[serde(rename = "MAIN_BUSINESS")]
    pub main_business: Option<String>,
    #[serde(rename = "HOST_BROKER")]
    pub host_broker: Option<String>,
    #[serde(rename = "TRANSFER_WAY")]
    pub transfer_way: Option<String>,
    #[serde(rename = "ACTUAL_HOLDER")]
    pub actual_holder: Option<String>,
    #[serde(rename = "MARKETING_START_DATE")]
    pub marketing_start_date: Option<String>,
    #[serde(rename = "MARKET_MAKER")]
    pub market_maker: Option<String>,
    #[serde(rename = "TRADE_MARKET_TYPE")]
    pub trade_market_type: Option<String>,
    #[serde(rename = "CURRENCY")]
    pub currency: Option<String>,
    #[serde(rename = "BOARD_NAME_LEVEL")]
    pub board_name_level: Option<String>,
}
/// 获取股票基本数据
pub async fn rpt_f10_basic_orginfo(tscode: &str) -> anyhow::Result<BasicOrgInfoResponse> {
    let url = format!(r#"https://datacenter.eastmoney.com/securities/api/data/v1/get?reportName=RPT_F10_BASIC_ORGINFO&columns=ALL&quoteColumns&filter=(SECUCODE="{}")&pageNumber=1"#, tscode);
    let resp = http::get(&url, None).await?;
    let response = resp.json().await?;
    Ok(response)
}

mod tests {
    use crate::dongcai::rpt_f10_basic_orginfo;

    #[tokio::test]
    pub async fn test() {
        let result = rpt_f10_basic_orginfo("300620.SZ").await;
        match result {
            Ok(response) => {
                println!("Success: {}", response.success);
                println!("Message: {}", response.message);
                if let Some(first_data) = response.result.data.first() {
                    println!("Company: {} ({})", 
                        first_data.org_name.as_deref().unwrap_or("N/A"), 
                        first_data.security_name_abbr.as_deref().unwrap_or("N/A"));
                    println!("Code: {}", first_data.secucode.as_deref().unwrap_or("N/A"));
                    println!("Industry: {}", first_data.em2016.as_deref().unwrap_or("N/A"));
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}