use entity::sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, Condition};
use entity::stock_daily;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tracing::{info, debug};
use rust_decimal::prelude::*;

/// 成交量分布分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDistributionResponse {
    /// 分析日期
    pub trade_date: String,
    
    /// 总成交量（手）
    pub total_volume: f64,
    
    /// 总成交额（元）
    pub total_amount: f64,
    
    /// 股票总数
    pub total_stocks: usize,
    
    /// Top N 成交量占比
    pub top_concentrations: TopConcentrations,
    
    /// 成交量集中度指标
    pub concentration_metrics: ConcentrationMetrics,
    
    /// Top股票详情
    pub top_stocks: Vec<TopStockDetail>,
}

/// Top N 成交量占比
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopConcentrations {
    /// Top 10 占比
    pub top10_pct: f64,
    
    /// Top 30 占比
    pub top30_pct: f64,
    
    /// Top 50 占比
    pub top50_pct: f64,
    
    /// Top 100 占比
    pub top100_pct: f64,
}

/// 成交量集中度指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationMetrics {
    /// 赫芬达尔-赫希曼指数 (HHI)
    /// 范围: 0-10000, 值越大表示越集中
    /// < 1500: 低集中度
    /// 1500-2500: 中等集中度
    /// > 2500: 高集中度
    pub hhi: f64,
    
    /// HHI 集中度等级
    pub hhi_level: String,
    
    /// 基尼系数 (Gini Coefficient)
    /// 范围: 0-1, 值越大表示分布越不均匀
    /// < 0.3: 相对均匀
    /// 0.3-0.5: 中等不均
    /// > 0.5: 高度不均
    pub gini_coefficient: f64,
    
    /// 基尼系数等级
    pub gini_level: String,
    
    /// CR4 (前4名集中度比率)
    pub cr4: f64,
    
    /// CR8 (前8名集中度比率)
    pub cr8: f64,
    
    /// 熵指数 (Entropy Index)
    /// 值越小表示越集中
    pub entropy: f64,
}

/// Top 股票详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopStockDetail {
    /// 排名
    pub rank: usize,
    
    /// 股票代码
    pub ts_code: String,
    
    /// 成交量（手）
    pub volume: f64,
    
    /// 成交额（元）
    pub amount: f64,
    
    /// 成交量占比
    pub volume_pct: f64,
    
    /// 成交额占比
    pub amount_pct: f64,
    
    /// 涨跌幅
    pub pct_chg: Option<f64>,
}

/// 获取某个交易日的成交量分布分析
pub async fn get_volume_distribution(
    conn: &DatabaseConnection,
    trade_date: &str,
    top_n: Option<usize>,
) -> Result<VolumeDistributionResponse> {
    info!("分析交易日 {} 的成交量分布", trade_date);
    
    // 验证日期格式
    let _date = NaiveDate::parse_from_str(trade_date, "%Y%m%d")
        .context("日期格式错误，应为 YYYYMMDD")?;
    
    // 获取当日所有股票数据，按成交量降序排列
    let stocks = stock_daily::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily::Column::TradeDate, trade_date))
        .filter(stock_daily::Column::Vol.gt(Decimal::ZERO))
        .order_by_desc(stock_daily::Column::Vol)
        .all(conn)
        .await
        .context("查询股票数据失败")?;
    
    if stocks.is_empty() {
        anyhow::bail!("交易日 {} 没有数据", trade_date);
    }
    
    info!("找到 {} 只股票", stocks.len());
    
    // 计算总成交量和总成交额
    let total_volume: f64 = stocks.iter()
        .map(|s| s.vol.to_f64().unwrap_or(0.0))
        .sum();
    
    let total_amount: f64 = stocks.iter()
        .map(|s| s.amount.to_f64().unwrap_or(0.0))
        .sum();
    
    let total_stocks = stocks.len();
    
    // 计算 Top N 占比
    let top_concentrations = calculate_top_concentrations(&stocks, total_volume);
    
    // 计算集中度指标
    let concentration_metrics = calculate_concentration_metrics(&stocks, total_volume);
    
    // 获取 Top N 股票详情
    let top_n = top_n.unwrap_or(50);
    let top_stocks = get_top_stocks(&stocks, total_volume, total_amount, top_n);
    
    Ok(VolumeDistributionResponse {
        trade_date: trade_date.to_string(),
        total_volume,
        total_amount,
        total_stocks,
        top_concentrations,
        concentration_metrics,
        top_stocks,
    })
}

/// 计算 Top N 成交量占比
fn calculate_top_concentrations(
    stocks: &[stock_daily::Model],
    total_volume: f64,
) -> TopConcentrations {
    let calc_pct = |n: usize| -> f64 {
        let top_volume: f64 = stocks.iter()
            .take(n)
            .map(|s| s.vol.to_f64().unwrap_or(0.0))
            .sum();
        (top_volume / total_volume * 100.0).round() / 100.0
    };
    
    TopConcentrations {
        top10_pct: calc_pct(10),
        top30_pct: calc_pct(30),
        top50_pct: calc_pct(50),
        top100_pct: calc_pct(100),
    }
}

/// 计算成交量集中度指标
fn calculate_concentration_metrics(
    stocks: &[stock_daily::Model],
    total_volume: f64,
) -> ConcentrationMetrics {
    // 计算每只股票的市场份额
    let market_shares: Vec<f64> = stocks.iter()
        .map(|s| s.vol.to_f64().unwrap_or(0.0))
        .map(|vol| vol / total_volume)
        .collect();
    
    // 1. 赫芬达尔-赫希曼指数 (HHI)
    let hhi = market_shares.iter()
        .map(|share| share * share)
        .sum::<f64>() * 10000.0;
    
    let hhi_level = if hhi < 1500.0 {
        "低集中度".to_string()
    } else if hhi < 2500.0 {
        "中等集中度".to_string()
    } else {
        "高集中度".to_string()
    };
    
    // 2. 基尼系数 (Gini Coefficient)
    let gini_coefficient = calculate_gini_coefficient(&market_shares);
    
    let gini_level = if gini_coefficient < 0.3 {
        "相对均匀".to_string()
    } else if gini_coefficient < 0.5 {
        "中等不均".to_string()
    } else {
        "高度不均".to_string()
    };
    
    // 3. CR4 和 CR8
    let cr4 = market_shares.iter().take(4).sum::<f64>() * 100.0;
    let cr8 = market_shares.iter().take(8).sum::<f64>() * 100.0;
    
    // 4. 熵指数 (Entropy Index)
    let entropy = -market_shares.iter()
        .filter(|&&share| share > 0.0)
        .map(|&share| share * share.ln())
        .sum::<f64>();
    
    ConcentrationMetrics {
        hhi: (hhi * 100.0).round() / 100.0,
        hhi_level,
        gini_coefficient: (gini_coefficient * 10000.0).round() / 10000.0,
        gini_level,
        cr4: (cr4 * 100.0).round() / 100.0,
        cr8: (cr8 * 100.0).round() / 100.0,
        entropy: (entropy * 10000.0).round() / 10000.0,
    }
}

/// 计算基尼系数
fn calculate_gini_coefficient(shares: &[f64]) -> f64 {
    if shares.is_empty() {
        return 0.0;
    }
    
    let n = shares.len() as f64;
    let mut sorted_shares = shares.to_vec();
    sorted_shares.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let sum_shares: f64 = sorted_shares.iter().sum();
    if sum_shares == 0.0 {
        return 0.0;
    }
    
    let mut numerator = 0.0;
    for (i, &share) in sorted_shares.iter().enumerate() {
        numerator += (2.0 * (i as f64 + 1.0) - n - 1.0) * share;
    }
    
    numerator / (n * sum_shares)
}

/// 获取 Top N 股票详情
fn get_top_stocks(
    stocks: &[stock_daily::Model],
    total_volume: f64,
    total_amount: f64,
    top_n: usize,
) -> Vec<TopStockDetail> {
    stocks.iter()
        .take(top_n)
        .enumerate()
        .map(|(idx, stock)| {
            let volume = stock.vol.to_f64().unwrap_or(0.0);
            let amount = stock.amount.to_f64().unwrap_or(0.0);
            
            TopStockDetail {
                rank: idx + 1,
                ts_code: stock.ts_code.clone(),
                volume,
                amount,
                volume_pct: (volume / total_volume * 10000.0).round() / 100.0,
                amount_pct: (amount / total_amount * 10000.0).round() / 100.0,
                pct_chg: stock.pct_chg.and_then(|d| d.to_f64()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gini_coefficient() {
        // 完全均等分布
        let equal = vec![0.25, 0.25, 0.25, 0.25];
        let gini = calculate_gini_coefficient(&equal);
        assert!(gini < 0.01, "完全均等分布的基尼系数应接近0");
        
        // 完全不均分布
        let unequal = vec![1.0, 0.0, 0.0, 0.0];
        let gini = calculate_gini_coefficient(&unequal);
        assert!(gini > 0.7, "完全不均分布的基尼系数应接近1");
    }
    
    #[test]
    fn test_hhi_calculation() {
        // 4家公司各占25%
        let shares = vec![0.25, 0.25, 0.25, 0.25];
        let hhi = shares.iter().map(|s| s * s).sum::<f64>() * 10000.0;
        assert_eq!(hhi, 2500.0, "4家均分的HHI应为2500");
        
        // 1家独大占100%
        let shares = vec![1.0];
        let hhi = shares.iter().map(|s| s * s).sum::<f64>() * 10000.0;
        assert_eq!(hhi, 10000.0, "垄断的HHI应为10000");
    }
}
