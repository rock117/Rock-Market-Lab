use std::collections::HashMap;
use anyhow::anyhow;
use chrono::NaiveDate;
use entity::sea_orm::{QueryFilter, QueryOrder, EntityTrait};

use entity::index_monthly;
use entity::sea_orm::{ColumnTrait, DatabaseConnection};

use crate::security::{SecurityMonthly, SecurityType};

pub async fn get_security_monthly_by_years(r#type: SecurityType, ts_code: &str, years: Vec<u32>, conn: &DatabaseConnection) -> anyhow::Result<HashMap<u32, Vec<SecurityMonthly>>> {
    let mut all = HashMap::new();
    for year in years {
        let (start, end) = get_year_begin_end(year)?;
        let start = start.format("%Y%m%d").to_string();
        let end = end.format("%Y%m%d").to_string();
        let datas = match r#type {
            SecurityType::Index => {
                index_monthly::Entity::find()
                    .filter(index_monthly::Column::TsCode.eq(ts_code))
                    .filter(index_monthly::Column::TradeDate.gte(&start))
                    .filter(index_monthly::Column::TradeDate.lte(&end))
                    .order_by_desc(index_monthly::Column::TradeDate)
                    .all(conn).await?.into_iter().map(|d| SecurityMonthly::from_index_monthly(d)).collect()
            }
            SecurityType::Stock => unimplemented!(),
            SecurityType::Fund => unimplemented!()
        };
        all.insert(year, datas);
    }
    Ok(all)
}

fn get_year_begin_end(year: u32) -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let start = NaiveDate::from_ymd_opt(year as i32, 1, 1).ok_or(anyhow!("invalid year"))?;
    let end = NaiveDate::from_ymd_opt(year as i32, 12, 31).ok_or(anyhow!("invalid year"))?;
    Ok((start, end))
}