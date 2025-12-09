//! Google Gemini 提供商实现
//! 注意：Gemini API 格式与 OpenAI 不同，需要特殊处理

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

/// Gemini API 请求格式
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
}

/// Gemini API 响应格式
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
    index: u32,
}

#[derive(Debug, Deserialize)]
struct GeminiUsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: u32,
    #[serde(rename = "totalTokenCount")]
    total_token_count: u32,
}

/// Google Gemini 提供商
pub struct GeminiProvider {
    config: LlmConfig,
    client: Client,
}


impl GeminiProvider {
    /// 创建新的 Gemini 提供商
    pub fn new(config: LlmConfig) -> LlmResult<Self> {
        config.validate()?;
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
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
    
    /// 转换 OpenAI 格式的消息为 Gemini 格式
    fn convert_messages(&self, messages: &[ChatMessage]) -> Vec<GeminiContent> {
        let mut gemini_contents = Vec::new();
        
        for message in messages {
            // Gemini 不支持 system 角色，将其转换为 user 消息
            let role = match message.role {
                MessageRole::System | MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "model".to_string(),
                MessageRole::Tool => continue, // 跳过工具消息
            };
            
            gemini_contents.push(GeminiContent {
                role,
                parts: vec![GeminiPart {
                    text: message.content.clone(),
                }],
            });
        }
        
        gemini_contents
    }
    
    /// 转换 Gemini 响应为 OpenAI 格式
    fn convert_response(&self, gemini_response: GeminiResponse, model: &str) -> ChatCompletionResponse {
        let choices = gemini_response.candidates
            .into_iter()
            .enumerate()
            .map(|(index, candidate)| {
                let content = candidate.content.parts
                    .into_iter()
                    .map(|part| part.text)
                    .collect::<Vec<_>>()
                    .join("");
                
                Choice {
                    index: index as u32,
                    message: ChatMessage::assistant(content),
                    finish_reason: candidate.finish_reason,
                    logprobs: None,
                }
            })
            .collect();
        
        let usage = gemini_response.usage_metadata.map(|meta| Usage {
            prompt_tokens: meta.prompt_token_count,
            completion_tokens: meta.candidates_token_count,
            total_tokens: meta.total_token_count,
        });
        
        ChatCompletionResponse {
            id: format!("gemini-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp() as u64,
            model: model.to_string(),
            choices,
            usage,
            system_fingerprint: None,
        }
    }
    
    /// 转换模型名称为 Gemini 格式
    fn convert_model_name(&self, model: &str) -> String {
        match model {
            "gpt-3.5-turbo" | "gpt-4" | "gemini-pro" => "gemini-pro".to_string(),
            "gemini-pro-vision" => "gemini-pro-vision".to_string(),
            "gemini-1.5-pro" => "gemini-1.5-pro".to_string(),
            other => other.to_string(),
        }
    }
}

#[async_trait]
impl LlmProviderTrait for GeminiProvider {
    fn name(&self) -> &'static str {
        "Gemini"
    }
    
    async fn chat_completion(&self, mut request: ChatCompletionRequest) -> LlmResult<ChatCompletionResponse> {
        // 如果没有指定模型，使用默认模型
        if request.model.is_empty() {
            if let Some(default_model) = &self.config.default_model {
                request.model = default_model.clone();
            } else {
                request.model = "gemini-pro".to_string();
            }
        }
        
        // 转换模型名称
        let model = self.convert_model_name(&request.model);
        
        // 转换消息格式
        let contents = self.convert_messages(&request.messages);
        
        // 构建 Gemini 请求
        let gemini_request = GeminiRequest {
            contents,
            generation_config: Some(GeminiGenerationConfig {
                temperature: request.temperature,
                top_p: request.top_p,
                max_output_tokens: request.max_tokens,
                stop_sequences: request.stop,
            }),
        };
        
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.config.get_api_endpoint(),
            model,
            self.config.api_key
        );
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Sending Gemini chat completion request to: {}", url);
            debug!("Request: {:?}", gemini_request);
        }
        
        let response = self.client
            .post(&url)
            .json(&gemini_request)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(self.handle_response_error(status, &body));
        }
        
        let gemini_response: GeminiResponse = serde_json::from_str(&body)
            .map_err(|e| LlmError::JsonError(e))?;
        
        let completion = self.convert_response(gemini_response, &model);
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Received Gemini response: {:?}", completion);
        }
        
        Ok(completion)
    }
    
    async fn chat_completion_stream(
        &self,
        _request: ChatCompletionRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<ChatCompletionChunk>> + Send>>> {
        // Gemini 流式实现
        Err(LlmError::UnknownError("Gemini streaming not implemented yet".to_string()))
    }
    
    async fn list_models(&self) -> LlmResult<ModelListResponse> {
        let url = format!(
            "{}/models?key={}",
            self.config.get_api_endpoint(),
            self.config.api_key
        );
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Fetching Gemini models from: {}", url);
        }
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(self.handle_response_error(status, &body));
        }
        
        // Gemini 的模型列表格式与 OpenAI 不同，这里简化处理
        // 实际实现需要解析 Gemini 的模型列表格式
        let models = ModelListResponse {
            object: "list".to_string(),
            data: vec![], // 简化版本，返回空列表
        };
        
        if self.config.enable_logging.unwrap_or(false) {
            info!("Found {} Gemini models", models.data.len());
        }
        
        Ok(models)
    }
    
    async fn validate_api_key(&self) -> LlmResult<bool> {
        // 通过尝试获取模型列表来验证 API 密钥
        match self.list_models().await {
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
