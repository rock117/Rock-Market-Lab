
use serde::Deserialize;
use common::{http, ExchangeId};
use crate::mstar::equity::StockListResp;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyBusinessDescriptionResp {
    pub message_info: MessageInfo,
    pub general_info: GeneralInfo,
    pub business_description_entity: BusinessDescriptionEntity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageInfo {
    pub message_code: u32,
    pub message_detail: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralInfo {
    pub share_class_id: Option<String>,
    pub company_name: Option<String>,
    pub exchange_id: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "CIK")]
    pub cik: Option<String>,
    #[serde(rename = "ISIN")]
    pub isin: Option<String>,
    #[serde(rename = "SEDOL")]
    pub sedol: Option<String>,
    pub country_id: Option<String>,
    #[serde(rename = "CompanyLEI")]
    pub company_lei: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessDescriptionEntity {
    pub long_description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyGeneralInfoResp {
    pub message_info: MessageInfo,
    pub general_info: GeneralInfo,
    pub company_info_entity: CompanyInfoEntity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompanyInfoEntity {
    pub company_status: Option<String>,
    pub status_type: Option<String>,
    pub local_name: Option<String>,
    pub local_name_language_code: Option<String>,
    pub short_name: Option<String>,
    pub business_country: Option<String>,
    pub domicile_country: Option<String>,
    pub place_of_in_corporation: Option<String>,
    pub year_established: Option<u32>,
    pub fiscal_year_end: Option<u32>,
    #[serde(rename = "IsREIT")]
    pub is_reit: Option<bool>,
    pub is_shell: Option<bool>,
    pub is_limited_partnership: Option<bool>,
    pub operation_status: Option<String>,
    pub web_address: Option<String>,
    pub address_language_code: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub total_employee: Option<u32>,
    pub full_time: Option<u32>,
    pub auditor: Option<String>,
    pub industry_id: Option<String>,
    pub industry_name: Option<String>,
    pub industry_group_id: Option<String>,
    pub industry_group_name: Option<String>,
    pub sector_id: Option<String>,
    pub sector_name: Option<String>,
    pub report_style_name: Option<String>,
    pub industry_template_name: Option<String>,
    #[serde(rename = "NACE")]
    pub nace: Option<String>,
    #[serde(rename = "ISIC")]
    pub isic: Option<String>,
    pub expected_fiscal_year_end: Option<String>,
    pub registered_address_language_code: Option<String>,
    pub registered_address_line1: Option<String>,
    pub registered_address_line2: Option<String>,
    pub registered_city: Option<String>,
    pub registered_country: Option<String>,
    pub registered_postal_code: Option<String>,
    pub registered_phone: Option<String>,
    pub registered_fax: Option<String>,
    pub is_head_office_same_with_registered_office: Option<String>,
    pub is_limited_liability_company: Option<String>,
    pub template_code: Option<String>,
    pub global_template_code: Option<String>,
    #[serde(rename = "IsSPAC")]
    pub is_spac: Option<bool>,
    #[serde(rename = "IsMLP")]
    pub is_mlp: Option<bool>,
    #[serde(rename = "IsBDC")]
    pub is_bdc: Option<bool>,
}

pub async fn get_company_business_description(exchange_id: &str, symbol: &str) -> anyhow::Result<CompanyBusinessDescriptionResp> {
    let token = crate::mstar::auth::get_equity_token().await?;
    let url = format!("https://equityapi.morningstar.com/WebService/InvestorRelationsService.asmx/GetBusinessDescription?category=GetBusinessDescription&responseType=JSON&Token={}&identifierType=Symbol&identifier={}&exchangeId={}"
                      ,token, symbol, exchange_id);
    let data = http::get(&url, None).await?;
    let resp: CompanyBusinessDescriptionResp = data.json().await?;
    Ok(resp)
}

pub async fn get_company_general_info(exchange_id: &str, symbol: &str) -> anyhow::Result<CompanyGeneralInfoResp> {
    let token = crate::mstar::auth::get_equity_token().await?;
    let url = format!("https://equityapi.morningstar.com/WebService/InvestorRelationsService.asmx/GetCompanyGeneralInformation?category=GetCompanyGeneralInformation&responseType=JSON&Token={}&identifierType=Symbol&identifier={}&exchangeId={}"
                      ,token, symbol, exchange_id);
    let data = http::get(&url, None).await?;
    let resp: CompanyGeneralInfoResp = data.json().await?;
    Ok(resp)
}
