use chrono::NaiveDate;

use entity::{fund_daily, index_daily, stock_daily};
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

use crate::security::{SecurityPrice, SecurityType};

pub async fn get_security_daily(r#type: SecurityType, ts_code: &str, start: &NaiveDate, end: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityPrice>> {
    let start = start.format("%Y%m%d").to_string();
    let end = end.format("%Y%m%d").to_string();
    let datas = match r#type {
        SecurityType::Stock => {
            stock_daily::Entity::find()
                .filter(stock_daily::Column::TsCode.eq(ts_code))
                .filter(stock_daily::Column::TradeDate.gte(&start))
                .filter(stock_daily::Column::TradeDate.lte(&end))
                .order_by_desc(stock_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_stock_daily(d)).collect()
        }
        SecurityType::Index => {
            index_daily::Entity::find()
                .filter(index_daily::Column::TsCode.eq(ts_code))
                .filter(index_daily::Column::TradeDate.gte(&start))
                .filter(index_daily::Column::TradeDate.lte(&end))
                .order_by_desc(index_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_daily(d)).collect()
        }
        SecurityType::Fund => {
            fund_daily::Entity::find()
                .filter(fund_daily::Column::TsCode.eq(ts_code))
                .filter(fund_daily::Column::TradeDate.gte(&start))
                .filter(fund_daily::Column::TradeDate.lte(&end))
                .order_by_desc(fund_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_fund_daily(d)).collect()
        }
    };
    Ok(datas)
}