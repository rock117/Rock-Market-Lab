use rocket::{get, State};
use rocket::serde::json::Json;
use rocket::http::Status;

use serde_derive::Serialize;
use tracing::info;
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use service::stock::stock_bias_ratio_service;
use service::stock::stock_bias_ratio_service::StockBiasRatio;


#[get("/api/stocks/bias_ratio?<ts_code>&<price>")]
pub async fn get_bias_ratio(ts_code: &str, price: Option<f64>, conn: &State<DatabaseConnection>) -> Result<Json<WebResponse<StockBiasRatio>>, Json<WebResponse<String>>>  {
    info!("get_bias_ratio: => ts_code = {ts_code}, price = {price:?}");
    let conn = conn as &DatabaseConnection;
    let data = stock_bias_ratio_service::get_bias_ratio(ts_code, price, &conn).await;
    match data {
        Ok(data) => {
            let res = Json(WebResponse::new(data));
            Ok(res)
        },
        Err(e) => {
            Err(Json(WebResponse::failed(e.to_string())))
        }
    }
}
