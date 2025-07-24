use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Debug, Clone, Default)]
pub struct WebResult<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl<T> WebResult<T> {
    pub fn new(success: bool, data: Option<T>, msg: Option<String>) -> Self {
        WebResult { success, data, msg }
    }
}

