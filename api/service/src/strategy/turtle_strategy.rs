//! 海龟交易策略 (Turtle Trading Strategy)
//! 
//! ## 策略起源
//! 
//! 1983年，传奇交易员理查德·丹尼斯（Richard Dennis）与威廉·埃克哈特（William Eckhardt）
//! 进行了一场著名的赌约：优秀的交易员是天生的还是可以培养的？
//! 
//! 丹尼斯招募了23名"海龟"（Turtles），用两周时间教授他们这套交易系统。
//! 结果证明，严格遵守规则的海龟们在4年内平均年化收益率达到80%。
//! 
//! ## 核心思想
//! 
//! ### 1. 趋势跟踪哲学
//! 
//! **"趋势是你的朋友"** - 海龟策略不预测市场方向，而是等待趋势形成后跟随。
//! 
//! - **顺势而为**：只在明确的趋势中交易，不逆势操作
//! - **让利润奔跑**：持有盈利仓位直到趋势反转
//! - **快速止损**：及时切断亏损，保护资本
//! 
//! ### 2. 突破交易原理
//! 
//! **价格突破是趋势开始的信号**
//! 
//! ```text
//! 价格
//!   ↑
//!   |     ╱╲  ← 突破点（买入）
//!   |    ╱  ╲╱
//!   |   ╱    
//!   |  ╱  ← N日最高价（突破线）
//!   | ╱─────────────
//!   |╱
//!   +──────────────→ 时间
//! ```
//! 
//! 当价格突破过去N天的最高价时，说明：
//! - 市场出现新的买盘力量
//! - 可能形成新的上升趋势
//! - 是建立多头仓位的时机
//! 
//! ### 3. ATR（平均真实波动幅度）的作用
//! 
//! **ATR是衡量市场波动性的指标，用于：**
//! 
//! #### a) 自适应止损
//! ```
//! 止损位 = 入场价 - (ATR × 2.0)
//! ```
//! - 波动大的市场：止损位更宽，避免被正常波动扫出
//! - 波动小的市场：止损位更窄，减少风险暴露
//! 
//! #### b) 仓位管理
//! ```
//! 单位规模 = 账户风险 / ATR
//! ```
//! - 波动大的品种：仓位更小
//! - 波动小的品种：仓位更大
//! - 确保每笔交易的风险相同
//! 
//! #### c) 加仓时机
//! ```
//! 加仓价 = 入场价 + (ATR × 0.5)
//! ```
//! - 趋势延续时逐步加仓
//! - 每次加仓间隔0.5个ATR
//! - 最多加仓4次（金字塔式建仓）
//! 
//! ### 4. 金字塔加仓策略
//! 
//! **在盈利的仓位上加仓，而不是在亏损的仓位上摊平成本**
//! 
//! ```text
//! 仓位规模
//!   ↑
//!   |  ▓        ← 第4次加仓（1单位）
//!   |  ▓▓       ← 第3次加仓（1单位）
//!   |  ▓▓▓      ← 第2次加仓（1单位）
//!   |  ▓▓▓▓     ← 首次建仓（1单位）
//!   +──────────→ 价格上涨
//! ```
//! 
//! 优势：
//! - 在正确的趋势中持有更多仓位
//! - 在错误的交易中损失更少
//! - 提高整体盈亏比
//! 
//! ### 5. 两个系统的设计
//! 
//! **系统1（20天突破）**：
//! - 更激进，信号更频繁
//! - 适合捕捉短期趋势
//! - 假突破较多，但不会错过大趋势
//! 
//! **系统2（55天突破）**：
//! - 更保守，信号更可靠
//! - 适合捕捉长期趋势
//! - 假突破较少，但可能错过早期机会
//! 
//! 原始海龟策略建议：
//! - 50%资金用于系统1
//! - 50%资金用于系统2
//! - 两个系统互补，提高稳定性
//! 
//! ## 风险管理原则
//! 
//! ### 1. 单次风险控制
//! ```
//! 单次风险 = 账户净值 × 1%
//! ```
//! 任何单笔交易的损失不超过账户的1%
//! 
//! ### 2. 相关性管理
//! ```
//! 同方向相关品种总风险 ≤ 账户净值 × 4%
//! ```
//! 避免在高度相关的品种上过度集中
//! 
//! ### 3. 总仓位限制
//! ```
//! 总风险暴露 ≤ 账户净值 × 12%
//! ```
//! 同时持有的所有仓位总风险不超过12%
//! 
//! ## 心理学原理
//! 
//! ### 为什么海龟策略有效？
//! 
//! 1. **克服人性弱点**
//!    - 规则明确，减少情绪干扰
//!    - 系统化执行，避免主观判断
//! 
//! 2. **接受不确定性**
//!    - 胜率可能只有40%，但盈亏比高
//!    - 小亏损多次，大盈利一次
//! 
//! 3. **纪律的力量**
//!    - 成功不在于预测，而在于执行
//!    - 坚持规则比聪明更重要
//! 
//! ## 适用市场
//! 
//! ✅ **适合**：
//! - 趋势性强的市场（牛市、熊市）
//! - 流动性好的品种
//! - 波动适中的标的
//! 
//! ⚠️ **不适合**：
//! - 长期横盘震荡的市场
//! - 流动性差的品种
//! - 波动极大的标的
//! 
//! ## 核心理念总结
//! 
//! > "交易成功的秘诀不在于预测市场，而在于：
//! > 1. 等待明确的趋势信号
//! > 2. 严格控制风险
//! > 3. 让利润充分增长
//! > 4. 坚持执行规则"
//! > 
//! > —— 理查德·丹尼斯

use super::traits::*;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// 海龟策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleConfig {
    /// 入场突破周期（天数）- 默认 20 天（系统1）
    /// 当价格突破过去N天的最高价时买入
    pub entry_breakout_period: usize,
    
    /// 出场突破周期（天数）- 默认 10 天
    /// 当价格跌破过去N天的最低价时卖出
    pub exit_breakout_period: usize,
    
    /// ATR周期（天数）- 用于计算波动率和止损 - 默认 20 天
    pub atr_period: usize,
    
    /// 止损ATR倍数 - 默认 2.0
    /// 止损位 = 入场价 - (ATR × 止损倍数)
    pub stop_loss_atr_multiple: f64,
    
    /// 加仓ATR倍数 - 默认 0.5
    /// 每上涨0.5个ATR可以加仓一次
    pub pyramid_atr_multiple: f64,
    
    /// 最大加仓次数 - 默认 4 次
    pub max_pyramid_units: usize,
    
    /// 是否使用系统2（55天突破）- 默认 false（使用系统1的20天突破）
    pub use_system2: bool,
}

impl Default for TurtleConfig {
    fn default() -> Self {
        Self {
            entry_breakout_period: 20,
            exit_breakout_period: 10,
            atr_period: 20,
            stop_loss_atr_multiple: 2.0,
            pyramid_atr_multiple: 0.5,
            max_pyramid_units: 4,
            use_system2: false,
        }
    }
}

impl StrategyConfig for TurtleConfig {
    fn strategy_name(&self) -> &str {
        "turtle"
    }
    
    fn analysis_period(&self) -> usize {
        // 需要足够的数据来计算ATR和突破
        self.entry_breakout_period.max(self.atr_period) + 10
    }
    
    fn validate(&self) -> Result<()> {
        if self.entry_breakout_period == 0 {
            bail!("入场突破周期不能为0");
        }
        if self.exit_breakout_period == 0 {
            bail!("出场突破周期不能为0");
        }
        if self.atr_period == 0 {
            bail!("ATR周期不能为0");
        }
        if self.stop_loss_atr_multiple <= 0.0 {
            bail!("止损ATR倍数必须大于0");
        }
        if self.pyramid_atr_multiple <= 0.0 {
            bail!("加仓ATR倍数必须大于0");
        }
        if self.max_pyramid_units == 0 {
            bail!("最大加仓次数不能为0");
        }
        if self.exit_breakout_period >= self.entry_breakout_period {
            bail!("出场周期应该小于入场周期");
        }
        Ok(())
    }
}

/// 海龟策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 入场突破价（N日最高价）
    pub entry_breakout_price: f64,
    
    /// 出场突破价（N日最低价）
    pub exit_breakout_price: f64,
    
    /// 是否突破入场
    pub is_entry_breakout: bool,
    
    /// 是否跌破出场
    pub is_exit_breakout: bool,
    
    /// ATR值（平均真实波动幅度）
    pub atr: f64,
    
    /// 建议止损价
    pub stop_loss_price: f64,
    
    /// 建议加仓价格列表
    pub pyramid_prices: Vec<f64>,
    
    /// 当前可加仓次数
    pub available_pyramid_units: usize,
    
    /// 距离入场突破的百分比（负数表示未突破）
    pub distance_to_entry_pct: f64,
    
    /// 距离出场突破的百分比（正数表示未跌破）
    pub distance_to_exit_pct: f64,
    
    /// 趋势强度（0-100）
    pub trend_strength: u8,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 海龟交易策略
pub struct TurtleStrategy {
    config: TurtleConfig,
}

impl TurtleStrategy {
    pub fn new(config: TurtleConfig) -> Self {
        Self { config }
    }
    
    /// 系统1配置（20天突破，更激进）
    /// 
    /// # 设计思想
    /// 
    /// 系统1是海龟策略的主要交易系统，使用20天突破作为入场信号。
    /// 
    /// **为什么选择20天？**
    /// - 20天约等于一个月的交易日，能够捕捉中短期趋势
    /// - 信号频率适中，既不会过于频繁产生假突破，也不会错过重要趋势
    /// - 历史回测表明，20天是突破交易的最佳平衡点之一
    /// 
    /// **10天出场的原因：**
    /// - 出场周期应该小于入场周期，避免过早离场
    /// - 10天能够给趋势足够的发展空间
    /// - 同时也能及时止损，保护利润
    pub fn system1() -> TurtleConfig {
        TurtleConfig {
            entry_breakout_period: 20,
            exit_breakout_period: 10,
            atr_period: 20,
            stop_loss_atr_multiple: 2.0,
            pyramid_atr_multiple: 0.5,
            max_pyramid_units: 4,
            use_system2: false,
        }
    }
    
    /// 系统2配置（55天突破，更保守）
    /// 
    /// # 设计思想
    /// 
    /// 系统2是海龟策略的备用系统，使用55天突破作为入场信号。
    /// 
    /// **为什么需要系统2？**
    /// - 捕捉更大级别的趋势，避免系统1错过的长期机会
    /// - 55天突破的可靠性更高，假突破更少
    /// - 与系统1互补，提高整体策略的稳定性
    /// 
    /// **原始海龟策略建议：**
    /// - 50%资金分配给系统1（捕捉中短期趋势）
    /// - 50%资金分配给系统2（捕捉长期趋势）
    /// - 两个系统同时运行，相互补充
    pub fn system2() -> TurtleConfig {
        TurtleConfig {
            entry_breakout_period: 55,
            exit_breakout_period: 20,
            atr_period: 20,
            stop_loss_atr_multiple: 2.0,
            pyramid_atr_multiple: 0.5,
            max_pyramid_units: 4,
            use_system2: true,
        }
    }
    
    /// 保守配置（更大的止损空间）
    pub fn conservative() -> TurtleConfig {
        TurtleConfig {
            entry_breakout_period: 30,
            exit_breakout_period: 15,
            atr_period: 20,
            stop_loss_atr_multiple: 3.0,  // 更大的止损空间
            pyramid_atr_multiple: 1.0,     // 更大的加仓间隔
            max_pyramid_units: 3,          // 更少的加仓次数
            use_system2: false,
        }
    }
    
    /// 激进配置（更小的止损，更多加仓）
    pub fn aggressive() -> TurtleConfig {
        TurtleConfig {
            entry_breakout_period: 10,
            exit_breakout_period: 5,
            atr_period: 10,
            stop_loss_atr_multiple: 1.5,   // 更小的止损空间
            pyramid_atr_multiple: 0.3,     // 更小的加仓间隔
            max_pyramid_units: 5,          // 更多的加仓次数
            use_system2: false,
        }
    }
    
    /// 计算真实波动幅度（True Range）
    /// 
    /// # ATR的核心作用
    /// 
    /// True Range（真实波动幅度）是ATR的基础，它衡量当日的实际波动。
    /// 
    /// **为什么不直接用 High - Low？**
    /// 
    /// 考虑跳空缺口的情况：
    /// ```text
    /// 昨日收盘: 10元
    /// 今日开盘: 12元（跳空高开）
    /// 今日最高: 13元
    /// 今日最低: 11元
    /// ```
    /// 
    /// - 简单的 High - Low = 13 - 11 = 2元
    /// - 但实际波动应该包含跳空：13 - 10 = 3元
    /// 
    /// **True Range 的三种计算方式取最大值：**
    /// 1. 当日最高价 - 当日最低价（日内波动）
    /// 2. |当日最高价 - 昨日收盘价|（向上跳空）
    /// 3. |当日最低价 - 昨日收盘价|（向下跳空）
    /// 
    /// 这样能够准确捕捉包括跳空在内的所有波动。
    fn calculate_true_range(&self, data: &SecurityData, prev_close: f64) -> f64 {
        let high_low = data.high - data.low;
        let high_prev_close = (data.high - prev_close).abs();
        let low_prev_close = (data.low - prev_close).abs();
        
        high_low.max(high_prev_close).max(low_prev_close)
    }
    
    /// 计算平均真实波动幅度（ATR）
    /// 
    /// # ATR在海龟策略中的三大应用
    /// 
    /// ATR（Average True Range）是海龟策略的核心指标，用于：
    /// 
    /// ## 1. 自适应止损
    /// ```
    /// 止损位 = 入场价 - (ATR × 2.0)
    /// ```
    /// - **波动大的股票**：ATR大，止损位更宽，避免被正常波动扫出
    /// - **波动小的股票**：ATR小，止损位更窄，减少风险暴露
    /// - 这样每个品种的止损都适应其自身的波动特性
    /// 
    /// ## 2. 仓位管理（单位规模）
    /// ```
    /// 单位规模 = (账户净值 × 1%) / ATR
    /// ```
    /// - **波动大的品种**：ATR大，单位规模小，仓位轻
    /// - **波动小的品种**：ATR小，单位规模大，仓位重
    /// - 确保每笔交易的风险相同（都是账户的1%）
    /// 
    /// ## 3. 加仓时机
    /// ```
    /// 第N次加仓价 = 入场价 + (ATR × 0.5 × N)
    /// ```
    /// - 趋势延续时，每上涨0.5个ATR就加仓一次
    /// - 加仓间隔随波动调整，波动大时间隔大，波动小时间隔小
    /// - 最多加仓4次，形成金字塔式仓位
    /// 
    /// **为什么用20天周期？**
    /// - 20天能够平滑短期噪音，反映真实的市场波动
    /// - 与入场周期一致，保持策略的协调性
    fn calculate_atr(&self, data: &[SecurityData]) -> f64 {
        if data.len() < self.config.atr_period + 1 {
            return 0.0;
        }
        
        let start_idx = data.len() - self.config.atr_period - 1;
        let mut true_ranges = Vec::new();
        
        for i in (start_idx + 1)..data.len() {
            let prev_close = data[i - 1].close;
            let tr = self.calculate_true_range(&data[i], prev_close);
            true_ranges.push(tr);
        }
        
        // 简单移动平均
        true_ranges.iter().sum::<f64>() / true_ranges.len() as f64
    }
    
    /// 计算N日最高价（突破线）
    /// 
    /// # 突破交易的核心逻辑
    /// 
    /// **为什么突破N日最高价是买入信号？**
    /// 
    /// 1. **供需关系改变**
    ///    - 价格创新高说明买盘力量超过了过去N天的所有卖盘
    ///    - 市场结构发生变化，可能形成新的上升趋势
    /// 
    /// 2. **技术面确认**
    ///    - 突破前期高点，打破了阻力位
    ///    - 技术分析中，突破往往伴随着趋势的开始
    /// 
    /// 3. **心理面影响**
    ///    - 创新高吸引更多买盘入场
    ///    - 空头被迫平仓，进一步推高价格
    /// 
    /// **注意：不包括当天数据**
    /// - 计算突破线时使用历史数据（不含当天）
    /// - 当天价格与历史最高价比较，判断是否突破
    /// - 这样避免了用当天数据计算当天信号的逻辑错误
    fn calculate_highest_high(&self, data: &[SecurityData], period: usize) -> f64 {
        if data.len() < period {
            return data.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);
        }
        
        let start_idx = data.len() - period;
        data[start_idx..].iter()
            .map(|d| d.high)
            .fold(f64::NEG_INFINITY, f64::max)
    }
    
    /// 计算N日最低价（出场线）
    /// 
    /// # 出场规则的设计思想
    /// 
    /// **为什么跌破N日最低价要卖出？**
    /// 
    /// 1. **趋势反转信号**
    ///    - 跌破前期低点说明下跌力量增强
    ///    - 原有的上升趋势可能已经结束
    /// 
    /// 2. **保护利润**
    ///    - 及时离场，锁定已有利润
    ///    - 避免从盈利变成亏损
    /// 
    /// 3. **止损原则**
    ///    - 对于亏损仓位，也要及时止损
    ///    - "截断亏损，让利润奔跑"的体现
    /// 
    /// **为什么出场周期小于入场周期？**
    /// - 入场需要更强的确认（20天突破）
    /// - 出场可以更灵活（10天跌破）
    /// - 这样既能捕捉趋势，又能及时止损
    fn calculate_lowest_low(&self, data: &[SecurityData], period: usize) -> f64 {
        if data.len() < period {
            return data.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
        }
        
        let start_idx = data.len() - period;
        data[start_idx..].iter()
            .map(|d| d.low)
            .fold(f64::INFINITY, f64::min)
    }
    
    /// 计算趋势强度（0-100分）
    /// 
    /// # 趋势强度的评估维度
    /// 
    /// 海龟策略虽然简单，但也需要评估趋势的质量。
    /// 
    /// **评分体系：**
    /// 
    /// 1. **价格位置（30分）**
    ///    - 当前价格在近期高低点区间的位置
    ///    - 越接近高点，得分越高
    ///    - 反映价格的相对强度
    /// 
    /// 2. **突破幅度（30分）**
    ///    - 突破入场线的百分比
    ///    - 突破越多，趋势越强
    ///    - 大幅突破往往伴随强劲趋势
    /// 
    /// 3. **上涨天数占比（40分）**
    ///    - 近期收阳线的天数比例
    ///    - 反映买盘的持续性
    ///    - 连续上涨说明趋势稳定
    /// 
    /// **为什么需要趋势强度？**
    /// - 不是所有突破都值得追随
    /// - 强趋势中可以更激进（加仓更多）
    /// - 弱趋势中应该更谨慎（减少加仓）
    fn calculate_trend_strength(&self, data: &[SecurityData], current_price: f64, entry_breakout: f64) -> u8 {
        if data.len() < 20 {
            return 50;
        }
        
        let recent_data = &data[data.len() - 20..];
        
        // 1. 价格位置（30分）
        let highest = recent_data.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);
        let lowest = recent_data.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
        let price_position = if highest > lowest {
            ((current_price - lowest) / (highest - lowest) * 30.0) as u8
        } else {
            15
        };
        
        // 2. 突破幅度（30分）
        let breakout_strength = if current_price > entry_breakout {
            let breakout_pct = (current_price - entry_breakout) / entry_breakout * 100.0;
            (breakout_pct.min(10.0) / 10.0 * 30.0) as u8
        } else {
            0
        };
        
        // 3. 上涨天数占比（40分）
        let up_days = recent_data.iter().filter(|d| d.close > d.open).count();
        let up_ratio = up_days as f64 / recent_data.len() as f64;
        let up_score = (up_ratio * 40.0) as u8;
        
        (price_position + breakout_strength + up_score).min(100)
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<TurtleResult> {
        let min_required = self.config.analysis_period();
        if data.len() < min_required {
            bail!("数据不足，需要至少{}天数据，当前只有{}天", min_required, data.len());
        }
        
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 计算ATR
        let atr = self.calculate_atr(data);
        
        // 计算入场突破价（不包括当天）
        let entry_data = &data[..data.len() - 1];
        let entry_breakout_price = self.calculate_highest_high(entry_data, self.config.entry_breakout_period);
        
        // 计算出场突破价（不包括当天）
        let exit_breakout_price = self.calculate_lowest_low(entry_data, self.config.exit_breakout_period);
        
        // 判断是否突破
        let is_entry_breakout = current_price > entry_breakout_price;
        let is_exit_breakout = current_price < exit_breakout_price;
        
        // 计算止损价
        let stop_loss_price = current_price - (atr * self.config.stop_loss_atr_multiple);
        
        // 计算加仓价格
        let mut pyramid_prices = Vec::new();
        for i in 1..=self.config.max_pyramid_units {
            let pyramid_price = current_price + (atr * self.config.pyramid_atr_multiple * i as f64);
            pyramid_prices.push(pyramid_price);
        }
        
        // 计算距离突破的百分比
        let distance_to_entry_pct = ((current_price - entry_breakout_price) / entry_breakout_price) * 100.0;
        let distance_to_exit_pct = ((current_price - exit_breakout_price) / exit_breakout_price) * 100.0;
        
        // 计算趋势强度
        let trend_strength = self.calculate_trend_strength(data, current_price, entry_breakout_price);
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            is_entry_breakout,
            is_exit_breakout,
            distance_to_entry_pct,
            distance_to_exit_pct,
            trend_strength,
            atr,
            current_price,
        );
        
        let analysis_description = self.generate_description(
            is_entry_breakout,
            is_exit_breakout,
            distance_to_entry_pct,
            distance_to_exit_pct,
            atr,
            current_price,
        );
        
        Ok(TurtleResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            entry_breakout_price,
            exit_breakout_price,
            is_entry_breakout,
            is_exit_breakout,
            atr,
            stop_loss_price,
            pyramid_prices,
            available_pyramid_units: self.config.max_pyramid_units,
            distance_to_entry_pct,
            distance_to_exit_pct,
            trend_strength,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        is_entry_breakout: bool,
        is_exit_breakout: bool,
        distance_to_entry_pct: f64,
        distance_to_exit_pct: f64,
        trend_strength: u8,
        atr: f64,
        current_price: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 如果跌破出场线，强烈卖出
        if is_exit_breakout {
            return (StrategySignal::StrongSell, 0, 5);
        }
        
        // 如果突破入场线
        if is_entry_breakout {
            // 突破幅度评分（30分）
            let breakout_score = if distance_to_entry_pct > 5.0 {
                30
            } else if distance_to_entry_pct > 3.0 {
                25
            } else if distance_to_entry_pct > 1.0 {
                20
            } else {
                15
            };
            signal_strength += breakout_score;
            
            // 趋势强度评分（40分）
            let trend_score = ((trend_strength as f64 / 100.0) * 40.0) as u8;
            signal_strength += trend_score;
            
            // 波动率评分（20分）- ATR越大，波动越大，风险越高
            let atr_pct = (atr / current_price) * 100.0;
            let volatility_score = if atr_pct < 2.0 {
                20  // 低波动
            } else if atr_pct < 4.0 {
                15  // 中等波动
            } else if atr_pct < 6.0 {
                10  // 高波动
            } else {
                5   // 极高波动
            };
            signal_strength += volatility_score;
            
            // 距离出场线的安全边际（10分）
            let safety_score = if distance_to_exit_pct > 20.0 {
                10
            } else if distance_to_exit_pct > 10.0 {
                7
            } else if distance_to_exit_pct > 5.0 {
                4
            } else {
                0
            };
            signal_strength += safety_score;
            
            // 根据波动率调整风险等级
            if atr_pct > 5.0 {
                risk_level = 4;
            } else if atr_pct > 3.0 {
                risk_level = 3;
            } else {
                risk_level = 2;
            }
        } else {
            // 未突破，根据距离给分
            if distance_to_entry_pct > -2.0 {
                signal_strength = 40;  // 接近突破
            } else if distance_to_entry_pct > -5.0 {
                signal_strength = 20;  // 较远
            } else {
                signal_strength = 0;   // 很远
            }
            risk_level = 3;
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
    
    /// 生成分析描述
    fn generate_description(
        &self,
        is_entry_breakout: bool,
        is_exit_breakout: bool,
        distance_to_entry_pct: f64,
        distance_to_exit_pct: f64,
        atr: f64,
        current_price: f64,
    ) -> String {
        if is_exit_breakout {
            return format!(
                "跌破{}日最低价，触发出场信号。当前价格距出场线{:.2}%",
                self.config.exit_breakout_period,
                distance_to_exit_pct
            );
        }
        
        if is_entry_breakout {
            let atr_pct = (atr / current_price) * 100.0;
            format!(
                "突破{}日最高价{:.2}%，ATR={:.2}({:.2}%)，距出场线{:.2}%",
                self.config.entry_breakout_period,
                distance_to_entry_pct,
                atr,
                atr_pct,
                distance_to_exit_pct
            )
        } else {
            format!(
                "未突破入场线，距离{:.2}%。当前价格在{}日高低点区间内",
                distance_to_entry_pct.abs(),
                self.config.entry_breakout_period
            )
        }
    }
}

impl TradingStrategy for TurtleStrategy {
    type Config = TurtleConfig;
    
    fn name(&self) -> &str {
        if self.config.use_system2 {
            "海龟交易策略 - 系统2"
        } else {
            "海龟交易策略 - 系统1"
        }
    }
    
    fn description(&self) -> &str {
        "经典趋势跟踪策略，基于价格突破和ATR止损"
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
        info!("开始分析股票 {} 的海龟策略信号", symbol);
        
        let result = self.analyze_internal(symbol, data)?;
        
        debug!(
            "股票 {} 分析完成：突破={}, 信号强度={}",
            symbol, result.is_entry_breakout, result.signal_strength
        );
        
        Ok(StrategyResult::Turtle(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data(days: usize, trend: &str) -> Vec<SecurityData> {
        let mut data = Vec::new();
        let base_date = 20240101;
        let mut price = 100.0;
        
        for i in 0..days {
            let (open, close, high, low) = match trend {
                "up" => {
                    price += 1.0;
                    (price - 1.0, price, price + 0.5, price - 1.5)
                },
                "down" => {
                    price -= 1.0;
                    (price + 1.0, price, price + 1.5, price - 0.5)
                },
                _ => {
                    (price, price, price + 1.0, price - 1.0)
                }
            };
            
            data.push(SecurityData {
                trade_date: format!("{}", base_date + i),
                open,
                close,
                high,
                low,
                volume: 1000000.0,
                amount: 100000000.0,
                pct_change: Some(1.0),
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
            });
        }
        
        data
    }
    
    #[test]
    fn test_turtle_breakout() {
        let config = TurtleStrategy::system1();
        let mut strategy = TurtleStrategy::new(config);
        
        // 创建上涨趋势数据
        let data = create_test_data(30, "up");
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::Turtle(r) = result {
            assert!(r.is_entry_breakout, "应该突破入场线");
            assert!(r.signal_strength > 50, "信号强度应该较高");
            assert!(r.atr > 0.0, "ATR应该大于0");
        } else {
            panic!("Expected Turtle result");
        }
    }
    
    #[test]
    fn test_turtle_no_breakout() {
        let config = TurtleStrategy::system1();
        let mut strategy = TurtleStrategy::new(config);
        
        // 创建震荡数据
        let data = create_test_data(30, "flat");
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::Turtle(r) = result {
            assert!(!r.is_entry_breakout, "不应该突破入场线");
        } else {
            panic!("Expected Turtle result");
        }
    }
}
