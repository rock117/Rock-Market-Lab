//! 优质价值策略 (Quality Value Strategy)
//! 
//! # 策略思想
//! 
//! **核心理念：寻找高质量且估值合理的公司**
//! 
//! 这是一个综合基本面分析策略，结合了三个关键维度：
//! 
//! ## 1. 盈利能力 - ROE（净资产收益率）
//! 
//! ROE = 净利润 / 净资产 × 100%
//! 
//! **为什么重要：**
//! - 衡量公司为股东创造价值的能力
//! - 巴菲特最看重的指标之一
//! - 持续高ROE说明公司有护城河
//! 
//! **判断标准：**
//! - ROE > 15%：优秀
//! - ROE > 20%：卓越
//! - ROE > 25%：顶级
//! 
//! ## 2. 现金流质量 - 经营活动现金流
//! 
//! **为什么重要：**
//! - 利润可以造假，现金流很难造假
//! - 正现金流说明业务真实创造价值
//! - 现金流/净利润 > 1 说明盈利质量高
//! 
//! **判断标准：**
//! - 经营现金流 > 0：基本要求
//! - 经营现金流 > 净利润：优质
//! - 经营现金流 >> 净利润：极优
//! 
//! ## 3. 估值水平 - 市值
//! 
//! **为什么重要：**
//! - 不同市值公司有不同特点
//! - 大盘股稳定，小盘股成长性强
//! - 根据风险偏好选择合适市值
//! 
//! **市值分类：**
//! - 小盘股：< 100亿
//! - 中盘股：100亿 - 500亿
//! - 大盘股：> 500亿
//! 
//! ## 策略优势
//! 
//! 1. **多维度验证**：三个指标相互印证，降低误判
//! 2. **质量优先**：高ROE + 正现金流 = 真正的好公司
//! 3. **灵活配置**：可根据市场环境调整各指标权重
//! 4. **长期有效**：基于公司基本面，不受短期波动影响

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{FinancialData, SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

/// 优质价值策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityValueConfig {
    /// 最小ROE要求（百分比）
    pub min_roe: f64,
    
    /// 是否要求正现金流
    pub require_positive_cashflow: bool,
    
    /// 现金流/净利润最小比率（可选）
    pub min_cashflow_profit_ratio: Option<f64>,
    
    /// 最小市值（亿元，可选）
    pub min_market_cap: Option<f64>,
    
    /// 最大市值（亿元，可选）
    pub max_market_cap: Option<f64>,
}

impl Default for QualityValueConfig {
    fn default() -> Self {
        Self {
            min_roe: 15.0,  // ROE至少15%
            require_positive_cashflow: true,
            min_cashflow_profit_ratio: Some(0.8),  // 现金流至少是净利润的80%
            min_market_cap: None,
            max_market_cap: None,
        }
    }
}

impl StrategyConfig for QualityValueConfig {
    fn strategy_name(&self) -> &str {
        "优质价值策略"
    }
    
    fn analysis_period(&self) -> usize {
        1  // 只需要最新的财务数据
    }
    
    fn validate(&self) -> Result<()> {
        if self.min_roe < 0.0 {
            bail!("min_roe 不能为负数");
        }
        
        if let Some(ratio) = self.min_cashflow_profit_ratio {
            if ratio < 0.0 {
                bail!("min_cashflow_profit_ratio 不能为负数");
            }
        }
        
        if let (Some(min), Some(max)) = (self.min_market_cap, self.max_market_cap) {
            if min > max {
                bail!("min_market_cap 不能大于 max_market_cap");
            }
        }
        
        Ok(())
    }
}

/// 优质价值策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityValueResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// ROE（百分比）
    pub roe: f64,
    
    /// ROE评级
    pub roe_rating: String,
    
    /// 经营现金流（元）
    pub operating_cashflow: f64,
    
    /// 净利润（元）
    pub net_profit: f64,
    
    /// 现金流/净利润比率
    pub cashflow_profit_ratio: f64,
    
    /// 现金流质量评级
    pub cashflow_quality: String,
    
    /// 市值（亿元）
    pub market_cap: Option<f64>,
    
    /// 市值分类
    pub market_cap_category: String,
    
    /// 综合得分 (0-100)
    pub quality_score: u8,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 优质价值策略
pub struct QualityValueStrategy {
    config: QualityValueConfig,
}

impl QualityValueStrategy {
    pub fn new(config: QualityValueConfig) -> Self {
        Self { config }
    }
    
    /// 标准配置（ROE>15%，正现金流）
    pub fn standard() -> QualityValueConfig {
        QualityValueConfig::default()
    }
    
    /// 严格配置（ROE>20%，现金流>净利润）
    pub fn strict() -> QualityValueConfig {
        QualityValueConfig {
            min_roe: 20.0,
            require_positive_cashflow: true,
            min_cashflow_profit_ratio: Some(1.0),
            min_market_cap: None,
            max_market_cap: None,
        }
    }
    
    /// 小盘成长配置（ROE>18%，市值<100亿）
    pub fn small_cap_growth() -> QualityValueConfig {
        QualityValueConfig {
            min_roe: 18.0,
            require_positive_cashflow: true,
            min_cashflow_profit_ratio: Some(0.9),
            min_market_cap: None,
            max_market_cap: Some(100.0),
        }
    }
    
    /// 大盘蓝筹配置（ROE>12%，市值>500亿）
    pub fn large_cap_blue_chip() -> QualityValueConfig {
        QualityValueConfig {
            min_roe: 12.0,
            require_positive_cashflow: true,
            min_cashflow_profit_ratio: Some(0.8),
            min_market_cap: Some(500.0),
            max_market_cap: None,
        }
    }
}

impl TradingStrategy for QualityValueStrategy {
    type Config = QualityValueConfig;
    
    fn name(&self) -> &str {
        "优质价值策略"
    }
    
    fn description(&self) -> &str {
        "基于ROE、现金流和市值的综合基本面选股策略，寻找高质量且估值合理的公司"
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
        Ok(StrategyResult::QualityValue(result))
    }
}

impl QualityValueStrategy {
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<QualityValueResult> {
        if data.is_empty() {
            bail!("数据为空");
        }
        
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 获取财务数据
        let financial_data = latest.financial_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("缺少财务数据"))?;
        
        // 检查ROE
        let roe = financial_data.roe
            .ok_or_else(|| anyhow::anyhow!("缺少ROE数据"))?;
        
        if roe < self.config.min_roe {
            bail!("ROE {:.2}% 低于最小要求 {:.2}%", roe, self.config.min_roe);
        }
        
        // 检查现金流
        let operating_cashflow = financial_data.operating_cash_flow
            .ok_or_else(|| anyhow::anyhow!("缺少经营现金流数据"))?;
        
        if self.config.require_positive_cashflow && operating_cashflow <= 0.0 {
            bail!("经营现金流为负: {:.2}元", operating_cashflow);
        }
        
        // 检查现金流/净利润比率
        let net_profit = financial_data.net_profit
            .ok_or_else(|| anyhow::anyhow!("缺少净利润数据"))?;
        
        let cashflow_profit_ratio = if net_profit != 0.0 {
            operating_cashflow / net_profit
        } else {
            0.0
        };
        
        if let Some(min_ratio) = self.config.min_cashflow_profit_ratio {
            if cashflow_profit_ratio < min_ratio {
                bail!("现金流/净利润比率 {:.2} 低于最小要求 {:.2}", 
                      cashflow_profit_ratio, min_ratio);
            }
        }
        
        // 检查市值
        let market_cap_yuan = financial_data.market_cap;
        let market_cap = market_cap_yuan.map(|v| v / 100_000_000.0);  // 转换为亿元
        
        if let Some(min_cap) = self.config.min_market_cap {
            if let Some(cap) = market_cap {
                if cap < min_cap {
                    bail!("市值 {:.2}亿 低于最小要求 {:.2}亿", cap, min_cap);
                }
            }
        }
        
        if let Some(max_cap) = self.config.max_market_cap {
            if let Some(cap) = market_cap {
                if cap > max_cap {
                    bail!("市值 {:.2}亿 高于最大限制 {:.2}亿", cap, max_cap);
                }
            }
        }
        
        // 生成评级和得分
        let roe_rating = self.rate_roe(roe);
        let cashflow_quality = self.rate_cashflow_quality(cashflow_profit_ratio);
        let market_cap_category = self.categorize_market_cap(market_cap);
        
        let quality_score = self.calculate_quality_score(
            roe,
            cashflow_profit_ratio,
            market_cap,
        );
        
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            quality_score,
            roe,
            cashflow_profit_ratio,
        );
        
        let analysis_description = self.generate_description(
            roe,
            &roe_rating,
            cashflow_profit_ratio,
            &cashflow_quality,
            market_cap,
            &market_cap_category,
        );
        
        Ok(QualityValueResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: 0.0,
            roe,
            roe_rating,
            operating_cashflow,
            net_profit,
            cashflow_profit_ratio,
            cashflow_quality,
            market_cap,
            market_cap_category,
            quality_score,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// ROE评级
    fn rate_roe(&self, roe: f64) -> String {
        if roe >= 25.0 {
            "卓越".to_string()
        } else if roe >= 20.0 {
            "优秀".to_string()
        } else if roe >= 15.0 {
            "良好".to_string()
        } else if roe >= 10.0 {
            "一般".to_string()
        } else {
            "较差".to_string()
        }
    }
    
    /// 现金流质量评级
    fn rate_cashflow_quality(&self, ratio: f64) -> String {
        if ratio >= 1.5 {
            "极优".to_string()
        } else if ratio >= 1.0 {
            "优秀".to_string()
        } else if ratio >= 0.8 {
            "良好".to_string()
        } else if ratio >= 0.5 {
            "一般".to_string()
        } else if ratio >= 0.0 {
            "较差".to_string()
        } else {
            "很差".to_string()
        }
    }
    
    /// 市值分类
    fn categorize_market_cap(&self, market_cap: Option<f64>) -> String {
        match market_cap {
            Some(cap) if cap >= 1000.0 => "超大盘股".to_string(),
            Some(cap) if cap >= 500.0 => "大盘股".to_string(),
            Some(cap) if cap >= 100.0 => "中盘股".to_string(),
            Some(cap) if cap >= 50.0 => "小盘股".to_string(),
            Some(_) => "微盘股".to_string(),
            None => "未知".to_string(),
        }
    }
    
    /// 计算综合质量得分
    fn calculate_quality_score(
        &self,
        roe: f64,
        cashflow_profit_ratio: f64,
        market_cap: Option<f64>,
    ) -> u8 {
        let mut score = 0u8;
        
        // ROE得分（50分）
        if roe >= 25.0 {
            score += 50;
        } else if roe >= 20.0 {
            score += 45;
        } else if roe >= 15.0 {
            score += 40;
        } else if roe >= 10.0 {
            score += 30;
        } else {
            score += 20;
        }
        
        // 现金流质量得分（40分）
        if cashflow_profit_ratio >= 1.5 {
            score += 40;
        } else if cashflow_profit_ratio >= 1.0 {
            score += 35;
        } else if cashflow_profit_ratio >= 0.8 {
            score += 30;
        } else if cashflow_profit_ratio >= 0.5 {
            score += 20;
        } else if cashflow_profit_ratio >= 0.0 {
            score += 10;
        }
        
        // 市值稳定性得分（10分）
        if let Some(cap) = market_cap {
            if cap >= 500.0 {
                score += 10;  // 大盘股更稳定
            } else if cap >= 100.0 {
                score += 8;
            } else if cap >= 50.0 {
                score += 6;
            } else {
                score += 4;
            }
        }
        
        score.min(100)
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        quality_score: u8,
        roe: f64,
        cashflow_profit_ratio: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = quality_score;
        
        // 根据综合得分确定信号
        let strategy_signal = if quality_score >= 85 {
            StrategySignal::StrongBuy
        } else if quality_score >= 70 {
            StrategySignal::Buy
        } else {
            StrategySignal::Hold
        };
        
        // 风险等级（质量越高风险越低）
        let risk_level = if quality_score >= 85 && roe >= 20.0 && cashflow_profit_ratio >= 1.0 {
            2  // 低风险
        } else if quality_score >= 70 {
            3  // 中等风险
        } else {
            4  // 较高风险
        };
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        roe: f64,
        roe_rating: &str,
        cashflow_profit_ratio: f64,
        cashflow_quality: &str,
        market_cap: Option<f64>,
        market_cap_category: &str,
    ) -> String {
        let market_cap_str = market_cap
            .map(|c| format!("{:.2}亿", c))
            .unwrap_or_else(|| "未知".to_string());
        
        format!(
            "ROE {:.2}%（{}），现金流/净利润比率 {:.2}（{}），市值 {}（{}）。{}",
            roe,
            roe_rating,
            cashflow_profit_ratio,
            cashflow_quality,
            market_cap_str,
            market_cap_category,
            if roe >= 20.0 && cashflow_profit_ratio >= 1.0 {
                "优质价值股，盈利能力强且现金流充沛"
            } else if roe >= 15.0 && cashflow_profit_ratio >= 0.8 {
                "质量良好，值得关注"
            } else {
                "满足基本条件"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::traits::{SecurityType, TimeFrame};
    
    fn create_test_data_with_financials(
        roe: f64,
        operating_cashflow: f64,
        net_profit: f64,
        market_cap: Option<f64>,
    ) -> Vec<SecurityData> {
        let financial_data = FinancialData {
            report_period: "2024Q3".to_string(),
            revenue: Some(1_000_000_000.0),
            net_profit: Some(net_profit),
            gross_profit_margin: Some(30.0),
            selling_expense_ratio: Some(10.0),
            admin_expense_ratio: Some(5.0),
            financial_expense_ratio: Some(2.0),
            operating_cash_flow: Some(operating_cashflow),
            inventory: Some(100_000_000.0),
            accounts_receivable: Some(200_000_000.0),
            advances_from_customers: Some(50_000_000.0),
            accounts_payable: Some(150_000_000.0),
            market_cap,
            roe: Some(roe),
        };
        
        vec![SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20241125".to_string(),
            open: 10.0,
            close: 10.5,
            high: 10.8,
            low: 9.9,
            pre_close: Some(10.0),
            change: Some(0.5),
            pct_change: Some(5.0),
            volume: 1000000.0,
            amount: 10500000.0,
            turnover_rate: None,
            time_frame: TimeFrame::Daily,
            security_type: SecurityType::Stock,
            financial_data: Some(financial_data),
            target: None,
        }]
    }
    
    #[test]
    fn test_quality_value_strategy_excellent() {
        let config = QualityValueStrategy::standard();
        let mut strategy = QualityValueStrategy::new(config);
        
        // 优质股票：ROE 22%，现金流 > 净利润
        let data = create_test_data_with_financials(
            22.0,
            120_000_000.0,
            100_000_000.0,
            Some(30_000_000_000.0),  // 300亿市值
        );
        
        let result = strategy.analyze("000001.SZ", &data);
        assert!(result.is_ok());
        
        if let Ok(StrategyResult::QualityValue(r)) = result {
            assert_eq!(r.roe, 22.0);
            assert_eq!(r.roe_rating, "优秀");
            assert!(r.cashflow_profit_ratio >= 1.0);
            assert!(r.quality_score >= 80);
        } else {
            panic!("Expected QualityValue result");
        }
    }
    
    #[test]
    fn test_quality_value_strategy_low_roe() {
        let config = QualityValueStrategy::standard();
        let mut strategy = QualityValueStrategy::new(config);
        
        // ROE不足
        let data = create_test_data_with_financials(
            10.0,  // ROE只有10%
            100_000_000.0,
            100_000_000.0,
            Some(50_000_000_000.0),
        );
        
        let result = strategy.analyze("000001.SZ", &data);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_quality_value_strategy_negative_cashflow() {
        let config = QualityValueStrategy::standard();
        let mut strategy = QualityValueStrategy::new(config);
        
        // 负现金流
        let data = create_test_data_with_financials(
            20.0,
            -50_000_000.0,  // 负现金流
            100_000_000.0,
            Some(50_000_000_000.0),
        );
        
        let result = strategy.analyze("000001.SZ", &data);
        assert!(result.is_err());
    }
}
