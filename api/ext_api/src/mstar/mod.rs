pub mod auth;
mod equity;

use serde::{Serialize, Deserialize};

use common::http;
pub async fn get_stock_list() -> anyhow::Result<()> {
    // let url = "https://equityapi.morningstar.com/DataCatalogOutput.aspx?category=GetExchangeList&identifier=TWN&identifierType=CountryId&responseType=JSON";
    let exchanges = &[
      "ARCX",
      "ASE",
      "BATS",
      "GREY",
      "NAS",
      "NYS",
      "OTC",
      "PINX"
    ];
    let exchanges = http::get("", None).await?;
    // for exchange in exchanges {
    //
    // }
    todo!()
}

pub async fn get_company_infos(exchange: &str) -> anyhow::Result<()> {
    todo!()
}

pub async  fn get_company_description(token: &str, exchangeId: &str, symbol: &str) -> anyhow::Result<CompanyBusinessDescriptionResp> {
    // let url = format!("https://equityapi.morningstar.com/WebService/InvestorRelationsService.asmx/GetCompanyGeneralInformation?exchangeId={}&identifier={}&identifierType=Symbolcategory=GetBusinessDescription&responseType=JSON&Token={}", exchangeId, symbol, token);
    // let resp = http::get(&url, None).await?;
    // let entity = serde_json::from_str::<CompanyBusinessDescriptionResp>(resp.as_str())?;
    // Ok(entity)
    todo!()
}

pub async  fn get_company_info(token: &str, exchangeId: &str, symbol: &str) -> anyhow::Result<CompanyInfoResp> {
    // let url = format!("https://equityapi.morningstar.com/WebService/InvestorRelationsService.asmx/GetBusinessDescription?exchangeId={}&identifier={}&identifierType=Symbolcategory=GetBusinessDescription&responseType=JSON&Token={}", exchangeId, symbol, token);
    // let resp = http::get(&url, None).await?;
    // let entity = serde_json::from_str::<CompanyInfoResp>(resp.as_str())?;
    // Ok(entity)
    todo!()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CompanyInfoResp {
    message_info: MessageInfo,
    company_info_entity: CompanyInfoEntity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MessageInfo {
    message_code: u32,
    message_detail: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CompanyInfoEntity {
    local_name: String,
    local_name_language_code: String,
    short_name: String,
    business_country: String,
    domicile_country: String,
    web_address: String,
    country: String,
    total_employee: u32,
    industry_name: String,
    industry_group_name: String,
    sector_name: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CompanyBusinessDescriptionResp  {
    message_info: MessageInfo,
    business_description_entity: BusinessDescriptionEntity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BusinessDescriptionEntity {
    long_description: String,
}



struct CompanyInfoEntity2 {
    symbol: String,
    exchange: String,
    name: String,
    description: String,
    business_long_desc: String,
    eng_long_desc: String,
    chi_long_desc: String,
    local_name: String,
    industryName: String,
    industryGroupName: String,
    sectorName: String,
    industry: String,
    web_address: String,
    country: String,
}