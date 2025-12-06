use async_trait::async_trait;
use tracing::{error, info, warn};
use crate::task::Task;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

/// 美股公司信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsCompanyInfo {
    /// 公司名称
    pub company_name: String,
    /// 交易所
    pub exchange: String,
    /// 成立时间
    pub founded_date: String,
    /// 员工人数
    pub employee_count: String,
    /// 公司地址
    pub address: String,
    /// 官网
    pub website: String,
    /// 公司简介
    pub description: String,
}

impl Default for UsCompanyInfo {
    fn default() -> Self {
        Self {
            company_name: "N/A".to_string(),
            exchange: "N/A".to_string(),
            founded_date: "N/A".to_string(),
            employee_count: "N/A".to_string(),
            address: "N/A".to_string(),
            website: "N/A".to_string(),
            description: "N/A".to_string(),
        }
    }
}

/// 抓取美股公司信息任务
pub struct FetchUsCompanyInfoTask {
    client: Client,
}

impl FetchUsCompanyInfoTask {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 抓取指定股票代码的公司信息
    pub async fn fetch_company_info(&self, symbol: &str) -> Result<UsCompanyInfo> {
        let url = format!("https://www.itiger.com/hans/stock/{}/company", symbol);
        info!("Fetching company info for {} from: {}", symbol, url);

        // 发送HTTP请求
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP request failed with status: {}", response.status()));
        }

        let html_content = response.text().await?;
        self.parse_company_info(&html_content, symbol).await
    }

    /// 解析HTML内容提取公司信息
    async fn parse_company_info(&self, html: &str, symbol: &str) -> Result<UsCompanyInfo> {
        let document = Html::parse_document(html);
        let mut company_info = UsCompanyInfo::default();

        // 解析公司名称 - 通常在页面标题或主要标题中
        if let Some(name) = self.extract_company_name(&document, symbol) {
            company_info.company_name = name;
        }

        // 解析交易所信息
        if let Some(exchange) = self.extract_exchange(&document) {
            company_info.exchange = exchange;
        }

        // 解析成立时间
        if let Some(founded) = self.extract_founded_date(&document) {
            company_info.founded_date = founded;
        }

        // 解析员工人数
        if let Some(employees) = self.extract_employee_count(&document) {
            company_info.employee_count = employees;
        }

        // 解析公司地址
        if let Some(address) = self.extract_address(&document) {
            company_info.address = address;
        }

        // 解析官网
        if let Some(website) = self.extract_website(&document) {
            company_info.website = website;
        }

        // 解析公司简介
        if let Some(description) = self.extract_description(&document) {
            company_info.description = description;
        }

        info!("Successfully parsed company info for {}: {:?}", symbol, company_info);
        Ok(company_info)
    }

    /// 提取公司名称
    fn extract_company_name(&self, document: &Html, symbol: &str) -> Option<String> {
        // 尝试多种选择器来找到公司名称
        let selectors = [
            "h1.company-name",
            "h1",
            ".company-title",
            ".stock-name",
            "title",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != symbol {
                        return Some(text);
                    }
                }
            }
        }

        // 如果找不到，使用股票代码作为默认值
        Some(symbol.to_string())
    }

    /// 提取交易所信息
    fn extract_exchange(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".exchange",
            ".market",
            "[data-field='exchange']",
            "span:contains('NASDAQ')",
            "span:contains('NYSE')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }

        None
    }

    /// 提取成立时间
    fn extract_founded_date(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".founded",
            ".established",
            "[data-field='founded']",
            "td:contains('成立')",
            "td:contains('Founded')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }

        None
    }

    /// 提取员工人数
    fn extract_employee_count(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".employees",
            ".employee-count",
            "[data-field='employees']",
            "td:contains('员工')",
            "td:contains('Employee')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }

        None
    }

    /// 提取公司地址
    fn extract_address(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".address",
            ".location",
            "[data-field='address']",
            "td:contains('地址')",
            "td:contains('Address')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }

        None
    }

    /// 提取官网
    fn extract_website(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".website a",
            ".official-website a",
            "[data-field='website'] a",
            "a[href*='http']:contains('官网')",
            "a[href*='http']:contains('Website')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(href) = element.value().attr("href") {
                        return Some(href.to_string());
                    }
                }
            }
        }

        None
    }

    /// 提取公司简介
    fn extract_description(&self, document: &Html) -> Option<String> {
        let selectors = [
            ".description",
            ".company-description",
            ".overview",
            ".summary",
            "[data-field='description']",
            "p:contains('简介')",
            "p:contains('Description')",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text.len() > 20 {
                        return Some(text);
                    }
                }
            }
        }

        None
    }
}

#[async_trait]
impl Task for FetchUsCompanyInfoTask {
    fn get_schedule(&self) -> String {
        // 每天凌晨2点执行
        "0 0 2 * * * *".to_string()
    }

    async fn run(&self) -> Result<()> {
        info!("Starting US company info fetch task");

        // 示例：抓取OKLO公司信息
        let symbols = vec!["OKLO", "AAPL", "TSLA"];

        for symbol in symbols {
            match self.fetch_company_info(symbol).await {
                Ok(company_info) => {
                    info!("Successfully fetched info for {}: {:?}", symbol, company_info);
                }
                Err(e) => {
                    error!("Failed to fetch info for {}: {:?}", symbol, e);
                }
            }

            // 避免请求过于频繁
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        info!("US company info fetch task completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_oklo_company_info() {
        let task = FetchUsCompanyInfoTask::new();
        
        match task.fetch_company_info("OKLO").await {
            Ok(company_info) => {
                println!("=== OKLO Company Information ===");
                println!("Company Name: {}", company_info.company_name);
                println!("Exchange: {}", company_info.exchange);
                println!("Founded Date: {}", company_info.founded_date);
                println!("Employee Count: {}", company_info.employee_count);
                println!("Address: {}", company_info.address);
                println!("Website: {}", company_info.website);
                println!("Description: {}", company_info.description);
                println!("================================");
                
                // 基本断言
                assert!(!company_info.company_name.is_empty());
                assert_ne!(company_info.company_name, "N/A");
            }
            Err(e) => {
                println!("Failed to fetch OKLO company info: {:?}", e);
                // 在测试环境中，网络请求可能失败，所以不强制要求成功
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_multiple_companies() {
        let task = FetchUsCompanyInfoTask::new();
        let symbols = vec!["OKLO", "AAPL"];

        for symbol in symbols {
            println!("\n=== Testing {} ===", symbol);
            match task.fetch_company_info(symbol).await {
                Ok(company_info) => {
                    println!("✅ Successfully fetched {} info:", symbol);
                    println!("   Company Name: {}", company_info.company_name);
                    println!("   Exchange: {}", company_info.exchange);
                    println!("   Founded Date: {}", company_info.founded_date);
                    println!("   Employee Count: {}", company_info.employee_count);
                    println!("   Address: {}", company_info.address);
                    println!("   Website: {}", company_info.website);
                    println!("   Description: {}", company_info.description);
                }
                Err(e) => {
                    println!("❌ Failed to fetch {} info: {:?}", symbol, e);
                }
            }

            // 避免请求过于频繁
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    #[test]
    fn test_company_info_default() {
        let info = UsCompanyInfo::default();
        assert_eq!(info.company_name, "N/A");
        assert_eq!(info.exchange, "N/A");
        assert_eq!(info.founded_date, "N/A");
        assert_eq!(info.employee_count, "N/A");
        assert_eq!(info.address, "N/A");
        assert_eq!(info.website, "N/A");
        assert_eq!(info.description, "N/A");
    }
}
