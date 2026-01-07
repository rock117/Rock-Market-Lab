use anyhow::{Context, Result};

use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use entity::sea_orm::sea_query::Expr;
use entity::{dc_index, dc_member};

pub async fn list_dc_index_latest(conn: &DatabaseConnection) -> Result<Vec<dc_index::Model>> {
    let pairs: Vec<(String, String)> = dc_index::Entity::find()
        .select_only()
        .column(dc_index::Column::TsCode)
        .column_as(Expr::col(dc_index::Column::TradeDate).max(), "max_trade_date")
        .group_by(dc_index::Column::TsCode)
        .order_by_asc(dc_index::Column::TsCode)
        .into_tuple::<(String, String)>()
        .all(conn)
        .await
        .context("Failed to query latest trade_date per dc_index.ts_code")?;

    let mut out = Vec::with_capacity(pairs.len());
    for (ts_code, trade_date) in pairs {
        if let Some(row) = dc_index::Entity::find_by_id((ts_code.clone(), trade_date.clone()))
            .one(conn)
            .await
            .context("Failed to fetch dc_index by id")?
        {
            out.push(row);
        }
    }

    out.sort_by(|a, b| a.ts_code.cmp(&b.ts_code));
    Ok(out)
}

pub async fn list_dc_members_by_concept(
    conn: &DatabaseConnection,
    ts_code: &str,
    trade_date: &str,
) -> Result<Vec<dc_member::Model>> {
    let rows = dc_member::Entity::find()
        .filter(ColumnTrait::eq(&dc_member::Column::TsCode, ts_code.to_string()))
        .filter(ColumnTrait::eq(&dc_member::Column::TradeDate, trade_date.to_string()))
        .order_by_asc(dc_member::Column::ConCode)
        .all(conn)
        .await
        .context("Failed to fetch dc_member rows")?;

    Ok(rows)
}
