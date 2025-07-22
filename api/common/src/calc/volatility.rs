//! 股票波动性分析模块
//! 
//! 本模块提供了一套完整的股票价格波动性分析工具，用于：
//! - 计算单个股票的波动性指标
//! - 比较不同股票之间的波动性
//! - 对多支股票按波动性进行排序
//! 
//! # 主要功能
//! 
//! - 波动性指标计算：
//!   - 标准差（Standard Deviation）：反映价格的绝对波动程度
//!   - 变异系数（Coefficient of Variation）：标准差/平均值，用于比较不同价格区间的股票
//!   - 最大价格波动幅度：反映极端情况下的价格波动
//!   - 平均日波动率：反映日常交易中的价格波动情况
//! 
//! # 使用示例
//! 
//! ```rust
//! use crate::calc::volatility::{PricePoint, calculate_volatility, rank_by_volatility};
//! 
//! // 计算单个股票的波动性
//! let price_points = vec![
//!     PricePoint { date: date1, price: 10.0, volume: 1000.0 },
//!     PricePoint { date: date2, price: 10.5, volume: 1200.0 },
//! ];
//! let volatility = calculate_volatility(&price_points);
//! 
//! // 比较多个股票的波动性
//! let stocks = vec![
//!     ("000001.SZ", &price_points1[..]),
//!     ("000002.SZ", &price_points2[..])
//! ];
//! let rankings = rank_by_volatility(&stocks);
//! ```
//! 
//! # 实现细节
//! 
//! 波动性比较采用综合评分方法，权重分配为：
//! - 变异系数: 40% - 最重要，因为它能反映价格的相对波动性
//! - 最大价格波动: 30% - 反映极端情况下的波动
//! - 平均日波动率: 30% - 反映日常交易的波动情况
//! 
//! 这种设计可以让不同价格区间的股票进行公平比较，同时兼顾了短期和长期的波动特征。

use chrono::NaiveDate;

/// 股票价格数据点
#[derive(Debug, Clone)]
pub struct PricePoint {
    pub date: NaiveDate,
    pub price: f64,
    pub volume: f64,
}

/// 波动性分析结果
#[derive(Debug, Clone)]
pub struct VolatilityMetrics {
    /// 标准差（Standard Deviation）
    pub std_dev: f64,
    /// 变异系数（Coefficient of Variation）- 标准差/平均值
    pub cv: f64,
    /// 最大价格波动幅度（百分比）
    pub max_price_swing: f64,
    /// 平均日波动率
    pub avg_daily_volatility: f64,
    /// 计算周期（天数）
    pub period_days: i64,
}

/// 波动性指标权重
const VARIATION_COEFFICIENT_WEIGHT: f64 = 0.4; // 变异系数权重 - 最重要，因为它能反映价格的相对波动性
const PRICE_SWING_WEIGHT: f64 = 0.3;          // 最大波动幅度权重 - 反映极端情况下的波动
const DAILY_VOLATILITY_WEIGHT: f64 = 0.3;     // 日波动率权重 - 反映日常交易的波动情况

impl VolatilityMetrics {
    /// 比较两个股票的波动性
    /// 返回值 > 0 表示 self 波动性更大
    /// 返回值 < 0 表示 other 波动性更大
    /// 返回值 = 0 表示波动性相同
    pub fn compare(&self, other: &VolatilityMetrics) -> i8 {
        // 使用综合评分方法，考虑多个指标
        let self_score = self.cv * VARIATION_COEFFICIENT_WEIGHT + 
            self.max_price_swing * PRICE_SWING_WEIGHT + 
            self.avg_daily_volatility * DAILY_VOLATILITY_WEIGHT;
        let other_score = other.cv * VARIATION_COEFFICIENT_WEIGHT + 
            other.max_price_swing * PRICE_SWING_WEIGHT + 
            other.avg_daily_volatility * DAILY_VOLATILITY_WEIGHT;
        
        if (self_score - other_score).abs() < 1e-6 {
            0
        } else if self_score > other_score {
            1
        } else {
            -1
        }
    }
}

/// 计算股票在给定时间段内的波动性指标
/// 
/// # Arguments
/// 
/// * `prices` - 按日期排序的价格数据点
pub fn calculate_volatility(prices: &[PricePoint]) -> VolatilityMetrics {
    if prices.is_empty() {
        return VolatilityMetrics {
            std_dev: 0.0,
            cv: 0.0,
            max_price_swing: 0.0,
            avg_daily_volatility: 0.0,
            period_days: 0,
        };
    }

    // 计算平均价格
    let avg_price: f64 = prices.iter().map(|p| p.price).sum::<f64>() / prices.len() as f64;

    // 计算标准差
    let variance: f64 = prices.iter()
        .map(|p| (p.price - avg_price).powi(2))
        .sum::<f64>() / prices.len() as f64;
    let std_dev = variance.sqrt();

    // 计算变异系数 (CV)
    let cv = if avg_price != 0.0 { std_dev / avg_price } else { 0.0 };

    // 计算最大价格波动幅度
    let max_price = prices.iter().map(|p| p.price).fold(f64::NEG_INFINITY, f64::max);
    let min_price = prices.iter().map(|p| p.price).fold(f64::INFINITY, f64::min);
    let max_price_swing = if min_price != 0.0 {
        ((max_price - min_price) / min_price) * 100.0
    } else {
        0.0
    };

    // 计算日波动率
    let daily_changes: Vec<f64> = prices.windows(2)
        .map(|w| ((w[1].price - w[0].price) / w[0].price).abs() * 100.0)
        .collect();
    let avg_daily_volatility = if !daily_changes.is_empty() {
        daily_changes.iter().sum::<f64>() / daily_changes.len() as f64
    } else {
        0.0
    };

    // 计算周期天数
    let period_days = if prices.len() >= 2 {
        (prices.last().unwrap().date - prices[0].date).num_days()
    } else {
        0
    };

    VolatilityMetrics {
        std_dev,
        cv,
        max_price_swing,
        avg_daily_volatility,
        period_days,
    }
}

/// 比较多个股票的波动性并返回排序后的股票代码和对应的波动性指标
pub fn rank_by_volatility(stock_prices: &[(&str, &[PricePoint])]) -> Vec<(String, VolatilityMetrics)> {
    let mut results: Vec<(String, VolatilityMetrics)> = stock_prices
        .iter()
        .map(|(code, prices)| {
            (code.to_string(), calculate_volatility(prices))
        })
        .collect();

    // 按波动性从大到小排序
    results.sort_by(|a, b| b.1.compare(&a.1).cmp(&0));
    results
}
