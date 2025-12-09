# 大模型调用模块 (LLM Module)

这个模块提供了统一的大模型调用接口，支持多种主流的大模型提供商，包括 OpenAI、DeepSeek、Google Gemini、Anthropic Claude 等。

## 🎯 主要特性

### 1. 多提供商支持
- **OpenAI** (ChatGPT) - 完整支持
- **DeepSeek** - 兼容 OpenAI API 格式
- **Google Gemini** - 自定义 API 格式适配
- **Anthropic Claude** - 自定义 API 格式适配
- **自定义提供商** - 支持任何兼容 OpenAI API 的服务

### 2. 统一接口
- 统一的请求/响应格式
- 自动格式转换和适配
- 透明的错误处理
- 一致的配置管理

### 3. 高级功能
- 会话管理
- 流式输出（部分支持）
- 自动重试机制
- 配额和速率限制处理
- 详细的日志记录

### 4. 易用性
- 简单的配置接口
- 构建器模式支持
- 异步/并发支持
- 完善的错误处理

## 📦 核心组件

### 类型系统 (`types.rs`)
```rust
// 提供商枚举
pub enum LlmProvider {
    OpenAI,
    DeepSeek, 
    Gemini,
    Claude,
    Custom(u32),
}

// 聊天消息
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    // ...
}

// 聊天完成请求
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    // ...
}
```

### 配置管理 (`config.rs`)
```rust
// 基础配置
let config = LlmConfig::openai("your-api-key")
    .with_default_model("gpt-3.5-turbo")
    .with_timeout(30)
    .with_retry(3, 1000)
    .with_logging(true);

// 多配置管理
let mut manager = LlmConfigManager::new();
manager.add_config("openai", openai_config)?;
manager.add_config("deepseek", deepseek_config)?;
manager.set_default("openai")?;
```

### 客户端接口 (`client.rs`)
```rust
// 创建客户端
let client = LlmClient::new();

// 添加配置
client.add_config("openai", config).await?;

// 简单聊天
let response = client.chat("Hello", None).await?;

// 完整聊天完成
let request = ChatCompletionRequest::simple(model, messages);
let response = client.chat_completion(request, None).await?;
```

## 🚀 快速开始

### 1. 基本使用

```rust
use common::llm::{LlmClient, LlmConfig, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = LlmClient::new();
    
    // 添加 OpenAI 配置
    let config = LlmConfig::openai("your-api-key");
    client.add_config("openai", config).await?;
    
    // 简单聊天
    let response = client.chat("你好", Some("openai")).await?;
    println!("回复: {}", response);
    
    Ok(())
}
```

### 2. 使用构建器

```rust
use common::llm::LlmClientBuilder;

let client = LlmClientBuilder::new()
    .with_openai("openai", "your-openai-key")
    .with_deepseek("deepseek", "your-deepseek-key")
    .with_default("openai")
    .build()
    .await?;

let response = client.chat("Hello", None).await?;
```

### 3. 会话管理

```rust
// 创建会话
let session = client.create_session().await;

// 在会话中聊天
let response1 = client.chat_in_session(session.id, "你好", None).await?;
let response2 = client.chat_in_session(session.id, "请继续", None).await?;

// 获取会话历史
let session = client.get_session(session.id).await.unwrap();
println!("消息数量: {}", session.messages.len());
```

### 4. 多提供商对比

```rust
let providers = vec!["openai", "deepseek", "claude"];
let question = "什么是人工智能？";

for provider in providers {
    match client.chat(question, Some(provider)).await {
        Ok(response) => println!("{}: {}", provider, response),
        Err(e) => println!("{} 失败: {}", provider, e),
    }
}
```

## ⚙️ 配置选项

### OpenAI 配置
```rust
let config = LlmConfig::openai("your-api-key")
    .with_organization_id("org-xxx")
    .with_project_id("proj-xxx")
    .with_default_model("gpt-4")
    .with_timeout(60)
    .with_proxy("http://proxy:8080");
```

### DeepSeek 配置
```rust
let config = LlmConfig::deepseek("your-api-key")
    .with_default_model("deepseek-chat")
    .with_timeout(30);
```

### Gemini 配置
```rust
let config = LlmConfig::gemini("your-api-key")
    .with_default_model("gemini-pro")
    .with_timeout(30);
```

### Claude 配置
```rust
let config = LlmConfig::claude("your-api-key")
    .with_default_model("claude-3-sonnet-20240229")
    .with_timeout(30);
```

### 自定义提供商
```rust
let config = LlmConfig::custom(
    "your-api-key",
    "https://api.example.com/v1",
    "custom-model"
)
.with_extra_header("Custom-Header", "value")
.with_timeout(30);
```

## 🔧 高级功能

### 1. 错误处理

```rust
use common::llm::{LlmError, LlmResult};

match client.chat("Hello", None).await {
    Ok(response) => println!("成功: {}", response),
    Err(LlmError::AuthenticationError(msg)) => println!("认证失败: {}", msg),
    Err(LlmError::QuotaExceededError(msg)) => println!("配额超限: {}", msg),
    Err(LlmError::NetworkError(e)) => println!("网络错误: {}", e),
    Err(e) => println!("其他错误: {}", e),
}
```

### 2. 重试机制

```rust
let config = LlmConfig::openai("your-api-key")
    .with_retry(5, 2000); // 最多重试5次，延迟2秒

// 客户端会自动处理可重试的错误
let response = client.chat("Hello", None).await?;
```

### 3. 流式输出

```rust
use futures::StreamExt;

let request = ChatCompletionRequest::simple(model, messages)
    .with_stream(true);

let mut stream = client.chat_completion_stream(request, None).await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(chunk) => {
            if let Some(choice) = chunk.choices.first() {
                print!("{}", choice.delta.content);
            }
        }
        Err(e) => eprintln!("流式错误: {}", e),
    }
}
```

### 4. 工具调用（Function Calling）

```rust
use common::llm::{Tool, Function};

let tool = Tool {
    tool_type: "function".to_string(),
    function: Function {
        name: "get_weather".to_string(),
        description: Some("获取天气信息".to_string()),
        parameters: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "城市名称"
                }
            },
            "required": ["location"]
        })),
    },
};

let request = ChatCompletionRequest::simple(model, messages)
    .with_tools(vec![tool]);

let response = client.chat_completion(request, None).await?;
```

## 🔍 调试和监控

### 1. 启用日志

```rust
// 在配置中启用日志
let config = LlmConfig::openai("your-api-key")
    .with_logging(true);

// 或者在环境变量中设置
// RUST_LOG=debug cargo run
```

### 2. 获取统计信息

```rust
// 会话统计
let stats = client.get_session_stats().await;
println!("总会话数: {}", stats.get("total_sessions").unwrap_or(&0));
println!("总消息数: {}", stats.get("total_messages").unwrap_or(&0));

// Token 使用统计
if let Some(usage) = response.usage {
    println!("Token 使用: 输入={}, 输出={}, 总计={}", 
        usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
}
```

### 3. 验证配置

```rust
// 验证 API 密钥
let is_valid = client.validate_api_key(Some("openai")).await?;
if !is_valid {
    println!("API 密钥无效");
}

// 获取可用模型
let models = client.list_models(Some("openai")).await?;
for model in models.data {
    println!("模型: {}", model.id);
}
```

## 🛡️ 安全注意事项

### 1. API 密钥管理
- 不要在代码中硬编码 API 密钥
- 使用环境变量或配置文件
- 定期轮换 API 密钥

```rust
use std::env;

let api_key = env::var("OPENAI_API_KEY")
    .expect("请设置 OPENAI_API_KEY 环境变量");
```

### 2. 网络安全
- 使用 HTTPS 端点
- 配置适当的代理和防火墙
- 验证 SSL 证书

### 3. 数据隐私
- 不要发送敏感信息到外部 API
- 考虑使用本地部署的模型
- 遵守数据保护法规

## 📈 性能优化

### 1. 连接池
```rust
// 客户端内部使用连接池，支持并发请求
let tasks: Vec<_> = (0..10)
    .map(|i| client.chat(format!("问题 {}", i), None))
    .collect();

let responses = futures::future::join_all(tasks).await;
```

### 2. 缓存
```rust
// 可以在应用层实现响应缓存
use std::collections::HashMap;

let mut cache = HashMap::new();
let question = "什么是AI？";

if let Some(cached) = cache.get(question) {
    println!("缓存命中: {}", cached);
} else {
    let response = client.chat(question, None).await?;
    cache.insert(question.to_string(), response.clone());
    println!("新响应: {}", response);
}
```

### 3. 批量处理
```rust
// 对于多个独立的请求，可以并行处理
let questions = vec!["问题1", "问题2", "问题3"];
let tasks: Vec<_> = questions
    .into_iter()
    .map(|q| client.chat(q, None))
    .collect();

let responses = futures::future::join_all(tasks).await;
```

## 🧪 测试

### 1. 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_validation() {
        let config = LlmConfig::openai("test-key");
        assert!(config.validate().is_ok());
        
        let invalid_config = LlmConfig::openai("")
            .with_timeout(0);
        assert!(invalid_config.validate().is_err());
    }
}
```

### 2. 集成测试
```rust
#[tokio::test]
async fn test_chat_completion() {
    let client = LlmClient::new();
    let config = LlmConfig::openai(env::var("TEST_API_KEY").unwrap());
    client.add_config("test", config).await.unwrap();
    
    let response = client.chat("Hello", Some("test")).await.unwrap();
    assert!(!response.is_empty());
}
```

## 🔄 扩展新提供商

要添加新的大模型提供商，需要实现 `LlmProvider` trait：

```rust
use async_trait::async_trait;
use super::providers::LlmProvider as LlmProviderTrait;

pub struct CustomProvider {
    config: LlmConfig,
    client: reqwest::Client,
}

#[async_trait]
impl LlmProviderTrait for CustomProvider {
    fn name(&self) -> &'static str {
        "Custom"
    }
    
    async fn chat_completion(&self, request: ChatCompletionRequest) -> LlmResult<ChatCompletionResponse> {
        // 实现聊天完成逻辑
        todo!()
    }
    
    // 实现其他必需的方法...
}
```

## 📚 更多示例

查看 `examples/llm_example.rs` 获取完整的使用示例，包括：
- 基本聊天功能
- 会话管理
- 多提供商对比
- 错误处理
- 配置管理

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个模块！

## 📄 许可证

本项目采用 MIT 许可证。
