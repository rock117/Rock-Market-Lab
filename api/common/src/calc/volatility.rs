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
//! use crate::calc::volatility::{DailyTradeRecord, calculate_volatility, rank_by_volatility};
//! 
//! // 计算单个股票的波动性
//! let trade_records = vec![
//!     DailyTradeRecord { 
//!         date: date1, 
//!         price: 10.0,
//!         volume: 1000.0,
//!     },
//!     DailyTradeRecord { 
//!         date: date2, 
//!         price: 10.5,
//!         volume: 2000.0,
//!     },
//! ];
//! let volatility = calculate_volatility(&trade_records);
//! 
//! // 比较多个股票的波动性
//! let stocks = vec![
//!     ("000001.SZ", &trade_records1[..]),
//!     ("000002.SZ", &trade_records2[..])
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
use tracing::warn;

/// 每日交易记录
#[derive(Debug, Clone)]
pub struct DailyTradeRecord {
    /// 交易日期
    pub date: NaiveDate,
    /// 交易价格
    pub price: f64,
    /// 成交量
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
    /// 成交量加权平均日波动率
    pub volume_weighted_volatility: f64,
    /// 计算周期（天数）
    pub period_days: i64,
}

/// 波动性指标权重
const VARIATION_COEFFICIENT_WEIGHT: f64 = 0.3; // 变异系数权重 - 反映价格的相对波动性
const PRICE_SWING_WEIGHT: f64 = 0.2;          // 最大波动幅度权重 - 反映极端情况下的波动
const DAILY_VOLATILITY_WEIGHT: f64 = 0.2;     // 日波动率权重 - 反映日常交易的波动情况
const VOLUME_WEIGHT: f64 = 0.3;               // 成交量加权波动率权重 - 考虑成交量对波动的影响

impl VolatilityMetrics {
    /// 计算波动性综合得分
    /// 
    /// 使用加权方式计算波动性的综合得分，权重分配为：
    /// - 变异系数: 30% - 取值范围 [0, +∞)
    /// - 最大价格波动: 20% - 取值范围 [0, +∞)，以百分比表示
    /// - 平均日波动率: 20% - 取值范围 [0, +∞)，以百分比表示
    /// - 成交量加权波动率: 30% - 取值范围 [0, +∞)，以百分比表示
    /// 
    /// 波动性得分是一个非负数，没有上限。得分越高表示股票的波动性越大。
    pub fn calculate_score(&self) -> f64 {
        // self.cv * VARIATION_COEFFICIENT_WEIGHT +
        // self.max_price_swing * PRICE_SWING_WEIGHT +
        // self.avg_daily_volatility * DAILY_VOLATILITY_WEIGHT +
        // self.volume_weighted_volatility * VOLUME_WEIGHT
        //
        self.cv
    }

    /// 比较两个股票的波动性
    /// 返回值 > 0 表示 self 波动性更大
    /// 返回值 < 0 表示 other 波动性更大
    /// 返回值 = 0 表示波动性相同
    pub fn compare(&self, other: &VolatilityMetrics) -> i8 {
        let self_score = self.calculate_score();
        let other_score = other.calculate_score();
        
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
pub fn calculate_volatility(prices: &[DailyTradeRecord]) -> VolatilityMetrics {
    if prices.is_empty() {
        return VolatilityMetrics {
            std_dev: 0.0,
            cv: 0.0,
            max_price_swing: 0.0,
            avg_daily_volatility: 0.0,
            volume_weighted_volatility: 0.0,
            period_days: 0,
        };
    }

    // 计算平均价格
    let avg_price: f64 = prices.iter().map(|p| p.price).sum::<f64>() / prices.len() as f64;
    if avg_price == 0f64 {
        warn!("===> avg_price is 0, prices: {:?}", prices);
    }
    // 计算标准差
    let variance: f64 = prices.iter()
        .map(|p| (p.price - avg_price).powi(2))
        .sum::<f64>() / prices.len() as f64;
    let std_dev = variance.sqrt();

    // 计算变异系数 (CV)
    let cv = if avg_price != 0.0 { std_dev / avg_price } else { 0.0 };
    if cv == 0f64 {
        warn!("===> cv is 0, prices: {:?}", prices);
    }
    // 计算最大价格波动幅度
    let max_price = prices.iter().map(|p| p.price).fold(f64::NEG_INFINITY, f64::max);
    let min_price = prices.iter().map(|p| p.price).fold(f64::INFINITY, f64::min);
    let max_price_swing = if min_price != 0.0 {
        ((max_price - min_price) / min_price) * 100.0
    } else {
        0.0
    };

    // 计算日波动率和成交量加权日波动率
    let mut total_changes = 0.0;
    let mut changes_count = 0;
    
    // 计算普通日波动率和成交量加权波动率
    let mut weighted_changes = 0.0;
    let mut total_weights = 0.0;

    for w in prices.windows(2) {
        let price_change = ((w[1].price - w[0].price) / w[0].price).abs() * 100.0;
        let avg_volume = (w[0].volume + w[1].volume) / 2.0;
        
        total_changes += price_change;
        
        // 使用成交量的对数作为权重，这样可以减小成交量差异过大带来的影响
        // 使用 1 + ln(volume) 确保权重始终大于1
        let volume_weight = 1.0 + avg_volume.ln().max(0.0);
        
        weighted_changes += price_change * volume_weight;
        total_weights += volume_weight;
        changes_count += 1;
    }

    let avg_daily_volatility = if changes_count > 0 {
        total_changes / changes_count as f64
    } else {
        0.0
    };

    // 成交量加权波动率：使用成交量的对数作为权重
    let volume_weighted_volatility = if total_weights > 0.0 && changes_count > 0 {
        weighted_changes / total_weights
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
        volume_weighted_volatility,
        period_days,
    }
}

/// 比较多个股票的波动性并返回排序后的股票代码和对应的波动性指标
pub fn rank_by_volatility<'a>(stocks: &[(&'a str, &[DailyTradeRecord])]) -> Vec<(&'a str, VolatilityMetrics)> {
    let mut results: Vec<(&'a str, VolatilityMetrics)> = stocks
        .iter()
        .map(|(code, prices)| {
            (*code, calculate_volatility(prices))
        })
        .collect();

    results.sort_by(|a, b| b.1.compare(&a.1).cmp(&0));
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 创建测试数据
    fn create_test_data() -> Vec<DailyTradeRecord> {
        vec![
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                price: 10.0,
                volume: 1000.0,
            },
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
                price: 10.5,  // +5%
                volume: 2000.0,
            },
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 3).unwrap(),
                price: 10.2,  // -2.86%
                volume: 1500.0,
            },
        ]
    }

    /// 创建另一组测试数据，波动性更大
    fn create_volatile_test_data() -> Vec<DailyTradeRecord> {
        vec![
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                price: 20.0,
                volume: 2000.0,
            },
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
                price: 22.0,  // +10%
                volume: 3000.0,
            },
            DailyTradeRecord {
                date: NaiveDate::from_ymd_opt(2025, 1, 3).unwrap(),
                price: 19.8,  // -10%
                volume: 4000.0,
            },
        ]
    }

    #[test]
    fn test_basic_volatility_calculation() {
        let data = create_test_data();
        let metrics = calculate_volatility(&data);

        // 检查基本指标
        assert!(metrics.std_dev > 0.0, "标准差应大于0");
        assert!(metrics.cv > 0.0, "变异系数应大于0");
        assert!(metrics.max_price_swing > 0.0, "最大波动幅度应大于0");
        assert_eq!(metrics.period_days, 2, "周期应为2天");

        // 检查日波动率
        let expected_avg_daily = (5.0 + 2.86) / 2.0; // (5% + 2.86%) / 2
        assert!((metrics.avg_daily_volatility - expected_avg_daily).abs() < 0.1, "日波动率计算错误");
    }

    #[test]
    fn test_volume_weighted_volatility() {
        let data = create_test_data();
        let metrics = calculate_volatility(&data);

        // 成交量加权后，应该更省50%的波动（因为成交量更大）
        assert!(metrics.volume_weighted_volatility > metrics.avg_daily_volatility, 
            "成交量加权后的波动率应该更大");
    }

    #[test]
    fn test_volatility_comparison() {
        let data1 = create_test_data();
        let data2 = create_volatile_test_data();

        let metrics1 = calculate_volatility(&data1);
        let metrics2 = calculate_volatility(&data2);

        // data2 的波动性应该更大
        assert_eq!(metrics1.compare(&metrics2), -1, "比较结果错误");
        assert_eq!(metrics2.compare(&metrics1), 1, "比较结果错误");
    }

    #[test]
    fn test_rank_by_volatility() {
        let data1 = create_test_data();
        let data2 = create_volatile_test_data();

        let stocks = vec![
            ("000001.SZ", &data1[..]),
            ("000002.SZ", &data2[..]),
        ];

        let rankings = rank_by_volatility(&stocks);
        assert_eq!(rankings[0].0, "000002.SZ", "排序结果错误");
        assert_eq!(rankings[1].0, "000001.SZ", "排序结果错误");
    }

    #[test]
    fn test_empty_data() {
        let metrics = calculate_volatility(&[]);
        assert_eq!(metrics.std_dev, 0.0);
        assert_eq!(metrics.cv, 0.0);
        assert_eq!(metrics.max_price_swing, 0.0);
        assert_eq!(metrics.avg_daily_volatility, 0.0);
        assert_eq!(metrics.volume_weighted_volatility, 0.0);
        assert_eq!(metrics.period_days, 0);
    }
}
