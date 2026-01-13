//! 均线粘合策略
//! 
//! 识别均线粘合形态，筛选股价下跌后出现均线粘合的潜在机会
//! 
//! ## 策略原理
//! 
//! 均线粘合是技术分析中的重要形态，表示股价在震荡整理，均线系统趋于收敛。
//! 
//! ### 1. 均线粘合的含义
//! - **多头蓄势**: 多条均线靠近，表示市场趋于共识，正在积蓄能量
//! - **方向选择**: 粘合后往往会出现方向选择，向上突破概率较高
//! - **时间窗口**: 粘合持续越久，后续爆发力度可能越大
//! 
//! ### 2. 粘合判断标准
//! - **粘合度**: 多条均线之间的最大差异占比
//! - **粘合持续时间**: 均线保持在粘合状态的天数
//! - **参与均线数量**: 至少2条以上均线参与粘合
//! 
//! ### 3. 前期下跌要求
//! - **下跌确认**: 粘合前股价有过明显的下跌
//! - **下跌幅度**: 下跌达到一定幅度才符合策略
//! - **下跌持续性**: 不是单日大跌，而是持续下跌
//! 
//! ### 4. 策略核心逻辑
//! - 计算指定周期的移动平均线（5/10/20/60日）
//! - 判断至少2条均线是否粘合（差异小于阈值）
//! - 确认粘合前股价有过下跌
//! - 评估粘合的持续时间和稳定性
//! 
//! ### 5. 应用场景
//! - **底部形态**: 识别底部区域的均线粘合
//! - **蓄势待发**: 捕捉上涨前的蓄势形态
//! - **趋势反转**: 在下跌趋势后的粘合形态可能预示反转
//! 
//! ## 风险提示
//! - 粘合后可能向下突破，需要结合成交量判断
//! - 在弱势市场，粘合后可能继续下跌
//! - 需要关注后续的突破方向确认

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    SecurityData, TimeFrame,
};

/// 均线类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaType {
    MA5,
    MA10,
    MA20,
    MA60,
}

impl MaType {
    pub fn period(&self) -> usize {
        match self {
            MaType::MA5 => 5,
            MaType::MA10 => 10,
            MaType::MA20 => 20,
            MaType::MA60 => 60,
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            MaType::MA5 => "MA5",
            MaType::MA10 => "MA10",
            MaType::MA20 => "MA20",
            MaType::MA60 => "MA60",
        }
    }
}

/// 均线粘合策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MaConvergenceConfig {
    /// 参与粘合的均线列表（可选5/10/20/60）
    pub ma_types: Vec<String>,
    
    /// 粘合度阈值（均线的最大差异占比，0-1之间）
    /// 0.05表示最大差异不超过5%
    pub convergence_threshold: f64,
    
    /// 粘合持续时间（至少N天满足粘合条件）
    pub min_convergence_days: usize,
    
    /// 粘合前下跌确认周期（天数）
    pub decline_check_period: usize,
    
    /// 下跌幅度要求（百分比，0.1表示下跌10%以上）
    pub min_decline_pct: f64,
    
    /// 时间周期（日线或周线）
    pub time_frame: String,
    
    /// 最大粘合持续时间（天数，超过此时间可能失效）
    pub max_convergence_days: usize,

    pub recent_turnover_rate_min: f64,
    pub recent_turnover_rate_max: f64,
}

impl Default for MaConvergenceConfig {
    fn default() -> Self {
        Self {
            ma_types: vec!["MA5".to_string(), "MA10".to_string(), "MA20".to_string()],
            convergence_threshold: 0.05,  // 5%粘合度
            min_convergence_days: 3,      // 至少粘合3天
            decline_check_period: 20,     // 检查过去20天的下跌
            min_decline_pct: 0.10,        // 至少下跌10%
            time_frame: "daily".to_string(),
            max_convergence_days: 20,     // 最多粘合20天

            recent_turnover_rate_min: 0.0,
            recent_turnover_rate_max: 0.0,
        }
    }
}

impl StrategyConfigTrait for MaConvergenceConfig {
    fn strategy_name(&self) -> &str {
        "均线粘合策略"
    }
    
    fn analysis_period(&self) -> usize {
        let max_ma_period = self.ma_types.iter()
            .filter_map(|ma| match ma.as_str() {
                "MA5" => Some(5),
                "MA10" => Some(10),
                "MA20" => Some(20),
                "MA60" => Some(60),
                _ => None,
            })
            .max()
            .unwrap_or(60);
        
        max_ma_period.max(self.decline_check_period) + self.max_convergence_days
    }
    
    fn validate(&self) -> Result<()> {
        if self.ma_types.len() < 2 {
            anyhow::bail!("至少需要选择2条均线参与粘合判断");
        }
        
        if self.ma_types.len() > 4 {
            anyhow::bail!("最多选择4条均线");
        }
        
        // 验证均线类型
        let valid_ma = ["MA5", "MA10", "MA20", "MA60"];
        for ma in &self.ma_types {
            if !valid_ma.contains(&ma.as_str()) {
                anyhow::bail!("无效的均线类型: {}，可选: MA5, MA10, MA20, MA60", ma);
            }
        }
        
        if self.convergence_threshold < 0.01 || self.convergence_threshold > 0.15 {
            anyhow::bail!("粘合度阈值应在1%-15%之间");
        }
        
        if self.min_convergence_days < 2 || self.min_convergence_days > 15 {
            anyhow::bail!("粘合持续时间应在2-15天之间");
        }
        
        if self.decline_check_period < 5 || self.decline_check_period > 60 {
            anyhow::bail!("下跌确认周期应在5-60天之间");
        }
        
        if self.min_decline_pct < 0.05 || self.min_decline_pct > 0.50 {
            anyhow::bail!("下跌幅度要求应在5%-50%之间");
        }
        
        if self.max_convergence_days < self.min_convergence_days {
            anyhow::bail!("最大粘合时间应大于等于最小粘合时间");
        }
        
        if self.time_frame != "daily" && self.time_frame != "weekly" {
            anyhow::bail!("时间周期必须是 'daily' 或 'weekly'");
        }

        if (self.recent_turnover_rate_min > 0.0) || (self.recent_turnover_rate_max > 0.0) {
            if self.recent_turnover_rate_min < 0.0 || self.recent_turnover_rate_min > 100.0 {
                anyhow::bail!("换手率范围最小值应在0-100之间");
            }
            if self.recent_turnover_rate_max < 0.0 || self.recent_turnover_rate_max > 100.0 {
                anyhow::bail!("换手率范围最大值应在0-100之间");
            }
            if self.recent_turnover_rate_max <= 0.0 {
                anyhow::bail!("请设置换手率范围最大值（>0）");
            }
            if self.recent_turnover_rate_min > self.recent_turnover_rate_max {
                anyhow::bail!("换手率范围最小值不能大于最大值");
            }
        }
        
        Ok(())
    }
}

/// 均线粘合策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaConvergenceResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 策略特有字段
    /// 参与粘合的均线列表
    pub ma_types: Vec<String>,
    
    /// 各均线当前值
    pub ma_values: Vec<(String, f64)>,
    
    /// 粘合度（均线的最大差异占比）
    pub convergence_degree: f64,
    
    /// 粘合持续天数
    pub convergence_days: usize,
    
    /// 是否满足粘合条件
    pub is_converged: bool,
    
    /// 粘合前最高价
    pub decline_start_price: f64,
    
    /// 粘合前最低价
    pub decline_bottom_price: f64,
    
    /// 粘合前下跌幅度（百分比）
    pub decline_pct: f64,
    
    /// 是否满足下跌条件
    pub has_declined: bool,
    
    /// 时间周期
    pub time_frame: String,
    
    /// 粘合强度评分（0-100）
    pub convergence_strength: u8,
}

/// 均线粘合策略
pub struct MaConvergenceStrategy {
    config: MaConvergenceConfig,
}

impl MaConvergenceStrategy {
    pub fn new(config: MaConvergenceConfig) -> Self {
        Self { config }
    }
    
    /// 创建日线标准配置
    pub fn daily_standard() -> Self {
        Self {
            config: MaConvergenceConfig {
                ma_types: vec!["MA5".to_string(), "MA10".to_string(), "MA20".to_string()],
                convergence_threshold: 0.05,
                min_convergence_days: 3,
                decline_check_period: 20,
                min_decline_pct: 0.10,
                time_frame: "daily".to_string(),
                max_convergence_days: 20,
                ..Default::default()
            },
        }
    }
    
    /// 创建日线保守配置（更严格的条件）
    pub fn daily_conservative() -> Self {
        Self {
            config: MaConvergenceConfig {
                ma_types: vec!["MA5".to_string(), "MA10".to_string(), "MA20".to_string(), "MA60".to_string()],
                convergence_threshold: 0.04,
                min_convergence_days: 5,
                decline_check_period: 30,
                min_decline_pct: 0.15,
                time_frame: "daily".to_string(),
                max_convergence_days: 15,
                ..Default::default()
            },
        }
    }
    
    /// 创建日线激进配置（较宽松的条件）
    pub fn daily_aggressive() -> Self {
        Self {
            config: MaConvergenceConfig {
                ma_types: vec!["MA5".to_string(), "MA10".to_string()],
                convergence_threshold: 0.08,
                min_convergence_days: 2,
                decline_check_period: 15,
                min_decline_pct: 0.08,
                time_frame: "daily".to_string(),
                max_convergence_days: 25,
                ..Default::default()
            },
        }
    }
    
    /// 创建周线标准配置
    pub fn weekly_standard() -> Self {
        Self {
            config: MaConvergenceConfig {
                ma_types: vec!["MA5".to_string(), "MA10".to_string(), "MA20".to_string()],
                convergence_threshold: 0.06,
                min_convergence_days: 2,
                decline_check_period: 10,
                min_decline_pct: 0.12,
                time_frame: "weekly".to_string(),
                max_convergence_days: 10,
                ..Default::default()
            },
        }
    }
    
    /// 计算移动平均线
    fn calculate_ma(&self, data: &[SecurityData], period: usize, index: usize) -> Option<f64> {
        if period == 0 || index < period - 1 {
            return None;
        }
        
        let start = index.saturating_sub(period - 1);
        let sum: f64 = data[start..=index].iter().map(|d| d.close).sum();
        Some(sum / period as f64)
    }
    
    /// 计算所有指定均线
    fn calculate_mas(&self, data: &[SecurityData], index: usize) -> Vec<(String, f64)> {
        let mut ma_values = Vec::new();
        
        for ma_str in &self.config.ma_types {
            let period = match ma_str.as_str() {
                "MA5" => 5,
                "MA10" => 10,
                "MA20" => 20,
                "MA60" => 60,
                _ => continue,
            };
            
            if let Some(ma_value) = self.calculate_ma(data, period, index) {
                ma_values.push((ma_str.clone(), ma_value));
            }
        }
        
        ma_values
    }
    
    /// 计算粘合度
    fn calculate_convergence_degree(&self, ma_values: &[(String, f64)]) -> f64 {
        if ma_values.is_empty() {
            return 0.0;
        }
        
        let values: Vec<f64> = ma_values.iter().map(|(_, v)| *v).collect();
        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        if min_val == 0.0 {
            return 0.0;
        }
        
        (max_val - min_val) / min_val
    }
    
    /// 检查是否粘合
    fn is_converged(&self, ma_values: &[(String, f64)]) -> bool {
        let convergence_degree = self.calculate_convergence_degree(ma_values);
        convergence_degree <= self.config.convergence_threshold
    }
    
    /// 计算粘合持续天数
    fn calculate_convergence_days(&self, data: &[SecurityData], end_index: usize) -> usize {
        let mut convergence_days = 0;
        
        for i in (0..=end_index).rev() {
            let ma_values = self.calculate_mas(data, i);
            if ma_values.len() < self.config.ma_types.len() {
                break; // 数据不足
            }
            
            if self.is_converged(&ma_values) {
                convergence_days += 1;
            } else {
                break;
            }
        }
        
        convergence_days
    }
    
    /// 检查粘合前是否有下跌
    fn check_decline(&self, data: &[SecurityData], convergence_start_index: usize) -> (bool, f64, f64, f64) {
        if convergence_start_index < self.config.decline_check_period {
            return (false, 0.0, 0.0, 0.0);
        }
        
        // 获取检查周期内的最高价和最低价
        let check_range_start = convergence_start_index - self.config.decline_check_period;
        let check_range_end = convergence_start_index;
        
        let mut highest_price = f64::NEG_INFINITY;
        let mut lowest_price = f64::INFINITY;
        
        for i in check_range_start..=check_range_end {
            if data[i].high > highest_price {
                highest_price = data[i].high;
            }
            if data[i].low < lowest_price {
                lowest_price = data[i].low;
            }
        }
        
        // 计算下跌幅度
        let decline_pct = if highest_price > 0.0 {
            (highest_price - lowest_price) / highest_price
        } else {
            0.0
        };
        
        let has_declined = decline_pct >= self.config.min_decline_pct;
        
        (has_declined, highest_price, lowest_price, decline_pct)
    }
    
    /// 计算粘合强度评分
    fn calculate_convergence_strength(
        &self,
        convergence_degree: f64,
        convergence_days: usize,
        decline_pct: f64,
        ma_count: usize,
    ) -> u8 {
        let mut score = 0.0;
        
        // 粘合度评分（40分）- 粘合度越小得分越高
        let degree_score = ((self.config.convergence_threshold - convergence_degree) / self.config.convergence_threshold)
            .max(0.0)
            .min(1.0)
            * 40.0;
        score += degree_score;
        
        // 粘合天数评分（25分）
        let days_ratio = convergence_days as f64 / self.config.max_convergence_days as f64;
        let days_score = days_ratio.min(1.0) * 25.0;
        score += days_score;
        
        // 下跌幅度评分（20分）- 下跌越多得分越高
        let decline_score = ((decline_pct - self.config.min_decline_pct) / 0.30)
            .max(0.0)
            .min(1.0)
            * 20.0;
        score += decline_score;
        
        // 均线数量评分（15分）
        let ma_count_score = (ma_count as f64 / 4.0) * 15.0;
        score += ma_count_score;
        
        score.min(100.0) as u8
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<MaConvergenceResult> {
        let required_period = self.config.analysis_period();
        
        if data.len() < required_period {
            anyhow::bail!(
                "数据不足: 需要 {} 个数据点，实际 {} 个",
                required_period,
                data.len()
            );
        }
        
        // 按日期排序（从旧到新）
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 取最新的数据点作为分析基准
        let latest_index = sorted_data.len() - 1;
        let latest = &sorted_data[latest_index];
        
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 计算当前均线值
        let ma_values = self.calculate_mas(&sorted_data, latest_index);
        
        // 计算粘合度
        let convergence_degree = self.calculate_convergence_degree(&ma_values);
        
        // 检查是否粘合
        let is_converged = self.is_converged(&ma_values);
        
        // 计算粘合持续天数
        let convergence_days = if is_converged {
            self.calculate_convergence_days(&sorted_data, latest_index)
        } else {
            0
        };
        
        // 检查粘合前是否有下跌
        let convergence_start_index = if convergence_days > 0 {
            latest_index.saturating_sub(convergence_days)
        } else {
            latest_index
        };
        
        let (has_declined, decline_start_price, decline_bottom_price, decline_pct) = 
            self.check_decline(&sorted_data, convergence_start_index);
        
        // 生成分析说明
        let mut conditions_met = Vec::new();
        let mut conditions_failed = Vec::new();
        
        // 1. 检查粘合条件
        if is_converged {
            conditions_met.push(format!(
                "均线粘合度{:.1}%（要求≤{:.1}%）",
                convergence_degree * 100.0,
                self.config.convergence_threshold * 100.0
            ));
            
            if convergence_days >= self.config.min_convergence_days {
                conditions_met.push(format!("持续粘合{}天", convergence_days));
            } else {
                conditions_failed.push(format!(
                    "粘合天数不足（{}天，要求≥{}天）",
                    convergence_days,
                    self.config.min_convergence_days
                ));
            }
        } else {
            conditions_failed.push(format!(
                "均线未粘合（当前{:.1}%，要求≤{:.1}%）",
                convergence_degree * 100.0,
                self.config.convergence_threshold * 100.0
            ));
        }
        
        // 2. 检查下跌条件
        if has_declined {
            conditions_met.push(format!("粘合前下跌{:.1}%（要求≥{:.1}%）", 
                decline_pct * 100.0, self.config.min_decline_pct * 100.0));
        } else {
            conditions_failed.push(format!(
                "前期下跌不足（{:.1}%，要求≥{:.1}%）",
                decline_pct * 100.0,
                self.config.min_decline_pct * 100.0
            ));
        }

        // 3. 检查最近5天换手率（至少2天落在范围内）
        if self.config.recent_turnover_rate_max > 0.0 {
            let start = latest_index.saturating_sub(4);
            let mut days_ok = 0usize;
            for i in start..=latest_index {
                let tr = sorted_data[i].turnover_rate.unwrap_or(0.0);
                if tr >= self.config.recent_turnover_rate_min && tr <= self.config.recent_turnover_rate_max {
                    days_ok += 1;
                }
            }

            if days_ok >= 2 {
                conditions_met.push(format!(
                    "近5日换手率在[{:.2}%,{:.2}%]的天数{}（要求≥2）",
                    self.config.recent_turnover_rate_min,
                    self.config.recent_turnover_rate_max,
                    days_ok
                ));
            } else {
                conditions_failed.push(format!(
                    "近5日换手率在[{:.2}%,{:.2}%]的天数不足（{}天，要求≥2天）",
                    self.config.recent_turnover_rate_min,
                    self.config.recent_turnover_rate_max,
                    days_ok
                ));
            }
        }
        
        // 4. 检查粘合持续时间上限
        if convergence_days <= self.config.max_convergence_days {
            conditions_met.push("粘合时间在有效范围内".to_string());
        } else {
            conditions_failed.push(format!(
                "粘合时间过长（{}天，要求≤{}天）",
                convergence_days,
                self.config.max_convergence_days
            ));
        }
        
        // 判断是否满足所有条件
        let all_conditions_met = conditions_failed.is_empty();
        
        // 计算粘合强度
        let convergence_strength = self.calculate_convergence_strength(
            convergence_degree,
            convergence_days,
            decline_pct,
            ma_values.len(),
        );
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = if all_conditions_met {
            self.generate_signal(
                convergence_degree,
                convergence_days,
                decline_pct,
                convergence_strength,
                ma_values.len(),
            )
        } else {
            (StrategySignal::Hold, 0, 3)
        };
        
        // 生成分析说明
        let analysis_description = if all_conditions_met {
            format!(
                "均线粘合信号：{} | 粘合强度{}分",
                conditions_met.join("，"),
                convergence_strength
            )
        } else {
            format!(
                "不符合条件：{}",
                conditions_failed.join("；")
            )
        };
        
        // 转换时间周期
        let time_frame_display = match self.config.time_frame.as_str() {
            "daily" => "日线",
            "weekly" => "周线",
            _ => &self.config.time_frame,
        };
        
        debug!(
            "股票 {}: 粘合度{:.1}%, 粘合天数{}, 下跌{:.1}%, 粘合强度{}, 信号={:?}",
            symbol, 
            convergence_degree * 100.0, 
            convergence_days, 
            decline_pct * 100.0, 
            convergence_strength, 
            strategy_signal
        );
        
        Ok(MaConvergenceResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            ma_types: self.config.ma_types.clone(),
            ma_values,
            convergence_degree,
            convergence_days,
            is_converged,
            decline_start_price,
            decline_bottom_price,
            decline_pct,
            has_declined,
            time_frame: time_frame_display.to_string(),
            convergence_strength,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        convergence_degree: f64,
        convergence_days: usize,
        decline_pct: f64,
        convergence_strength: u8,
        ma_count: usize,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = convergence_strength;
        let mut risk_level = 3u8;
        
        // 调整信号强度
        // 粘合度越小，信号越强
        let degree_bonus = if convergence_degree < self.config.convergence_threshold * 0.5 {
            10
        } else {
            0
        };
        
        // 粘合天数适中加分
        let days_bonus = if convergence_days >= self.config.min_convergence_days 
            && convergence_days <= self.config.max_convergence_days / 2 {
            5
        } else {
            0
        };
        
        // 均线数量加分
        let ma_bonus = if ma_count >= 3 { 5 } else { 0 };
        
        signal_strength = (signal_strength as i32 + degree_bonus + days_bonus + ma_bonus)
            .min(100)
            .max(0) as u8;
        
        // 根据粘合强度调整风险等级
        if convergence_strength >= 80 {
            risk_level = 2; // 强粘合，低风险
        } else if convergence_strength >= 60 {
            risk_level = 3; // 中等粘合，中等风险
        } else {
            risk_level = 4; // 弱粘合，较高风险
        }
        
        // 根据粘合天数调整风险
        if convergence_days > self.config.max_convergence_days / 2 {
            risk_level = risk_level.saturating_sub(1); // 粘合时间长，降低风险
        }
        
        let strategy_signal = if signal_strength >= 80 {
            StrategySignal::StrongBuy
        } else if signal_strength >= 65 {
            StrategySignal::Buy
        } else if signal_strength >= 50 {
            StrategySignal::Hold
        } else {
            StrategySignal::Sell
        };
        
        (strategy_signal, signal_strength, risk_level.max(1).min(5))
    }
}

impl Default for MaConvergenceStrategy {
    fn default() -> Self {
        Self::new(MaConvergenceConfig::default())
    }
}

impl TradingStrategy for MaConvergenceStrategy {
    type Config = MaConvergenceConfig;
    
    fn name(&self) -> &str {
        "均线粘合策略"
    }
    
    fn description(&self) -> &str {
        "识别均线粘合形态，筛选股价下跌后出现均线粘合的潜在机会"
    }
    
    fn config(&self) -> &Self::Config {
        &self.config
    }
    
    fn update_config(&mut self, config: Self::Config) -> Result<()> {
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn analyze(&mut self, symbol: &str, data: &[SecurityData]) -> Result<StrategyResult> {
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::MaConvergence(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data(
        close: f64, 
        volume: f64,
        date: &str
    ) -> SecurityData {
        SecurityData {
            trade_date: date.to_string(),
            symbol: "TEST001".to_string(),
            open: close,
            high: close * 1.01,
            low: close * 0.99,
            close,
            pre_close: Some(close),
            change: Some(0.0),
            volume,
            amount: volume * close,
            turnover_rate: Some(1.0),
            pct_change: Some(0.0),
            time_frame: crate::strategy::traits::TimeFrame::Daily,
            security_type: crate::strategy::traits::SecurityType::Stock,
            financial_data: None,
            target: None,
        }
    }
    
    #[test]
    fn test_ma_calculation() {
        let mut strategy = MaConvergenceStrategy::daily_standard();
        
        let mut data = Vec::new();
        // 创建60个数据点
        for i in 0..60 {
            data.push(create_test_data(100.0, 1000000.0, &format!("202401{:02}", i + 1)));
        }
        
        // 测试MA5计算
        if let Some(ma5) = strategy.calculate_ma(&data, 5, 59) {
            assert_eq!(ma5, 100.0);
        }
        
        // 测试数据不足的情况
        let result = strategy.calculate_ma(&data, 60, 59);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_convergence_detection() {
        let mut strategy = MaConvergenceStrategy::daily_standard();
        
        // 创建粘合数据：价格在99-101之间震荡
        let mut data = Vec::new();
        for i in 0..30 {
            let price = 100.0 + (i % 3) as f64 - 1.0; // 99, 100, 101 循环
            data.push(create_test_data(price, 1000000.0, &format!("202401{:02}", i + 1)));
        }
        
        // 添加前期下跌数据
        let mut decline_data = Vec::new();
        for i in 0..30 {
            let price = 120.0 - (i as f64 * 0.7); // 从120跌到99
            decline_data.push(create_test_data(price, 1000000.0, &format!("202312{:02}", i + 1)));
        }
        
        // 合并数据
        let mut all_data = decline_data;
        all_data.extend(data);
        
        let result = strategy.analyze("TEST001", &all_data).unwrap();
        
        if let StrategyResult::MaConvergence(r) = result {
            assert!(r.convergence_degree < 0.05, "应该检测到粘合");
            assert!(r.has_declined, "应该检测到前期下跌");
            assert!(r.signal_strength > 0, "应该有信号强度");
        }
    }
    
    #[test]
    fn test_config_validation() {
        let config = MaConvergenceConfig::default();
        assert!(config.validate().is_ok());
        
        let invalid_config = MaConvergenceConfig {
            ma_types: vec!["MA5".to_string()], // 只有一条均线
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
        
        let invalid_config2 = MaConvergenceConfig {
            ma_types: vec!["MA5".to_string(), "MA99".to_string()], // 无效的均线类型
            ..Default::default()
        };
        assert!(invalid_config2.validate().is_err());
        
        let invalid_config3 = MaConvergenceConfig {
            convergence_threshold: 0.20, // 超出范围
            ..Default::default()
        };
        assert!(invalid_config3.validate().is_err());
    }
}
