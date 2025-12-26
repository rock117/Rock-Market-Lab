use anyhow::{Context, Result};
use reqwest::header;
use scraper::{Html, Selector};

/// Simple HTTP-based implementation (works for static content)
pub async fn get() -> Result<String> {
    let url = "https://www.futunn.com/stock/IRDM-US/company";
    let css_selector = "#view-page  div.stock-page.router-page  section  div  section";
    // let css_selector = "#view-page > div.stock-page.router-page > section > div > section > div:nth-child(1) > div.company-info > div:nth-child(3) > span";
    let selector = Selector::parse(css_selector)
        .map_err(|e| anyhow::anyhow!("Invalid CSS selector: {}", e))?;
    
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT, 
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    );
    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
        
    let html = client.get(url).send().await?.text().await?;
    println!("html: {}", html);
    let document = Html::parse_document(&html);
    
    match document.select(&selector).next() {
        Some(element) => Ok(element.text().collect::<String>().trim().to_string()),
        None => Err(anyhow::anyhow!("Element not found with selector: {}", css_selector))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        match get().await {
            Ok(content) => {
                println!("Extracted content: {}", content);
                assert!(!content.is_empty(), "Content should not be empty");
            }
            Err(e) => {
                println!("Test error: {}", e);
            }
        }
    }
}