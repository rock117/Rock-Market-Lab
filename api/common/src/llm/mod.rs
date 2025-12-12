use std::collections::HashMap;
use anyhow::bail;
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

pub async fn translate_finance_eng(eng: &str) -> anyhow::Result<String> {
    let req = r#"
        {
        "model": "deepseek-chat",
        "messages": [
          {"role": "system", "content": "你是一个英文翻译, 翻译美股上市公司的资料为中文"},
          {"role": "user", "content": "{eng}"}
        ],
        "stream": false
      }
        "#.replace("{eng}", eng);
    let req = serde_json::from_str::<ChatRequest>(&req)?;
    let res = chat(&req).await?;

    res.choices
        .and_then(|c| c.first().cloned())
        .and_then(|choice| choice.message)
        .map(|msg| msg.content)
        .ok_or_else(|| anyhow::anyhow!("No valid response from LLM"))
}


mod tests {
    use crate::http;
    use crate::llm::{chat, ChatRequest, translate_finance_eng};

    #[tokio::test]
    async fn test() {
      let txt = translate_finance_eng("EVI Industries Inc is a value-added distributor and service provider in the commercial laundry industry. It sells and leases commercial laundry equipment, specializing in washing, drying, finishing, material handling, water heating, power generation, and water reuse applications. The company supports its equipment offerings with installation, maintenance, and repair services through a large network of trained technicians. It serves a wide range of customers, including commercial, industrial, institutional, government, and retail sectors. Geographically, the company serves various countries including United States, Canada, the Caribbean, and Latin America.").await.unwrap();
      println!("{}", txt);
    }
}