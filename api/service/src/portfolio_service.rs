use anyhow::{Result, Context, bail, anyhow};
use futures::future::err;
use entity::sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, Set, 
    TransactionTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect
};
use entity::{portfolio, holding, us_stock, stock, stock_daily};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use entity::sea_orm::sea_query::ExprTrait;
use entity::sea_orm::prelude::Decimal;
use std::collections::HashMap;

use crate::pct_chg::PeriodPctChg;

enum StockDto {
    UsStock(us_stock::Model),
    CnStock(stock::Model)
}

impl StockDto {
    pub fn exchange_id(&self) -> Option<String> {
        match self {
            StockDto::UsStock(stock) => Some(stock.exchange_id.clone()),
            StockDto::CnStock(stock) => None,
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            StockDto::UsStock(stock) => stock.name.clone(),
            StockDto::CnStock(stock) => stock.name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub id: i32,
    pub name: String,
    pub holdings_num: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingResponse {
    pub id: i32,
    pub exchange_id: String,
    pub symbol: String,
    pub name: Option<String>,
    pub portfolio_id: i32,
    pub desc: Option<String>,

    pub order: i32,

    pub current_price: Option<f64>,

    pub pct_chg: Option<f64>,
    pub pct5: Option<f64>,
    pub pct10: Option<f64>,
    pub pct20: Option<f64>,
    pub pct60: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddHoldingRequest {
    pub exchange_id: Option<String>,
    pub symbol: String,
    pub desc: Option<String>,
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHoldingDescRequest {
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePortfolioRequest {
    pub name: Option<String>,
    pub desc: Option<String>,
}

pub async fn create_portfolio(
    conn: &DatabaseConnection,
    req: CreatePortfolioRequest,
) -> Result<PortfolioResponse> {
    info!("Creating portfolio: {}", req.name);
    
    let portfolio_model = portfolio::ActiveModel {
        name: Set(req.name),
        ..Default::default()
    };
    
    let result = portfolio_model.insert(conn).await
        .context("Failed to insert portfolio")?;
    
    Ok(PortfolioResponse {
        id: result.id,
        name: result.name,
        holdings_num: 0,
    })
}

pub async fn list_portfolios(conn: &DatabaseConnection) -> Result<Vec<PortfolioResponse>> {
    info!("Listing all portfolios");
    
    let portfolios = portfolio::Entity::find()
        .all(conn)
        .await
        .context("Failed to fetch portfolios")?;
    
    let mut results = Vec::new();
    
    for p in portfolios {
        let holdings = holding::Entity::find()
            .filter(holding::Column::PortfolioId.eq(p.id))
            .all(conn)
            .await
            .context("Failed to fetch holdings")?;
        
        results.push(PortfolioResponse {
            id: p.id,
            name: p.name,
            holdings_num: holdings.len(),
        });
    }
    
    Ok(results)
}

pub async fn get_portfolio(
    conn: &DatabaseConnection,
    portfolio_id: i32,
) -> Result<PortfolioResponse> {
    info!("Getting portfolio: {}", portfolio_id);
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(conn)
        .await
        .context("Failed to fetch portfolio")?
        .ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    
    let holdings = holding::Entity::find()
        .filter(holding::Column::PortfolioId.eq(portfolio_id))
        .order_by_asc(holding::Column::Order)
        .order_by_asc(holding::Column::Id)
        .all(conn)
        .await
        .context("Failed to fetch holdings")?;
    
    Ok(PortfolioResponse {
        id: portfolio.id,
        name: portfolio.name,
        holdings_num: holdings.len(),
    })
}

pub async fn update_portfolio(
    conn: &DatabaseConnection,
    portfolio_id: i32,
    req: UpdatePortfolioRequest,
) -> Result<PortfolioResponse> {
    info!("Updating portfolio: {}", portfolio_id);
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(conn)
        .await?.ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    let mut portfolio_active: portfolio::ActiveModel = portfolio.into();
    if let Some(name) = req.name {
        portfolio_active.name = Set(name);
    }

    let updated = portfolio_active.update(conn).await
        .context("Failed to update portfolio")?;
    
    let holdings = holding::Entity::find()
        .filter(holding::Column::PortfolioId.eq(portfolio_id))
        .all(conn)
        .await
        .context("Failed to fetch holdings")?;
    
    info!("Portfolio {} updated successfully", portfolio_id);
    
    Ok(PortfolioResponse {
        id: updated.id,
        name: updated.name,
        holdings_num: holdings.len(),
    })
}

pub async fn delete_portfolio(
    conn: &DatabaseConnection,
    portfolio_id: i32,
) -> Result<()> {
    info!("Deleting portfolio: {}", portfolio_id);
    
    let txn = conn.begin().await
        .context("Failed to start transaction")?;
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(&txn)
        .await
        .context("Failed to fetch portfolio")?
        .ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    
    holding::Entity::delete_many()
        .filter(holding::Column::PortfolioId.eq(portfolio_id))
        .exec(&txn)
        .await
        .context("Failed to delete holdings")?;
    
    let portfolio_active: portfolio::ActiveModel = portfolio.into();
    portfolio_active.delete(&txn).await
        .context("Failed to delete portfolio")?;
    
    txn.commit().await
        .context("Failed to commit transaction")?;
    
    info!("Portfolio {} deleted successfully", portfolio_id);
    Ok(())
}

pub async fn add_holding(
    conn: &DatabaseConnection,
    portfolio_id: i32,
    mut req: AddHoldingRequest,
) -> Result<HoldingResponse> {
    info!(
        "Adding holding to portfolio {}: exchange_id={:?} symbol={}",
        portfolio_id,
        req.exchange_id,
        req.symbol
    );
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(conn)
        .await
        .context("Failed to fetch portfolio")?
        .ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    
    let is_cn = req.symbol.contains(".");
    let exchange_id = if is_cn {
        "cn".to_string()
    } else {
        req.exchange_id
            .clone()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow!("exchange_id is required for US stocks"))?
    };

    // find by exchange_id and symbol
    let stock = if is_cn {
        stock::Entity::find_by_id(&req.symbol)
            .one(conn)
            .await?
            .map(StockDto::CnStock)
            .ok_or_else(|| anyhow!("no stock found by {}", req.symbol))?
    } else {
        us_stock::Entity::find_by_id((exchange_id.clone(), req.symbol.clone()))
            .one(conn)
            .await?
            .map(StockDto::UsStock)
            .ok_or_else(|| anyhow!("no stock found by {} {}", exchange_id, req.symbol))?
    };

    let order_value: i32 = if let Some(v) = req.order {
        v
    } else {
        let max_order: Option<i32> = holding::Entity::find()
            .select_only()
            .column(holding::Column::Order)
            .filter(holding::Column::PortfolioId.eq(portfolio_id))
            .order_by_desc(holding::Column::Order)
            .limit(1)
            .into_tuple::<i32>()
            .one(conn)
            .await?;

        max_order.unwrap_or(0).saturating_add(1)
    };

    let holding_model = holding::ActiveModel {
        exchange_id: Set(exchange_id),
        symbol: Set(req.symbol.clone()),
        portfolio_id: Set(portfolio.id),
        name: Set(stock.name().clone()),
        desc: Set(req.desc.clone()),
        order: Set(order_value),
        ..Default::default()
    };

    let result = holding_model.insert(conn).await
        .context("Failed to insert holding")?;
    
    Ok(HoldingResponse {
        id: result.id,
        exchange_id: result.exchange_id,
        symbol: result.symbol,
        name: stock.name(),
        portfolio_id: result.portfolio_id,
        desc: result.desc,

        order: result.order,
        current_price: None,
        pct_chg: None,
        pct5: None,
        pct10: None,
        pct20: None,
        pct60: None,
    })
}

pub async fn get_holdings(
    conn: &DatabaseConnection,
    portfolio_id: i32,
) -> Result<Vec<HoldingResponse>> {
    info!("Getting holdings for portfolio: {}", portfolio_id);
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(conn)
        .await
        .context("Failed to fetch portfolio")?
        .ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    
    let holdings = holding::Entity::find()
        .filter(holding::Column::PortfolioId.eq(portfolio_id))
        .all(conn)
        .await
        .context("Failed to fetch holdings")?;

    let cn_symbols: Vec<String> = holdings
        .iter()
        .filter(|h| h.exchange_id == "cn")
        .map(|h| h.symbol.clone())
        .collect();

    let mut latest_price_map: HashMap<String, Option<f64>> = HashMap::new();
    let mut latest_pct_map: HashMap<String, Option<f64>> = HashMap::new();
    let mut closes_desc_map: HashMap<String, Vec<Decimal>> = HashMap::new();

    if !cn_symbols.is_empty() {
        let latest_trade_date: Option<String> = stock_daily::Entity::find()
            .select_only()
            .column(stock_daily::Column::TradeDate)
            .order_by_desc(stock_daily::Column::TradeDate)
            .limit(1)
            .into_tuple::<String>()
            .one(conn)
            .await?;

        if let Some(latest_trade_date) = latest_trade_date {
            let latest_dailies = stock_daily::Entity::find()
                .filter(ColumnTrait::eq(&stock_daily::Column::TradeDate, latest_trade_date.clone()))
                .filter(stock_daily::Column::TsCode.is_in(cn_symbols.clone()))
                .all(conn)
                .await?;

            for d in latest_dailies {
                let close_v = d.close.to_string().parse::<f64>().ok();
                latest_price_map.insert(d.ts_code.clone(), close_v);
                let v = d
                    .pct_chg
                    .and_then(|x| x.to_string().parse::<f64>().ok());
                latest_pct_map.insert(d.ts_code.clone(), v);
            }

            let last_60_dates: Vec<String> = stock_daily::Entity::find()
                .select_only()
                .column(stock_daily::Column::TradeDate)
                .distinct()
                .order_by_desc(stock_daily::Column::TradeDate)
                .limit(65)
                .into_tuple::<String>()
                .all(conn)
                .await?;

            if !last_60_dates.is_empty() {
                let rows = stock_daily::Entity::find()
                    .filter(stock_daily::Column::TradeDate.is_in(last_60_dates))
                    .filter(stock_daily::Column::TsCode.is_in(cn_symbols.clone()))
                    .order_by_desc(stock_daily::Column::TradeDate)
                    .all(conn)
                    .await?;

                for r in rows {
                    closes_desc_map.entry(r.ts_code.clone()).or_default().push(r.close);
                }
            }
        }
    }

    let results = holdings
        .into_iter()
        .map(|h| {
            let (current_price, pct_chg, pct5, pct10, pct20, pct60) = if h.exchange_id == "cn" {
                let closes_desc = closes_desc_map.get(&h.symbol).map(|v| v.as_slice()).unwrap_or(&[]);
                let pct = PeriodPctChg::from_closes_desc(closes_desc);
                let (pct5, pct10, pct20, pct60) = pct.to_f64_tuple();
                (
                    latest_price_map.get(&h.symbol).cloned().flatten(),
                    latest_pct_map.get(&h.symbol).cloned().flatten(),
                    pct5,
                    pct10,
                    pct20,
                    pct60,
                )
            } else {
                (None, None, None, None, None, None)
            };

            HoldingResponse {
                id: h.id,
                exchange_id: h.exchange_id,
                symbol: h.symbol,
                name: h.name,
                portfolio_id: h.portfolio_id,
                desc: h.desc,

                order: h.order,
                current_price,
                pct_chg,
                pct5,
                pct10,
                pct20,
                pct60,
            }
        })
        .collect();

    Ok(results)
}

pub async fn update_holding_desc(
    conn: &DatabaseConnection,
    portfolio_id: i32,
    holding_id: i32,
    req: UpdateHoldingDescRequest,
) -> Result<HoldingResponse> {
    info!("Updating holding {} desc in portfolio {}", holding_id, portfolio_id);
    
    let holding = holding::Entity::find_by_id(holding_id)
        .one(conn)
        .await
        .context("Failed to fetch holding")?
        .ok_or_else(|| anyhow::anyhow!("Holding not found: {}", holding_id))?;
    
    if holding.portfolio_id != portfolio_id {
        bail!("Holding {} does not belong to portfolio {}", holding_id, portfolio_id);
    }
    
    let mut holding_active: holding::ActiveModel = holding.into();
    holding_active.desc = Set(req.desc.clone());
    
    let updated = holding_active.update(conn).await
        .context("Failed to update holding")?;
    
    info!("Holding {} desc updated successfully", holding_id);
    
    Ok(HoldingResponse {
        id: updated.id,
        exchange_id: updated.exchange_id,
        symbol: updated.symbol,
        name: updated.name,
        portfolio_id: updated.portfolio_id,
        desc: updated.desc,

        order: updated.order,
        current_price: None,
        pct_chg: None,
        pct5: None,
        pct10: None,
        pct20: None,
        pct60: None,
    })
}

pub async fn remove_holding(
    conn: &DatabaseConnection,
    portfolio_id: i32,
    holding_id: i32,
) -> Result<()> {
    info!("Removing holding {} from portfolio {}", holding_id, portfolio_id);
    
    let holding = holding::Entity::find_by_id(holding_id)
        .one(conn)
        .await
        .context("Failed to fetch holding")?
        .ok_or_else(|| anyhow::anyhow!("Holding not found: {}", holding_id))?;
    
    if holding.portfolio_id != portfolio_id {
        bail!("Holding {} does not belong to portfolio {}", holding_id, portfolio_id);
    }
    
    let holding_active: holding::ActiveModel = holding.into();
    holding_active.delete(conn).await
        .context("Failed to delete holding")?;
    
    info!("Holding {} removed successfully", holding_id);
    Ok(())
}
