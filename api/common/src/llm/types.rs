//! 大模型调用的通用类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 大模型提供商枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LlmProvider {
    /// OpenAI (ChatGPT)
    OpenAI,
    /// DeepSeek
    DeepSeek,
    /// Google Gemini
    Gemini,
    /// Anthropic Claude
    Claude,
    /// 自定义提供商
    Custom(u32),
}

impl LlmProvider {
    /// 获取提供商名称
    pub fn name(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "OpenAI",
            LlmProvider::DeepSeek => "DeepSeek",
            LlmProvider::Gemini => "Gemini",
            LlmProvider::Claude => "Claude",
            LlmProvider::Custom(_) => "Custom",
        }
    }
    
    /// 获取默认的 API 端点
    pub fn default_endpoint(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "https://api.openai.com/v1",
            LlmProvider::DeepSeek => "https://api.deepseek.com/v1",
            LlmProvider::Gemini => "https://generativelanguage.googleapis.com/v1",
            LlmProvider::Claude => "https://api.anthropic.com/v1",
            LlmProvider::Custom(_) => "",
        }
    }
}

/// 消息角色
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// 系统消息
    System,
    /// 用户消息
    User,
    /// 助手消息
    Assistant,
    /// 工具消息
    Tool,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// 消息角色
    pub role: MessageRole,
    /// 消息内容
    pub content: String,
    /// 消息名称（可选）
    pub name: Option<String>,
    /// 工具调用（可选）
    pub tool_calls: Option<Vec<ToolCall>>,
    /// 工具调用 ID（可选）
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    /// 创建系统消息
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    
    /// 创建用户消息
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    
    /// 创建助手消息
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
}

/// 工具调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// 调用 ID
    pub id: String,
    /// 工具类型
    #[serde(rename = "type")]
    pub tool_type: String,
    /// 函数调用
    pub function: FunctionCall,
}

/// 函数调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// 函数名称
    pub name: String,
    /// 函数参数（JSON 字符串）
    pub arguments: String,
}

/// 聊天完成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// 模型名称
    pub model: String,
    /// 消息列表
    pub messages: Vec<ChatMessage>,
    /// 温度参数 (0.0-2.0)
    pub temperature: Option<f32>,
    /// Top-p 参数 (0.0-1.0)
    pub top_p: Option<f32>,
    /// 最大生成 token 数
    pub max_tokens: Option<u32>,
    /// 停止词
    pub stop: Option<Vec<String>>,
    /// 是否流式输出
    pub stream: Option<bool>,
    /// 随机种子
    pub seed: Option<u64>,
    /// 频率惩罚 (-2.0-2.0)
    pub frequency_penalty: Option<f32>,
    /// 存在惩罚 (-2.0-2.0)
    pub presence_penalty: Option<f32>,
    /// 用户 ID
    pub user: Option<String>,
    /// 工具列表
    pub tools: Option<Vec<Tool>>,
    /// 工具选择
    pub tool_choice: Option<ToolChoice>,
    /// 额外参数
    pub extra_params: Option<HashMap<String, serde_json::Value>>,
}

impl ChatCompletionRequest {
    /// 创建简单的聊天请求
    pub fn simple(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
            stream: None,
            seed: None,
            frequency_penalty: None,
            presence_penalty: None,
            user: None,
            tools: None,
            tool_choice: None,
            extra_params: None,
        }
    }
    
    /// 设置温度
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
    
    /// 设置最大 token 数
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    /// 设置流式输出
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// 工具类型
    #[serde(rename = "type")]
    pub tool_type: String,
    /// 函数定义
    pub function: Function,
}

/// 函数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// 函数名称
    pub name: String,
    /// 函数描述
    pub description: Option<String>,
    /// 参数定义
    pub parameters: Option<serde_json::Value>,
}

/// 工具选择
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// 自动选择
    Auto,
    /// 不使用工具
    None,
    /// 强制使用工具
    Required,
    /// 指定特定工具
    Specific { 
        #[serde(rename = "type")]
        tool_type: String,
        function: FunctionChoice 
    },
}

/// 函数选择
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionChoice {
    /// 函数名称
    pub name: String,
}

/// 聊天完成响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// 响应 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 模型名称
    pub model: String,
    /// 选择列表
    pub choices: Vec<Choice>,
    /// 使用统计
    pub usage: Option<Usage>,
    /// 系统指纹
    pub system_fingerprint: Option<String>,
}

/// 选择项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// 索引
    pub index: u32,
    /// 消息
    pub message: ChatMessage,
    /// 完成原因
    pub finish_reason: Option<String>,
    /// 日志概率
    pub logprobs: Option<serde_json::Value>,
}

/// 使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// 提示 token 数
    pub prompt_tokens: u32,
    /// 完成 token 数
    pub completion_tokens: u32,
    /// 总 token 数
    pub total_tokens: u32,
}

/// 流式响应块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// 响应 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 模型名称
    pub model: String,
    /// 选择列表
    pub choices: Vec<ChunkChoice>,
    /// 系统指纹
    pub system_fingerprint: Option<String>,
}

/// 流式选择项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkChoice {
    /// 索引
    pub index: u32,
    /// 增量消息
    pub delta: ChatMessage,
    /// 完成原因
    pub finish_reason: Option<String>,
    /// 日志概率
    pub logprobs: Option<serde_json::Value>,
}

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// 模型 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间
    pub created: u64,
    /// 拥有者
    pub owned_by: String,
    /// 权限
    pub permission: Option<Vec<serde_json::Value>>,
    /// 根模型
    pub root: Option<String>,
    /// 父模型
    pub parent: Option<String>,
}

/// 模型列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    /// 对象类型
    pub object: String,
    /// 模型列表
    pub data: Vec<ModelInfo>,
}

/// 聊天会话
#[derive(Debug, Clone)]
pub struct ChatSession {
    /// 会话 ID
    pub id: Uuid,
    /// 消息历史
    pub messages: Vec<ChatMessage>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 会话元数据
    pub metadata: HashMap<String, String>,
}

impl ChatSession {
    /// 创建新会话
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }
    
    /// 添加消息
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
        self.updated_at = chrono::Utc::now();
    }
    
    /// 获取最近的消息
    pub fn recent_messages(&self, count: usize) -> &[ChatMessage] {
        let start = self.messages.len().saturating_sub(count);
        &self.messages[start..]
    }
    
    /// 清空消息历史
    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.updated_at = chrono::Utc::now();
    }
}
