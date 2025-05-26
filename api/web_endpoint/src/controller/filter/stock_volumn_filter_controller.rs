use std::collections::HashMap;
use rocket::{post, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::security::{security_history_compare_service, SecurityPrice, Year};
use service::stock::filter::stock_volumn_filter_service;
use service::stock::filter::stock_volumn_filter_service::*;
use crate::response::WebResponse;
use crate::result::IntoResult;

#[post("/api/stocks/filter/volumn", format = "json", data = "<query>")]
pub async fn filter_by_volumn(query: Json<VolumnFilter>, conn: &State<DatabaseConnection>) -> crate::result::Result<WebResponse<VolumnFilterResult>> {
    let conn = conn as &DatabaseConnection;
    let filter = VolumnFilter { days: query.days, rate: query.rate };
    let datas = stock_volumn_filter_service::filter(&filter, &conn).await?;
    WebResponse::new(datas).into_result()
}

