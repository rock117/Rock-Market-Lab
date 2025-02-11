use async_trait::async_trait;
use entity::sea_orm::ActiveModelTrait;

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
pub mod fetch_margin_trading_summary_task;
pub mod fetch_stock_margin_detail_task;
pub mod fetch_index_weekly_task;
pub mod fetch_index_monthly_task;

#[async_trait]
pub trait Task: Send + Sync {
    fn get_schedule(&self) -> String;
    async fn run(&self) -> anyhow::Result<()>;
}
