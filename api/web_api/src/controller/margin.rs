use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use entity::sea_orm::DatabaseConnection;

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

