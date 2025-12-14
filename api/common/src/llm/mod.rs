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

#[derive(Debug, Clone)]
pub struct CNStock {
    pub concepts: String,
    pub main_business: String,
    pub business_scope: String,
    pub broad_name: String,
}

#[derive(Debug, Clone)]
pub struct USStock {
    pub main_business: String,
    pub industry: String,
    pub sector: String,
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
    chat_str_result(&req).await
}



pub async fn calculate_stock_similarity(cn_stock: &CNStock, us_stock: &USStock) -> anyhow::Result<String> {
    let cn_symbol = "";
    let cn_main_business = &cn_stock.main_business;
    let cn_business_scope = &cn_stock.business_scope;
    let cn_concepts = &cn_stock.concepts;
    let cn_broad_name = &cn_stock.broad_name;

    let us_symbol = "";
    let us_main_business = &us_stock.main_business;
    let us_industry = &us_stock.industry;
    let us_sector = &us_stock.sector;
    let promote = format!(
r#"
【输入数据】
A股：
- 股票代码：{cn_symbol}
- 主营业务：{cn_main_business} {cn_business_scope}
- 行业板块：{cn_broad_name}
- 概念板块：{cn_concepts}

美股：
- 股票代码：{us_symbol}
- 主营业务：{us_main_business}
- 行业板块：{us_sector}
- 概念板块：{us_industry}

【任务要求】
1. 严格基于输入数据分析，不依赖外部信息。
2. 对三个维度分别给出：
    - 关联说明（为什么相似或不相似）
    - 相似度评分（0～100）
3. 最后给出一个综合关联度评分（0～100）。
4. 输出必须结构化、规则化，方便程序解析。

【输出格式】
### 一、维度分析
#### 1. 主营业务关联性
- 分析说明：……
- 主营业务相似度：X / 100

#### 2. 行业板块关联性
- 分析说明：……
- 行业板块相似度：X / 100

#### 3. 概念板块关联性
- 分析说明：……
- 概念板块相似度：X / 100

### 二、综合结果
- 综合关联度：X / 100
- 关联等级：强 / 中等 / 弱（根据分数自动判断）
- 关键原因总结（简短）：……

请严格按照以上格式输出。
     "#);

    let req = r#"
        {
        "model": "deepseek-chat",
        "messages": [
          {"role": "system", "content": "你是一个擅长结构化分析的金融研究助手。现在给你两只股票的结构化信息, 一个是A股(中国股票)，一个是美股，请你从「主营业务」「行业板块」「概念板块」三个维度分析它们的相似度和关联性，并输出一个综合关联评分。"},
          {"role": "user", "content": "{promote}"}
        ],
        "stream": false
      }
        "#.replace("{promote}", &promote);
    chat_str_result(&req).await
}

async fn chat_str_result(promote: &str) -> anyhow::Result<String> {
    let req = serde_json::from_str::<ChatRequest>(&promote)?;
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