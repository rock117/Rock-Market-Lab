use axum::extract::{Query, State};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use entity::sea_orm::DatabaseConnection;
use crate::domain::ToAppResult;

#[derive(Deserialize, Debug)]
pub struct MarginQuery {
    date_range: String,
    statics_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarginDetailQuery {
    pub trade_date: String,
    pub ts_code: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct MarginDetail;

pub async fn margin_detail(
    margin_detail: Query<MarginDetailQuery>,
    State(conn): State<DatabaseConnection>
) -> crate::result::Result<Vec<MarginDetail>> {
    // let trade_date =
    //     NaiveDate::parse_from_str(&margin_detail.trade_date, common::constant::DATE_YMD)?;
    // let start_date =
    //     NaiveDate::parse_from_str(&margin_detail.trade_date, common::constant::DATE_YMD)?;
    // let end_date =
    //     NaiveDate::parse_from_str(&margin_detail.trade_date, common::constant::DATE_YMD)?;
    // service::margin::margin_details(&trade_date, &margin_detail.ts_code, &start_date, &end_date)
    //     .await
    //     .to_app_result()
    todo!()
}
