//! 大模型调用模块使用示例
//! 
//! 展示如何使用统一的大模型调用接口

use common::llm::{
    LlmClient, LlmClientBuilder, LlmConfig, ChatMessage, ChatCompletionRequest
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    println!("=== 大模型调用模块示例 ===\n");
    
    // 方式1: 手动创建客户端并添加配置
    let client = LlmClient::new();
    
    // 从环境变量获取 API 密钥（实际使用时）
    // let openai_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| "your-openai-key".to_string());
    // let deepseek_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "your-deepseek-key".to_string());
    
    // 示例配置（请替换为真实的 API 密钥）
    let openai_key = "your-openai-api-key";
    let deepseek_key = "your-deepseek-api-key";
    
    // 添加 OpenAI 配置
    if openai_key != "your-openai-api-key" {
        let openai_config = LlmConfig::openai(openai_key)
            .with_default_model("gpt-3.5-turbo")
            .with_timeout(30)
            .with_logging(true);
        
        match client.add_config("openai", openai_config).await {
            Ok(_) => println!("✅ OpenAI 配置添加成功"),
            Err(e) => println!("❌ OpenAI 配置添加失败: {}", e),
        }
    }
    
    // 添加 DeepSeek 配置
    if deepseek_key != "your-deepseek-api-key" {
        let deepseek_config = LlmConfig::deepseek(deepseek_key)
            .with_default_model("deepseek-chat")
            .with_timeout(30)
            .with_logging(true);
        
        match client.add_config("deepseek", deepseek_config).await {
            Ok(_) => println!("✅ DeepSeek 配置添加成功"),
            Err(e) => println!("❌ DeepSeek 配置添加失败: {}", e),
        }
    }
    
    // 方式2: 使用构建器模式（如果有有效的 API 密钥）
    /*
    let client = LlmClientBuilder::new()
        .with_openai("openai", openai_key)
        .with_deepseek("deepseek", deepseek_key)
        .with_gemini("gemini", "your-gemini-key")
        .with_claude("claude", "your-claude-key")
        .with_default("openai")
        .build()
        .await?;
    */
    
    // 列出所有配置
    let configs = client.list_configs().await;
    println!("\n📋 已配置的提供商: {:?}", configs);
    
    if !configs.is_empty() {
        // 设置默认配置
        if let Err(e) = client.set_default_config(&configs[0]).await {
            println!("❌ 设置默认配置失败: {}", e);
        } else {
            println!("✅ 默认配置设置为: {}", configs[0]);
        }
        
        // 示例1: 简单聊天
        println!("\n🤖 示例1: 简单聊天");
        match client.chat("你好，请简单介绍一下你自己", None).await {
            Ok(response) => println!("回复: {}", response),
            Err(e) => println!("聊天失败: {}", e),
        }
        
        // 示例2: 使用完整的聊天完成接口
        println!("\n🤖 示例2: 完整聊天完成");
        let request = ChatCompletionRequest::simple(
            "".to_string(), // 使用默认模型
            vec![
                ChatMessage::system("你是一个专业的金融分析师"),
                ChatMessage::user("请分析一下当前股市的趋势"),
            ],
        )
        .with_temperature(0.7)
        .with_max_tokens(500);
        
        match client.chat_completion(request, None).await {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    println!("分析结果: {}", choice.message.content);
                    if let Some(usage) = response.usage {
                        println!("Token 使用: 输入={}, 输出={}, 总计={}", 
                            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens);
                    }
                }
            }
            Err(e) => println!("分析失败: {}", e),
        }
        
        // 示例3: 会话管理
        println!("\n💬 示例3: 会话管理");
        let session = client.create_session().await;
        println!("创建会话: {}", session.id);
        
        // 在会话中发送多条消息
        let questions = vec![
            "什么是量化交易？",
            "量化交易有哪些常见策略？",
            "如何评估策略的风险？",
        ];
        
        for question in questions {
            match client.chat_in_session(session.id, question, None).await {
                Ok(answer) => println!("Q: {}\nA: {}\n", question, answer),
                Err(e) => println!("会话聊天失败: {}", e),
            }
        }
        
        // 获取会话信息
        if let Some(updated_session) = client.get_session(session.id).await {
            println!("会话消息数量: {}", updated_session.messages.len());
        }
        
        // 示例4: 多提供商对比（如果配置了多个提供商）
        if configs.len() > 1 {
            println!("\n🔄 示例4: 多提供商对比");
            let question = "用一句话解释什么是人工智能";
            
            for config_name in &configs {
                match client.chat(question, Some(config_name)).await {
                    Ok(response) => println!("{}: {}", config_name, response),
                    Err(e) => println!("{} 失败: {}", config_name, e),
                }
            }
        }
        
        // 示例5: 获取模型列表
        println!("\n📝 示例5: 获取模型列表");
        match client.list_models(None).await {
            Ok(models) => {
                println!("可用模型数量: {}", models.data.len());
                for model in models.data.iter().take(5) {
                    println!("- {}", model.id);
                }
            }
            Err(e) => println!("获取模型列表失败: {}", e),
        }
        
        // 清理会话
        if client.delete_session(session.id).await {
            println!("✅ 会话已删除");
        }
    } else {
        println!("⚠️  没有配置任何提供商，请设置有效的 API 密钥");
        
        // 展示配置示例
        println!("\n📖 配置示例:");
        println!("1. OpenAI:");
        println!("   let config = LlmConfig::openai(\"your-api-key\");");
        println!("   client.add_config(\"openai\", config).await?;");
        
        println!("\n2. DeepSeek:");
        println!("   let config = LlmConfig::deepseek(\"your-api-key\");");
        println!("   client.add_config(\"deepseek\", config).await?;");
        
        println!("\n3. 自定义提供商:");
        println!("   let config = LlmConfig::custom(\"your-api-key\", \"https://api.example.com/v1\", \"model-name\");");
        println!("   client.add_config(\"custom\", config).await?;");
    }
    
    // 获取统计信息
    let stats = client.get_session_stats().await;
    println!("\n📊 统计信息: {:?}", stats);
    
    println!("\n✅ 大模型调用模块示例运行完成!");
    
    Ok(())
}

/// 展示错误处理
async fn demonstrate_error_handling() {
    println!("\n🚨 错误处理示例:");
    
    let client = LlmClient::new();
    
    // 1. 无效的 API 密钥
    let invalid_config = LlmConfig::openai("invalid-key");
    match client.add_config("invalid", invalid_config).await {
        Ok(_) => println!("不应该成功"),
        Err(e) => println!("预期的错误: {}", e),
    }
    
    // 2. 使用不存在的配置
    match client.chat("Hello", Some("nonexistent")).await {
        Ok(_) => println!("不应该成功"),
        Err(e) => println!("预期的错误: {}", e),
    }
    
    // 3. 没有默认配置
    match client.chat("Hello", None).await {
        Ok(_) => println!("不应该成功"),
        Err(e) => println!("预期的错误: {}", e),
    }
}
