//! OpenAI 提供商实现

use async_openai::{
    Client as OpenAIClient,
    config::OpenAIConfig,
    types::{
        CreateChatCompletionRequest,
        ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage,
        ChatCompletionRequestAssistantMessage,
        Role,
    },
};
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use tracing::{debug, info};

use super::LlmProvider as LlmProviderTrait;
use crate::llm::{
    ChatCompletionRequest, ChatCompletionResponse, ChatCompletionChunk, ChatMessage,
    ModelListResponse, LlmResult, LlmError, LlmConfig, MessageRole, Choice, Usage,
};

/// OpenAI 提供商
pub struct OpenAiProvider {
    config: LlmConfig,
    client: OpenAIClient<OpenAIConfig>,
}

impl OpenAiProvider {
    /// 创建新的 OpenAI 提供商
    pub fn new(config: LlmConfig) -> LlmResult<Self> {
        config.validate()?;
        
        // 创建 OpenAI 配置
        let mut openai_config = OpenAIConfig::new()
            .with_api_key(&config.api_key);
        
        // 设置 API 端点
        if let Some(endpoint) = &config.api_endpoint {
            openai_config = openai_config.with_api_base(endpoint);
        }
        
        // 设置组织 ID
        if let Some(org_id) = &config.organization_id {
            openai_config = openai_config.with_org_id(org_id);
        }
        
        // 创建客户端
        let client = OpenAIClient::with_config(openai_config);
        
        Ok(Self { config, client })
    }
    
    /// 转换我们的消息格式为 OpenAI 格式
    fn convert_messages(&self, messages: &[ChatMessage]) -> Vec<ChatCompletionRequestMessage> {
        messages.iter().map(|msg| {
            match msg.role {
                MessageRole::System => ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        content: msg.content.clone(),
                        name: msg.name.clone(),
                    }
                ),
                MessageRole::User => ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage {
                        content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                            msg.content.clone()
                        ),
                        name: msg.name.clone(),
                    }
                ),
                MessageRole::Assistant => ChatCompletionRequestMessage::Assistant(
                    ChatCompletionRequestAssistantMessage {
                        content: Some(msg.content.clone()),
                        name: msg.name.clone(),
                        tool_calls: None,
                        function_call: None,
                    }
                ),
                MessageRole::Tool => {
                    // 简化处理，转换为用户消息
                    ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessage {
                            content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                                msg.content.clone()
                            ),
                            name: msg.name.clone(),
                        }
                    )
                }
            }
        }).collect()
    }
    
    /// 转换 OpenAI 响应为我们的格式
    fn convert_response(&self, response: async_openai::types::CreateChatCompletionResponse) -> ChatCompletionResponse {
        let choices = response.choices.into_iter().map(|choice| {
            Choice {
                index: choice.index,
                message: ChatMessage {
                    role: match choice.message.role {
                        Role::System => MessageRole::System,
                        Role::User => MessageRole::User,
                        Role::Assistant => MessageRole::Assistant,
                        Role::Tool => MessageRole::Tool,
                        Role::Function => MessageRole::Assistant,
                    },
                    content: choice.message.content.unwrap_or_default(),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                finish_reason: choice.finish_reason.map(|r| format!("{:?}", r)),
                logprobs: None,
            }
        }).collect();
        
        let usage = response.usage.map(|u| Usage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });
        
        ChatCompletionResponse {
            id: response.id,
            object: response.object,
            created: response.created as u64,
            model: response.model,
            choices,
            usage,
            system_fingerprint: response.system_fingerprint,
        }
    }
}

#[async_trait]
impl LlmProviderTrait for OpenAiProvider {
    fn name(&self) -> &'static str {
        "OpenAI"
    }
    
    async fn chat_completion(&self, mut request: ChatCompletionRequest) -> LlmResult<ChatCompletionResponse> {
        // 如果没有指定模型，使用默认模型
        if request.model.is_empty() {
            if let Some(default_model) = &self.config.default_model {
                request.model = default_model.clone();
            } else {
                return Err(LlmError::InvalidRequestError("Model name is required".to_string()));
            }
        }
        
        // 转换消息格式
        let messages = self.convert_messages(&request.messages);
        
        // 构建 OpenAI 请求
        let openai_request = CreateChatCompletionRequest {
            model: request.model.clone(),
            messages,
            temperature: request.temperature,
            top_p: request.top_p,
            max_tokens: request.max_tokens,
            stop: request.stop.as_ref().map(|stops| {
                if stops.len() == 1 {
                    async_openai::types::Stop::String(stops[0].clone())
                } else {
                    async_openai::types::Stop::StringArray(stops.clone())
                }
            }),
            stream: Some(false),
            frequency_penalty: request.frequency_penalty,
            presence_penalty: request.presence_penalty,
            user: request.user.clone(),
            ..Default::default()
        };
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Sending OpenAI chat completion request");
        }
        
        // 调用 OpenAI API
        let response = self.client
            .chat()
            .create(openai_request)
            .await
            .map_err(|e| LlmError::api_error(e.to_string(), None))?;
        
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Received OpenAI response");
        }
        
        Ok(self.convert_response(response))
    }
    
    async fn chat_completion_stream(
        &self,
        mut request: ChatCompletionRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<ChatCompletionChunk>> + Send>>> {
        // 暂时不实现流式，使用 async-openai 的流式功能需要更复杂的处理
        Err(LlmError::UnknownError("Streaming not implemented yet".to_string()))
    }
    
    async fn list_models(&self) -> LlmResult<ModelListResponse> {
        if self.config.enable_logging.unwrap_or(false) {
            debug!("Fetching OpenAI models");
        }
        
        let response = self.client
            .models()
            .list()
            .await
            .map_err(|e| LlmError::api_error(e.to_string(), None))?;
        
        // 转换为我们的格式
        let models = ModelListResponse {
            object: response.object,
            data: response.data.into_iter().map(|model| crate::llm::ModelInfo {
                id: model.id,
                object: model.object,
                created: model.created as u64,
                owned_by: model.owned_by,
                permission: None,
                root: None,
                parent: None,
            }).collect(),
        };
        
        if self.config.enable_logging.unwrap_or(false) {
            info!("Found {} OpenAI models", models.data.len());
        }
        
        Ok(models)
    }
    
    async fn validate_api_key(&self) -> LlmResult<bool> {
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
