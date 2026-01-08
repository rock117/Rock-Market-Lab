use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{Days, Local, NaiveDate};
use entity::prelude::TradeCalendar;
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection};
use entity::trade_calendar;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::QueryFilter;

pub mod fetch_stock_list_task;
pub mod fetch_stock_daily_task;
pub mod fetch_trade_calendar_task;
pub mod fetch_stock_holder_number_task;
pub mod fetch_income_task;
pub mod fetch_cashflow_task;
pub mod fetch_finance_indicator_task;
pub mod fetch_balancesheet_task;
pub mod fetch_fund_task;
pub mod fetch_index_task;
pub mod fetch_index_daily_task;
pub mod fetch_moneyflow_task;
pub mod fetch_index_weekly_task;
pub mod fetch_index_monthly_task;
pub mod fetch_stock_daily_basic_task;
pub mod fetch_margin_task;
pub mod fetch_margin_detail_task;
pub mod fetch_stock_monthly_task;
pub mod fetch_fund_daily_task;
pub mod fetch_ths_index_task;
pub mod fetch_ths_member_task;
pub mod fetch_ths_daily_task;
pub(crate) mod fetch_etf_task;
pub mod fetch_fund_portfolio_task;
pub mod fetch_stk_holdertrade_task;
pub mod fetch_dc_index_task;
pub mod fetch_dc_member_task;
pub mod us;
pub mod fetch_block_trade_task;
pub mod fetch_basic_org_info_task;
pub mod fetch_eng_translate_task;
mod fetch_hm_detail_task;

#[async_trait]
pub trait Task: Send + Sync {
    fn get_schedule(&self) -> String;
    async fn run(&self) -> anyhow::Result<()>;
}

fn get_start_end_date_from_default() -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let today = Local::now();
    let start = Local::now().checked_sub_days(Days::new(10)).ok_or(anyhow!("failed to sub days"))?;
    Ok((start.date_naive(), today.date_naive()))
}

fn get_start_end_date_from_now(days_num_before_today: u64) -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let today = Local::now();
    let start = Local::now().checked_sub_days(Days::new(days_num_before_today)).ok_or(anyhow!("failed to sub days"))?;
    Ok((start.date_naive(), today.date_naive()))
}

fn get_start_end_date(days_num_before_today: u64) -> anyhow::Result<(String, String)> {
    let today = Local::now().format("%Y%m%d").to_string();
    let start = Local::now().checked_sub_days(Days::new(days_num_before_today)).ok_or(anyhow!("date is none"))?.format("%Y%m%d").to_string();
    Ok((start, today))
}

async fn get_calendar_dates(days_num_before_today: u64, conn: &DatabaseConnection) -> anyhow::Result<Vec<NaiveDate>> {
    let (start, end) = get_start_end_date(days_num_before_today)?;
    let mut dates: Vec<trade_calendar::Model> = TradeCalendar::find()
        .filter(
            Condition::all()
                .add(trade_calendar::Column::CalDate.lte(end))
                .add(trade_calendar::Column::CalDate.gte(start))
                .add(ColumnTrait::eq(&trade_calendar::Column::IsOpen, 1))
        )
        .order_by_desc(trade_calendar::Column::CalDate)
        .all(conn)
        .await?;
    let dates = dates.iter().map(|v| NaiveDate::parse_from_str(&v.cal_date, "%Y%m%d").unwrap()).collect();
    Ok(dates)
}