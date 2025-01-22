use crate::ToAnyHowResult;
use anyhow::anyhow;

pub mod date_util;
pub mod html_util;
pub mod math_util;
pub mod pdf_util;
pub mod compress_util;
pub mod csv_util;
mod rate_limit;

pub fn to_result<T>(option: Option<T>) -> anyhow::Result<T> {
    option.to_result()
}

pub fn to_result_with<T>(option: Option<T>, msg: &'static str) -> anyhow::Result<T> {
    option.ok_or(anyhow!(msg))
}

pub fn contains(word: &str, word_opt: &Option<String>) -> bool {
    match word_opt {
        None => false,
        Some(e) => e.contains(word),
    }
}

/// 600000.SH -> 600000
pub fn get_symbol_by_tscode(tscode: &str) -> String {
    tscode.split(".").next().unwrap_or("").to_string()
}
