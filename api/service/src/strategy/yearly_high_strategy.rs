//! 年内新高策略
//! 
//! 识别创出年内新高的股票，这通常是强势股的标志

use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, 
    StrategyResult, StrategySignal, SecurityData,
};

/// 年内新高策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyHighConfig {
    /// 是否只检查当天 - true: 只检查当天是否新高, false: 检查最近N天内是否创新高
    /// 默认 true
    pub check_today_only: bool,
    
    /// 当 check_today_only=false 时，检查最近N天内是否创新高
    /// 默认 1 天（即只检查当天）
    pub recent_days: usize,
}

impl Default for YearlyHighConfig {
    fn default() -> Self {
        Self {
            check_today_only: true,
            recent_days: 1,
        }
    }
}

impl StrategyConfigTrait for YearlyHighConfig {
    fn strategy_name(&self) -> &str {
        "年内新高策略"
    }
    
    fn validate(&self) -> Result<()> {
        if self.recent_days == 0 {
            bail!("检查天数必须大于0");
        }
        Ok(())
    }
    
    fn analysis_period(&self) -> usize {
        // 需要从年初到现在的数据，最多365天
        365
    }
}

/// 年内新高策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyHighResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 股票名称（可选，需要从外部传入）
    pub stock_name: Option<String>,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 前期高点价格
    pub previous_high: f64,
    
    /// 前期高点日期
    pub previous_high_date: NaiveDate,
    
    /// 距离前高的天数
    pub days_since_previous_high: i64,
    
    /// 是否为年内新高
    pub is_yearly_high: bool,
    
    /// 年初日期
    pub year_start_date: NaiveDate,
    
    /// 年内交易天数
    pub trading_days_in_year: usize,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5, 1最低5最高)
    pub risk_level: u8,
}

/// 年内新高策略
pub struct YearlyHighStrategy {
    config: YearlyHighConfig,
}

impl YearlyHighStrategy {
    /// 创建新的年内新高策略实例
    pub fn new(config: YearlyHighConfig) -> Self {
        Self { config }
    }
    
    
    /// 内部分析方法
    /// 
    /// 执行年内新高分析流程：
    /// 1. 获取当年数据（从年初到当天）
    /// 2. 查找年内最高价
    /// 3. 判断当天是否为年内新高
    /// 4. 生成策略信号
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) 
        -> Result<YearlyHighResult> {
        
        if data.is_empty() {
            bail!("数据为空");
        }
        
        let latest = data.last().unwrap();
        let current_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        let current_price = latest.close;
        let current_high = latest.high;
        
        // 1. 确定年初日期（当年1月1日）
        // 从日期字符串中提取年份（格式：YYYYMMDD）
        let current_year: i32 = latest.trade_date[..4].parse()
            .map_err(|_| anyhow::anyhow!("无法解析年份"))?;
        let year_start_date = NaiveDate::from_ymd_opt(current_year, 1, 1)
            .ok_or_else(|| anyhow::anyhow!("无法构造年初日期"))?;
        
        // 2. 筛选出年内数据（从年初到当天）
        let year_data: Vec<&SecurityData> = data.iter()
            .filter(|d| {
                if let Ok(date) = NaiveDate::parse_from_str(&d.trade_date, "%Y%m%d") {
                    date >= year_start_date && date <= current_date
                } else {
                    false
                }
            })
            .collect();
        
        if year_data.is_empty() {
            bail!("没有找到年内数据");
        }
        
        let trading_days_in_year = year_data.len();
        
        // 3. 查找年内前期最高价（根据配置决定是否排除当天）
        let (previous_high, previous_high_date, previous_high_index) = 
            self.find_year_high(&year_data, current_date)?;
        
        // 4. 判断是否为年内新高
        let is_yearly_high = if self.config.check_today_only {
            // 只检查当天：当天最高价 >= 年内前期最高价
            current_high >= previous_high
        } else {
            // 检查最近N天：最近N天内有任何一天的最高价 >= 年内前期最高价
            self.check_recent_new_high(&year_data, previous_high)?
        };
        
        // 5. 计算距离前高的天数
        let days_since_previous_high = trading_days_in_year as i64 - previous_high_index as i64 - 1;
        
        // 6. 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = 
            if is_yearly_high {
                (StrategySignal::Buy, 100, 3)
            } else {
                (StrategySignal::Sell, 0, 3)
            };
        
        let analysis_description = if is_yearly_high {
            format!("创年内新高，当前价 {:.2}，前高 {:.2} ({})", 
                current_price, previous_high, previous_high_date)
        } else {
            format!("未创年内新高，当前价 {:.2}，年内最高 {:.2} ({})", 
                current_price, previous_high, previous_high_date)
        };
        
        debug!("年内新高分析: {} - {}", symbol, analysis_description);
        
        Ok(YearlyHighResult {
            stock_code: symbol.to_string(),
            stock_name: None,
            analysis_date: current_date,
            current_price,
            previous_high,
            previous_high_date,
            days_since_previous_high,
            is_yearly_high,
            year_start_date,
            trading_days_in_year,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 查找年内最高价
    /// 
    /// 根据配置决定是否排除当天/最近N天
    /// 
    /// # 返回值
    /// (年内最高价, 最高价日期, 最高价索引)
    fn find_year_high(&self, year_data: &[&SecurityData], _current_date: NaiveDate) 
        -> Result<(f64, NaiveDate, usize)> {
        
        // 确定要搜索的数据范围
        let search_data = if self.config.check_today_only {
            // 排除当天，查找之前的最高价
            if year_data.len() <= 1 {
                // 如果只有当天数据，返回当天作为前高
                year_data
            } else {
                &year_data[..year_data.len() - 1]
            }
        } else {
            // 排除最近N天
            let exclude_count = self.config.recent_days.min(year_data.len());
            if year_data.len() <= exclude_count {
                year_data
            } else {
                &year_data[..year_data.len() - exclude_count]
            }
        };
        
        if search_data.is_empty() {
            // 如果没有历史数据，使用第一天作为基准
            let first = year_data[0];
            let first_date = NaiveDate::parse_from_str(&first.trade_date, "%Y%m%d")?;
            return Ok((first.high, first_date, 0));
        }
        
        // 查找最高价
        let mut max_price = 0.0;
        let mut max_index = 0;
        
        for (i, d) in search_data.iter().enumerate() {
            if d.high > max_price {
                max_price = d.high;
                max_index = i;
            }
        }
        
        let max_date = NaiveDate::parse_from_str(
            &search_data[max_index].trade_date, 
            "%Y%m%d"
        )?;
        
        debug!("年内前期最高: {:.2} 于 {}", max_price, max_date);
        
        Ok((max_price, max_date, max_index))
    }
    
    /// 检查最近N天是否创新高
    /// 
    /// 判断最近N天内是否有任何一天的最高价 >= 年内前期最高价
    fn check_recent_new_high(&self, year_data: &[&SecurityData], previous_high: f64) 
        -> Result<bool> {
        
        let recent_count = self.config.recent_days.min(year_data.len());
        let recent_data = &year_data[year_data.len() - recent_count..];
        
        for d in recent_data {
            if d.high >= previous_high {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}

impl Default for YearlyHighStrategy {
    fn default() -> Self {
        Self::new(YearlyHighConfig::default())
    }
}

impl TradingStrategy for YearlyHighStrategy {
    type Config = YearlyHighConfig;
    
    fn name(&self) -> &str { "年内新高策略" }
    fn description(&self) -> &str { "识别创出年内新高的强势股票" }
    fn config(&self) -> &Self::Config { &self.config }
    fn update_config(&mut self, config: Self::Config) -> Result<()> { 
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn analyze(&mut self, symbol: &str, data: &[SecurityData]) -> Result<StrategyResult> {
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::YearlyHigh(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation() {
        let config = YearlyHighConfig::default();
        assert!(config.validate().is_ok());
        
        let invalid_config = YearlyHighConfig {
            recent_days: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
    
    #[test]
    fn test_strategy_creation() {
        let strategy = YearlyHighStrategy::default();
        assert_eq!(strategy.name(), "年内新高策略");
        assert_eq!(strategy.config.check_today_only, true);
        assert_eq!(strategy.config.recent_days, 1);
    }
}
