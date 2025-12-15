use super::traits::{TradingStrategy, StrategyConfig, SecurityData, StrategySignal, StrategyResult};
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// 相似度策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityStrategyConfig {
    /// 相似度阈值（0.0-1.0），高于此值认为相似
    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f64,
    
    /// 比较的天数
    #[serde(default = "default_comparison_days")]
    pub comparison_days: usize,
    
    /// 价格相关系数权重
    #[serde(default = "default_price_weight")]
    pub price_weight: f64,
    
    /// 成交量相关系数权重
    #[serde(default = "default_volume_weight")]
    pub volume_weight: f64,
    
    /// 涨跌幅相关系数权重
    #[serde(default = "default_change_weight")]
    pub change_weight: f64,
}

fn default_similarity_threshold() -> f64 { 0.7 }
fn default_comparison_days() -> usize { 20 }
fn default_price_weight() -> f64 { 0.4 }
fn default_volume_weight() -> f64 { 0.3 }
fn default_change_weight() -> f64 { 0.3 }

impl Default for SimilarityStrategyConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: default_similarity_threshold(),
            comparison_days: default_comparison_days(),
            price_weight: default_price_weight(),
            volume_weight: default_volume_weight(),
            change_weight: default_change_weight(),
        }
    }
}

impl StrategyConfig for SimilarityStrategyConfig {
    fn strategy_name(&self) -> &str {
        "相似度策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.comparison_days
    }
    
    fn validate(&self) -> Result<()> {
        if self.comparison_days < 5 {
            bail!("比较天数至少需要5天");
        }
        if self.similarity_threshold < 0.0 || self.similarity_threshold > 1.0 {
            bail!("相似度阈值应在0.0-1.0之间");
        }
        let weight_sum = self.price_weight + self.volume_weight + self.change_weight;
        if (weight_sum - 1.0).abs() > 0.01 {
            bail!("权重之和应为1.0，当前为{}", weight_sum);
        }
        Ok(())
    }
}

/// 相似度策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,
    pub pct_chg: f64,
    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
    
    /// 综合相似度（0.0-1.0）
    pub similarity_score: f64,
    /// 价格相关系数
    pub price_correlation: f64,
    /// 成交量相关系数
    pub volume_correlation: f64,
    /// 涨跌幅相关系数
    pub change_correlation: f64,
    /// 目标股票最新涨跌幅
    pub target_latest_change: f64,
}

/// 相似度策略
/// 
/// 通过计算当前股票与目标股票的价格、成交量、涨跌幅的相关系数，
/// 判断两只股票的走势是否相似
pub struct SimilarityStrategy {
    config: SimilarityStrategyConfig,
}

impl SimilarityStrategy {
    pub fn new(config: SimilarityStrategyConfig) -> Self {
        Self { config }
    }
    
    /// 计算两个序列的皮尔逊相关系数
    fn calculate_correlation(x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }
        
        let n = x.len() as f64;
        let mean_x: f64 = x.iter().sum::<f64>() / n;
        let mean_y: f64 = y.iter().sum::<f64>() / n;
        
        let mut numerator = 0.0;
        let mut sum_sq_x = 0.0;
        let mut sum_sq_y = 0.0;
        
        for i in 0..x.len() {
            let diff_x = x[i] - mean_x;
            let diff_y = y[i] - mean_y;
            numerator += diff_x * diff_y;
            sum_sq_x += diff_x * diff_x;
            sum_sq_y += diff_y * diff_y;
        }
        
        let denominator = (sum_sq_x * sum_sq_y).sqrt();
        if denominator == 0.0 {
            return None;
        }
        
        Some(numerator / denominator)
    }
    
    /// 计算涨跌幅序列
    fn calculate_changes(prices: &[f64]) -> Vec<f64> {
        let mut changes = Vec::new();
        for i in 1..prices.len() {
            if prices[i - 1] != 0.0 {
                changes.push((prices[i] - prices[i - 1]) / prices[i - 1] * 100.0);
            } else {
                changes.push(0.0);
            }
        }
        changes
    }
    
    /// 归一化序列（Min-Max归一化）
    fn normalize(data: &[f64]) -> Vec<f64> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        if max == min {
            return vec![0.5; data.len()];
        }
        
        data.iter().map(|&x| (x - min) / (max - min)).collect()
    }
}

impl TradingStrategy for SimilarityStrategy {
    type Config = SimilarityStrategyConfig;
    
    fn name(&self) -> &str {
        "相似度策略"
    }
    
    fn description(&self) -> &str {
        "通过计算价格、成交量、涨跌幅的相关系数，判断股票走势与目标股票的相似度"
    }
    
    fn config(&self) -> &Self::Config {
        &self.config
    }
    
    fn update_config(&mut self, config: Self::Config) -> Result<()> {
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn analyze(&mut self, ts_code: &str, data: &[SecurityData]) -> Result<StrategyResult> {
        if data.len() < self.config.comparison_days {
            bail!("数据不足，需要至少 {} 个数据点", self.config.comparison_days);
        }
        
        // 获取最近的数据
        let recent_data = &data[data.len() - self.config.comparison_days..];
        
        // 检查第一个数据点是否有目标数据
        // 注意：target 是 Option<Box<SecurityData>>，只包含一个目标股票的单个数据点
        // 我们需要从所有数据点中提取目标数据来构建目标序列
        let mut target_prices = Vec::new();
        let mut target_volumes = Vec::new();
        
        for d in recent_data {
            if let Some(ref target) = d.target {
                target_prices.push(target.close);
                target_volumes.push(target.volume);
            } else {
                bail!("数据点缺少目标股票信息");
            }
        }
        
        if target_prices.len() < self.config.comparison_days {
            bail!("目标股票数据不足，需要至少 {} 个数据点", self.config.comparison_days);
        }
        
        // 提取当前股票的价格和成交量数据
        let current_prices: Vec<f64> = recent_data.iter().map(|d| d.close).collect();
        let current_volumes: Vec<f64> = recent_data.iter().map(|d| d.volume).collect();
        
        // 归一化价格数据（消除价格量级差异）
        let norm_current_prices = Self::normalize(&current_prices);
        let norm_target_prices = Self::normalize(&target_prices);
        
        // 归一化成交量数据
        let norm_current_volumes = Self::normalize(&current_volumes);
        let norm_target_volumes = Self::normalize(&target_volumes);
        
        // 计算涨跌幅
        let current_changes = Self::calculate_changes(&current_prices);
        let target_changes = Self::calculate_changes(&target_prices);
        
        // 计算相关系数
        let price_corr = Self::calculate_correlation(&norm_current_prices, &norm_target_prices)
            .unwrap_or(0.0);
        let volume_corr = Self::calculate_correlation(&norm_current_volumes, &norm_target_volumes)
            .unwrap_or(0.0);
        let change_corr = Self::calculate_correlation(&current_changes, &target_changes)
            .unwrap_or(0.0);
        
        // 计算加权相似度
        let similarity = price_corr * self.config.price_weight
            + volume_corr * self.config.volume_weight
            + change_corr * self.config.change_weight;
        
        // 计算最近涨跌幅
        let current_latest_change = if current_prices.len() >= 2 {
            let last = current_prices[current_prices.len() - 1];
            let prev = current_prices[current_prices.len() - 2];
            if prev != 0.0 {
                (last - prev) / prev * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        let target_latest_change = if target_prices.len() >= 2 {
            let last = target_prices[target_prices.len() - 1];
            let prev = target_prices[target_prices.len() - 2];
            if prev != 0.0 {
                (last - prev) / prev * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // 判断信号
        let (signal, strength) = if similarity >= self.config.similarity_threshold {
            // 相似度高，根据目标股票的走势判断
            if target_latest_change > 2.0 {
                (StrategySignal::StrongBuy, (similarity * 100.0) as u8)
            } else if target_latest_change > 0.0 {
                (StrategySignal::Buy, (similarity * 80.0) as u8)
            } else if target_latest_change < -2.0 {
                (StrategySignal::Sell, (similarity * 100.0) as u8)
            } else {
                (StrategySignal::Hold, (similarity * 60.0) as u8)
            }
        } else if similarity >= self.config.similarity_threshold * 0.7 {
            // 中等相似度
            (StrategySignal::Hold, (similarity * 50.0) as u8)
        } else {
            // 相似度低，不建议操作
            (StrategySignal::Hold, 20)
        };
        
        let risk_level = if similarity >= self.config.similarity_threshold { 2 } else { 3 };
        
        let reason = format!(
            "相似度: {:.2}% (价格相关: {:.2}, 成交量相关: {:.2}, 涨跌幅相关: {:.2}), \
             当前涨跌: {:.2}%, 目标涨跌: {:.2}%",
            similarity * 100.0,
            price_corr,
            volume_corr,
            change_corr,
            current_latest_change,
            target_latest_change
        );
        
        let latest = recent_data.last().unwrap();
        
        // 解析日期字符串为 NaiveDate
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        
        Ok(StrategyResult::Similarity(SimilarityResult {
            stock_code: ts_code.to_string(),
            analysis_date,
            current_price: latest.close,
            pct_chg: current_latest_change,
            strategy_signal: signal,
            signal_strength: strength,
            analysis_description: reason,
            risk_level,
            similarity_score: similarity,
            price_correlation: price_corr,
            volume_correlation: volume_corr,
            change_correlation: change_corr,
            target_latest_change,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let corr = SimilarityStrategy::calculate_correlation(&x, &y);
        assert!(corr.is_some());
        assert!((corr.unwrap() - 1.0).abs() < 0.001); // 完全正相关
    }

    #[test]
    fn test_normalize() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let normalized = SimilarityStrategy::normalize(&data);
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[4], 1.0);
        assert!((normalized[2] - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_calculate_changes() {
        let prices = vec![100.0, 110.0, 105.0, 115.0];
        let changes = SimilarityStrategy::calculate_changes(&prices);
        assert_eq!(changes.len(), 3);
        assert!((changes[0] - 10.0).abs() < 0.001);
        assert!((changes[1] - (-4.545)).abs() < 0.01);
    }
}
