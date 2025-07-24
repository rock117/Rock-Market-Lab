use rocket::{post, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::stock::filter::security_volatility_service;
use service::stock::filter::security_volatility_service::{VolatilityFilter, SecurityVolatility};
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[post("/api/security/filter/volatility", format = "json", data = "<query>")]
pub async fn filter_by_volatility(query: Json<VolatilityFilter>, conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<SecurityVolatility>>> {
    let conn = conn as &DatabaseConnection;
    let datas = security_volatility_service::filter(&query, conn).await?;
    WebResponse::new(datas).into_result()
}
