mod date_format;
mod date_range;
mod num_or_string;
mod range;
pub mod period;
mod all_single;

pub use self::date_range::{DateRange, StartEnd};
pub use self::num_or_string::NumOrString;
use crate::ToAnyHowResult;
use anyhow::anyhow;
pub use date_format::DateFormat;
use serde::{Deserialize, Serialize};
pub use all_single::AllSingle;

/// such as 600051.SH
pub type TsCode = String;
pub use range::Range;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Default)]
pub enum DateType {
    #[default]
    Custom,
    Days5,
    Days10,
    Days20,
    Days60,
    Days120,
    Days250,
    Days(u64),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum InvestmentType {
    Stock,
    Index,
}

pub trait SingleElement<T> {
    fn take(self) -> anyhow::Result<T>;
}
impl<T> SingleElement<T> for Option<T> {
    fn take(self) -> anyhow::Result<T> {
        self.to_result()
    }
}
impl<T: Clone> SingleElement<T> for Vec<T> {
    fn take(self) -> anyhow::Result<T> {
        if self.len() != 1 {
            Err(anyhow!("not single size"))
        } else {
            self.first().cloned().to_result()
        }
    }
}
