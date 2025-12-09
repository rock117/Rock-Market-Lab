//! 大模型提供商实现

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use super::{
    ChatCompletionRequest, ChatCompletionResponse, ChatCompletionChunk,
    ModelListResponse, LlmResult, LlmConfig,
};

pub mod openai;
pub mod deepseek;
pub mod gemini;
pub mod claude;

/// 大模型提供商 trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// 获取提供商名称
    fn name(&self) -> &'static str;
    
    /// 聊天完成（非流式）
    async fn chat_completion(&self, request: ChatCompletionRequest) -> LlmResult<ChatCompletionResponse>;
    
    /// 聊天完成（流式）
    async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<ChatCompletionChunk>> + Send>>>;
    
    /// 获取可用模型列表
    async fn list_models(&self) -> LlmResult<ModelListResponse>;
    
    /// 验证 API 密钥
    async fn validate_api_key(&self) -> LlmResult<bool>;
    
    /// 获取配置
    fn config(&self) -> &LlmConfig;
    
    /// 更新配置
    fn update_config(&mut self, config: LlmConfig) -> LlmResult<()>;
}

/// 提供商工厂
pub struct ProviderFactory;

impl ProviderFactory {
    /// 创建提供商实例
    pub fn create_provider(config: LlmConfig) -> LlmResult<Box<dyn LlmProvider>> {
        match config.provider {
            super::LlmProvider::OpenAI => {
                Ok(Box::new(openai::OpenAiProvider::new(config)?))
            }
            super::LlmProvider::DeepSeek => {
                Ok(Box::new(deepseek::DeepSeekProvider::new(config)?))
            }
            super::LlmProvider::Gemini => {
                Ok(Box::new(gemini::GeminiProvider::new(config)?))
            }
            super::LlmProvider::Claude => {
                Ok(Box::new(claude::ClaudeProvider::new(config)?))
            }
            super::LlmProvider::Custom(_) => {
                // 对于自定义提供商，使用 OpenAI 兼容的实现
                Ok(Box::new(openai::OpenAiProvider::new(config)?))
            }
        }
    }
}
