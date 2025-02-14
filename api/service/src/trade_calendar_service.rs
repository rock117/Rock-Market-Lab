use entity::{stock_daily, trade_calendar};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::PaginatorTrait;

use chrono::{Local, NaiveDate};

/// 获取过去 day_num 个交易日
pub async fn get_trade_calendar(day_num: u64, conn: &DatabaseConnection) -> anyhow::Result<Vec<trade_calendar::Model>> {
    let now = Local::now().date_naive().format("%Y%m%d").to_string();
    let dates: Vec<trade_calendar::Model> = trade_calendar::Entity::find()
        .filter(trade_calendar::Column::CalDate.lte(&now))
        .filter(trade_calendar::Column::IsOpen.eq(1))
        .order_by_desc(stock_daily::Column::TradeDate)
        .paginate(conn, day_num)
        .fetch_page(0)
        .await?;
    Ok(dates)
}