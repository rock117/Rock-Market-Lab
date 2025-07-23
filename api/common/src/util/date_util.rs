use anyhow::{anyhow, bail};
use chrono::{DateTime, Datelike, IsoWeek, Local, NaiveDate, NaiveDateTime, Days};
use itertools::Itertools;

pub fn group_days_by_year(dates: Vec<&'static str>) -> Vec<(i32, Vec<String>)> {
    let dates: Vec<NaiveDate> = to_naivedate(dates);
    let groups = dates.iter().group_by(|day| day.year());
    let mut result = vec![];
    for (key, value) in groups.into_iter() {
        let dates: Vec<String> = value
            .into_iter()
            .map(|i| i.clone().format("%Y%m%d").to_string())
            .collect();
        result.push((key, dates));
    }
    result.sort_by_key(|e| e.0.clone());
    result
}

pub fn group_days_by_month(dates: Vec<&'static str>) -> Vec<(String, Vec<String>)> {
    let dates: Vec<NaiveDate> = to_naivedate(dates);
    let groups = dates
        .iter()
        .group_by(|day| format!("{}-{}", day.year(), day.month()));
    let mut result = vec![];
    for (key, value) in groups.into_iter() {
        let dates: Vec<String> = value
            .into_iter()
            .map(|i| i.clone().format("%Y%m%d").to_string())
            .collect();
        result.push((key, dates));
    }
    result.sort_by_key(|e| e.0.clone());
    result
}

pub fn group_days_by_week(dates: Vec<&'static str>) -> Vec<(IsoWeek, Vec<String>)> {
    let dates: Vec<NaiveDate> = to_naivedate(dates);
    let groups = dates.iter().group_by(|day| day.iso_week());
    let mut result = vec![];
    for (key, value) in groups.into_iter() {
        let dates: Vec<String> = value
            .into_iter()
            .map(|i| i.clone().format("%Y%m%d").to_string())
            .collect();
        result.push((key, dates));
    }
    result.sort_by_key(|e| e.0);
    result
}

fn to_naivedate(dates: Vec<&'static str>) -> Vec<NaiveDate> {
    let mut dates: Vec<NaiveDate> = dates
        .into_iter()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap())
        .collect();
    dates.sort();
    dates
}

pub fn now() -> Option<NaiveDate> {
    let now = Local::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
}

pub fn format_date(timestamp: u64, format: &str) -> anyhow::Result<String> {
    let time = DateTime::from_timestamp_millis(timestamp as i64);
    let Some(time) = time else {
        bail!("can't parse to datetime for {}", timestamp)
    };
    Ok(time.format(format).to_string())
}


/// 获取当前时间和n天前的日期
pub fn get_start_end_from_now(days_ago: u64) -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let end = Local::now().date_naive();
    let start = end.checked_sub_days(Days::new(days_ago)).ok_or(anyhow!("can't get start date"))?;
    Ok((start, end))
}
