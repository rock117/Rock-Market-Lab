//! 底部放量上涨策略
//! 
//! 判断某个证券近N日内是否出现底部放量并且价格上涨的信号
//! 
//! 核心条件：
//! 1. 处于底部区域（价格在底部波动范围内）
//! 2. 成交量放大（超过均量的指定倍数）
//! 3. 价格上涨（相对底部价格上涨超过阈值）
//! 4. 当天涨幅达标（相对前一天收盘价）
//! 5. 当天收阳线（收盘价 >= 开盘价）

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    BottomVolumeSurgeResult, SecurityData,
};

/// 底部放量上涨策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomVolumeSurgeConfig {
    /// 分析周期（天数）
    pub analysis_period: usize,
    
    /// 成交量均线周期
    pub volume_ma_period: usize,
    
    /// 成交量放大倍数阈值
    pub volume_surge_threshold: f64,
    
    /// 价格上涨阈值（百分比）
    pub price_rise_threshold: f64,
    
    /// 底部判断周期（天数）
    pub bottom_period: usize,
    
    /// 底部价格波动范围（百分比）
    pub bottom_price_range: f64,
    
    /// 当天最小上涨幅度（百分比，相对前一天收盘价）
    pub min_daily_rise_pct: f64,
}

impl Default for BottomVolumeSurgeConfig {
    fn default() -> Self {
        Self {
            analysis_period: 30,
            volume_ma_period: 5,
            volume_surge_threshold: 3.0,
            price_rise_threshold: 2.0,
            bottom_period: 10,
            bottom_price_range: 10.0,
            min_daily_rise_pct: 3.0,  // 默认要求当天至少上涨0.5%
        }
    }
}

impl StrategyConfigTrait for BottomVolumeSurgeConfig {
    fn strategy_name(&self) -> &str {
        "BottomVolumeSurge"
    }
    
    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.analysis_period == 0 {
            return Err(anyhow::anyhow!("分析周期不能为0"));
        }
        if self.volume_ma_period == 0 {
            return Err(anyhow::anyhow!("成交量均线周期不能为0"));
        }
        if self.volume_ma_period > self.analysis_period {
            return Err(anyhow::anyhow!("成交量均线周期不能大于分析周期"));
        }
        if self.bottom_period > self.analysis_period {
            return Err(anyhow::anyhow!("底部判断周期不能大于分析周期"));
        }
        Ok(())
    }
}

// BottomVolumeSurgeResult 现在在 traits.rs 中定义

/// 底部放量上涨策略
pub struct BottomVolumeSurgeStrategy {
    config: BottomVolumeSurgeConfig,
}

impl BottomVolumeSurgeStrategy {
    /// 创建新的策略实例
    pub fn new(config: BottomVolumeSurgeConfig) -> Self {
        Self { config }
    }
    
    /// 使用默认配置创建
    pub fn default_config() -> Self {
        Self::new(BottomVolumeSurgeConfig::default())
    }
    
    /// 保守配置（更严格的条件）
    pub fn conservative() -> Self {
        Self::new(BottomVolumeSurgeConfig {
            analysis_period: 30,
            volume_ma_period: 10,
            volume_surge_threshold: 2.0,  // 需要2倍放量
            price_rise_threshold: 3.0,     // 需要3%涨幅
            bottom_period: 15,
            bottom_price_range: 3.0,       // 底部波动范围更小
            min_daily_rise_pct: 1.0,       // 当天至少上涨1%
        })
    }
    /// 激进配置（更宽松的条件）
    pub fn aggressive() -> Self {
        Self::new(BottomVolumeSurgeConfig {
            analysis_period: 15,
            volume_ma_period: 3,
            volume_surge_threshold: 1.2,   // 1.2倍放量即可
            price_rise_threshold: 1.0,     // 1%涨幅即可
            bottom_period: 7,
            bottom_price_range: 8.0,       // 底部波动范围更大
            min_daily_rise_pct: 0.1,       // 当天至少上涨0.1%
        })
    }
    
    /// 分析内部实现
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<BottomVolumeSurgeResult> {
        // 按日期排序
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 获取最新数据
        let latest = sorted_data.last()
            .ok_or_else(|| anyhow::anyhow!("数据为空"))?;
        let current_price = latest.close;
        let current_volume = latest.volume;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;

        // 检查当天是否为阳线（收盘价 >= 开盘价）
        if latest.close < latest.open {
            // 当天收阴线，不符合条件
            return Ok(BottomVolumeSurgeResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price,
                strategy_signal: StrategySignal::Sell,
                signal_strength: 0,
                analysis_description: format!(
                    "当天收阴线（开盘: {:.2}, 收盘: {:.2}），不符合放量上涨条件",
                    latest.open, latest.close
                ),
                risk_level: 5,
                is_at_bottom: false,
                bottom_price: 0.0,
                bottom_date: String::new(),
                current_volume,
                volume_ma: 0.0,
                volume_surge_ratio: 0.0,
                price_rise_pct: 0.0,
                recent_low: 0.0,
                recent_high: 0.0,
            });
        }

        // 0. 检查当天是否上涨（相对于前一天）
        if sorted_data.len() < 2 {
            return Ok(BottomVolumeSurgeResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price,
                strategy_signal: StrategySignal::Hold,
                signal_strength: 0,
                analysis_description: "数据不足，无法判断".to_string(),
                risk_level: 3,
                is_at_bottom: false,
                bottom_price: 0.0,
                bottom_date: String::new(),
                current_volume,
                volume_ma: 0.0,
                volume_surge_ratio: 0.0,
                price_rise_pct: 0.0,
                recent_low: 0.0,
                recent_high: 0.0,
            });
        }
        
        let previous = &sorted_data[sorted_data.len() - 2];
        let daily_rise_pct = ((latest.close - previous.close) / previous.close) * 100.0;
        
        if daily_rise_pct < self.config.min_daily_rise_pct {
            // 当天价格上涨幅度不足，不符合放量上涨条件
            return Ok(BottomVolumeSurgeResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price,
                strategy_signal: StrategySignal::Sell,
                signal_strength: 0,
                analysis_description: format!(
                    "当天涨幅不足（前一天收盘: {:.2}, 当天收盘: {:.2}, 涨幅: {:.2}%, 要求: {:.2}%），不符合放量上涨条件", 
                    previous.close, latest.close, daily_rise_pct, self.config.min_daily_rise_pct
                ),
                risk_level: 5,
                is_at_bottom: false,
                bottom_price: 0.0,
                bottom_date: String::new(),
                current_volume,
                volume_ma: 0.0,
                volume_surge_ratio: 0.0,
                price_rise_pct: 0.0,
                recent_low: 0.0,
                recent_high: 0.0,
            });
        }

        // 1. 判断是否处于底部（不包括当天数据）
        let historical_data = if sorted_data.len() > 1 {
            &sorted_data[..sorted_data.len() - 1]
        } else {
            return Ok(BottomVolumeSurgeResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price,
                strategy_signal: StrategySignal::Hold,
                signal_strength: 0,
                analysis_description: "历史数据不足".to_string(),
                risk_level: 3,
                is_at_bottom: false,
                bottom_price: 0.0,
                bottom_date: String::new(),
                current_volume,
                volume_ma: 0.0,
                volume_surge_ratio: 0.0,
                price_rise_pct: 0.0,
                recent_low: 0.0,
                recent_high: 0.0,
            });
        };
        
        let (is_at_bottom, bottom_price, bottom_date, recent_low, recent_high) = 
            self.check_bottom(historical_data)?;
        
        // 2. 计算成交量均值和放大倍数
        let (volume_ma, volume_surge_ratio) = self.calculate_volume_surge(&sorted_data)?;
        
        // 3. 计算价格涨幅
        let price_rise_pct = ((current_price - bottom_price) / bottom_price) * 100.0;
        
        // 4. 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = 
            self.generate_signal(
                is_at_bottom,
                volume_surge_ratio,
                price_rise_pct,
                current_price,
                recent_low,
                recent_high
            );
        
        // 5. 生成分析说明
        let analysis_description = self.generate_description(
            is_at_bottom,
            volume_surge_ratio,
            price_rise_pct,
            bottom_price,
            current_price
        );
        
        Ok(BottomVolumeSurgeResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            is_at_bottom,
            bottom_price,
            bottom_date,
            current_volume,
            volume_ma,
            volume_surge_ratio,
            price_rise_pct,
            recent_low,
            recent_high,
        })
    }
    
    /// 判断是否处于底部（不包括当天数据）
    /// 返回：(是否底部, 底部价格, 底部日期, 近期最低价, 近期最高价)
    /// 
    /// 底部判断条件（需同时满足）：
    /// 1. 价格波动范围小于阈值（横盘特征）
    /// 2. 最低价出现在前80%时间段内（不是持续下跌）
    /// 3. 最后一天价格不低于最低价太多（已经企稳）
    /// 4. 近期没有持续的下跌趋势
    fn check_bottom(&self, data: &[SecurityData]) -> Result<(bool, f64, String, f64, f64)> {
        if data.len() < self.config.bottom_period {
            return Ok((false, 0.0, String::new(), 0.0, 0.0));
        }
        
        // 取最近 bottom_period 天的数据（不包括当天）
        let recent_data = &data[data.len().saturating_sub(self.config.bottom_period)..];
        
        // 找到最低价和最高价
        let mut min_price = f64::MAX;
        let mut max_price = f64::MIN;
        let mut bottom_date = String::new();
        let mut bottom_index = 0;
        
        for (i, d) in recent_data.iter().enumerate() {
            if d.low < min_price {
                min_price = d.low;
                bottom_date = d.trade_date.clone();
                bottom_index = i;
            }
            if d.high > max_price {
                max_price = d.high;
            }
        }
        
        // 计算价格波动范围
        let price_range_pct = ((max_price - min_price) / min_price) * 100.0;
        
        // 条件1：价格波动范围小于阈值
        let condition1_low_volatility = price_range_pct <= self.config.bottom_price_range;
        
        // 条件2：最低价出现在前80%时间段内（不是最近才创新低）
        // 允许最低价出现得稍晚一些，但不能是最后几天
        let condition2_bottom_not_recent = bottom_index < (recent_data.len() * 4 / 5);
        
        // 条件3：最后一天价格不低于最低价太多（已经企稳或反弹）
        // 注意：这里的 recent_data 已经不包括当天了
        let last_price = recent_data.last().unwrap().close;
        let price_from_bottom_pct = ((last_price - min_price) / min_price) * 100.0;
        let condition3_price_stable = price_from_bottom_pct >= -1.0; // 允许略低于最低价1%
        
        // 条件4：检查是否有持续下跌趋势
        // 将数据分为前半段和后半段，比较平均价格
        let mid_point = recent_data.len() / 2;
        let first_half_avg = recent_data[..mid_point]
            .iter()
            .map(|d| d.close)
            .sum::<f64>() / mid_point as f64;
        let second_half_avg = recent_data[mid_point..]
            .iter()
            .map(|d| d.close)
            .sum::<f64>() / (recent_data.len() - mid_point) as f64;
        
        let trend_pct = ((second_half_avg - first_half_avg) / first_half_avg) * 100.0;
        
        // 后半段价格不应该明显低于前半段（不是持续下跌）
        let condition4_no_downtrend = trend_pct >= -5.0; // 允许小幅下跌5%
        
        // 综合判断：核心条件必须满足，其他条件满足大部分即可
        // 核心条件：低波动 + 价格企稳
        // 辅助条件：底部非最近 或 无下跌趋势（至少满足一个）
        let is_at_bottom = condition1_low_volatility 
            && condition3_price_stable 
            && (condition2_bottom_not_recent || condition4_no_downtrend);
        
        debug!(
            "底部判断: 最低价={:.2}, 最高价={:.2}, 波动范围={:.2}%, 最低价位置={}/{}, 当前相对底部={:.2}%, 趋势={:.2}%",
            min_price, max_price, price_range_pct, bottom_index, recent_data.len(), price_from_bottom_pct, trend_pct
        );
        debug!(
            "  条件1(低波动)={}, 条件2(底部非最近)={}, 条件3(价格企稳)={}, 条件4(无下跌趋势)={}, 最终判断={}",
            condition1_low_volatility, condition2_bottom_not_recent, condition3_price_stable, condition4_no_downtrend, is_at_bottom
        );
        
        Ok((is_at_bottom, min_price, bottom_date, min_price, max_price))
    }
    
    /// 计算成交量放大倍数
    /// 返回：(成交量均值, 放大倍数)
    fn calculate_volume_surge(&self, data: &[SecurityData]) -> Result<(f64, f64)> {
        if data.len() < self.config.volume_ma_period {
            return Ok((0.0, 0.0));
        }
        
        let latest = data.last().unwrap();
        let current_volume = latest.volume;
        
        // 计算成交量均值（不包括最新一天）
        let volume_sum: f64 = data.iter()
            .rev()
            .skip(1)  // 跳过最新一天
            .take(self.config.volume_ma_period)
            .map(|d| d.volume)
            .sum();
        
        let volume_ma = volume_sum / self.config.volume_ma_period as f64;
        
        // 计算放大倍数
        let volume_surge_ratio = current_volume / volume_ma;
        
        debug!(
            "成交量分析: 当前成交量={}, 均值={}, 放大倍数={}",
            current_volume, volume_ma, volume_surge_ratio
        );
        
        Ok((volume_ma, volume_surge_ratio))
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        is_at_bottom: bool,
        volume_surge_ratio: f64,
        price_rise_pct: f64,
        current_price: f64,
        recent_low: f64,
        recent_high: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;  // 默认中等风险
        
        // 底部判断得分 (40分)
        if is_at_bottom {
            signal_strength += 40;
        }
        
        // 成交量放大得分 (30分)
        if volume_surge_ratio >= self.config.volume_surge_threshold * 2.0 {
            signal_strength += 30;
        } else if volume_surge_ratio >= self.config.volume_surge_threshold * 1.5 {
            signal_strength += 20;
        } else if volume_surge_ratio >= self.config.volume_surge_threshold {
            signal_strength += 15;
        }
        
        // 价格上涨得分 (30分)
        if price_rise_pct >= self.config.price_rise_threshold * 2.0 {
            signal_strength += 30;
        } else if price_rise_pct >= self.config.price_rise_threshold * 1.5 {
            signal_strength += 20;
        } else if price_rise_pct >= self.config.price_rise_threshold {
            signal_strength += 15;
        }
        
        // 价格位置分析（影响风险等级）
        let price_position = if recent_high > recent_low {
            (current_price - recent_low) / (recent_high - recent_low)
        } else {
            0.5
        };
        
        // 根据价格位置调整风险等级
        if price_position < 0.3 {
            risk_level = 2;  // 低位，风险较低
        } else if price_position > 0.7 {
            risk_level = 4;  // 高位，风险较高
        }
        
        // 根据信号强度确定策略信号
        let strategy_signal = if signal_strength >= 80 {
            StrategySignal::StrongBuy
        } else if signal_strength >= 60 {
            StrategySignal::Buy
        } else if signal_strength >= 40 {
            StrategySignal::Hold
        } else if signal_strength >= 20 {
            StrategySignal::Sell
        } else {
            StrategySignal::StrongSell
        };
        
        info!(
            "信号生成: 底部={}, 放量倍数={:.2}, 涨幅={:.2}%, 信号强度={}, 信号={:?}",
            is_at_bottom, volume_surge_ratio, price_rise_pct, signal_strength, strategy_signal
        );
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        is_at_bottom: bool,
        volume_surge_ratio: f64,
        price_rise_pct: f64,
        bottom_price: f64,
        current_price: f64,
    ) -> String {
        let mut parts = Vec::new();
        
        if is_at_bottom {
            parts.push(format!("处于底部区域（底部价格: {:.2}）", bottom_price));
        } else {
            parts.push("未处于明显底部".to_string());
        }
        
        if volume_surge_ratio >= self.config.volume_surge_threshold {
            parts.push(format!("成交量放大 {:.2} 倍", volume_surge_ratio));
        } else {
            parts.push(format!("成交量未明显放大（{:.2}倍）", volume_surge_ratio));
        }
        
        if price_rise_pct >= self.config.price_rise_threshold {
            parts.push(format!("价格上涨 {:.2}%", price_rise_pct));
        } else if price_rise_pct > 0.0 {
            parts.push(format!("价格小幅上涨 {:.2}%", price_rise_pct));
        } else {
            parts.push(format!("价格下跌 {:.2}%", price_rise_pct.abs()));
        }
        
        parts.push(format!("当前价格: {:.2}", current_price));
        
        parts.join("，")
    }
}

impl TradingStrategy for BottomVolumeSurgeStrategy {
    type Config = BottomVolumeSurgeConfig;
    
    fn name(&self) -> &str {
        "底部放量上涨策略"
    }
    
    fn description(&self) -> &str {
        "判断证券是否处于底部区域，并出现放量上涨信号。\
         适用于捕捉底部反转机会。"
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
        // 验证数据
        self.validate_data(data)?;
        
        // 执行分析并包装为 enum
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::BottomVolumeSurge(result))
    }
    
    fn required_data_points(&self) -> usize {
        self.config.analysis_period.max(self.config.bottom_period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::traits::{SecurityType, TimeFrame};
    
    fn create_test_data_uptrend() -> Vec<SecurityData> {
        // 创建底部横盘后放量上涨的数据
        let mut data = vec![];
        
        // 前12天：底部横盘（价格在9.9-10.3之间，成交量正常）
        for i in 0..12 {
            let day_num = i + 1;
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: if day_num < 10 {
                    format!("2024010{}", day_num)
                } else {
                    format!("202401{}", day_num)
                },
                open: 10.0,
                high: 10.3,
                low: 9.9,
                close: 10.0 + (i % 3) as f64 * 0.1,  // 在10.0-10.2之间波动
                pre_close: Some(10.0),
                change: Some(0.0),
                pct_change: Some(0.0),
                volume: 1000000.0,
                amount: 10000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        // 第13-19天：继续横盘，成交量正常
        for i in 12..19 {
            let day_num = i + 1;
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("202401{}", day_num),
                open: 10.0,
                high: 10.25,
                low: 9.95,
                close: 10.0 + ((i - 12) % 3) as f64 * 0.08,  // 在10.0-10.16之间波动
                pre_close: Some(10.0),
                change: Some(0.0),
                pct_change: Some(0.0),
                volume: 1000000.0,
                amount: 10000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        // 最后一天：明显放量上涨
        data.push(SecurityData {
            symbol: "TEST".to_string(),
            trade_date: "20240120".to_string(),
            open: 10.1,
            high: 10.5,
            low: 10.05,
            close: 10.3,  // 相对底部9.9上涨约4%
            pre_close: Some(10.08),
            change: Some(0.22),
            pct_change: Some(2.2),
            volume: 2000000.0,  // 2倍放量
            amount: 20600000.0,
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
        });
        
        data
    }
    
    #[test]
    fn test_bottom_volume_surge_detection() {
        let strategy = BottomVolumeSurgeStrategy::default_config();
        let data = create_test_data_uptrend();
        
        let result = strategy.analyze_internal("TEST", &data);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.stock_code, "TEST");
        
        // 打印调试信息
        println!("信号: {:?}", result.strategy_signal);
        println!("信号强度: {}", result.signal_strength);
        println!("是否底部: {}", result.is_at_bottom);
        println!("成交量放大倍数: {:.2}", result.volume_surge_ratio);
        println!("价格涨幅: {:.2}%", result.price_rise_pct);
        println!("说明: {}", result.analysis_description);
        
        // 应该是买入或强烈买入信号
        assert!(
            result.strategy_signal == StrategySignal::Buy 
            || result.strategy_signal == StrategySignal::StrongBuy,
            "期望 Buy 或 StrongBuy，实际得到 {:?}，信号强度: {}",
            result.strategy_signal,
            result.signal_strength
        );
        
        // 信号强度应该较高
        assert!(result.signal_strength >= 60);
    }
    
    #[test]
    fn test_conservative_strategy() {
        let strategy = BottomVolumeSurgeStrategy::conservative();
        let data = create_test_data_uptrend();
        
        let result = strategy.analyze_internal("TEST", &data);
        assert!(result.is_ok());
        
        // 保守策略要求更严格，信号强度可能较低
        let result = result.unwrap();
        assert!(result.signal_strength > 0);
    }
    
    #[test]
    fn test_aggressive_strategy() {
        let strategy = BottomVolumeSurgeStrategy::aggressive();
        let data = create_test_data_uptrend();
        
        let result = strategy.analyze_internal("TEST", &data);
        assert!(result.is_ok());
        
        // 激进策略更容易触发买入信号
        let result = result.unwrap();
        // 激进策略阈值更低，但仍需要满足基本条件
        assert!(result.signal_strength >= 40);
        assert!(
            result.strategy_signal == StrategySignal::Buy 
            || result.strategy_signal == StrategySignal::StrongBuy
            || result.strategy_signal == StrategySignal::Hold
        );
    }
    
    /// 创建单边下跌的测试数据
    fn create_test_data_downtrend() -> Vec<SecurityData> {
        let mut data = vec![];
        
        // 20天持续小幅下跌，每天跌0.5%左右
        for i in 0..20 {
            let base_price = 10.0 - (i as f64 * 0.05);  // 从10.0跌到9.0
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("202401{:02}", i + 1),  // 20240101-20240120
                open: base_price + 0.05,
                high: base_price + 0.08,
                low: base_price - 0.02,
                close: base_price,
                pre_close: Some(base_price + 0.05),
                change: Some(-0.05),
                pct_change: Some(-0.5),
                volume: 1000000.0,
                amount: base_price * 1000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        data
    }
    
    #[test]
    fn test_downtrend_not_bottom() {
        let strategy = BottomVolumeSurgeStrategy::default_config();
        let data = create_test_data_downtrend();
        
        let result = strategy.analyze_internal("TEST", &data);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        
        // 单边下跌不应该被判断为底部
        // 即使波动范围可能不大，但趋势向下
        // 信号强度应该很低
        assert!(
            result.strategy_signal == StrategySignal::Sell 
            || result.strategy_signal == StrategySignal::StrongSell
            || result.strategy_signal == StrategySignal::Hold
        );
        
        // 信号强度应该较低
        assert!(result.signal_strength < 60);
    }
    
    /// 创建先下跌后企稳的测试数据
    fn create_test_data_stabilizing() -> Vec<SecurityData> {
        let mut data = vec![];
        
        // 前10天：下跌
        for i in 0..10 {
            let base_price = 10.0 - (i as f64 * 0.1);  // 从10.0跌到9.0
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("2024010{}", i + 1),  // 20240101-20240110
                open: base_price + 0.05,
                high: base_price + 0.08,
                low: base_price - 0.02,
                close: base_price,
                pre_close: Some(base_price + 0.1),
                change: Some(-0.1),
                pct_change: Some(-1.0),
                volume: 1000000.0,
                amount: base_price * 1000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        // 后10天：在9.0附近横盘
        for i in 10..20 {
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("202401{}", i + 1),  // 20240111-20240120
                open: 8.95,
                high: 9.1,
                low: 8.9,
                close: 9.0 + ((i - 10) % 3) as f64 * 0.03,
                pre_close: Some(9.0),
                change: Some(0.0),
                pct_change: Some(0.0),
                volume: 1000000.0,
                amount: 9000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        // 最后一天：放量上涨
        data.push(SecurityData {
            symbol: "TEST".to_string(),
            trade_date: "20240121".to_string(),
            open: 9.1,
            high: 9.5,
            low: 9.0,
            close: 9.4,  // 相对底部8.9上涨约5.6%
            pre_close: Some(9.0),
            change: Some(0.4),
            pct_change: Some(4.4),
            volume: 2000000.0,  // 2倍放量
            amount: 18800000.0,
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
        });
        
        data
    }
    
    #[test]
    fn test_stabilizing_after_downtrend() {
        let strategy = BottomVolumeSurgeStrategy::default_config();
        let data = create_test_data_stabilizing();
        
        let result = strategy.analyze_internal("TEST", &data);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        
        // 先下跌后企稳，应该能识别为底部
        // 因为：
        // 1. 最低价出现在前半段
        // 2. 后半段横盘企稳
        // 3. 最后放量上涨
        assert!(
            result.strategy_signal == StrategySignal::Buy 
            || result.strategy_signal == StrategySignal::StrongBuy
        );
        
        // 信号强度应该较高
        assert!(result.signal_strength >= 60);
    }
}
