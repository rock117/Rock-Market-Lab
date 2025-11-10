use chrono::{Local, Months, NaiveDate};
use rocket::{post, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::stock::filter::stock_volumn_filter_service;
use service::stock::filter::stock_volumn_filter_service::{VolumnFilter, VolumnFilterResult};
use crate::response::WebResponse;
use service::strategy::traits::StrategySignal;
use service::stock_picker_service::*;
use crate::result::IntoResult;


#[post("/api/stocks/pick")]
pub async fn pick(conn: &State<DatabaseConnection>) -> crate::result::Result<WebResponse<Vec<StockPickResult>>> {
    let conn = conn as &DatabaseConnection;

    let picker_service = StockPickerService::new(conn.clone());
    let end = Local::now().date_naive();
    let start = end.checked_sub_months(Months::new(3)).unwrap();
    let datas = picker_service.pick_stocks2(&start, &end, None).await?;
    WebResponse::new(datas).into_result()
}

