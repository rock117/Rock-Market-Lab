//! 大模型调用模块
//! 
//! 提供统一的大模型调用接口，支持多种大模型提供商：
//! - OpenAI (ChatGPT)
//! - DeepSeek
//! - Google Gemini
//! - Anthropic Claude
//! - 其他兼容 OpenAI API 的模型

pub mod types;
pub mod providers;
pub mod client;
pub mod config;
pub mod error;

pub use types::*;
pub use client::{LlmClient, LlmClientBuilder};
pub use config::{LlmConfig, LlmConfigManager};
pub use error::{LlmError, LlmResult};

// 重新导出主要的提供商
pub use providers::openai::OpenAiProvider;
pub use providers::deepseek::DeepSeekProvider;
pub use providers::gemini::GeminiProvider;
pub use providers::claude::ClaudeProvider;


mod tests {
    use crate::http;

    #[tokio::test]
    async fn get_page() {
        let res = http::get("https://emweb.securities.eastmoney.com/pc_hsf10/pages/index.html?type=web&code=SZ300620&color=b#/gsgk", None).await.unwrap();
        let s = res.text().await.unwrap();
        println!("{}", s);
    }
}