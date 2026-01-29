use crate::types::*;
use async_trait::async_trait;
use serde_json::Value;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub struct ShellCommandExecutor;

#[async_trait]
impl TaskExecutor for ShellCommandExecutor {
    async fn execute(&self, config: Value, context: ExecutionContext) -> anyhow::Result<TaskResult> {
        let command = config.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: command"))?;
        
        let working_dir = config.get("working_dir")
            .and_then(|v| v.as_str());

        let shell = config.get("shell")
            .and_then(|v| v.as_str())
            .unwrap_or(if cfg!(windows) { "cmd" } else { "sh" });

        let env_vars = config.get("env")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let capture_output = config.get("capture_output")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        context.logger.info(&format!("Executing command: {}", command)).await;
        if let Some(dir) = working_dir {
            context.logger.info(&format!("Working directory: {}", dir)).await;
        }

        let mut cmd = if cfg!(windows) {
            let mut cmd = Command::new(shell);
            cmd.args(&["/C", command]);
            cmd
        } else {
            let mut cmd = Command::new(shell);
            cmd.args(&["-c", command]);
            cmd
        };

        // Set working directory if provided
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for (key, value) in env_vars {
            if let Some(value_str) = value.as_str() {
                cmd.env(key, value_str);
                context.logger.info(&format!("Setting env var: {}={}", key, value_str)).await;
            }
        }

        // Configure output capture
        if capture_output {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
        } else {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        match cmd.spawn() {
            Ok(mut child) => {
                let mut output_lines = Vec::new();
                let mut error_lines = Vec::new();

                if capture_output {
                    // Capture stdout
                    if let Some(stdout) = child.stdout.take() {
                        let reader = BufReader::new(stdout);
                        let mut lines = reader.lines();
                        
                        tokio::spawn(async move {
                            let mut captured_lines = Vec::new();
                            while let Ok(Some(line)) = lines.next_line().await {
                                captured_lines.push(line);
                            }
                            captured_lines
                        });
                    }

                    // Capture stderr
                    if let Some(stderr) = child.stderr.take() {
                        let reader = BufReader::new(stderr);
                        let mut lines = reader.lines();
                        
                        tokio::spawn(async move {
                            let mut captured_lines = Vec::new();
                            while let Ok(Some(line)) = lines.next_line().await {
                                captured_lines.push(line);
                            }
                            captured_lines
                        });
                    }
                }

                match child.wait().await {
                    Ok(status) => {
                        let exit_code = status.code().unwrap_or(-1);
                        context.logger.info(&format!("Command exited with code: {}", exit_code)).await;

                        if status.success() {
                            let output = if capture_output && !output_lines.is_empty() {
                                output_lines.join("\n")
                            } else {
                                format!("Command completed successfully (exit code: {})", exit_code)
                            };
                            
                            context.logger.info("Command completed successfully").await;
                            Ok(TaskResult::success(Some(output)))
                        } else {
                            let error_output = if capture_output && !error_lines.is_empty() {
                                error_lines.join("\n")
                            } else {
                                format!("Command failed with exit code: {}", exit_code)
                            };
                            
                            context.logger.error(&format!("Command failed: {}", error_output)).await;
                            Ok(TaskResult::failure(error_output))
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to wait for command: {}", e);
                        context.logger.error(&error_msg).await;
                        Ok(TaskResult::failure(error_msg))
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to execute command: {}", e);
                context.logger.error(&error_msg).await;
                Ok(TaskResult::failure(error_msg))
            }
        }
    }

    fn validate_config(&self, config: &Value) -> anyhow::Result<()> {
        if !config.is_object() {
            return Err(anyhow::anyhow!("Config must be an object"));
        }

        let command = config.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required field: command"))?;

        if command.trim().is_empty() {
            return Err(anyhow::anyhow!("Command cannot be empty"));
        }

        // Validate working directory if provided
        if let Some(working_dir) = config.get("working_dir").and_then(|v| v.as_str()) {
            if !std::path::Path::new(working_dir).exists() {
                return Err(anyhow::anyhow!("Working directory does not exist: {}", working_dir));
            }
        }

        // Validate shell if provided
        if let Some(shell) = config.get("shell").and_then(|v| v.as_str()) {
            if shell.trim().is_empty() {
                return Err(anyhow::anyhow!("Shell cannot be empty"));
            }
        }

        // Validate environment variables if provided
        if let Some(env) = config.get("env") {
            if !env.is_object() {
                return Err(anyhow::anyhow!("Environment variables must be an object"));
            }
        }

        // Validate capture_output if provided
        if let Some(capture) = config.get("capture_output") {
            if !capture.is_boolean() {
                return Err(anyhow::anyhow!("capture_output must be a boolean"));
            }
        }

        Ok(())
    }

    fn get_schema(&self) -> TaskConfigSchema {
        TaskConfigSchema {
            name: "Shell Command".to_string(),
            description: "Execute shell commands and scripts".to_string(),
            fields: vec![
                TaskConfigField {
                    name: "command".to_string(),
                    field_type: "text".to_string(),
                    required: true,
                    description: "Command to execute".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "working_dir".to_string(),
                    field_type: "string".to_string(),
                    required: false,
                    description: "Working directory for command execution".to_string(),
                    default_value: None,
                    options: None,
                },
                TaskConfigField {
                    name: "shell".to_string(),
                    field_type: "select".to_string(),
                    required: false,
                    description: "Shell to use for command execution".to_string(),
                    default_value: Some(Value::String(
                        if cfg!(windows) { "cmd".to_string() } else { "sh".to_string() }
                    )),
                    options: Some(if cfg!(windows) {
                        vec!["cmd".to_string(), "powershell".to_string()]
                    } else {
                        vec!["sh".to_string(), "bash".to_string(), "zsh".to_string()]
                    }),
                },
                TaskConfigField {
                    name: "env".to_string(),
                    field_type: "object".to_string(),
                    required: false,
                    description: "Environment variables as key-value pairs".to_string(),
                    default_value: Some(Value::Object(serde_json::Map::new())),
                    options: None,
                },
                TaskConfigField {
                    name: "capture_output".to_string(),
                    field_type: "boolean".to_string(),
                    required: false,
                    description: "Whether to capture command output".to_string(),
                    default_value: Some(Value::Bool(true)),
                    options: None,
                },
            ],
        }
    }

    fn get_type_name(&self) -> &'static str {
        "shell_command"
    }
}
