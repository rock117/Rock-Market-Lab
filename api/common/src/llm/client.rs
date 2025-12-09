//! 大模型客户端
//! 提供统一的大模型调用接口

use futures::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::providers::{LlmProvider as LlmProviderTrait, ProviderFactory};
use super::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, ChatMessage, ChatSession,
    LlmConfig, LlmConfigManager, LlmError, LlmProvider, LlmResult, ModelListResponse,
};

/// 大模型客户端
pub struct LlmClient {
    /// 配置管理器
    config_manager: Arc<RwLock<LlmConfigManager>>,
    /// 提供商实例缓存
    providers: Arc<RwLock<HashMap<String, Box<dyn LlmProviderTrait>>>>,
    /// 会话管理
    sessions: Arc<RwLock<HashMap<uuid::Uuid, ChatSession>>>,
}

impl LlmClient {
    /// 创建新的客户端
    pub fn new() -> Self {
        Self {
            config_manager: Arc::new(RwLock::new(LlmConfigManager::new())),
            providers: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加配置
    pub async fn add_config(&self, name: impl Into<String>, config: LlmConfig) -> LlmResult<()> {
        let name = name.into();

        // 验证配置
        config.validate()?;

        // 创建提供商实例
        let provider = ProviderFactory::create_provider(config.clone())?;

        // 验证 API 密钥
        if !provider.validate_api_key().await? {
            return Err(LlmError::auth_error("Invalid API key"));
        }

        // 添加到配置管理器
        {
            let mut manager = self.config_manager.write().await;
            manager.add_config(&name, config)?;
        }

        // 缓存提供商实例
        {
            let mut providers = self.providers.write().await;
            providers.insert(name.clone(), provider);
        }

        info!("Added LLM config: {}", name);
        Ok(())
    }

    /// 获取配置
    pub async fn get_config(&self, name: &str) -> Option<LlmConfig> {
        let manager = self.config_manager.read().await;
        manager.get_config(name).cloned()
    }

    /// 列出所有配置
    pub async fn list_configs(&self) -> Vec<String> {
        let manager = self.config_manager.read().await;
        manager.list_configs().into_iter().cloned().collect()
    }

    /// 设置默认配置
    pub async fn set_default_config(&self, name: impl Into<String>) -> LlmResult<()> {
        let mut manager = self.config_manager.write().await;
        manager.set_default(name)
    }

    /// 移除配置
    pub async fn remove_config(&self, name: &str) -> Option<LlmConfig> {
        // 从配置管理器中移除
        let config = {
            let mut manager = self.config_manager.write().await;
            manager.remove_config(name)
        };

        // 从提供商缓存中移除
        {
            let mut providers = self.providers.write().await;
            providers.remove(name);
        }

        if config.is_some() {
            info!("Removed LLM config: {}", name);
        }

        config
    }

    /// 获取提供商实例
    async fn get_provider(
        &self,
        config_name: Option<&str>,
    ) -> LlmResult<Box<dyn LlmProviderTrait>> {
        let providers = self.providers.read().await;

        if let Some(name) = config_name {
            // 使用指定的配置
            if let Some(provider) = providers.get(name) {
                // 这里需要克隆提供商，但 trait object 不能直接克隆
                // 实际实现中可能需要使用 Arc<dyn LlmProviderTrait> 或其他方式
                return Err(LlmError::config_error("Provider cloning not implemented"));
            } else {
                return Err(LlmError::config_error(format!(
                    "Config '{}' not found",
                    name
                )));
            }
        } else {
            // 使用默认配置
            let manager = self.config_manager.read().await;
            if let Some(default_config) = manager.get_default_config() {
                let provider = ProviderFactory::create_provider(default_config.clone())?;
                return Ok(provider);
            } else {
                return Err(LlmError::config_error("No default config set"));
            }
        }
    }

    /// 聊天完成（非流式）
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
        config_name: Option<&str>,
    ) -> LlmResult<ChatCompletionResponse> {
        let provider = self.get_provider(config_name).await?;

        debug!(
            "Sending chat completion request using provider: {}",
            provider.name()
        );

        let response = provider.chat_completion(request).await?;

        debug!(
            "Received chat completion response with {} choices",
            response.choices.len()
        );

        Ok(response)
    }

    /// 聊天完成（流式）
    pub async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
        config_name: Option<&str>,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<ChatCompletionChunk>> + Send>>> {
        let provider = self.get_provider(config_name).await?;

        debug!(
            "Sending streaming chat completion request using provider: {}",
            provider.name()
        );

        provider.chat_completion_stream(request).await
    }

    /// 简单的聊天接口
    pub async fn chat(
        &self,
        message: impl Into<String>,
        config_name: Option<&str>,
    ) -> LlmResult<String> {
        let request = ChatCompletionRequest::simple(
            "".to_string(), // 模型名称由提供商的默认配置决定
            vec![ChatMessage::user(message.into())],
        );

        let response = self.chat_completion(request, config_name).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(LlmError::UnknownError("No response choices".to_string()))
        }
    }

    /// 获取可用模型列表
    pub async fn list_models(&self, config_name: Option<&str>) -> LlmResult<ModelListResponse> {
        let provider = self.get_provider(config_name).await?;
        provider.list_models().await
    }

    /// 验证 API 密钥
    pub async fn validate_api_key(&self, config_name: Option<&str>) -> LlmResult<bool> {
        let provider = self.get_provider(config_name).await?;
        provider.validate_api_key().await
    }

    /// 创建新的聊天会话
    pub async fn create_session(&self) -> ChatSession {
        let session = ChatSession::new();
        let session_id = session.id;

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id, session.clone());
        }

        debug!("Created new chat session: {}", session_id);
        session
    }

    /// 获取聊天会话
    pub async fn get_session(&self, session_id: uuid::Uuid) -> Option<ChatSession> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    /// 更新聊天会话
    pub async fn update_session(&self, session: ChatSession) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
    }

    /// 删除聊天会话
    pub async fn delete_session(&self, session_id: uuid::Uuid) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id).is_some()
    }

    /// 在会话中发送消息
    pub async fn chat_in_session(
        &self,
        session_id: uuid::Uuid,
        message: impl Into<String>,
        config_name: Option<&str>,
    ) -> LlmResult<String> {
        // 获取会话
        let mut session = self
            .get_session(session_id)
            .await
            .ok_or_else(|| LlmError::config_error("Session not found"))?;

        // 添加用户消息
        let user_message = ChatMessage::user(message.into());
        session.add_message(user_message.clone());

        // 构建请求
        let request = ChatCompletionRequest::simple("".to_string(), session.messages.clone());

        // 发送请求
        let response = self.chat_completion(request, config_name).await?;

        if let Some(choice) = response.choices.first() {
            // 添加助手消息到会话
            session.add_message(choice.message.clone());

            // 更新会话
            self.update_session(session).await;

            Ok(choice.message.content.clone())
        } else {
            Err(LlmError::UnknownError("No response choices".to_string()))
        }
    }

    /// 清空会话历史
    pub async fn clear_session(&self, session_id: uuid::Uuid) -> LlmResult<()> {
        let mut session = self
            .get_session(session_id)
            .await
            .ok_or_else(|| LlmError::config_error("Session not found"))?;

        session.clear_messages();
        self.update_session(session).await;

        Ok(())
    }

    /// 获取会话统计信息
    pub async fn get_session_stats(&self) -> HashMap<String, usize> {
        let sessions = self.sessions.read().await;
        let mut stats = HashMap::new();

        stats.insert("total_sessions".to_string(), sessions.len());

        let total_messages: usize = sessions
            .values()
            .map(|session| session.messages.len())
            .sum();
        stats.insert("total_messages".to_string(), total_messages);

        stats
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷的构建器
pub struct LlmClientBuilder {
    configs: Vec<(String, LlmConfig)>,
    default_config: Option<String>,
}

impl LlmClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            default_config: None,
        }
    }

    /// 添加 OpenAI 配置
    pub fn with_openai(mut self, name: impl Into<String>, api_key: impl Into<String>) -> Self {
        let config = LlmConfig::openai(api_key);
        self.configs.push((name.into(), config));
        self
    }

    /// 添加 DeepSeek 配置
    pub fn with_deepseek(mut self, name: impl Into<String>, api_key: impl Into<String>) -> Self {
        let config = LlmConfig::deepseek(api_key);
        self.configs.push((name.into(), config));
        self
    }

    /// 添加 Gemini 配置
    pub fn with_gemini(mut self, name: impl Into<String>, api_key: impl Into<String>) -> Self {
        let config = LlmConfig::gemini(api_key);
        self.configs.push((name.into(), config));
        self
    }

    /// 添加 Claude 配置
    pub fn with_claude(mut self, name: impl Into<String>, api_key: impl Into<String>) -> Self {
        let config = LlmConfig::claude(api_key);
        self.configs.push((name.into(), config));
        self
    }

    /// 添加自定义配置
    pub fn with_config(mut self, name: impl Into<String>, config: LlmConfig) -> Self {
        self.configs.push((name.into(), config));
        self
    }

    /// 设置默认配置
    pub fn with_default(mut self, name: impl Into<String>) -> Self {
        self.default_config = Some(name.into());
        self
    }

    /// 构建客户端
    pub async fn build(self) -> LlmResult<LlmClient> {
        let client = LlmClient::new();

        // 添加所有配置
        for (name, config) in self.configs {
            client.add_config(&name, config).await?;
        }

        // 设置默认配置
        if let Some(default) = self.default_config {
            client.set_default_config(default).await?;
        }

        Ok(client)
    }
}

impl Default for LlmClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

mod tests {
    use crate::llm::{LlmClient, LlmClientBuilder, LlmConfig};

    #[tokio::test]
    async fn test() {
        let mut client_builder = LlmClientBuilder::new();
        let client = client_builder.with_deepseek("deepseek", "sk-47b29c3eac324b2a8a137b4a7838a93b").build().await.unwrap();
        let res = client.chat("1+1等于几", Some("deepseek")).await;
        dbg!(res);
    }
}
