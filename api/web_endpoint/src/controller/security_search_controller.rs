use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use service::security::Security;
use service::security::security_search_service;

#[get("/api/securities/search?<keyword>")]
pub async fn search_securities(keyword: &str,  conn: &State<DatabaseConnection>) -> Json<WebResponse<Vec<Security>>> {
    let conn = conn as &DatabaseConnection;
    let stocks = security_search_service::search_securities(keyword, &conn).await.unwrap();
    Json(WebResponse::new(stocks))
}
