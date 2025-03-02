use chrono::NaiveDate;
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::stock_daily;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

pub async fn get_stock_prices(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<Vec<stock_daily::Model>> {
    let start = start_date.format(common::date::FORMAT).to_string();
    let end = end_date.format(common::date::FORMAT).to_string();
    let stock_prices: Vec<stock_daily::Model> = stock_daily::Entity::find()
        .filter(stock_daily::Column::TsCode.eq(ts_code))
        .filter(stock_daily::Column::TradeDate.gte(&start))
        .filter(stock_daily::Column::TradeDate.lte(&end))
        .order_by_desc(stock_daily::Column::TradeDate)
        .all(conn)
        .await?;
    Ok(stock_prices)
}