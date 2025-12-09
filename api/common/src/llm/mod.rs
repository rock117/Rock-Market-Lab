use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::http;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    #[serde(rename = "type")]
    pub thinking_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub thinking: Option<ThinkingConfig>,
    pub frequency_penalty: Option<f64>,
    pub max_tokens: Option<u32>,
    pub presence_penalty: Option<f64>,
    pub response_format: Option<ResponseFormat>,
    pub stop: Option<String>,
    pub stream: Option<bool>,
    pub stream_options: Option<serde_json::Value>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub tools: Option<serde_json::Value>,
    pub tool_choice: Option<String>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: Option<u32>,
    pub message: Option<ChatMessage>,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    pub cached_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub prompt_cache_hit_tokens: Option<u32>,
    pub prompt_cache_miss_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Option<Vec<ChatChoice>>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
}

pub async fn chat(request: &ChatRequest) -> anyhow::Result<ChatResponse>{
    let request = serde_json::to_string(&request)?;
    let key = "sk-47b29c3eac324b2a8a137b4a7838a93b";
    let mut headers = HashMap::new();
    headers.insert("Content-Type".into(), "application/json".into());
    headers.insert("Authorization".into(), format!("Bearer {}", key));
    let res = http::post("https://api.deepseek.com/chat/completions", Some(request), Some(&headers)).await?;
    Ok(res.json().await?)
}


mod tests {
    use crate::http;
    use crate::llm::{chat, ChatRequest};

    #[tokio::test]
    async fn get_page() {
        let req = r#"
        {
        "model": "deepseek-chat",
        "messages": [
          {"role": "system", "content": "You are a helpful assistant."},
          {"role": "user", "content": "Hello!"}
        ],
        "stream": false
      }
        "#;
        let req = serde_json::from_str::<ChatRequest>(req).unwrap();
        let s = chat(&req).await.unwrap();
        println!("{:?}", serde_json::to_string(&s));
    }
}