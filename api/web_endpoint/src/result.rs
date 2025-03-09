use rocket::http::{ContentType, Status};
use rocket::{Request, Response, response};
use rocket::response::Responder;
use rocket::serde::json::Json;
use crate::response::WebResponse;

pub struct Error(anyhow::Error);
pub type Result<T> = std::result::Result<Json<T>, Error>;

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error(e)
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let msg = Json(WebResponse::failed(self.0.to_string()));
        Response::build_from(msg. respond_to(req)?)
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

pub trait IntoResult<T> {
    fn into_result(self) -> Result<T>;
}

impl<T> IntoResult<T> for T {
    fn into_result(self) -> Result<T> {
        Ok(Json(self))
    }
}




