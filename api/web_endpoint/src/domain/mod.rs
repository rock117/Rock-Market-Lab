use axum::Json;

use crate::result::AppError;
use serde::Serialize;
use std::fmt::Debug;

pub type Result<T> = std::result::Result<Json<WebResult<T>>, AppError>;

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

pub trait ToAppResult<T> {
    fn to_app_result(self) -> Result<T>;
}
impl<T> ToAppResult<T> for anyhow::Result<T> {
    fn to_app_result(self) -> Result<T> {
        match self {
            Ok(data) => Ok(Json(WebResult::new(true, Some(data), None))),
            Err(err) => Err(AppError(err)),
        }
    }
}
