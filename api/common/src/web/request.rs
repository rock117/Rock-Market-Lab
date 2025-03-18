use rocket::FromForm;
use rocket::serde::Deserialize;
use serde::Deserializer;
use crate::data_type::AllSingle;

#[derive(Debug, Deserialize, FromForm)]
pub struct StockQueryParams {
    pub page: usize,
    pub page_size: usize,
    pub order_by: String,
    pub order: String, // prop: 'pct_chg', order: 'ascending' descending
    #[serde(deserialize_with = "deserialize_all_single")]
    pub area: AllSingle<String>,
    #[serde(deserialize_with = "deserialize_all_single")]
    pub industry: AllSingle<String>,
}


// Custom deserialization for Area
fn deserialize_all_single<'de, D>(deserializer: D) -> std::result::Result<AllSingle<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    if s == "All" {
        Ok(AllSingle::All)
    } else {
        Ok(AllSingle::Single(s))
    }
}

// Implement FromFormField for Area to support query parameters
impl<'v> rocket::form::FromFormField<'v> for AllSingle<String> {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        let value = field.value;
        if value == "All" {
            Ok(AllSingle::All)
        } else {
            Ok(AllSingle::Single(value.to_string()))
        }
    }
}
