//! 策略 Trait 定义
//! 
//! 定义所有交易策略的通用接口

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::strategy::TimeFrame::Daily;

/// 通用金融产品数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityData {
    /// 证券代码 (股票/基金/指数等)
    pub symbol: String,
    /// 交易日期
    pub trade_date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 前收盘价
    pub pre_close: Option<f64>,
    /// 涨跌额
    pub change: Option<f64>,
    /// 涨跌幅 (%)
    pub pct_change: Option<f64>,
    /// 成交量
    pub volume: f64,
    /// 成交额
    pub amount: f64,
    /// 产品类型
    pub security_type: SecurityType,
    /// 时间周期
    pub time_frame: TimeFrame,
    /// 财务数据（可选）
    pub financial_data: Option<FinancialData>,
}

/// 财务数据（单季度）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialData {
    /// 报告期（如 "2024Q3"）
    pub report_period: String,
    
    /// 营业收入（元）
    pub revenue: Option<f64>,
    
    /// 净利润（元）
    pub net_profit: Option<f64>,
    
    /// 毛利率（百分比）
    pub gross_profit_margin: Option<f64>,
    
    /// 销售费用率（百分比）
    pub selling_expense_ratio: Option<f64>,
    
    /// 管理费用率（百分比）
    pub admin_expense_ratio: Option<f64>,
    
    /// 财务费用率（百分比）
    pub financial_expense_ratio: Option<f64>,
    
    /// 经营活动现金流（元）
    pub operating_cash_flow: Option<f64>,
    
    // ========== 营运资本相关指标 ==========
    
    /// 存货（元）
    pub inventory: Option<f64>,
    
    /// 应收账款（元）
    pub accounts_receivable: Option<f64>,
    
    /// 预收账款（元）
    pub advances_from_customers: Option<f64>,
    
    /// 应付账款（元）
    pub accounts_payable: Option<f64>,
}

/// 时间周期枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeFrame {
    /// 日线
    Daily,
    /// 周线
    Weekly,
    /// 月线
    Monthly,
    /// 分钟线
    Minute(u32), // 1分钟、5分钟、15分钟、30分钟、60分钟等
    /// 小时线
    Hour(u32),   // 1小时、4小时等
}
impl Default for TimeFrame {
    fn default() -> Self {
        Daily
    }
}


/// 金融产品类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityType {
    /// 股票
    Stock,
    /// 基金
    Fund,
    /// 指数
    Index,
    /// ETF
    Etf,
    /// 债券
    Bond,
    /// 期货
    Future,
    /// 期权
    Option,
}
impl Default for SecurityType {
    fn default() -> Self {
        SecurityType::Stock
    }
}
impl SecurityData {
    /// 从股票日线数据转换
    pub fn from_stock_daily(data: &entity::stock_daily::Model) -> Self {
        Self {
            symbol: data.ts_code.clone(),
            trade_date: data.trade_date.clone(),
            open: decimal_to_f64(&data.open),
            high: decimal_to_f64(&data.high),
            low: decimal_to_f64(&data.low),
            close: decimal_to_f64(&data.close),
            pre_close: data.pre_close.as_ref().map(decimal_to_f64),
            change: data.change.as_ref().map(decimal_to_f64),
            pct_change: data.pct_chg.as_ref().map(decimal_to_f64),
            volume: decimal_to_f64(&data.vol),
            amount: decimal_to_f64(&data.amount),
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
            financial_data: None,
        }
    }
    
    /// 从股票周线数据转换
    pub fn from_stock_weekly(data: &entity::stock_weekly::Model) -> Self {
        Self {
            symbol: data.ts_code.clone(),
            trade_date: data.trade_date.clone(),
            open: decimal_to_f64(&data.open),
            high: decimal_to_f64(&data.high),
            low: decimal_to_f64(&data.low),
            close: decimal_to_f64(&data.close),
            pre_close: data.pre_close.as_ref().map(decimal_to_f64),
            change: data.change.as_ref().map(decimal_to_f64),
            pct_change: data.pct_chg.as_ref().map(decimal_to_f64),
            volume: decimal_to_f64(&data.vol),
            amount: decimal_to_f64(&data.amount),
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Weekly,
            financial_data: None,
        }
    }
    
    /// 从股票月线数据转换
    pub fn from_stock_monthly(data: &entity::stock_monthly::Model) -> Self {
        Self {
            symbol: data.ts_code.clone(),
            trade_date: data.trade_date.clone(),
            open: decimal_to_f64(&data.open),
            high: decimal_to_f64(&data.high),
            low: decimal_to_f64(&data.low),
            close: decimal_to_f64(&data.close),
            pre_close: data.pre_close.as_ref().map(decimal_to_f64),
            change: data.change.as_ref().map(decimal_to_f64),
            pct_change: data.pct_chg.as_ref().map(decimal_to_f64),
            volume: decimal_to_f64(&data.vol),
            amount: decimal_to_f64(&data.amount),
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Monthly,
            financial_data: None,
        }
    }
    
    /// 从基金日线数据转换
    pub fn from_fund_daily(data: &entity::fund_daily::Model) -> Self {
        Self {
            symbol: data.ts_code.clone(),
            trade_date: data.trade_date.clone(),
            open: decimal_to_f64(&data.open),
            high: decimal_to_f64(&data.high),
            low: decimal_to_f64(&data.low),
            close: decimal_to_f64(&data.close),
            pre_close: data.pre_close.as_ref().map(decimal_to_f64),
            change: data.change.as_ref().map(decimal_to_f64),
            pct_change: data.pct_chg.as_ref().map(decimal_to_f64),
            volume: decimal_to_f64(&data.vol),
            amount: decimal_to_f64(&data.amount),
            security_type: SecurityType::Fund,
            time_frame: TimeFrame::Daily,
            financial_data: None,
        }
    }
    
    /// 从指数日线数据转换
    pub fn from_index_daily(data: &entity::index_daily::Model) -> Self {
        Self {
            symbol: data.ts_code.clone(),
            trade_date: data.trade_date.clone(),
            open: data.open.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            high: data.high.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            low: data.low.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            close: data.close.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            pre_close: data.pre_close.as_ref().map(|d| decimal_to_f64(d)),
            change: data.change.as_ref().map(|d| decimal_to_f64(d)),
            pct_change: data.pct_chg.as_ref().map(|d| decimal_to_f64(d)),
            volume: data.vol.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            amount: data.amount.as_ref().map(|d| decimal_to_f64(d)).unwrap_or(0.0),
            security_type: SecurityType::Index,
            time_frame: TimeFrame::Daily,
            financial_data: None,
        }
    }
    
    /// 获取价格范围 (high - low)
    pub fn price_range(&self) -> f64 {
        self.high - self.low
    }
    
    /// 获取实体大小 |close - open|
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }
    
    /// 获取上影线长度
    pub fn upper_shadow(&self) -> f64 {
        self.high - self.open.max(self.close)
    }
    
    /// 获取下影线长度
    pub fn lower_shadow(&self) -> f64 {
        self.open.min(self.close) - self.low
    }
    
    /// 判断是否为阳线
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
    
    /// 判断是否为阴线
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
    
    /// 判断是否为十字星 (实体很小)
    pub fn is_doji(&self, threshold_pct: f64) -> bool {
        let range = self.price_range();
        if range == 0.0 {
            return true;
        }
        let body_ratio = self.body_size() / range;
        body_ratio <= threshold_pct / 100.0
    }
    
    /// 获取时间周期描述
    pub fn time_frame_desc(&self) -> String {
        match self.time_frame {
            TimeFrame::Daily => "日线".to_string(),
            TimeFrame::Weekly => "周线".to_string(),
            TimeFrame::Monthly => "月线".to_string(),
            TimeFrame::Minute(m) => format!("{}分钟线", m),
            TimeFrame::Hour(h) => format!("{}小时线", h),
        }
    }
}

/// Decimal 转 f64 的辅助函数
fn decimal_to_f64(decimal: &Decimal) -> f64 {
    decimal.to_string().parse().unwrap_or(0.0)
}

/// 策略信号枚举
/// 
/// 信号强度从高到低：StrongBuy > Buy > Hold > Sell > StrongSell
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StrategySignal {
    /// 强烈卖出（最弱）
    StrongSell,
    /// 卖出
    Sell,
    /// 持有
    Hold,
    /// 买入
    Buy,
    /// 强烈买入（最强）
    StrongBuy,
}

/// 策略分析结果枚举
/// 
/// 每个策略有自己的结果类型，包含该策略特有的数据
/// 
/// 使用 `#[serde(untagged)]` 序列化时直接输出内部结构体，不包含枚举变体名
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrategyResult {
    /// 底部放量上涨策略结果
    BottomVolumeSurge(BottomVolumeSurgeResult),
    /// 价量K线策略结果
    PriceVolumeCandlestick(PriceVolumeCandlestickResult),
    /// 长期底部反转策略结果
    LongTermBottomReversal(super::long_term_bottom_reversal_strategy::LongTermBottomReversalResult),
    /// 年内新高策略结果
    YearlyHigh(super::yearly_high_strategy::YearlyHighResult),
    /// 价格强弱策略结果
    PriceStrength(super::price_strength_strategy::PriceStrengthResult),
    /// 困境反转策略结果
    DistressedReversal(super::distressed_reversal_strategy::DistressedReversalResult),
}
impl StrategyResult {
    /// 获取股票代码
    pub fn stock_code(&self) -> &str {
        match self {
            StrategyResult::BottomVolumeSurge(r) => &r.stock_code,
            StrategyResult::PriceVolumeCandlestick(r) => &r.stock_code,
            StrategyResult::LongTermBottomReversal(r) => &r.stock_code,
            StrategyResult::YearlyHigh(r) => &r.stock_code,
            StrategyResult::PriceStrength(r) => &r.stock_code,
            StrategyResult::DistressedReversal(r) => &r.stock_code,
        }
    }
    
    /// 获取分析日期
    pub fn analysis_date(&self) -> NaiveDate {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.analysis_date,
            StrategyResult::PriceVolumeCandlestick(r) => r.analysis_date,
            StrategyResult::LongTermBottomReversal(r) => r.analysis_date,
            StrategyResult::YearlyHigh(r) => r.analysis_date,
            StrategyResult::PriceStrength(r) => r.analysis_date,
            StrategyResult::DistressedReversal(r) => r.analysis_date,
        }
    }
    
    /// 获取当前价格
    pub fn current_price(&self) -> f64 {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.current_price,
            StrategyResult::PriceVolumeCandlestick(r) => r.current_price,
            StrategyResult::LongTermBottomReversal(r) => r.current_price,
            StrategyResult::YearlyHigh(r) => r.current_price,
            StrategyResult::PriceStrength(r) => r.current_price,
            StrategyResult::DistressedReversal(_) => 0.0,  // 困境反转策略不使用价格字段
        }
    }
    
    /// 获取策略信号
    pub fn strategy_signal(&self) -> StrategySignal {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.strategy_signal.clone(),
            StrategyResult::PriceVolumeCandlestick(r) => r.strategy_signal.clone(),
            StrategyResult::LongTermBottomReversal(r) => r.strategy_signal.clone(),
            StrategyResult::YearlyHigh(r) => r.strategy_signal.clone(),
            StrategyResult::PriceStrength(r) => r.strategy_signal.clone(),
            StrategyResult::DistressedReversal(r) => r.strategy_signal.clone(),
        }
    }
    
    /// 获取信号强度 (0-100)
    pub fn signal_strength(&self) -> u8 {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.signal_strength,
            StrategyResult::PriceVolumeCandlestick(r) => r.signal_strength,
            StrategyResult::LongTermBottomReversal(r) => r.signal_strength,
            StrategyResult::YearlyHigh(r) => r.signal_strength,
            StrategyResult::PriceStrength(r) => r.signal_strength,
            StrategyResult::DistressedReversal(r) => r.signal_strength,
        }
    }
    
    /// 获取分析说明
    pub fn analysis_description(&self) -> String {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.analysis_description.clone(),
            StrategyResult::PriceVolumeCandlestick(r) => r.analysis_description.clone(),
            StrategyResult::LongTermBottomReversal(r) => r.analysis_description.clone(),
            StrategyResult::YearlyHigh(r) => r.analysis_description.clone(),
            StrategyResult::PriceStrength(r) => r.analysis_description.clone(),
            StrategyResult::DistressedReversal(r) => r.analysis_description.clone(),
        }
    }
    
    /// 获取风险等级 (1-5)
    pub fn risk_level(&self) -> u8 {
        match self {
            StrategyResult::BottomVolumeSurge(r) => r.risk_level,
            StrategyResult::PriceVolumeCandlestick(r) => r.risk_level,
            StrategyResult::LongTermBottomReversal(r) => r.risk_level,
            StrategyResult::YearlyHigh(r) => r.risk_level,
            StrategyResult::PriceStrength(r) => r.risk_level,
            StrategyResult::DistressedReversal(r) => r.risk_level,
        }
    }
}

/// 底部放量上涨策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomVolumeSurgeResult {
    /// 股票代码
    pub stock_code: String,
    /// 分析日期
    pub analysis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 策略信号
    pub strategy_signal: StrategySignal,
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    /// 分析说明
    pub analysis_description: String,
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 策略特有字段
    /// 是否处于底部
    pub is_at_bottom: bool,
    /// 底部价格
    pub bottom_price: f64,
    /// 底部日期
    pub bottom_date: String,
    /// 当前成交量
    pub current_volume: f64,
    /// 成交量均值
    pub volume_ma: f64,
    /// 成交量放大倍数
    pub volume_surge_ratio: f64,
    /// 价格涨幅（百分比，相对底部价格）
    pub price_rise_pct: f64,
    /// 当天涨幅（百分比，相对前一天收盘价）
    pub daily_rise_pct: f64,
    /// 近期最低价
    pub recent_low: f64,
    /// 近期最高价
    pub recent_high: f64,
}

/// 价量K线策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceVolumeCandlestickResult {
    /// 股票代码
    pub stock_code: String,
    /// 分析日期
    pub analysis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 策略信号
    pub strategy_signal: StrategySignal,
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    /// 分析说明
    pub analysis_description: String,
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 策略特有字段
    /// K线形态
    pub candlestick_pattern: String,
    /// 成交量信号
    pub volume_signal: String,
    /// 价格波动率
    pub price_volatility: f64,
    /// 成交量比率
    pub volume_ratio: f64,
}

/// 策略配置基础 trait
pub trait StrategyConfig: Clone + Send + Sync {
    /// 获取策略名称
    fn strategy_name(&self) -> &str;
    
    /// 获取分析周期
    fn analysis_period(&self) -> usize;
    
    /// 验证配置是否有效
    fn validate(&self) -> Result<()> {
        if self.analysis_period() == 0 {
            return Err(anyhow::anyhow!("分析周期不能为0"));
        }
        Ok(())
    }
}

/// 交易策略 trait
pub trait TradingStrategy: Send + Sync {
    /// 策略配置类型
    type Config: StrategyConfig;
    
    /// 获取策略名称
    fn name(&self) -> &str;
    
    /// 获取策略版本
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    /// 获取策略描述
    fn description(&self) -> &str;
    
    /// 获取当前配置
    fn config(&self) -> &Self::Config;
    
    /// 更新配置
    fn update_config(&mut self, config: Self::Config) -> Result<()>;
    
    /// 分析单只证券
    fn analyze(&mut self, symbol: &str, data: &[SecurityData]) -> Result<StrategyResult>;
    
    /// 批量分析多只证券
    fn batch_analyze(&mut self, securities_data: &[(String, Vec<SecurityData>)]) -> Vec<StrategyResult> {
        let mut results = Vec::new();
        
        for (symbol, daily_data) in securities_data {
            match self.analyze(symbol, daily_data) {
                Ok(result) => results.push(result),
                Err(e) => {
                    tracing::warn!("分析证券 {} 失败: {}", symbol, e);
                }
            }
        }
        
        // 按信号强度排序
        results.sort_by(|a, b| b.signal_strength().cmp(&a.signal_strength()));
        
        results
    }
    
    /// 检查策略是否需要的最小数据量
    fn required_data_points(&self) -> usize {
        self.config().analysis_period()
    }
    
    /// 验证输入数据是否足够
    fn validate_data(&self, data: &[SecurityData]) -> Result<()> {
        if data.len() < self.required_data_points() {
            return Err(anyhow::anyhow!(
                "数据不足：需要至少 {} 个数据点，实际只有 {} 个",
                self.required_data_points(),
                data.len()
            ));
        }
        Ok(())
    }
    
    /// 重置策略状态（如果有状态的话）
    fn reset(&mut self) {
        // 默认实现为空，有状态的策略可以重写
    }
}

// 移除 Factory 模式，保持简洁的设计

/// 策略信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyInfo {
    /// 策略名称
    pub name: String,
    /// 策略版本
    pub version: String,
    /// 策略描述
    pub description: String,
    /// 策略类型
    pub strategy_type: StrategyType,
    /// 适用市场
    pub applicable_markets: Vec<String>,
    /// 时间框架
    pub time_frames: Vec<String>,
    /// 风险等级
    pub risk_level: RiskLevel,
}

/// 策略类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StrategyType {
    /// 技术分析策略
    Technical,
    /// 基本面分析策略
    Fundamental,
    /// 量化策略
    Quantitative,
    /// 混合策略
    Hybrid,
}

/// 风险等级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// 低风险
    Low,
    /// 中等风险
    Medium,
    /// 高风险
    High,
    /// 极高风险
    VeryHigh,
}

/// 策略性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPerformance {
    /// 总交易次数
    pub total_trades: u32,
    /// 胜率
    pub win_rate: f64,
    /// 平均收益率
    pub average_return: f64,
    /// 最大回撤
    pub max_drawdown: f64,
    /// 夏普比率
    pub sharpe_ratio: f64,
    /// 分析时间段
    pub analysis_period: (NaiveDate, NaiveDate),
}

/// 策略回测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// 策略名称
    pub strategy_name: String,
    /// 回测时间段
    pub period: (NaiveDate, NaiveDate),
    /// 性能指标
    pub performance: StrategyPerformance,
    /// 详细交易记录
    pub trades: Vec<TradeRecord>,
}

/// 交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    /// 股票代码
    pub stock_code: String,
    /// 交易日期
    pub trade_date: NaiveDate,
    /// 交易类型
    pub trade_type: TradeType,
    /// 交易价格
    pub price: f64,
    /// 交易数量
    pub quantity: u32,
    /// 信号强度
    pub signal_strength: u8,
}

/// 交易类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeType {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strategy_signal_serialization() {
        let signal = StrategySignal::StrongBuy;
        let json = serde_json::to_string(&signal).unwrap();
        let deserialized: StrategySignal = serde_json::from_str(&json).unwrap();
        assert_eq!(signal, deserialized);
    }
    
    #[test]
    fn test_strategy_result_creation() {
        let result = StrategyResult::BottomVolumeSurge(BottomVolumeSurgeResult {
            stock_code: "000001.SZ".to_string(),
            analysis_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            current_price: 10.5,
            strategy_signal: StrategySignal::Buy,
            signal_strength: 75,
            analysis_description: "测试分析".to_string(),
            risk_level: 3,
            is_at_bottom: true,
            bottom_price: 10.0,
            bottom_date: "20240101".to_string(),
            current_volume: 1000000.0,
            volume_ma: 800000.0,
            volume_surge_ratio: 1.25,
            price_rise_pct: 5.0,
            daily_rise_pct: 2.5,
            recent_low: 9.8,
            recent_high: 10.8,
        });
        
        assert_eq!(result.stock_code(), "000001.SZ");
        assert_eq!(result.signal_strength(), 75);
    }
}
