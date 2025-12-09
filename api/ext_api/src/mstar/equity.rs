use common::http;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StockListResp {
    pub message_info: MessageInfo,
    pub full_stock_exchange_security_entity_list: Vec<StockEntity>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageInfo {
    pub message_code: u32,
    pub message_detail: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StockEntity {
    pub share_class_id: Option<String>,
    pub company_id: Option<String>,
    pub investment_id: Option<String>,
    pub country_id: Option<String>,
    pub company_name: Option<String>,
    pub exchange_id: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "CUSIP")]
    pub cusip: Option<String>,
    #[serde(rename = "CIK")]
    pub cik: Option<String>,
    #[serde(rename = "ISIN")]
    pub isin: Option<String>,
    #[serde(rename = "SEDOL")]
    pub sedol: Option<String>,
    #[serde(rename = "CompanyLEI")]
    pub company_lei: Option<String>,
    pub investment_type_id: Option<String>,
    pub stock_status: Option<String>,
    pub suspended_flag: Option<String>,
    pub market_data_id: Option<String>,
}

pub async fn get_stock_list(exchange_id: &str) -> anyhow::Result<StockListResp> {
    let token = crate::mstar::auth::get_equity_token().await?;
    let url = format!("https://equityapi.morningstar.com/WebService/GlobalMasterListsService.asmx/GetFullStockExchangeSecurityList?category=GetFullStockExchangeSecurityList&responseType=JSON&Token={}&exchangeId={}&identifier={}&identifierType=ExchangeId&stockStatus=Active"
    ,token, exchange_id, exchange_id);
    println!("url: {}", url);
    let data = http::get(&url, None).await?;
    let resp: StockListResp = data.json().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stock_list() {
        unsafe { std::env::set_var("PROJECT_DIR", "C:/rock/coding/code/my/rust/Rock-Market-Lab/api"); }
        
        let result = get_stock_list("NAS").await;
        match result {
            Ok(resp) => {
                println!("MessageCode: {}", resp.message_info.message_code);
                println!("Stock count: {}", resp.full_stock_exchange_security_entity_list.len());
                if let Some(first) = resp.full_stock_exchange_security_entity_list.first() {
                    println!("First stock: {:?} - {:?}", first.symbol, first.company_name);
                }
                assert!(!resp.full_stock_exchange_security_entity_list.is_empty());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
