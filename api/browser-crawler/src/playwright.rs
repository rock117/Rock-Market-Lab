use anyhow::{anyhow, Context};
use playwright::Playwright;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::OnceCell;

/// Main struct for browser automation using Playwright
pub struct BrowserCrawlerPlaywright {
    headless: bool,
    idle_wait: Duration,
    user_data_dir: Option<PathBuf>,
}

impl BrowserCrawlerPlaywright {
    /// Creates a new BrowserCrawlerPlaywright instance
    pub fn new() -> Self {
        Self {
            headless: true,
            idle_wait: Duration::from_millis(1500),
            user_data_dir: None,
        }
    }

    /// Sets the headless mode for the browser
    pub fn with_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    /// Sets the idle wait time for the browser
    pub fn with_idle_wait(mut self, idle_wait: Duration) -> Self {
        self.idle_wait = idle_wait;
        self
    }

    /// Enables session persistence by using a fixed Chrome user data directory
    pub fn with_user_data_dir(mut self, user_data_dir: impl Into<PathBuf>) -> Self {
        self.user_data_dir = Some(user_data_dir.into());
        self
    }

    /// Opens a visible browser window for manual login and waits
    pub async fn open_for_login(&self, url: &str, wait: Duration) -> anyhow::Result<()> {
        let user_data_dir = self
            .user_data_dir
            .clone()
            .ok_or_else(|| anyhow!("open_for_login requires with_user_data_dir(...)"))?;

        ensure_dir(&user_data_dir).await?;

        let url = url.to_string();

        let pw = playwright().await?;
        pw.prepare().map_err(|e| anyhow!("playwright.prepare failed: {e:?}"))?;

        let chromium = pw.chromium();

        let context = chromium
            .persistent_context_launcher(&user_data_dir)
            .headless(false)
            .launch()
            .await
            .map_err(|e| anyhow!("launch persistent context failed: {e:?}"))?;

        let page = context
            .new_page()
            .await
            .map_err(|e| anyhow!("new_page failed: {e:?}"))?;

        page.goto_builder(&url)
            .goto()
            .await
            .map_err(|e| anyhow!("goto {} failed: {e:?}", url))?;

        tokio::time::sleep(wait).await;

        context
            .close()
            .await
            .map_err(|e| anyhow!("context.close failed: {e:?}"))?;

        Ok(())
    }

    /// Crawls the HTML content of the specified URL
    pub async fn crawl_html(&self, url: &str) -> anyhow::Result<CrawlHtmlResult> {
        let user_data_dir = self
            .user_data_dir
            .clone()
            .ok_or_else(|| anyhow!("crawl_html requires with_user_data_dir(...) for stable behavior"))?;

        ensure_dir(&user_data_dir).await?;

        let pw = playwright().await?;
        pw.prepare().map_err(|e| anyhow!("playwright.prepare failed: {e:?}"))?;

        let chromium = pw.chromium();
        let context = chromium
            .persistent_context_launcher(&user_data_dir)
            .headless(self.headless)
            .launch()
            .await
            .map_err(|e| anyhow!("launch persistent context failed: {e:?}"))?;

        let page = context
            .new_page()
            .await
            .map_err(|e| anyhow!("new_page failed: {e:?}"))?;

        page.goto_builder(url)
            .goto()
            .await
            .map_err(|e| anyhow!("goto failed: {e:?}"))?;

        tokio::time::sleep(self.idle_wait).await;

        let final_url = page.url().ok();

        let content = page
            .content()
            .await
            .map_err(|e| anyhow!("page.content failed: {e:?}"))?;

        context
            .close()
            .await
            .map_err(|e| anyhow!("context.close failed: {e:?}"))?;

        Ok(CrawlHtmlResult {
            final_url,
            content,
        })
    }
}

static PLAYWRIGHT: OnceCell<Playwright> = OnceCell::const_new();

async fn playwright() -> anyhow::Result<&'static Playwright> {
    PLAYWRIGHT
        .get_or_try_init(|| async {
            Playwright::initialize()
                .await
                .map_err(|e| anyhow!("Playwright::initialize failed: {e:?}"))
        })
        .await
}

async fn ensure_dir(dir: &Path) -> anyhow::Result<()> {
    tokio::fs::create_dir_all(dir)
        .await
        .with_context(|| format!("create dir: {}", dir.display()))
}

impl Default for BrowserCrawlerPlaywright {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlHtmlResult {
    pub final_url: Option<String>,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_crawl_html() {
        let crawler = BrowserCrawlerPlaywright::new()
            .with_user_data_dir("./tmp/playwright-profile")
            .with_idle_wait(Duration::from_secs(5));

        let result = crawler.crawl_html("https://example.com").await;
        assert!(result.is_ok());
    }
}
