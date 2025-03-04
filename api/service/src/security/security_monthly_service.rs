use anyhow::anyhow;
use chrono::NaiveDate;

use entity::{index_monthly, stock_daily};
use entity::sea_orm::{ColumnTrait, DatabaseConnection};

use crate::security::{SecurityDaily, SecurityType};

pub async fn get_security_monthly(r#type: SecurityType, ts_code: &str, start: &NaiveDate, end: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<Vec<stock_daily::Model>> {
    // let start = start.format("%Y%m%d").to_string();
    // let end = end.format("%Y%m%d").to_string();
    // match r#type {
    //     SecurityType::Index => {
    //         index_monthly::Entity::find()
    //             .filter(stock_daily::Column::TsCode.eq(ts_code))
    //
    //             .filter(stock_daily::Column::TradeDate.gte(&start))
    //             .filter(stock_daily::Column::TradeDate.lte(&end))
    //             .order_by_desc(stock_daily::Column::TradeDate)
    //             .all(conn).await?.into_iter().map(|d| SecurityDaily::from_index_daily(d)).collect()
    //     }
    //     SecurityType::Stock => unimplemented!(),
    //     SecurityType::Fund => unimplemented!()
    // }
    todo!()
}

pub async fn get_security_monthly_by_years(r#type: SecurityType, years: Vec<u32>, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityDaily>> {

    todo!()
}

fn get_year_begin_end(year: u32) -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let start = NaiveDate::from_ymd_opt(year as i32, 1, 1).ok_or(anyhow!("invalid year"))?;
    let end = NaiveDate::from_ymd_opt(year as i32, 12, 31).ok_or(anyhow!("invalid year"))?;
    Ok((start, end))
}