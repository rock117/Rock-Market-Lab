use crate::data_type::DateRange::{Custom, Month, Week, Year};
use crate::ToAnyHowResult;

use chrono::{Days, Local, Months, NaiveDate};

#[derive(Debug, Clone)]
pub enum DateRange {
    Week(usize),
    Month(usize),
    Year(usize),
    Custom(StartEnd),
}

#[derive(Debug, Clone)]
pub struct StartEnd {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl DateRange {
    pub fn to_start_end(&self) -> anyhow::Result<StartEnd> {
        let start_end = match self {
            Week(n) => {
                let end = Local::now().naive_local().date();
                let start = end
                    .checked_sub_days(Days::new((*n * 7) as u64))
                    .to_result()?;
                StartEnd { start, end }
            }
            Month(n) => {
                let end = Local::now().naive_local().date();
                let start = end.checked_sub_months(Months::new(*n as u32)).to_result()?;
                StartEnd { start, end }
            }
            Year(n) => {
                let end = Local::now().naive_local().date();
                let start = end
                    .checked_sub_days(Days::new((*n * 365) as u64))
                    .to_result()?;
                StartEnd { start, end }
            }
            Custom(start_end) => start_end.clone(),
        };
        Ok(start_end)
    }
}
