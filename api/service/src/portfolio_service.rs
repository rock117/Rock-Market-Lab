use anyhow::{Result, Context, bail};
use entity::sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, Set, 
    TransactionTrait, QueryFilter, ColumnTrait
};
use entity::{portfolio, holding};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub id: i32,
    pub name: String,
    pub holdings: Vec<HoldingResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingResponse {
    pub id: i32,
    pub exchange_id: Option<String>,
    pub symbol: Option<String>,
    pub portfolio_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddHoldingRequest {
    pub exchange_id: Option<String>,
    pub symbol: Option<String>,
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
        holdings: vec![],
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
            holdings: holdings.into_iter().map(|h| HoldingResponse {
                id: h.id,
                exchange_id: h.exchange_id,
                symbol: h.symbol,
                portfolio_id: h.portfolio_id,
            }).collect(),
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
        .all(conn)
        .await
        .context("Failed to fetch holdings")?;
    
    Ok(PortfolioResponse {
        id: portfolio.id,
        name: portfolio.name,
        holdings: holdings.into_iter().map(|h| HoldingResponse {
            id: h.id,
            exchange_id: h.exchange_id,
            symbol: h.symbol,
            portfolio_id: h.portfolio_id,
        }).collect(),
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
    req: AddHoldingRequest,
) -> Result<HoldingResponse> {
    info!("Adding holding to portfolio {}: {:?}/{:?}", 
        portfolio_id, req.exchange_id, req.symbol);
    
    let portfolio = portfolio::Entity::find_by_id(portfolio_id)
        .one(conn)
        .await
        .context("Failed to fetch portfolio")?
        .ok_or_else(|| anyhow::anyhow!("Portfolio not found: {}", portfolio_id))?;
    
    let holding_model = holding::ActiveModel {
        exchange_id: Set(req.exchange_id.clone()),
        symbol: Set(req.symbol.clone()),
        portfolio_id: Set(portfolio.id),
        ..Default::default()
    };
    
    let result = holding_model.insert(conn).await
        .context("Failed to insert holding")?;
    
    Ok(HoldingResponse {
        id: result.id,
        exchange_id: result.exchange_id,
        symbol: result.symbol,
        portfolio_id: result.portfolio_id,
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
