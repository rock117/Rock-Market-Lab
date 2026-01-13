//! A browser automation library for monitoring network responses and searching for specific text.
//!
//! # Examples
//! ```no_run
//! use browser_crawler::{BrowserCrawler, BrowserCrawlerPlaywright};
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let crawler = BrowserCrawler::new(true)?;
//!     crawler.navigate("https://example.com").await?;
//!     
//!     let matches = crawler.find_text_in_responses("target text", 10).await?;
//!     println!("Found matches in responses: {:?}", matches);
//!     
//!     let playwright_crawler = BrowserCrawlerPlaywright::new()
//!         .with_user_data_dir("./tmp/playwright-profile")
//!         .with_idle_wait(Duration::from_secs(5));
//!     let result = playwright_crawler.crawl_html("https://xueqiu.com/S/SZ300063").await?;
//!     println!("html content: {:?}", result.content);
//!     Ok(())
//! }
//! ```

mod playwright;

pub use playwright::{BrowserCrawlerPlaywright, CrawlHtmlResult};

use anyhow::{Result};
use headless_chrome::{Browser, Tab};
use headless_chrome::browser::tab::ResponseHandler;
use headless_chrome::protocol::network::events::ResponseReceivedEventParams;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

/// Main struct for browser automation and response monitoring
pub struct BrowserCrawler {
    browser: Browser,
    tab: Arc<Tab>,
}

impl BrowserCrawler {
    /// Creates a new BrowserCrawler instance
    pub fn new(headless: bool) -> Result<Self> {
        info!("Initializing browser crawler (headless: {})", headless);
        let _ = headless;
        let browser = Browser::default()
            .map_err(|e| anyhow::anyhow!("Failed to initialize browser: {e:?}"))?;
        let tab = browser
            .new_tab()
            .map_err(|e| anyhow::anyhow!("Failed to create new tab: {e:?}"))?;
        
        Ok(Self { browser, tab })
    }

    /// Navigates to the specified URL
    pub async fn navigate(&self, url: &str) -> Result<()> {
        info!("Navigating to: {}", url);
        self.tab
            .navigate_to(url)
            .map_err(|e| anyhow::anyhow!("Failed to navigate_to {}: {e:?}", url))?
            .wait_until_navigated()
            .map_err(|e| anyhow::anyhow!("Failed to wait_until_navigated {}: {e:?}", url))?;
        Ok(())
    }

    /// Searches for text in network responses
    pub async fn find_text_in_responses(&self, target_text: &str, timeout_secs: u64) -> Result<Vec<String>> {
        info!("Searching for text in responses: '{}'", target_text);
        let matching_urls: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let target_text = target_text.to_string();
        let matching_urls_cloned = Arc::clone(&matching_urls);

        let handler: ResponseHandler = Box::new(move |params: ResponseReceivedEventParams, fetch_body| {
            let url = params.response.url.clone();
            if let Ok(body) = fetch_body() {
                let body_text = format!("{body:?}");
                if body_text.contains(&target_text) {
                    info!("Found match in response from: {}", url);
                    if let Ok(mut locked) = matching_urls_cloned.lock() {
                        locked.push(url);
                    }
                }
            }
        });

        self.tab
            .enable_response_handling(handler)
            .map_err(|e| anyhow::anyhow!("Failed to enable response handling: {e:?}"))?;

        tokio::time::sleep(Duration::from_secs(timeout_secs)).await;

        let urls = matching_urls
            .lock()
            .map(|v| v.clone())
            .unwrap_or_default();

        if urls.is_empty() {
            warn!("No matches found in responses within {} seconds", timeout_secs);
        }

        Ok(urls)
    }
}
