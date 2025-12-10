//! 诊股结果数据结构

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// 诊股结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosisResult {
    /// 股票代码
    pub stock_code: String,
    /// 诊断日期
    pub diagnosis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 综合诊断等级
    pub overall_level: DiagnosisLevel,
    /// 综合评分 (0-100)
    pub overall_score: u8,
    /// 综合诊断描述
    pub overall_description: String,
    /// 各项指标分析
    pub indicators: Vec<IndicatorAnalysis>,
    /// 风险提示
    pub risk_warnings: Vec<String>,
    /// 投资建议
    pub investment_advice: String,
}

/// 诊断等级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiagnosisLevel {
    /// 强烈看好
    StrongBullish,
    /// 看好
    Bullish,
    /// 中性
    Neutral,
    /// 看空
    Bearish,
    /// 强烈看空
    StrongBearish,
}

impl DiagnosisLevel {
    /// 获取等级描述
    pub fn description(&self) -> &str {
        match self {
            DiagnosisLevel::StrongBullish => "强烈看好",
            DiagnosisLevel::Bullish => "看好",
            DiagnosisLevel::Neutral => "中性",
            DiagnosisLevel::Bearish => "看空",
            DiagnosisLevel::StrongBearish => "强烈看空",
        }
    }

    /// 从评分获取等级
    pub fn from_score(score: u8) -> Self {
        match score {
            80..=100 => DiagnosisLevel::StrongBullish,
            60..=79 => DiagnosisLevel::Bullish,
            40..=59 => DiagnosisLevel::Neutral,
            20..=39 => DiagnosisLevel::Bearish,
            0..=19 => DiagnosisLevel::StrongBearish,
            // 处理超出正常范围的值（101-255），视为强烈看多
            _ => DiagnosisLevel::StrongBullish,
        }
    }
}

/// 单项指标分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorAnalysis {
    /// 指标名称
    pub indicator_name: String,
    /// 指标类型
    pub indicator_type: IndicatorType,
    /// 当前值
    pub current_value: Option<f64>,
    /// 评分 (0-100)
    pub score: u8,
    /// 诊断等级
    pub level: DiagnosisLevel,
    /// 分析描述
    pub description: String,
    /// 具体数据
    pub details: IndicatorDetails,
}

/// 指标类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndicatorType {
    /// 成交量指标
    Volume,
    /// 换手率指标
    TurnoverRate,
    /// 价格指标
    Price,
    /// MACD指标
    Macd,
    /// RSI指标
    Rsi,
    /// KDJ指标
    Kdj,
}

/// 指标详细数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IndicatorDetails {
    /// 成交量分析详情
    Volume {
        /// 当前成交量
        current_volume: f64,
        /// 平均成交量
        average_volume: f64,
        /// 成交量比率
        volume_ratio: f64,
        /// 成交量趋势
        volume_trend: String,
    },
    /// 换手率分析详情
    TurnoverRate {
        /// 当前换手率
        current_rate: f64,
        /// 平均换手率
        average_rate: f64,
        /// 换手率水平
        rate_level: String,
    },
    /// 价格分析详情
    Price {
        /// 当前价格
        current_price: f64,
        /// 价格趋势
        price_trend: String,
        /// 支撑位
        support_level: Option<f64>,
        /// 阻力位
        resistance_level: Option<f64>,
        /// 涨跌幅
        price_change_pct: f64,
    },
    /// MACD分析详情
    Macd {
        /// MACD线
        macd_line: f64,
        /// 信号线
        signal_line: f64,
        /// 柱状图
        histogram: f64,
        /// 趋势信号
        trend_signal: String,
    },
    /// RSI分析详情
    Rsi {
        /// RSI值
        rsi_value: f64,
        /// 超买超卖状态
        overbought_oversold: String,
        /// RSI趋势
        rsi_trend: String,
    },
    /// KDJ分析详情
    Kdj {
        /// K值
        k_value: f64,
        /// D值
        d_value: f64,
        /// J值
        j_value: f64,
        /// KDJ信号
        kdj_signal: String,
    },
}
