use crate::types::*;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

pub struct RustFunctionExecutor;

// Registry for Rust functions that can be executed
type RustFunction = Box<dyn Fn(&Value, &ExecutionContext) -> Result<TaskResult, anyhow::Error> + Send + Sync>;

pub struct FunctionRegistry {
    functions: HashMap<String, RustFunction>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
        // Register built-in functions
        registry.register_builtin_functions();
        registry
    }

    pub fn register_function<F>(&mut self, name: String, func: F)
    where
        F: Fn(&Value, &ExecutionContext) -> Result<TaskResult, anyhow::Error> + Send + Sync + 'static,
    {
        self.functions.insert(name, Box::new(func));
    }

    pub fn get_function(&self, name: &str) -> Option<&RustFunction> {
        self.functions.get(name)
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    fn register_builtin_functions(&mut self) {
        // Example: Data processing function
        self.register_function("data_processing".to_string(), |config, context| {
            let operation = config.get("operation")
                .and_then(|v| v.as_str())
                .unwrap_or("count");
            
            let data = config.get("data")
                .and_then(|v| v.as_array())
                .unwrap_or(&vec![]);

            let result = match operation {
                "count" => {
                    format!("Data count: {}", data.len())
                }
                "sum" => {
                    let sum: f64 = data.iter()
                        .filter_map(|v| v.as_f64())
                        .sum();
                    format!("Data sum: {}", sum)
                }
                "average" => {
                    let numbers: Vec<f64> = data.iter()
                        .filter_map(|v| v.as_f64())
                        .collect();
                    if numbers.is_empty() {
                        "No numeric data found".to_string()
                    } else {
                        let avg = numbers.iter().sum::<f64>() / numbers.len() as f64;
                        format!("Data average: {:.2}", avg)
                    }
                }
                _ => format!("Unknown operation: {}", operation)
            };

            Ok(TaskResult::success(Some(result)))
        });

        // Example: File operations function
        self.register_function("file_operations".to_string(), |config, context| {
            let operation = config.get("operation")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing operation"))?;
            
            let file_path = config.get("file_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;

            match operation {
                "exists" => {
                    let exists = std::path::Path::new(file_path).exists();
                    Ok(TaskResult::success(Some(format!("File exists: {}", exists))))
                }
                "size" => {
                    match std::fs::metadata(file_path) {
                        Ok(metadata) => {
                            let size = metadata.len();
                            Ok(TaskResult::success(Some(format!("File size: {} bytes", size))))
                        }
                        Err(e) => Ok(TaskResult::failure(format!("Failed to get file size: {}", e)))
                    }
                }
                "read" => {
                    match std::fs::read_to_string(file_path) {
                        Ok(content) => {
                            let preview = if content.len() > 1000 {
                                format!("{}... (truncated, total {} chars)", &content[..1000], content.len())
                            } else {
                                content
                            };
                            Ok(TaskResult::success(Some(format!("File content:\n{}", preview))))
                        }
                        Err(e) => Ok(TaskResult::failure(format!("Failed to read file: {}", e)))
                    }
                }
                _ => Ok(TaskResult::failure(format!("Unknown file operation: {}", operation)))
            }
        });

        // Example: System information function
        self.register_function("system_info".to_string(), |config, context| {
            let info_type = config.get("info_type")
                .and_then(|v| v.as_str())
                .unwrap_or("basic");

            let result = match info_type {
                "basic" => {
                    format!("OS: {}, Arch: {}", std::env::consts::OS, std::env::consts::ARCH)
                }
                "env" => {
                    let var_name = config.get("var_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("PATH");
                    
                    match std::env::var(var_name) {
                        Ok(value) => format!("{}={}", var_name, value),
                        Err(_) => format!("Environment variable '{}' not found", var_name)
                    }
                }
                "current_dir" => {
                    match std::env::current_dir() {
                        Ok(dir) => format!("Current directory: {}", dir.display()),
                        Err(e) => format!("Failed to get current directory: {}", e)
                    }
                }
                _ => format!("Unknown info type: {}", info_type)
            };

            Ok(TaskResult::success(Some(result)))
        });

        // Example: Database query function (placeholder)
        self.register_function("database_query".to_string(), |config, context| {
            let query = config.get("query")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing query"))?;

            // This is a placeholder - in a real implementation, you would
            // connect to the database and execute the query
            let result = format!("Would execute query: {}", query);
            Ok(TaskResult::success(Some(result)))
        });
    }
}

// Global function registry (in a real implementation, this might be injected)
lazy_static::lazy_static! {
    static ref FUNCTION_REGISTRY: std::sync::Mutex<FunctionRegistry> = 
        std::sync::Mutex::new(FunctionRegistry::new());
}

#[async_trait]
impl TaskExecutor for RustFunctionExecutor {
    async fn execute(&self, config: Value, context: ExecutionContext) -> anyhow::Result<TaskResult> {
        let function_name = config.get("function")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: function"))?;

        let parameters = config.get("parameters")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));

        context.logger.info(&format!("Executing Rust function: {}", function_name)).await;

        let registry = FUNCTION_REGISTRY.lock().unwrap();
        
        match registry.get_function(function_name) {
            Some(func) => {
                context.logger.info("Function found, executing...").await;
                
                // Execute the function synchronously (since most Rust functions are sync)
                let result = tokio::task::spawn_blocking({
                    let func = func as *const RustFunction;
                    let parameters = parameters.clone();
                    let context = context.clone();
                    
                    move || {
                        // Safety: We know the function pointer is valid within this scope
                        unsafe {
                            (*func)(&parameters, &context)
                        }
                    }
                }).await;

                match result {
                    Ok(Ok(task_result)) => {
                        context.logger.info("Function executed successfully").await;
                        Ok(task_result)
                    }
                    Ok(Err(e)) => {
                        let error_msg = format!("Function execution failed: {}", e);
                        context.logger.error(&error_msg).await;
                        Ok(TaskResult::failure(error_msg))
                    }
                    Err(e) => {
                        let error_msg = format!("Function execution panicked: {}", e);
                        context.logger.error(&error_msg).await;
                        Ok(TaskResult::failure(error_msg))
                    }
                }
            }
            None => {
                let available_functions = registry.list_functions();
                let error_msg = format!(
                    "Function '{}' not found. Available functions: {}", 
                    function_name, 
                    available_functions.join(", ")
                );
                context.logger.error(&error_msg).await;
                Ok(TaskResult::failure(error_msg))
            }
        }
    }

    fn validate_config(&self, config: &Value) -> anyhow::Result<()> {
        if !config.is_object() {
            return Err(anyhow::anyhow!("Config must be an object"));
        }

        let function_name = config.get("function")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: function"))?;

        if function_name.trim().is_empty() {
            return Err(anyhow::anyhow!("Function name cannot be empty"));
        }

        // Check if function exists
        let registry = FUNCTION_REGISTRY.lock().unwrap();
        if registry.get_function(function_name).is_none() {
            let available = registry.list_functions();
            return Err(anyhow::anyhow!(
                "Function '{}' not found. Available: {}", 
                function_name, 
                available.join(", ")
            ));
        }

        // Validate parameters if provided
        if let Some(parameters) = config.get("parameters") {
            if !parameters.is_object() && !parameters.is_null() {
                return Err(anyhow::anyhow!("Parameters must be an object or null"));
            }
        }

        Ok(())
    }

    fn get_schema(&self) -> TaskConfigSchema {
        let registry = FUNCTION_REGISTRY.lock().unwrap();
        let available_functions = registry.list_functions();

        TaskConfigSchema {
            name: "Rust Function".to_string(),
            description: "Execute predefined Rust functions with custom logic".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "function".to_string(),
                    field_type: "select".to_string(),
                    required: true,
                    description: "Name of the Rust function to execute".to_string(),
                    default_value: None,
                    options: Some(available_functions),
                },
                TaskConfigField {
                    name: "parameters".to_string(),
                    field_type: "object".to_string(),
                    required: false,
                    description: "Parameters to pass to the function".to_string(),
                    default_value: Some(Value::Object(serde_json::Map::new())),
                    options: None,
                },
            ],
        }
    }

    fn get_type_name(&self) -> &'static str {
        "rust_function"
    }
}
