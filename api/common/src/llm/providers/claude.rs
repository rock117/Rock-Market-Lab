//! Anthropic Claude 提供商实现
//! Claude API 格式与 OpenAI 略有不同

use async_trait::async_trait;
use futures::Stream;
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
use tracing::{debug, info, warn};

use super::LlmProvider as LlmProviderTrait;
use crate::llm::{
    ChatCompletionRequest, ChatCompletionResponse, ChatCompletionChunk, ChatMessage,
    ModelListResponse, LlmResult, LlmError, LlmConfig, LlmProvider, MessageRole,
    Choice, Usage,
};

/// Claude API 请求格式
#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

/// Claude API 响应格式
#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    id: String,
    #[serde(rename = "type")]
    response_type: String,
    role: String,
    content: Vec<ClaudeContent>,
    model: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

/// Anthropic Claude 提供商
pub struct ClaudeProvider {
    config: LlmConfig,
    client: Client,
}

impl ClaudeProvider {
    /// 创建新的 Claude 提供商
    pub fn new(config: LlmConfig) -> LlmResult<Self> {
        config.validate()?;
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&config.api_key)
                .map_err(|e| LlmError::config_error(format!("Invalid API key format: {}", e)))?,
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
        );
        
        // 添加额外的头部
        if let Some(extra_headers) = &config.extra_headers {
            for (key, value) in extra_headers {
                let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                    .map_err(|e| LlmError::config_error(format!("Invalid header key '{}': {}", key, e)))?;
                let header_value = HeaderValue::from_str(value)
                    .map_err(|e| LlmError::config_error(format!("Invalid header value for '{}': {}", key, e)))?;
                headers.insert(header_name, header_value);
            }
        }
        
        let mut client_builder = Client::builder()
            .timeout(config.get_timeout())
            .default_headers(headers);
        
        // 设置代理（如果有）
        if let Some(proxy_url) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)
                .map_err(|e| LlmError::config_error(format!("Invalid proxy URL: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        let client = client_builder
            .build()
            .map_err(|e| LlmError::config_error(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self { config, client })
    }
    
    /// 处理 HTTP 响应错误
    fn handle_response_error(&self, status: reqwest::StatusCode, body: &str) -> LlmError {
        match status.as_u16() {
            401 => LlmError::auth_error("Invalid API key"),
            403 => LlmError::auth_error("API key does not have required permissions"),
            429 => LlmError::quota_exceeded("Rate limit exceeded or quota exhausted"),
            400 => LlmError::InvalidRequestError(format!("Bad request: {}", body)),
            404 => LlmError::ModelNotFoundError("Model not found".to_string()),
            500..=599 => LlmError::api_error(format!("Server error: {}", body), Some(status.as_u16())),
            _ => LlmError::api_error(format!("HTTP {}: {}", status, body), Some(status.as_u16())),
        }
    }
    
    /// 转换 OpenAI 格式的消息为 Claude 格式
    fn convert_messages(&self, messages: &[ChatMessage]) -> (Option<String>, Vec<ClaudeMessage>) {
        let mut system_message = None;
        let mut claude_messages = Vec::new();
        
        for message in messages {
            match message.role {
                MessageRole::System => {
                    // Claude 的 system 消息是单独的字段
                    system_message = Some(message.content.clone());
                }
                MessageRole::User => {
                    claude_messages.push(ClaudeMessage {
                        role: "user".to_string(),
                        content: message.content.clone(),
                    });
                }
                MessageRole::Assistant => {
                    claude_messages.push(ClaudeMessage {
                        role: "assistant".to_string(),
                        content: message.content.clone(),
                    });
                }
                MessageRole::Tool => {
                    // 跳过工具消息，Claude 的工具调用格式不同
                    continue;
                }
            }
        }
        
        (system_message, claude_messages)
    }
    
    /// 转换 Claude 响应为 OpenAI 格式
    fn convert_response(&self, claude_response: ClaudeResponse) -> ChatCompletionResponse {
        let content = claude_response.content
            .into_iter()
            .map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");
        
        let choice = Choice {
            index: 0,
            message: ChatMessage::assistant(content),
            finish_reason: claude_response.stop_reason,
            logprobs: None,
        };
        
        let usage = Usage {
            prompt_tokens: claude_response.usage.input_tokens,
            completion_tokens: claude_response.usage.output_tokens,
            total_tokens: claude_response.usage.input_tokens + claude_response.usage.output_tokens,
        };
        
        ChatCompletionResponse {
            id: claude_response.id,
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp() as u64,
            model: claude_response.model,
            choices: vec![choice],
            usage: Some(usage),
            system_fingerprint: None,
        }
    }
    
    /// 转换模型名称为 Claude 格式
    fn convert_model_name(&self, model: &str) -> String {
        match model {
            "gpt-3.5-turbo" | "claude-3-haiku-20240307" => "claude-3-haiku-20240307".to_string(),
            "gpt-4" | "claude-3-sonnet-20240229" => "claude-3-sonnet-20240229".to_string(),
            "claude-3-opus-20240229" => "claude-3-opus-20240229".to_string(),
            other => other.to_string(),
        }
    }
}

#[async_trait]
impl LlmProviderTrait for ClaudeProvider {
    fn name(&self) -> &'static str {
        "Claude"
    }
    
    async fn chat_completion(&self, mut request: ChatCompletionRequest) -> LlmResult<ChatCompletionResponse> {
        // 如果没有指定模型，使用默认模型
        if request.model.is_empty() {
            if let Some(default_model) = &self.config.default_model {
                request.model = default_model.clone();
            } else {
                request.model = "claude-3-sonnet-20240229".to_string();
            }
        }
        
        // 转换模型名称
        let model = self.convert_model_name(&request.model);
        
        // 转换消息格式
        let (system, messages) = self.convert_messages(&request.messages);
        
        // Claude 要求必须有 max_tokens
        let max_tokens = request.max_tokens.unwrap_or(4096);
        
        // 构建 Claude 请求
        let claude_request = ClaudeRequest {
            model,
            max_tokens,
            messages,
            system,
            temperature: request.temperature,
            top_p: request.top_p,
            stop_sequences: request.stop,
            stream: Some(false),
        };
        
        let url = format!("{}/messages", self.config.get_api_endpoint());
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Sending Claude chat completion request to: {}", url);
            debug!("Request: {:?}", claude_request);
        }
        
        let response = self.client
            .post(&url)
            .json(&claude_request)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(self.handle_response_error(status, &body));
        }
        
        let claude_response: ClaudeResponse = serde_json::from_str(&body)
            .map_err(|e| LlmError::JsonError(e))?;
        
        let completion = self.convert_response(claude_response);
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Received Claude response: {:?}", completion);
        }
        
        Ok(completion)
    }
    
    async fn chat_completion_stream(
        &self,
        _request: ChatCompletionRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<ChatCompletionChunk>> + Send>>> {
        // Claude 流式实现
        Err(LlmError::UnknownError("Claude streaming not implemented yet".to_string()))
    }
    
    async fn list_models(&self) -> LlmResult<ModelListResponse> {
        // Claude 没有公开的模型列表 API，返回已知的模型
        let models = ModelListResponse {
            object: "list".to_string(),
            data: vec![], // 简化版本，返回空列表
        };
        
        if self.config.enable_logging.unwrap_or(false) {
            info!("Claude models list (hardcoded): {:?}", models);
        }
        
        Ok(models)
    }
    
    async fn validate_api_key(&self) -> LlmResult<bool> {
        // 通过发送一个简单的请求来验证 API 密钥
        let test_request = ChatCompletionRequest::simple(
            "claude-3-haiku-20240307",
            vec![ChatMessage::user("Hello")],
        ).with_max_tokens(10);
        
        match self.chat_completion(test_request).await {
            Ok(_) => Ok(true),
            Err(e) if e.is_auth_error() => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    fn config(&self) -> &LlmConfig {
        &self.config
    }
    
    fn update_config(&mut self, config: LlmConfig) -> LlmResult<()> {
        config.validate()?;
        *self = Self::new(config)?;
        Ok(())
    }
}
