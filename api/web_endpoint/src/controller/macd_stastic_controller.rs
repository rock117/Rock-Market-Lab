use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use service::stastic::{macd_stastic_service, macd_stastic_service::MacdStastics};
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
/// TODO 添加行业或基金分类参数
#[get("/api/stocks/macd_stastic")]
pub async fn macd_stastic(conn: &State<DatabaseConnection>) -> Result<WebResponse<MacdStastics>> {
    let data = macd_stastic_service::macd_stastic(conn).await?;
    WebResponse::new(data).into_result()
}