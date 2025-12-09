//! 大模型配置管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{LlmProvider, LlmError, LlmResult};

/// 大模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// 提供商
    pub provider: LlmProvider,
    /// API 密钥
    pub api_key: String,
    /// API 端点
    pub api_endpoint: Option<String>,
    /// 组织 ID（OpenAI）
    pub organization_id: Option<String>,
    /// 项目 ID（OpenAI）
    pub project_id: Option<String>,
    /// 默认模型
    pub default_model: Option<String>,
    /// 请求超时时间（秒）
    pub timeout_seconds: Option<u64>,
    /// 最大重试次数
    pub max_retries: Option<u32>,
    /// 重试延迟（毫秒）
    pub retry_delay_ms: Option<u64>,
    /// 额外的 HTTP 头
    pub extra_headers: Option<HashMap<String, String>>,
    /// 代理设置
    pub proxy: Option<String>,
    /// 是否启用日志
    pub enable_logging: Option<bool>,
    /// 自定义参数
    pub custom_params: Option<HashMap<String, serde_json::Value>>,
}

impl LlmConfig {
    /// 创建 OpenAI 配置
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self {
            provider: LlmProvider::OpenAI,
            api_key: api_key.into(),
            api_endpoint: None,
            organization_id: None,
            project_id: None,
            default_model: Some("gpt-3.5-turbo".to_string()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            retry_delay_ms: Some(1000),
            extra_headers: None,
            proxy: None,
            enable_logging: Some(true),
            custom_params: None,
        }
    }
    
    /// 创建 DeepSeek 配置
    pub fn deepseek(api_key: impl Into<String>) -> Self {
        Self {
            provider: LlmProvider::DeepSeek,
            api_key: api_key.into(),
            api_endpoint: Some("https://api.deepseek.com/v1".to_string()),
            organization_id: None,
            project_id: None,
            default_model: Some("deepseek-chat".to_string()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            retry_delay_ms: Some(1000),
            extra_headers: None,
            proxy: None,
            enable_logging: Some(true),
            custom_params: None,
        }
    }
    
    /// 创建 Gemini 配置
    pub fn gemini(api_key: impl Into<String>) -> Self {
        Self {
            provider: LlmProvider::Gemini,
            api_key: api_key.into(),
            api_endpoint: Some("https://generativelanguage.googleapis.com/v1".to_string()),
            organization_id: None,
            project_id: None,
            default_model: Some("gemini-pro".to_string()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            retry_delay_ms: Some(1000),
            extra_headers: None,
            proxy: None,
            enable_logging: Some(true),
            custom_params: None,
        }
    }
    
    /// 创建 Claude 配置
    pub fn claude(api_key: impl Into<String>) -> Self {
        Self {
            provider: LlmProvider::Claude,
            api_key: api_key.into(),
            api_endpoint: Some("https://api.anthropic.com/v1".to_string()),
            organization_id: None,
            project_id: None,
            default_model: Some("claude-3-sonnet-20240229".to_string()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            retry_delay_ms: Some(1000),
            extra_headers: None,
            proxy: None,
            enable_logging: Some(true),
            custom_params: None,
        }
    }
    
    /// 创建自定义配置
    pub fn custom(
        api_key: impl Into<String>,
        api_endpoint: impl Into<String>,
        default_model: impl Into<String>,
    ) -> Self {
        Self {
            provider: LlmProvider::Custom(0),
            api_key: api_key.into(),
            api_endpoint: Some(api_endpoint.into()),
            organization_id: None,
            project_id: None,
            default_model: Some(default_model.into()),
            timeout_seconds: Some(30),
            max_retries: Some(3),
            retry_delay_ms: Some(1000),
            extra_headers: None,
            proxy: None,
            enable_logging: Some(true),
            custom_params: None,
        }
    }
    
    /// 设置组织 ID
    pub fn with_organization_id(mut self, organization_id: impl Into<String>) -> Self {
        self.organization_id = Some(organization_id.into());
        self
    }
    
    /// 设置项目 ID
    pub fn with_project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }
    
    /// 设置默认模型
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }
    
    /// 设置超时时间
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }
    
    /// 设置重试配置
    pub fn with_retry(mut self, max_retries: u32, delay_ms: u64) -> Self {
        self.max_retries = Some(max_retries);
        self.retry_delay_ms = Some(delay_ms);
        self
    }
    
    /// 添加额外的 HTTP 头
    pub fn with_extra_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.extra_headers.is_none() {
            self.extra_headers = Some(HashMap::new());
        }
        self.extra_headers.as_mut().unwrap().insert(key.into(), value.into());
        self
    }
    
    /// 设置代理
    pub fn with_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    
    /// 设置是否启用日志
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = Some(enable);
        self
    }
    
    /// 获取 API 端点
    pub fn get_api_endpoint(&self) -> String {
        self.api_endpoint
            .clone()
            .unwrap_or_else(|| self.provider.default_endpoint().to_string())
    }
    
    /// 获取超时时间
    pub fn get_timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds.unwrap_or(30))
    }
    
    /// 获取最大重试次数
    pub fn get_max_retries(&self) -> u32 {
        self.max_retries.unwrap_or(3)
    }
    
    /// 获取重试延迟
    pub fn get_retry_delay(&self) -> Duration {
        Duration::from_millis(self.retry_delay_ms.unwrap_or(1000))
    }
    
    /// 验证配置
    pub fn validate(&self) -> LlmResult<()> {
        if self.api_key.is_empty() {
            return Err(LlmError::config_error("API key is required"));
        }
        
        if let Some(timeout) = self.timeout_seconds {
            if timeout == 0 || timeout > 300 {
                return Err(LlmError::config_error("Timeout must be between 1 and 300 seconds"));
            }
        }
        
        if let Some(retries) = self.max_retries {
            if retries > 10 {
                return Err(LlmError::config_error("Max retries cannot exceed 10"));
            }
        }
        
        Ok(())
    }
}

/// 多提供商配置管理器
#[derive(Debug, Clone)]
pub struct LlmConfigManager {
    configs: HashMap<String, LlmConfig>,
    default_config: Option<String>,
}

impl LlmConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            default_config: None,
        }
    }
    
    /// 添加配置
    pub fn add_config(&mut self, name: impl Into<String>, config: LlmConfig) -> LlmResult<()> {
        config.validate()?;
        let name = name.into();
        self.configs.insert(name.clone(), config);
        
        // 如果是第一个配置，设为默认
        if self.default_config.is_none() {
            self.default_config = Some(name);
        }
        
        Ok(())
    }
    
    /// 获取配置
    pub fn get_config(&self, name: &str) -> Option<&LlmConfig> {
        self.configs.get(name)
    }
    
    /// 获取默认配置
    pub fn get_default_config(&self) -> Option<&LlmConfig> {
        self.default_config
            .as_ref()
            .and_then(|name| self.configs.get(name))
    }
    
    /// 设置默认配置
    pub fn set_default(&mut self, name: impl Into<String>) -> LlmResult<()> {
        let name = name.into();
        if !self.configs.contains_key(&name) {
            return Err(LlmError::config_error(format!("Config '{}' not found", name)));
        }
        self.default_config = Some(name);
        Ok(())
    }
    
    /// 列出所有配置名称
    pub fn list_configs(&self) -> Vec<&String> {
        self.configs.keys().collect()
    }
    
    /// 移除配置
    pub fn remove_config(&mut self, name: &str) -> Option<LlmConfig> {
        let config = self.configs.remove(name);
        
        // 如果移除的是默认配置，清空默认设置
        if self.default_config.as_ref() == Some(&name.to_string()) {
            self.default_config = None;
        }
        
        config
    }
}

impl Default for LlmConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
