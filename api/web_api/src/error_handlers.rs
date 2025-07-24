use rocket::{catch, Request};
use rocket::serde::json::{Json};
use tracing::error;
use crate::response::WebResponse;

#[catch(500)]
pub fn internal_error(req: &Request) -> Json<WebResponse<String>> {
    let msg = format!("Internal server error: {:?}", req);
    error!(msg);
    Json(WebResponse::failed(msg))
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<WebResponse<String>> {
    let msg = format!("Resource not found: {:?}", req);
    error!(msg);
    Json(WebResponse::failed(msg))
}