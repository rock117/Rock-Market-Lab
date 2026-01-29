use crate::types::*;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct TaskExecutorRegistry {
    executors: HashMap<String, Box<dyn TaskExecutor>>,
}

impl TaskExecutorRegistry {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    pub fn register(&mut self, executor: Box<dyn TaskExecutor>) {
        let type_name = executor.get_type_name().to_string();
        self.executors.insert(type_name, executor);
    }

    pub fn get_executor(&self, task_type: &str) -> Option<&Box<dyn TaskExecutor>> {
        self.executors.get(task_type)
    }

    pub fn get_all_schemas(&self) -> Vec<TaskConfigSchema> {
        self.executors
            .values()
            .map(|executor| executor.get_schema())
            .collect()
    }

    pub fn validate_config(&self, task_type: &str, config: &Value) -> anyhow::Result<()> {
        match self.get_executor(task_type) {
            Some(executor) => executor.validate_config(config),
            None => Err(anyhow::anyhow!("Unknown task type: {}", task_type)),
        }
    }
}

impl Default for TaskExecutorRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Register built-in executors
        registry.register(Box::new(HttpRequestExecutor));
        registry.register(Box::new(ShellCommandExecutor));
        
        registry
    }
}

// HTTP Request Executor
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

        context.logger.info(&format!("Making {} request to {}", method, url)).await;

        let client = reqwest::Client::new();
        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
        };

        // Add headers
        for (key, value) in headers {
            if let Some(value_str) = value.as_str() {
                request = request.header(key, value_str);
            }
        }

        // Add body if present
        if let Some(body_str) = body {
            request = request.body(body_str.to_string());
        }

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                let response_text = response.text().await.unwrap_or_default();
                
                context.logger.info(&format!("Response status: {}", status)).await;
                
                if status.is_success() {
                    Ok(TaskResult::success(Some(response_text)))
                } else {
                    Ok(TaskResult::failure(format!("HTTP {} - {}", status, response_text)))
                }
            }
            Err(e) => {
                context.logger.error(&format!("Request failed: {}", e)).await;
                Ok(TaskResult::failure(e.to_string()))
            }
        }
    }

    fn validate_config(&self, config: &Value) -> anyhow::Result<()> {
        if !config.is_object() {
            return Err(anyhow::anyhow!("Config must be an object"));
        }

        if config.get("url").and_then(|v| v.as_str()).is_none() {
            return Err(anyhow::anyhow!("Missing required field: url"));
        }

        Ok(())
    }

    fn get_schema(&self) -> TaskConfigSchema {
        TaskConfigSchema {
            name: "HTTP Request".to_string(),
            description: "Make HTTP requests to external APIs".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "url".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: "Target URL".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "method".to_string(),
                    field_type: "select".to_string(),
                    required: false,
                    description: "HTTP method".to_string(),
                    default_value: Some(Value::String("GET".to_string())),
                    options: Some(vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]),
                },
                TaskConfigField {
                    name: "headers".to_string(),
                    field_type: "object".to_string(),
                    required: false,
                    description: "HTTP headers".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "body".to_string(),
                    field_type: "text".to_string(),
                    required: false,
                    description: "Request body".to_string(),
                    default_value: None,
                    options: None,
                },
            ],
        }
    }

    fn get_type_name(&self) -> &'static str {
        "http_request"
    }
}

// Shell Command Executor
pub struct ShellCommandExecutor;

#[async_trait]
impl TaskExecutor for ShellCommandExecutor {
    async fn execute(&self, config: Value, context: ExecutionContext) -> anyhow::Result<TaskResult> {
        let command = config.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: command"))?;
        
        let working_dir = config.get("working_dir")
            .and_then(|v| v.as_str());

        context.logger.info(&format!("Executing command: {}", command)).await;

        let mut cmd = tokio::process::Command::new("cmd");
        cmd.args(&["/C", command]);
        
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        match cmd.output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                if output.status.success() {
                    context.logger.info("Command completed successfully").await;
                    Ok(TaskResult::success(Some(stdout.to_string())))
                } else {
                    context.logger.error(&format!("Command failed: {}", stderr)).await;
                    Ok(TaskResult::failure(stderr.to_string()))
                }
            }
            Err(e) => {
                context.logger.error(&format!("Failed to execute command: {}", e)).await;
                Ok(TaskResult::failure(e.to_string()))
            }
        }
    }

    fn validate_config(&self, config: &Value) -> anyhow::Result<()> {
        if !config.is_object() {
            return Err(anyhow::anyhow!("Config must be an object"));
        }

        if config.get("command").and_then(|v| v.as_str()).is_none() {
            return Err(anyhow::anyhow!("Missing required field: command"));
        }

        Ok(())
    }

    fn get_schema(&self) -> TaskConfigSchema {
        TaskConfigSchema {
            name: "Shell Command".to_string(),
            description: "Execute shell commands".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "command".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: "Command to execute".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "working_dir".to_string(),
                    field_type: "string".to_string(),
                    required: false,
                    description: "Working directory".to_string(),
                    default_value: None,
                    options: None,
                },
            ],
        }
    }

    fn get_type_name(&self) -> &'static str {
        "shell_command"
    }
}
