use std::collections::HashMap;
use std::time::Instant;

use chrono::{Datelike, Duration, Local, NaiveDate};
use entity::sea_orm::DatabaseConnection;
use entity::stock;
use rust_decimal::prelude::ToPrimitive;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::{ColumnTrait, Condition, QueryFilter};
use entity::stock_daily_basic;
use tracing::info;
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
    pub current_price: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub pct_chg: Option<f64>,
    pub pct5: Option<f64>,
    pub pct10: Option<f64>,
    pub pct20: Option<f64>,
    pub pct60: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StockSimilarityKLinePoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pct_chg: f64,
    pub turnover_rate: f64,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSimilarityResp {
    pub items: Vec<StockSimilarityItem>,
    pub kline: HashMap<String, Vec<StockSimilarityKLinePoint>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityAlgo {
    ZScoreCosine,
    PearsonReturns,
    BestLagCosine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityFreq {
    Day,
    Week,
    Month,
}

fn parse_freq(freq: Option<&str>) -> SimilarityFreq {
    match freq.unwrap_or("").trim().to_lowercase().as_str() {
        "week" | "w" | "weekly" => SimilarityFreq::Week,
        "month" | "m" | "monthly" => SimilarityFreq::Month,
        _ => SimilarityFreq::Day,
    }
}

fn sample_rows_by_freq_desc(
    rows_desc: &[entity::stock_daily::Model],
    freq: SimilarityFreq,
    take: usize,
) -> Vec<entity::stock_daily::Model> {
    if take == 0 || rows_desc.is_empty() {
        return vec![];
    }

    match freq {
        SimilarityFreq::Day => rows_desc.iter().take(take).cloned().collect(),
        SimilarityFreq::Week => {
            let mut out: Vec<entity::stock_daily::Model> = Vec::with_capacity(take);
            let mut last_key: Option<(i32, u32)> = None;
            for r in rows_desc.iter() {
                let d = match chrono::NaiveDate::parse_from_str(&r.trade_date, "%Y%m%d") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let key = (d.iso_week().year(), d.iso_week().week());
                if last_key == Some(key) {
                    continue;
                }
                last_key = Some(key);
                out.push(r.clone());
                if out.len() >= take {
                    break;
                }
            }
            out
        }
        SimilarityFreq::Month => {
            let mut out: Vec<entity::stock_daily::Model> = Vec::with_capacity(take);
            let mut last_key: Option<(i32, u32)> = None;
            for r in rows_desc.iter() {
                let d = match chrono::NaiveDate::parse_from_str(&r.trade_date, "%Y%m%d") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let key = (d.year(), d.month());
                if last_key == Some(key) {
                    continue;
                }
                last_key = Some(key);
                out.push(r.clone());
                if out.len() >= take {
                    break;
                }
            }
            out
        }
    }
}

fn parse_algo(algo: Option<&str>) -> SimilarityAlgo {
    // 前端/调用方建议传字符串：
    // - zscore_cosine
    // - pearson
    // - best_lag_cosine
    //
    // 为了兼容旧版本，也接受 "1"/"2"/"3" 作为别名。
    match algo.unwrap_or("").trim().to_lowercase().as_str() {
        "pearson" | "pearson_returns" => SimilarityAlgo::PearsonReturns,
        "best_lag_cosine" | "best_lag" => SimilarityAlgo::BestLagCosine,
        _ => SimilarityAlgo::ZScoreCosine,
    }
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

fn pearson_correlation(a: &[f64], b: &[f64]) -> Option<f64> {
    // Pearson 相关系数：cov(a,b) / (std(a) * std(b))
    // 取值范围 [-1, 1]，越接近 1 越同向。
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let n = a.len() as f64;
    let mean_a = a.iter().copied().sum::<f64>() / n;
    let mean_b = b.iter().copied().sum::<f64>() / n;

    let mut cov = 0.0;
    let mut va = 0.0;
    let mut vb = 0.0;
    for i in 0..a.len() {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        va += da * da;
        vb += db * db;
    }

    if va == 0.0 || vb == 0.0 {
        return None;
    }

    let denom = va.sqrt() * vb.sqrt();
    if denom == 0.0 {
        return None;
    }

    let v = cov / denom;
    if v.is_finite() { Some(v) } else { None }
}

fn to_returns(closes_desc: &[f64]) -> Option<Vec<f64>> {
    // 将 close 序列（倒序：最新 -> 最旧）转换为收益率序列（按时间正序）。
    if closes_desc.len() < 2 {
        return None;
    }

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
    Some(rets)
}

fn zscore_norm(xs: &[f64]) -> Option<Vec<f64>> {
    if xs.is_empty() {
        return None;
    }

    let mean = xs.iter().copied().sum::<f64>() / xs.len() as f64;
    let var = xs
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / xs.len() as f64;
    let std = var.sqrt();
    if std == 0.0 || !std.is_finite() {
        return None;
    }

    Some(xs.iter().map(|v| (v - mean) / std).collect())
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

fn best_lag_cosine(target: &[f64], cand: &[f64], max_lag: isize) -> Option<f64> {
    // 在有限滞后窗口内（[-max_lag, max_lag]）寻找最大余弦相似度。
    //
    // target / cand 都应当是同口径的“标准化收益率”序列（或其他已标准化的序列）。
    //
    // lag > 0：cand 向右移动（cand 更滞后）。
    if target.is_empty() || cand.is_empty() {
        return None;
    }

    let mut best: Option<f64> = None;

    for lag in -max_lag..=max_lag {
        let (a_start, b_start) = if lag >= 0 {
            (0usize, lag as usize)
        } else {
            ((-lag) as usize, 0usize)
        };

        if a_start >= target.len() || b_start >= cand.len() {
            continue;
        }

        let len = (target.len() - a_start).min(cand.len() - b_start);
        if len < 5 {
            continue;
        }

        let a_slice = &target[a_start..a_start + len];
        let b_slice = &cand[b_start..b_start + len];
        let sim = match cosine_similarity(a_slice, b_slice) {
            Some(s) if s.is_finite() => s,
            _ => continue,
        };

        best = match best {
            Some(cur) if cur >= sim => Some(cur),
            _ => Some(sim),
        };
    }

    best
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
    get_similar_stocks_by_algo(conn, ts_code, days, top, None, None).await
}

pub async fn get_similar_stocks_by_algo(
    conn: &DatabaseConnection,
    ts_code: &str,
    days: usize,
    top: usize,
    algo: Option<&str>,
    freq: Option<&str>,
) -> anyhow::Result<Vec<StockSimilarityItem>> {
    let t_total = Instant::now();
    // 对外主入口：计算与 ts_code 最相似的 top 支股票。
    //
    // 重要约束：
    // - 目标股票与候选股票都必须拥有 >= days 条 close 数据（按交易日）
    // - 任一股票序列在收益率计算或标准化失败，会被跳过
    let days = sanitize_days(days);
    let top = sanitize_top(top);
    let algo = parse_algo(algo);
    let freq = parse_freq(freq);

    info!("[similarity] start ts_code={} days={} top={} algo={:?} freq={:?}", ts_code, days, top, algo, freq);

    // Get all stocks (for name mapping + candidate list)
    let t_stocks = Instant::now();
    let stocks: Vec<stock::Model> = stock::Entity::find().all(conn).await?;
    info!("[similarity] load_stocks done elapsed_ms={} stocks={} ", t_stocks.elapsed().as_millis(), stocks.len());
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

    // 两阶段：
    // 1) 相似度计算：按 freq 的“周期”（day/week/month）取最近 days 个采样点
    // 2) 展示指标：按“前端选择的频率 + 长度(days)”计算（严格在所选时间范围内）
    let metrics_take = days;
    let end: NaiveDate = Local::now().date_naive();

    // phase1: 相似度窗口
    let start_sim: NaiveDate = match freq {
        SimilarityFreq::Day => end - Duration::days((days as i64) * 3),
        SimilarityFreq::Week => end - Duration::days((days as i64) * 7 * 3),
        SimilarityFreq::Month => end - Duration::days((days as i64) * 31 * 3),
    };

    // 批量拉取所有股票在区间内的日线（一次性查询，避免 N+1）。
    let t_phase1_fetch = Instant::now();
    let mut prices_map = stock_price_service::get_stock_prices_batch(&all_codes, &start_sim, &end, conn).await?;
    let phase1_total_rows: usize = prices_map.values().map(|v| v.len()).sum();
    info!(
        "[similarity] phase1_fetch done elapsed_ms={} codes={} groups={} rows={} start={} end={}",
        t_phase1_fetch.elapsed().as_millis(),
        all_codes.len(),
        prices_map.len(),
        phase1_total_rows,
        start_sim.format("%Y-%m-%d").to_string(),
        end.format("%Y-%m-%d").to_string(),
    );

    // Extract target close series (phase1)
    let target_rows_sim = prices_map.remove(ts_code).unwrap_or_default();
    let target_rows_sim = sample_rows_by_freq_desc(&target_rows_sim, freq, days);
    let target_closes_desc_days: Vec<f64> = target_rows_sim
        .iter()
        .map(|r| r.close.to_f64().unwrap_or(0.0))
        .collect();

    if target_closes_desc_days.len() < days {
        // 目标股票数据不足：无法形成指定窗口，直接返回空。
        return Ok(vec![]);
    }

    let target_rets = match to_returns(&target_closes_desc_days) {
        Some(v) => v,
        None => return Ok(vec![]),
    };

    let target_vec_norm = match zscore_norm(&target_rets) {
        Some(v) => v,
        None => return Ok(vec![]),
    };

    let mut scored: Vec<StockSimilarityItem> = Vec::new();

    // 遍历候选股票：构造同样长度的向量并计算相似度。
    let t_phase1_calc = Instant::now();
    let mut skipped_short = 0usize;
    let mut skipped_invalid = 0usize;
    for (code, mut rows) in prices_map {
        if code == ts_code {
            continue;
        }
        let rows_s = sample_rows_by_freq_desc(&rows, freq, days);
        let closes_desc: Vec<f64> = rows_s
            .iter()
            .map(|r| r.close.to_f64().unwrap_or(0.0))
            .collect();

        if closes_desc.len() < days {
            // 候选股票数据不足：跳过
            skipped_short += 1;
            continue;
        }

        let rets = match to_returns(&closes_desc) {
            Some(v) => v,
            None => {
                skipped_invalid += 1;
                continue;
            }
        };

        let v_norm = match zscore_norm(&rets) {
            Some(v) => v,
            None => {
                skipped_invalid += 1;
                continue;
            }
        };

        let sim = match algo {
            SimilarityAlgo::ZScoreCosine => match cosine_similarity(&target_vec_norm, &v_norm) {
                Some(s) if s.is_finite() => s,
                _ => continue,
            },
            SimilarityAlgo::PearsonReturns => match pearson_correlation(&target_rets, &rets) {
                Some(s) if s.is_finite() => s,
                _ => continue,
            },
            SimilarityAlgo::BestLagCosine => match best_lag_cosine(&target_vec_norm, &v_norm, 5) {
                Some(s) if s.is_finite() => s,
                _ => {
                    skipped_invalid += 1;
                    continue;
                }
            },
        };

        scored.push(StockSimilarityItem {
            ts_code: code.clone(),
            name: name_map.get(&code).cloned().unwrap_or(None),
            similarity: sim,
            current_price: None,
            turnover_rate: None,
            pct_chg: None,
            pct5: None,
            pct10: None,
            pct20: None,
            pct60: None,
        });
    }

    info!(
        "[similarity] phase1_calc done elapsed_ms={} scored={} skipped_short={} skipped_invalid={}",
        t_phase1_calc.elapsed().as_millis(),
        scored.len(),
        skipped_short,
        skipped_invalid
    );

    // 相似度降序，取 top。
    let t_sort = Instant::now();
    scored.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top);
    info!("[similarity] sort_truncate done elapsed_ms={} top_returned={}", t_sort.elapsed().as_millis(), scored.len());

    // phase2: 仅对目标股票 + top 候选补齐展示字段
    // 指标严格按 freq 的采样序列计算，因此需要拉取覆盖最近 metrics_take 个周期的日线（带缓冲）。
    let start_metrics: NaiveDate = match freq {
        SimilarityFreq::Day => end - Duration::days((metrics_take as i64) * 3),
        SimilarityFreq::Week => end - Duration::days((metrics_take as i64) * 7 * 3),
        SimilarityFreq::Month => end - Duration::days((metrics_take as i64) * 31 * 3),
    };
    let mut metric_codes: Vec<String> = scored.iter().map(|x| x.ts_code.clone()).collect();
    metric_codes.push(ts_code.to_string());

    let t_phase2_fetch = Instant::now();
    let metrics_map = stock_price_service::get_stock_prices_batch(&metric_codes, &start_metrics, &end, conn).await?;
    let phase2_total_rows: usize = metrics_map.values().map(|v| v.len()).sum();
    info!(
        "[similarity] phase2_fetch done elapsed_ms={} codes={} groups={} rows={} start={} end={} metrics_take={} freq={:?}",
        t_phase2_fetch.elapsed().as_millis(),
        metric_codes.len(),
        metrics_map.len(),
        phase2_total_rows,
        start_metrics.format("%Y-%m-%d").to_string(),
        end.format("%Y-%m-%d").to_string(),
        days,
        freq
    );

    let calc_metrics = |rows: &Vec<entity::stock_daily::Model>| -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
        if rows.is_empty() {
            return (None, None, None, None, None, None);
        }

        // rows 是 trade_date desc；按 freq 采样得到最近 metrics_take 个“周期点”（仍然 desc）。
        let sampled_desc = sample_rows_by_freq_desc(rows, freq, metrics_take);
        if sampled_desc.is_empty() {
            return (None, None, None, None, None, None);
        }

        let closes_desc: Vec<f64> = sampled_desc
            .iter()
            .map(|r| r.close.to_f64().unwrap_or(0.0))
            .collect();

        let current_price = closes_desc.first().copied();

        // pct_chg：按所选 freq 的“相邻采样点”计算（例如：周频=环比上一周）。
        let pct_chg = if closes_desc.len() >= 2 {
            let latest = closes_desc[0];
            let prev = closes_desc[1];
            if prev == 0.0 {
                None
            } else {
                let v = (latest / prev - 1.0) * 100.0;
                if v.is_finite() { Some(v) } else { None }
            }
        } else {
            None
        };

        let calc_pct_n = |n: usize| -> Option<f64> {
            if closes_desc.len() <= n {
                return None;
            }
            let latest = closes_desc[0];
            let base = closes_desc[n];
            if base == 0.0 {
                return None;
            }
            let v = (latest / base - 1.0) * 100.0;
            if v.is_finite() { Some(v) } else { None }
        };

        (current_price, pct_chg, calc_pct_n(5), calc_pct_n(10), calc_pct_n(20), calc_pct_n(60))
    };

    for item in &mut scored {
        if let Some(rows) = metrics_map.get(&item.ts_code) {
            let (cp, pc, p5, p10, p20, p60) = calc_metrics(rows);
            item.current_price = cp;
            item.pct_chg = pc;
            item.pct5 = p5;
            item.pct10 = p10;
            item.pct20 = p20;
            item.pct60 = p60;
        }
    }

    let mut target_item = StockSimilarityItem {
        ts_code: ts_code.to_string(),
        name: name_map.get(ts_code).cloned().unwrap_or(None),
        similarity: 1.0,
        current_price: None,
        turnover_rate: None,
        pct_chg: None,
        pct5: None,
        pct10: None,
        pct20: None,
        pct60: None,
    };

    if let Some(rows) = metrics_map.get(ts_code) {
        let (cp, pc, p5, p10, p20, p60) = calc_metrics(rows);
        target_item.current_price = cp;
        target_item.pct_chg = pc;
        target_item.pct5 = p5;
        target_item.pct10 = p10;
        target_item.pct20 = p20;
        target_item.pct60 = p60;
    }

    // 批量补充换手率（来自 stock_daily_basic 的 turnover_rate，取最近一条）。
    {
        let t_turnover = Instant::now();
        let codes: Vec<String> = metric_codes;
        let start_s = start_metrics.format("%Y%m%d").to_string();
        let end_s = end.format("%Y%m%d").to_string();

        let code_condition = codes
            .iter()
            .map(|code| ColumnTrait::eq(&stock_daily_basic::Column::TsCode, code.as_str()))
            .fold(Condition::any(), |acc, condition| acc.add(condition));

        let basic_rows = stock_daily_basic::Entity::find()
            .filter(code_condition)
            .filter(stock_daily_basic::Column::TradeDate.gte(start_s))
            .filter(stock_daily_basic::Column::TradeDate.lte(end_s))
            .all(conn)
            .await?;
        info!(
            "[similarity] phase2_turnover_fetch done elapsed_ms={} rows={}",
            t_turnover.elapsed().as_millis(),
            basic_rows.len()
        );

        let mut latest_turnover: HashMap<String, (String, f64)> = HashMap::new();
        for r in basic_rows {
            let tr = r.turnover_rate.and_then(|d| d.to_f64()).unwrap_or(0.0);
            let entry = latest_turnover.entry(r.ts_code.clone()).or_insert((r.trade_date.clone(), tr));
            if r.trade_date > entry.0 {
                *entry = (r.trade_date, tr);
            }
        }

        for item in &mut scored {
            item.turnover_rate = latest_turnover.get(&item.ts_code).map(|(_, v)| *v);
        }

        target_item.turnover_rate = latest_turnover.get(&target_item.ts_code).map(|(_, v)| *v);
    }

    // 置顶目标股票
    scored.insert(0, target_item);

    info!("[similarity] done elapsed_ms={} returned={}", t_total.elapsed().as_millis(), scored.len());

    Ok(scored)
}

pub async fn get_similar_stocks_with_kline_by_algo(
    conn: &DatabaseConnection,
    ts_code: &str,
    days: usize,
    top: usize,
    algo: Option<&str>,
    freq: Option<&str>,
) -> anyhow::Result<StockSimilarityResp> {
    let t_total = Instant::now();

    let items = get_similar_stocks_by_algo(conn, ts_code, days, top, algo, freq).await?;
    if items.is_empty() {
        return Ok(StockSimilarityResp { items, kline: HashMap::new() });
    }

    let freq = parse_freq(freq);

    let days = sanitize_days(days);
    let end: NaiveDate = Local::now().date_naive();
    // KLine 返回序列：按 freq 的“周期”取最近 days 个点。
    let kline_take = days;
    let start_metrics: NaiveDate = match freq {
        SimilarityFreq::Day => end - Duration::days((kline_take as i64) * 3),
        SimilarityFreq::Week => end - Duration::days((kline_take as i64) * 7 * 3),
        SimilarityFreq::Month => end - Duration::days((kline_take as i64) * 31 * 3),
    };

    let codes: Vec<String> = items.iter().map(|x| x.ts_code.clone()).collect();
    let t_kline_fetch = Instant::now();
    let metrics_map = stock_price_service::get_stock_prices_batch(&codes, &start_metrics, &end, conn).await?;
    let metrics_total_rows: usize = metrics_map.values().map(|v| v.len()).sum();
    info!(
        "[similarity] kline_fetch done elapsed_ms={} codes={} groups={} rows={} start={} end={} kline_take={} freq={:?}",
        t_kline_fetch.elapsed().as_millis(),
        codes.len(),
        metrics_map.len(),
        metrics_total_rows,
        start_metrics.format("%Y-%m-%d").to_string(),
        end.format("%Y-%m-%d").to_string(),
        kline_take,
        freq
    );

    // daily_basic: turnover_rate by (ts_code, trade_date)
    let start_s = start_metrics.format("%Y%m%d").to_string();
    let end_s = end.format("%Y%m%d").to_string();

    let code_condition = codes
        .iter()
        .map(|code| ColumnTrait::eq(&stock_daily_basic::Column::TsCode, code.as_str()))
        .fold(Condition::any(), |acc, condition| acc.add(condition));

    let t_basic = Instant::now();
    let basic_rows = stock_daily_basic::Entity::find()
        .filter(code_condition)
        .filter(stock_daily_basic::Column::TradeDate.gte(start_s))
        .filter(stock_daily_basic::Column::TradeDate.lte(end_s))
        .all(conn)
        .await?;
    info!(
        "[similarity] kline_turnover_fetch done elapsed_ms={} rows={}",
        t_basic.elapsed().as_millis(),
        basic_rows.len()
    );

    let mut turnover_by_date: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for r in basic_rows {
        let tr = r.turnover_rate.and_then(|d| d.to_f64()).unwrap_or(0.0);
        turnover_by_date
            .entry(r.ts_code.clone())
            .or_insert_with(HashMap::new)
            .insert(r.trade_date.clone(), tr);
    }

    let mut kline: HashMap<String, Vec<StockSimilarityKLinePoint>> = HashMap::new();
    for code in &codes {
        let rows = metrics_map.get(code).cloned().unwrap_or_default();
        let tmap = turnover_by_date.get(code);

        let mut points: Vec<StockSimilarityKLinePoint> = Vec::new();
        let sampled_desc = sample_rows_by_freq_desc(&rows, freq, kline_take);
        // sampled_desc 是 trade_date desc，这里反转为 asc
        for r in sampled_desc.iter().rev() {
            let date = match chrono::NaiveDate::parse_from_str(&r.trade_date, "%Y%m%d") {
                Ok(d) => d.format(common::date::FORMAT_DASH).to_string(),
                Err(_) => continue,
            };
            let turnover_rate = tmap
                .and_then(|m| m.get(&r.trade_date))
                .copied()
                .unwrap_or(0.0);

            points.push(StockSimilarityKLinePoint {
                date,
                open: r.open.to_f64().unwrap_or(0.0),
                high: r.high.to_f64().unwrap_or(0.0),
                low: r.low.to_f64().unwrap_or(0.0),
                close: r.close.to_f64().unwrap_or(0.0),
                pct_chg: r.pct_chg.and_then(|d| d.to_f64()).unwrap_or(0.0),
                turnover_rate,
                amount: r.amount.to_f64(),
            });
        }
        kline.insert(code.clone(), points);
    }

    info!("[similarity] with_kline done elapsed_ms={} returned_items={} returned_kline={}", t_total.elapsed().as_millis(), items.len(), kline.len());
    Ok(StockSimilarityResp { items, kline })
}
