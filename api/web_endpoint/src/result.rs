use crate::domain::WebResult;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::json::to_json;
use http::StatusCode;

pub type Result<T> = std::result::Result<Json<WebResult<T>>, AppError>;

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("error occur in AppError: {:?}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            to_json(&WebResult::<String>::new(
                false,
                None,
                Some(self.0.to_string()),
            ))
                .unwrap_or("Failed to serized".to_string()),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
