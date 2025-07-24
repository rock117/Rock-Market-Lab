use rocket::{post};

pub struct Query {
    pub pct_chg: f64,
    pub start: String,
    pub end: String,
}


#[post("/api/stocks/filter/price")]
pub fn filter_stocks() {

}