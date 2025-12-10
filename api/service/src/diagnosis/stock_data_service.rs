use anyhow::Result;
use chrono::NaiveDate;
use entity::{stock_daily_basic};
use entity::sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

/// 批量获取股票基本面数据
pub async fn get_stock_daily_basic_batch(
    ts_code: &str, 
    start_date: &NaiveDate, 
    end_date: &NaiveDate, 
    conn: &DatabaseConnection
) -> Result<Vec<stock_daily_basic::Model>> {
    let start = start_date.format("%Y%m%d").to_string();
    let end = end_date.format("%Y%m%d").to_string();
    
    let data = stock_daily_basic::Entity::find()
        .filter(stock_daily_basic::Column::TsCode.eq(ts_code))
        .filter(stock_daily_basic::Column::TradeDate.gte(&start))
        .filter(stock_daily_basic::Column::TradeDate.lte(&end))
        .order_by_desc(stock_daily_basic::Column::TradeDate)
        .all(conn)
        .await?;
        
    Ok(data)
}
