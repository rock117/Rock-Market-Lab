use rocket::{post, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::stock::filter::security_volatility_service;
use service::stock::filter::security_volatility_service::{VolatilityFilter, SecurityVolatility};
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[post("/api/security/filter/volatility", format = "json", data = "<query>")]
pub async fn filter_by_volumn(query: Json<VolatilityFilter>, conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<SecurityVolatility>>> {
    let conn = conn as &DatabaseConnection;
    let q = VolatilityFilter::new(query.num, query.days, query.sort, query.r#type);
    let datas = security_volatility_service::filter(&q, conn).await?;
    WebResponse::new(datas).into_result()
}
