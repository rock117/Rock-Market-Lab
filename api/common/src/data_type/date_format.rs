use anyhow::anyhow;
use chrono::NaiveDate;

#[derive(Copy, Clone, Debug)]
pub enum DateFormat {
    /// yyyy-MM-dd
    YYYYMMDD_DASH,
    /// yyyyMMdd
    YYYYMMDD,
}

impl DateFormat {
    pub fn format_date(&self, date: &NaiveDate) -> String {
        match self {
            DateFormat::YYYYMMDD_DASH => date.format("%Y-%m-%d").to_string(),
            DateFormat::YYYYMMDD => date.format("%Y%m%d").to_string(),
        }
    }
    pub fn parse_date(&self, date: &str) -> anyhow::Result<NaiveDate> {
        match self {
            DateFormat::YYYYMMDD_DASH => {
                NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|e| anyhow!(e))
            }
            DateFormat::YYYYMMDD => {
                NaiveDate::parse_from_str(date, "%Y%m%d").map_err(|e| anyhow!(e))
            }
        }
    }
}
