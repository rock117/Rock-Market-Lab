use crate::types::*;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct HttpRequestExecutor;

#[async_trait]
impl TaskExecutor for HttpRequestExecutor {
    async fn execute(&self, config: Value, context: ExecutionContext) -> anyhow::Result<TaskResult> {
        let url = config.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: url"))?;
        
        let method = config.get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");
        
        let headers = config.get("headers")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        
        let body = config.get("body")
            .and_then(|v| v.as_str());

        let timeout_seconds = config.get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(30);

        context.logger.info(&format!("Making {} request to {}", method, url)).await;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_seconds))
            .build()?;

        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "PATCH" => client.patch(url),
            "HEAD" => client.head(url),
            _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
        };

        // Add headers
        for (key, value) in headers {
            if let Some(value_str) = value.as_str() {
                request = request.header(key, value_str);
            }
        }

        // Add body if present and method supports it
        if let Some(body_str) = body {
            if matches!(method.to_uppercase().as_str(), "POST" | "PUT" | "PATCH") {
                // Try to parse as JSON first, otherwise send as text
                if let Ok(json_value) = serde_json::from_str::<Value>(body_str) {
                    request = request.json(&json_value);
                    context.logger.info("Sending JSON body").await;
                } else {
                    request = request.body(body_str.to_string());
                    context.logger.info("Sending text body").await;
                }
            }
        }

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                let status_code = status.as_u16();
                
                context.logger.info(&format!("Response status: {} ({})", status_code, status)).await;
                
                // Get response headers for logging
                let headers_info = response.headers()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("invalid")))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                if !headers_info.is_empty() {
                    context.logger.info(&format!("Response headers: {}", headers_info)).await;
                }

                let response_text = response.text().await.unwrap_or_default();
                
                // Log response size
                context.logger.info(&format!("Response size: {} bytes", response_text.len())).await;
                
                if status.is_success() {
                    // Try to pretty-print JSON responses
                    let output = if let Ok(json_value) = serde_json::from_str::<Value>(&response_text) {
                        serde_json::to_string_pretty(&json_value).unwrap_or(response_text)
                    } else {
                        response_text
                    };
                    
                    context.logger.info("Request completed successfully").await;
                    Ok(TaskResult::success(Some(output)))
                } else {
                    let error_msg = format!("HTTP {} - {}", status_code, response_text);
                    context.logger.error(&error_msg).await;
                    Ok(TaskResult::failure(error_msg))
                }
            }
            Err(e) => {
                let error_msg = format!("Request failed: {}", e);
                context.logger.error(&error_msg).await;
                
                // Provide more specific error information
                if e.is_timeout() {
                    context.logger.error("Request timed out").await;
                } else if e.is_connect() {
                    context.logger.error("Connection failed").await;
                } else if e.is_request() {
                    context.logger.error("Request configuration error").await;
                }
                
                Ok(TaskResult::failure(error_msg))
            }
        }
    }

    fn validate_config(&self, config: &Value) -> anyhow::Result<()> {
        if !config.is_object() {
            return Err(anyhow::anyhow!("Config must be an object"));
        }

        let url = config.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: url"))?;

        // Validate URL format
        url::Url::parse(url)
            .map_err(|e| anyhow::anyhow!("Invalid URL format: {}", e))?;

        // Validate method if provided
        if let Some(method) = config.get("method").and_then(|v| v.as_str()) {
            match method.to_uppercase().as_str() {
                "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" => {},
                _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
            }
        }

        // Validate headers if provided
        if let Some(headers) = config.get("headers") {
            if !headers.is_object() {
                return Err(anyhow::anyhow!("Headers must be an object"));
            }
        }

        // Validate timeout if provided
        if let Some(timeout) = config.get("timeout") {
            if !timeout.is_number() {
                return Err(anyhow::anyhow!("Timeout must be a number"));
            }
        }

        Ok(())
    }

    fn get_schema(&self) -> TaskConfigSchema {
        TaskConfigSchema {
            name: "HTTP Request".to_string(),
            description: "Make HTTP requests to external APIs and web services".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "url".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: "Target URL (must include http:// or https://)".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "method".to_string(),
                    field_type: "select".to_string(),
                    required: false,
                    description: "HTTP method".to_string(),
                    default_value: Some(Value::String("GET".to_string())),
                    options: Some(vec![
                        "GET".to_string(), 
                        "POST".to_string(), 
                        "PUT".to_string(), 
                        "DELETE".to_string(),
                        "PATCH".to_string(),
                        "HEAD".to_string()
                    ]),
                },
                TaskConfigField {
                    name: "headers".to_string(),
                    field_type: "object".to_string(),
                    required: false,
                    description: "HTTP headers as key-value pairs".to_string(),
                    default_value: Some(Value::Object(serde_json::Map::new())),
                    options: None,
                },
                TaskConfigField {
                    name: "body".to_string(),
                    field_type: "text".to_string(),
                    required: false,
                    description: "Request body (JSON or plain text)".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "timeout".to_string(),
                    field_type: "number".to_string(),
                    required: false,
                    description: "Request timeout in seconds".to_string(),
                    default_value: Some(Value::Number(serde_json::Number::from(30))),
                    options: None,
                },
            ],
        }
    }

    fn get_type_name(&self) -> &'static str {
        "http_request"
    }
}
