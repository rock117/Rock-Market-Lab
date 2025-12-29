use chrono::NaiveDate;
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Condition};
use entity::stock_daily;
use futures::stream::{StreamExt, TryStreamExt};
use std::collections::HashMap;

pub async fn get_stock_prices(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<Vec<stock_daily::Model>> {
    let start = start_date.format(common::date::FORMAT).to_string();
    let end = end_date.format(common::date::FORMAT).to_string();
    let stock_prices: Vec<stock_daily::Model> = stock_daily::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily::Column::TsCode, ts_code))
        .filter(stock_daily::Column::TradeDate.gte(&start))
        .filter(stock_daily::Column::TradeDate.lte(&end))
        .order_by_desc(stock_daily::Column::TradeDate)
        .all(conn)
        .await?;
    Ok(stock_prices)
}

/// 批量查询多支股票的价格数据
/// 
/// 使用单次查询获取多支股票的价格数据，减少数据库连接次数
/// 
/// # Arguments
/// * `ts_codes` - 股票代码列表
/// * `start_date` - 开始日期
/// * `end_date` - 结束日期
/// * `conn` - 数据库连接
/// 批量查询的分片大小，避免 IN 子句元素过多
const BATCH_SIZE: usize = 500;

pub async fn get_stock_prices_batch(ts_codes: &[String], start_date: &NaiveDate, end_date: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<HashMap<String, Vec<stock_daily::Model>>> {

    let start = start_date.format(common::date::FORMAT).to_string();
    let end = end_date.format(common::date::FORMAT).to_string();

    // 将股票代码列表分批处理
    let chunks: Vec<Vec<String>> = ts_codes.chunks(BATCH_SIZE).map(|c| c.to_vec()).collect();
    
    // 使用并发流处理每个分片
    let all_prices = futures::stream::iter(chunks)
        .map(|chunk| {
            let conn = conn.clone();
            let start = start.clone();
            let end = end.clone();
            
            async move {
                // 创建当前批次的 IN 查询条件
                let ts_code_condition = chunk.iter()
                    .map(|code| ColumnTrait::eq(&stock_daily::Column::TsCode, code.as_str()))
                    .fold(Condition::any(), |acc, condition| acc.add(condition));

                let batch_prices: Vec<stock_daily::Model> = stock_daily::Entity::find()
                    .filter(ts_code_condition)
                    .filter(stock_daily::Column::TradeDate.gte(&start))
                    .filter(stock_daily::Column::TradeDate.lte(&end))
                    .order_by_asc(stock_daily::Column::TsCode)
                    .order_by_desc(stock_daily::Column::TradeDate)
                    .all(&conn)
                    .await?;

                Ok::<_, anyhow::Error>(batch_prices)
            }
        })
        .buffer_unordered(4) // 限制并发查询数量，避免数据库负载过大
        .try_fold(Vec::new(), |mut acc, batch| async move {
            acc.extend(batch);
            Ok(acc)
        })
        .await?;

    // 按 ts_code 分组
    let mut grouped_prices: HashMap<String, Vec<stock_daily::Model>> = HashMap::new();
    for price in all_prices {
        grouped_prices.entry(price.ts_code.clone()).or_insert_with(Vec::new).push(price);
    }

    Ok(grouped_prices)
}