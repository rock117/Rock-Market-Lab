use std::collections::HashMap;

use chrono::{Duration, Local, NaiveDate};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use rust_decimal::prelude::ToPrimitive;
use entity::sea_orm::EntityTrait;
use super::stock_price_service;

// 股票走势相似度服务
//
// 目标：给定一个目标股票 ts_code + 过去 N 天窗口，在所有股票中寻找“走势形态”相近的股票。
//
// 这里的“走势形态”并不直接用价格水平比较，而是：
// 1) 先取 close 序列
// 2) 转为日收益率序列 r_t = close_t / close_{t-1} - 1
// 3) 对收益率做 z-score 标准化（消除均值/波动率差异）
// 4) 用余弦相似度 cosine(a, b) 衡量两段序列方向的一致性
//
// 这样做的直觉：
// - 收益率而非价格：避免高价股/低价股量纲不同
// - z-score：避免“整体涨跌幅/波动率更大”主导相似度
// - cosine：强调走势方向（序列形状）相似，而非幅度

#[derive(Debug, Clone, serde::Serialize)]
pub struct StockSimilarityItem {
    pub ts_code: String,
    pub name: Option<String>,
    pub similarity: f64,
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> Option<f64> {
    // 余弦相似度定义：dot(a, b) / (||a|| * ||b||)
    //
    // 取值范围通常在 [-1, 1]：
    // - 越接近 1：两个向量方向越一致（走势越相似）
    // - 越接近 -1：方向相反
    // - 接近 0：不相关
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

    // 任意一个向量范数为 0，都无法定义相似度。
    if na == 0.0 || nb == 0.0 {
        return None;
    }

    Some(dot / (na.sqrt() * nb.sqrt()))
}

fn to_returns_norm(closes_desc: &[f64]) -> Option<Vec<f64>> {
    // 将 close 序列（按时间倒序：最新 -> 最旧）转换为“标准化收益率向量”。
    //
    // 返回向量长度为 closes.len() - 1，与输入相比少一天。
    if closes_desc.len() < 2 {
        return None;
    }

    // input: desc (newest -> oldest); convert to asc for returns
    let mut closes = closes_desc.to_vec();
    closes.reverse();

    // 计算简单收益率序列 r_t = close_t / close_{t-1} - 1
    // 这里不使用对数收益率，主要为了直观 & 计算简单。
    let mut rets: Vec<f64> = Vec::with_capacity(closes.len() - 1);
    for i in 1..closes.len() {
        let prev = closes[i - 1];
        let curr = closes[i];
        if prev == 0.0 {
            // 出现 0 会导致除零（通常意味着数据异常），直接判定该序列不可用。
            return None;
        }
        rets.push(curr / prev - 1.0);
    }

    // z-score normalize
    // z-score：x' = (x - mean) / std
    // 目的：
    // - 消除不同股票“平均收益率水平”的影响
    // - 消除不同股票“波动率大小”的影响
    // 最终更关注序列形状/方向。
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
        // 标准差为 0（完全无波动）或非有限值（NaN/Inf）时无法标准化。
        return None;
    }

    Some(rets.into_iter().map(|v| (v - mean) / std).collect())
}

fn sanitize_days(days: usize) -> usize {
    // 限制计算窗口，避免前端传入极端参数导致：
    // - 过短窗口：相似度不稳定
    // - 过长窗口：查询范围变大、性能下降、且缺失数据更多
    let days = days.clamp(5, 250);
    days
}

fn sanitize_top(top: usize) -> usize {
    // 限制返回数量，避免返回过多导致网络/渲染压力。
    top.clamp(1, 200)
}

pub async fn get_similar_stocks(
    conn: &DatabaseConnection,
    ts_code: &str,
    days: usize,
    top: usize,
) -> anyhow::Result<Vec<StockSimilarityItem>> {
    // 对外主入口：计算与 ts_code 最相似的 top 支股票。
    //
    // 重要约束：
    // - 目标股票与候选股票都必须拥有 >= days 条 close 数据（按交易日）
    // - 任一股票序列在收益率计算或标准化失败，会被跳过
    let days = sanitize_days(days);
    let top = sanitize_top(top);

    // Get all stocks (for name mapping + candidate list)
    let stocks: Vec<stock::Model> = stock::Entity::find().all(conn).await?;
    if stocks.is_empty() {
        return Ok(vec![]);
    }

    // 这里提前取 name 映射：前端展示时需要名字。
    let mut all_codes: Vec<String> = Vec::with_capacity(stocks.len());
    let mut name_map: HashMap<String, Option<String>> = HashMap::with_capacity(stocks.len());
    for s in stocks {
        name_map.insert(s.ts_code.clone(), s.name.clone());
        all_codes.push(s.ts_code);
    }

    // Build a loose date range to cover trading day gaps.
    // 由于交易日不是自然日连续的，这里用较宽松的自然日区间来“覆盖”至少 days 个交易日。
    // 经验上乘以 3 足以应对周末/节假日缺口（并不保证，但能显著降低取不到 days 条的问题）。
    let end: NaiveDate = Local::now().date_naive();
    let start: NaiveDate = end - Duration::days((days as i64) * 3);

    // 批量拉取所有股票在区间内的日线 close（一次性查询，避免 N+1）。
    let mut prices_map = stock_price_service::get_stock_prices_batch(&all_codes, &start, &end, conn).await?;

    // Extract target close series (latest N)
    let target_rows = prices_map.remove(ts_code).unwrap_or_default();
    let mut target_rows = target_rows;
    // trade_date 为字符串（yyyymmdd），按字符串倒序即可得到最近日期优先。
    target_rows.sort_by(|a, b| b.trade_date.cmp(&a.trade_date));
    let target_closes_desc: Vec<f64> = target_rows
        .into_iter()
        .take(days)
        .map(|r| r.close.to_f64().unwrap_or(0.0))
        .collect();

    if target_closes_desc.len() < days {
        // 目标股票数据不足：无法形成指定窗口，直接返回空。
        return Ok(vec![]);
    }

    // 目标股票的标准化收益率向量（后续与每个候选做 cosine）。
    let target_vec = match to_returns_norm(&target_closes_desc) {
        Some(v) => v,
        None => return Ok(vec![]),
    };

    let mut scored: Vec<StockSimilarityItem> = Vec::new();

    // 遍历候选股票：构造同样长度的向量并计算相似度。
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
            // 候选股票数据不足：跳过
            continue;
        }

        let v = match to_returns_norm(&closes_desc) {
            Some(v) => v,
            None => continue,
        };

        // 余弦相似度（只保留有限值，避免 NaN 破坏排序）。
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

    // 相似度降序，取 top。
    scored.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top);

    Ok(scored)
}
