use anyhow::{anyhow, Context};
use playwright::Playwright;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BrowserCrawler {
    headless: bool,
    idle_wait: Duration,
    user_data_dir: Option<PathBuf>,
}

impl BrowserCrawler {
    pub fn new() -> Self {
        Self {
            headless: true,
            idle_wait: Duration::from_millis(1500),
            user_data_dir: None,
        }
    }

    pub fn with_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    pub fn with_idle_wait(mut self, idle_wait: Duration) -> Self {
        self.idle_wait = idle_wait;
        self
    }

    /// Enable session persistence by using a fixed Chrome user data directory.
    ///
    /// After you complete an interactive login once (e.g. QR scan), cookies and other
    /// browser storage can be reused across runs as long as this directory remains.
    pub fn with_user_data_dir(mut self, user_data_dir: impl Into<PathBuf>) -> Self {
        self.user_data_dir = Some(user_data_dir.into());
        self
    }

    /// Open a visible browser window for manual login (e.g. QR code scan) and wait.
    ///
    /// Notes:
    /// - This requires `headless=false` internally so you can see the QR code.
    /// - To reuse login state later, you should also set `with_user_data_dir(...)`.
    /// - `wait` is just a simple time window for you to finish login.
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

impl Default for BrowserCrawler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlHtmlResult {
    pub final_url: Option<String>,
    pub content: String,
}


mod tests {

    use std::time::Duration;

    use crate::BrowserCrawler;
    

    #[tokio::test]
    async fn test() {
        let crawler = BrowserCrawler::new()
            .with_user_data_dir("./tmp/playwright-profile")
            .with_idle_wait(Duration::from_secs(5));

        // First time may require you to call open_for_login() manually in a separate run.
        let result = crawler.crawl_html("https://xueqiu.com/S/SZ300063").await;
     //   println!("html content: {:?}", result.unwrap().content);
        std::fs::write(r"C:/rock/coding/code/my/rust/Rock-Market-Lab/tmp/content.html", result.unwrap().content).unwrap();
        println!("html content saved to ./tmp/playwright-profile/content.html");
    }
}

