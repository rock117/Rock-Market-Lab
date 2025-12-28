use entity::{etf, fund_portfolio};
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

pub async fn get_etf_list(conn: &DatabaseConnection) -> anyhow::Result<Vec<etf::Model>> {
    let items = etf::Entity::find()
        .order_by_asc(etf::Column::TsCode)
        .all(conn)
        .await?;
    Ok(items)
}

pub async fn get_etf_holdings_latest(
    conn: &DatabaseConnection,
    ts_code: &str,
) -> anyhow::Result<Vec<fund_portfolio::Model>> {
    let latest = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .order_by_desc(fund_portfolio::Column::EndDate)
        .one(conn)
        .await?;

    let Some(latest) = latest else {
        return Ok(vec![]);
    };

    let items = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .filter(ColumnTrait::eq(
            &fund_portfolio::Column::EndDate,
            latest.end_date.clone(),
        ))
        .order_by_desc(fund_portfolio::Column::Mkv)
        .all(conn)
        .await?;

    Ok(items)
}
