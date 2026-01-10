# browser-crawler

A small Rust crate that exposes a stable API for browser-based crawling using Chromium DevTools Protocol (CDP) via `headless_chrome`.

## Usage

```rust
use browser_crawler::BrowserCrawler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let crawler = BrowserCrawler::new();
    let resp = crawler.crawl_html("https://example.com").await?;
    println!("{}", resp.content.len());
    Ok(())
}
```
