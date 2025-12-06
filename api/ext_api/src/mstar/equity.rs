use common::http;

pub async fn get_stock_list(exchange_id: &str) -> anyhow::Result<String> {
    let token = crate::mstar::auth::get_equity_token().await?;
    let url = format!("https://equityapi.morningstar.com/WebService/GlobalMasterListsService.asmx/GetFullStockExchangeSecurityList?category=GetFullStockExchangeSecurityList&responseType=JSON&Token={}&exchangeId={}&identifier={}&identifierType=ExchangeId&stockStatus=Active"
    ,token, exchange_id, exchange_id);
    println!("url: {}", url);
    let data = http::get(&url, None).await?;
    let data = data.text().await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stock_list() {
        unsafe { std::env::set_var("PROJECT_DIR", "C:/rock/coding/code/my/rust/Rock-Market-Lab/api"); }
        
        let result = get_stock_list("NAS").await;
        match result {
            Ok(data) => {
                println!("Response length: {}", data.len());
                println!("Response preview: {}", &data[..data.len().min(500)]);
                assert!(!data.is_empty());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
