use anyhow::Result;
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use entity::{finance_main_business, stock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinanceMainBusinessQueryParams {
    pub r#type: String,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceMainBusinessItem {
    pub ts_code: String,
    pub stock_name: Option<String>,
    pub end_date: String,
    pub bz_item: String,
    pub bz_sales: Option<String>,
    pub bz_profit: Option<String>,
    pub bz_cost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceMainBusinessListResponse {
    pub data: Vec<FinanceMainBusinessItem>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

pub async fn get_finance_main_business_list(
    params: &FinanceMainBusinessQueryParams,
    conn: &DatabaseConnection,
) -> Result<FinanceMainBusinessListResponse> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut base_query = finance_main_business::Entity::find();

    base_query = base_query.filter(ColumnTrait::eq(
        &finance_main_business::Column::Type,
        params.r#type.trim(),
    ));

    let sort_by = params.sort_by.as_deref().unwrap_or("end_date");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");
    let desc = sort_dir.eq_ignore_ascii_case("desc");

    base_query = match sort_by {
        "end_date" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::EndDate)
            } else {
                base_query.order_by_asc(finance_main_business::Column::EndDate)
            }
        }
        "ts_code" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::TsCode)
            } else {
                base_query.order_by_asc(finance_main_business::Column::TsCode)
            }
        }
        "bz_item" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::BzItem)
            } else {
                base_query.order_by_asc(finance_main_business::Column::BzItem)
            }
        }
        "bz_sales" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::BzSales)
            } else {
                base_query.order_by_asc(finance_main_business::Column::BzSales)
            }
        }
        "bz_profit" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::BzProfit)
            } else {
                base_query.order_by_asc(finance_main_business::Column::BzProfit)
            }
        }
        "bz_cost" => {
            if desc {
                base_query.order_by_desc(finance_main_business::Column::BzCost)
            } else {
                base_query.order_by_asc(finance_main_business::Column::BzCost)
            }
        }
        _ => base_query.order_by_desc(finance_main_business::Column::EndDate),
    };

    let total = base_query.clone().count(conn).await?;

    let rows = base_query
        .offset(offset)
        .limit(page_size)
        .all(conn)
        .await?;

    let ts_codes: Vec<String> = rows.iter().map(|r| r.ts_code.clone()).collect();
    let mut name_map: std::collections::HashMap<String, Option<String>> = std::collections::HashMap::new();
    if !ts_codes.is_empty() {
        let stocks = stock::Entity::find()
            .filter(stock::Column::TsCode.is_in(ts_codes.clone()))
            .all(conn)
            .await?;
        for s in stocks {
            name_map.insert(s.ts_code, s.name);
        }
    }

    let data = rows
        .into_iter()
        .map(|r| FinanceMainBusinessItem {
            ts_code: r.ts_code.clone(),
            stock_name: name_map.get(&r.ts_code).cloned().unwrap_or(None),
            end_date: r.end_date,
            bz_item: r.bz_item,
            bz_sales: r.bz_sales.map(|v| v.to_string()),
            bz_profit: r.bz_profit.map(|v| v.to_string()),
            bz_cost: r.bz_cost.map(|v| v.to_string()),
        })
        .collect();

    let total_pages = (total + page_size - 1) / page_size;

    Ok(FinanceMainBusinessListResponse {
        data,
        total,
        page,
        page_size,
        total_pages,
    })
}
