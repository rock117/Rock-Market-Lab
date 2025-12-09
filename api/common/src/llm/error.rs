//! 大模型调用错误类型定义

use thiserror::Error;

/// 大模型调用结果类型
pub type LlmResult<T> = Result<T, LlmError>;

/// 大模型调用错误类型
#[derive(Error, Debug)]
pub enum LlmError {
    /// API 调用错误
    #[error("API调用失败: {message}")]
    ApiError { message: String, status_code: Option<u16> },
    
    /// 认证错误
    #[error("认证失败: {0}")]
    AuthenticationError(String),
    
    /// 配额超限错误
    #[error("配额超限: {0}")]
    QuotaExceededError(String),
    
    /// 请求参数错误
    #[error("请求参数错误: {0}")]
    InvalidRequestError(String),
    
    /// 模型不存在错误
    #[error("模型不存在: {0}")]
    ModelNotFoundError(String),
    
    /// 内容过滤错误
    #[error("内容被过滤: {0}")]
    ContentFilterError(String),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    /// JSON 序列化/反序列化错误
    #[error("JSON处理错误: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    /// 超时错误
    #[error("请求超时")]
    TimeoutError,
    
    /// 未知错误
    #[error("未知错误: {0}")]
    UnknownError(String),
}

impl LlmError {
    /// 创建 API 错误
    pub fn api_error(message: impl Into<String>, status_code: Option<u16>) -> Self {
        Self::ApiError {
            message: message.into(),
            status_code,
        }
    }
    
    /// 创建认证错误
    pub fn auth_error(message: impl Into<String>) -> Self {
        Self::AuthenticationError(message.into())
    }
    
    /// 创建配额超限错误
    pub fn quota_exceeded(message: impl Into<String>) -> Self {
        Self::QuotaExceededError(message.into())
    }
    
    /// 创建配置错误
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
    }
    
    /// 判断是否为可重试的错误
    pub fn is_retryable(&self) -> bool {
        match self {
            LlmError::NetworkError(_) | LlmError::TimeoutError => true,
            LlmError::ApiError { status_code: Some(code), .. } if *code >= 500 => true,
            _ => false,
        }
    }
    
    /// 判断是否为认证相关错误
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            LlmError::AuthenticationError(_) | LlmError::ApiError { status_code: Some(401 | 403), .. }
        )
    }
    
    /// 判断是否为配额相关错误
    pub fn is_quota_error(&self) -> bool {
        matches!(
            self,
            LlmError::QuotaExceededError(_) | LlmError::ApiError { status_code: Some(429), .. }
        )
    }
}
