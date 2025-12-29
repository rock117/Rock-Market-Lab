use std::collections::HashMap;

use chrono::{Duration, Local, NaiveDate};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use rust_decimal::prelude::ToPrimitive;
use entity::sea_orm::EntityTrait;
use super::stock_price_service;

#[derive(Debug, Clone, serde::Serialize)]
pub struct StockSimilarityItem {
    pub ts_code: String,
    pub name: Option<String>,
    pub similarity: f64,
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }

    if na == 0.0 || nb == 0.0 {
        return None;
    }

    Some(dot / (na.sqrt() * nb.sqrt()))
}

fn to_returns_norm(closes_desc: &[f64]) -> Option<Vec<f64>> {
    if closes_desc.len() < 2 {
        return None;
    }

    // input: desc (newest -> oldest); convert to asc for returns
    let mut closes = closes_desc.to_vec();
    closes.reverse();

    let mut rets: Vec<f64> = Vec::with_capacity(closes.len() - 1);
    for i in 1..closes.len() {
        let prev = closes[i - 1];
        let curr = closes[i];
        if prev == 0.0 {
            return None;
        }
        rets.push(curr / prev - 1.0);
    }

    // z-score normalize
    let mean = rets.iter().copied().sum::<f64>() / rets.len() as f64;
    let var = rets
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / rets.len() as f64;
    let std = var.sqrt();
    if std == 0.0 || !std.is_finite() {
        return None;
    }

    Some(rets.into_iter().map(|v| (v - mean) / std).collect())
}

fn sanitize_days(days: usize) -> usize {
    let days = days.clamp(5, 250);
    days
}

fn sanitize_top(top: usize) -> usize {
    top.clamp(1, 200)
}

pub async fn get_similar_stocks(
    conn: &DatabaseConnection,
    ts_code: &str,
    days: usize,
    top: usize,
) -> anyhow::Result<Vec<StockSimilarityItem>> {
    let days = sanitize_days(days);
    let top = sanitize_top(top);

    // Get all stocks (for name mapping + candidate list)
    let stocks: Vec<stock::Model> = stock::Entity::find().all(conn).await?;
    if stocks.is_empty() {
        return Ok(vec![]);
    }

    let mut all_codes: Vec<String> = Vec::with_capacity(stocks.len());
    let mut name_map: HashMap<String, Option<String>> = HashMap::with_capacity(stocks.len());
    for s in stocks {
        name_map.insert(s.ts_code.clone(), s.name.clone());
        all_codes.push(s.ts_code);
    }

    // Build a loose date range to cover trading day gaps.
    let end: NaiveDate = Local::now().date_naive();
    let start: NaiveDate = end - Duration::days((days as i64) * 3);

    let mut prices_map = stock_price_service::get_stock_prices_batch(&all_codes, &start, &end, conn).await?;

    // Extract target close series (latest N)
    let target_rows = prices_map.remove(ts_code).unwrap_or_default();
    let mut target_rows = target_rows;
    target_rows.sort_by(|a, b| b.trade_date.cmp(&a.trade_date));
    let target_closes_desc: Vec<f64> = target_rows
        .into_iter()
        .take(days)
        .map(|r| r.close.to_f64().unwrap_or(0.0))
        .collect();

    if target_closes_desc.len() < days {
        return Ok(vec![]);
    }

    let target_vec = match to_returns_norm(&target_closes_desc) {
        Some(v) => v,
        None => return Ok(vec![]),
    };

    let mut scored: Vec<StockSimilarityItem> = Vec::new();

    for (code, mut rows) in prices_map {
        if code == ts_code {
            continue;
        }

        rows.sort_by(|a, b| b.trade_date.cmp(&a.trade_date));
        let closes_desc: Vec<f64> = rows
            .into_iter()
            .take(days)
            .map(|r| r.close.to_f64().unwrap_or(0.0))
            .collect();

        if closes_desc.len() < days {
            continue;
        }

        let v = match to_returns_norm(&closes_desc) {
            Some(v) => v,
            None => continue,
        };

        let sim = match cosine_similarity(&target_vec, &v) {
            Some(s) if s.is_finite() => s,
            _ => continue,
        };

        scored.push(StockSimilarityItem {
            ts_code: code.clone(),
            name: name_map.get(&code).cloned().unwrap_or(None),
            similarity: sim,
        });
    }

    scored.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top);

    Ok(scored)
}
