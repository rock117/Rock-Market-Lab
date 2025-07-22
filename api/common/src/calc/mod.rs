use std::iter::Sum;
use std::slice::Iter;
use chrono::{Datelike, NaiveDate};
use itertools::Itertools;

mod volatility;
pub use volatility::*;

#[derive(Debug, Clone)]
pub struct Vol {
    pub vol: f64,
    pub date: NaiveDate,
}

///
/// # Arguments
/// - `cols` 成交量, 日期按逆序排序
pub fn calc(date_period: &str, vols: &Vec<Vol>) {
    let total_vol: f64 = vols.iter().map(|v| v.vol).sum();
    let avg_vol = total_vol / vols.len() as f64;
    let m_groups = vols.iter().group_by(|vol| vol.date.month());
    // group_by_week(vols, m_groups);
}

fn group_by_week(vols: &Vec<Vol>) -> Vec<(u32, f64)> {
    let m_groups = vols.iter().group_by(|vol| vol.date.month());
    let w_groups = vols.iter().group_by(|vol| vol.date.iso_week().week());
    let mut datas = vec![];
    for (week, data) in &w_groups {
        let sum = data.collect_vec().iter().map(|v| v.vol).sum::<f64>();
        datas.push((week, sum));
    }
    datas.sort_by(|v1, v2| v2.0.cmp(&v1.0));
    datas
}