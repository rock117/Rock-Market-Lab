use anyhow::anyhow;
use serde::Serialize;
use entity::sea_orm::DatabaseConnection;
use entity::{fund, index, stock};
use crate::stock::get_stock_list;

use entity::sea_orm::EntityTrait;
use crate::security::Security;
use crate::security::SecurityType;

pub async fn search_securities(keyword: &str, conn: &DatabaseConnection) -> anyhow::Result<Vec<Security>> {
    let keyword_own = keyword.to_lowercase();
    let keyword = keyword_own.as_str();
    let stocks: Vec<stock::Model> = get_stock_list(conn).await?;
    let stocks: Vec<stock::Model> = stocks
        .into_iter()
        .filter(|s| s.name_py.as_ref().map(|v| v.to_lowercase().contains(keyword)).unwrap_or(false) || s.ts_code.contains(keyword) || s.name.as_ref().map(|name| name.to_lowercase().contains(keyword)).unwrap_or(false))
        .collect();
    let stocks: Vec<Security> = stocks.into_iter().map(|s| Security { ts_code: s.ts_code.clone(), name: s.name.clone(), r#type: SecurityType::Stock }).collect();

    let indexes: Vec<index::Model> = index::Entity::find().all(conn).await.map_err(|err| anyhow!("get index list failed, error: {:?}", err))?;
    let indexes: Vec<index::Model> = indexes
        .into_iter()
        .filter(|s| s.name_py.as_ref().map(|v| v.to_lowercase().contains(keyword)).unwrap_or(false) || s.ts_code.contains(keyword) || s.name.as_ref().map(|name| name.to_lowercase().contains(keyword)).unwrap_or(false))
        .collect();
    let indexes: Vec<Security> = indexes.into_iter().map(|s| Security { ts_code: s.ts_code.clone(), name: s.name.clone(), r#type: SecurityType::Index }).collect();

    let funds: Vec<fund::Model> = fund::Entity::find().all(conn).await.map_err(|err| anyhow!("get fund list failed, error: {:?}", err))?;
    let funds: Vec<fund::Model> = funds
        .into_iter()
        .filter(|s| s.name_py.as_ref().map(|v| v.to_lowercase().contains(keyword)).unwrap_or(false) || s.ts_code.contains(keyword) || s.name.as_ref().map(|name| name.to_lowercase().contains(keyword)).unwrap_or(false))
        .collect();
    let funds: Vec<Security> = funds.into_iter().map(|s| Security { ts_code: s.ts_code.clone(), name: s.name.clone(), r#type: SecurityType::Fund }).collect();

    let mut all = vec![];
    all.extend(take(stocks, 100));
    all.extend(take(indexes, 100));
    all.extend(take(funds, 100));
    Ok(all)
}

fn take(datas: Vec<Security>, n: usize) -> Vec<Security> {
    datas[0..n.min(datas.len())].into_iter().map(|v| v.clone()).collect::<Vec<Security>>()
}