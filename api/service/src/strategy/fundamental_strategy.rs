//! 基本面选股策略
//! 
//! 基于财务指标筛选优质股票，关注盈利能力、成长性、现金流等核心指标

use super::traits::*;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// 基本面策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalConfig {
    /// 最小营收增长率（%）
    pub min_revenue_growth: f64,
    
    /// 最小净利润增长率（%）
    pub min_profit_growth: f64,
    
    /// 最小毛利率（%）
    pub min_gross_margin: f64,
    
    /// 最小净利率（%）
    pub min_net_margin: f64,
    
    /// 最小ROE（净资产收益率，%）
    pub min_roe: f64,
    
    /// 最大负债率（%）
    pub max_debt_ratio: f64,
    
    /// 要求经营现金流为正
    pub require_positive_cash_flow: bool,
    
    /// 最小市盈率（PE）
    pub min_pe: Option<f64>,
    
    /// 最大市盈率（PE）
    pub max_pe: Option<f64>,
    
    /// 最小市净率（PB）
    pub min_pb: Option<f64>,
    
    /// 最大市净率（PB）
    pub max_pb: Option<f64>,
    
    /// 最小市值（亿元）
    pub min_market_cap: Option<f64>,
    
    /// 最大市值（亿元）
    pub max_market_cap: Option<f64>,
}

impl Default for FundamentalConfig {
    fn default() -> Self {
        Self {
            min_revenue_growth: 10.0,      // 营收增长至少10%
            min_profit_growth: 15.0,       // 净利润增长至少15%
            min_gross_margin: 20.0,        // 毛利率至少20%
            min_net_margin: 5.0,           // 净利率至少5%
            min_roe: 10.0,                 // ROE至少10%
            max_debt_ratio: 70.0,          // 负债率不超过70%
            require_positive_cash_flow: true, // 要求正现金流
            min_pe: Some(5.0),             // PE不低于5
            max_pe: Some(50.0),            // PE不超过50
            min_pb: Some(1.0),             // PB不低于1
            max_pb: Some(10.0),            // PB不超过10
            min_market_cap: None,          // 不限制最小市值
            max_market_cap: None,          // 不限制最大市值
        }
    }
}

impl StrategyConfig for FundamentalConfig {
    fn strategy_name(&self) -> &str {
        "fundamental"
    }
    
    fn analysis_period(&self) -> usize {
        1 // 基本面策略只需要最新的财务数据
    }
    
    fn validate(&self) -> Result<()> {
        if self.min_revenue_growth < -100.0 || self.min_revenue_growth > 1000.0 {
            bail!("营收增长率范围应在 -100% 到 1000% 之间");
        }
        
        if self.min_profit_growth < -100.0 || self.min_profit_growth > 1000.0 {
            bail!("净利润增长率范围应在 -100% 到 1000% 之间");
        }
        
        if self.min_gross_margin < 0.0 || self.min_gross_margin > 100.0 {
            bail!("毛利率范围应在 0% 到 100% 之间");
        }
        
        if self.min_net_margin < -50.0 || self.min_net_margin > 100.0 {
            bail!("净利率范围应在 -50% 到 100% 之间");
        }
        
        if self.min_roe < -100.0 || self.min_roe > 100.0 {
            bail!("ROE范围应在 -100% 到 100% 之间");
        }
        
        if self.max_debt_ratio < 0.0 || self.max_debt_ratio > 100.0 {
            bail!("负债率范围应在 0% 到 100% 之间");
        }
        
        if let (Some(min), Some(max)) = (self.min_pe, self.max_pe) {
            if min > max {
                bail!("最小PE不能大于最大PE");
            }
        }
        
        if let (Some(min), Some(max)) = (self.min_pb, self.max_pb) {
            if min > max {
                bail!("最小PB不能大于最大PB");
            }
        }
        
        if let (Some(min), Some(max)) = (self.min_market_cap, self.max_market_cap) {
            if min < 0.0 {
                bail!("最小市值不能为负数");
            }
            if max < 0.0 {
                bail!("最大市值不能为负数");
            }
            if min > max {
                bail!("最小市值不能大于最大市值");
            }
        }
        
        Ok(())
    }
}

/// 基本面选股策略
pub struct FundamentalStrategy {
    config: FundamentalConfig,
}

impl FundamentalStrategy {
    pub fn new(config: FundamentalConfig) -> Self {
        Self { config }
    }
    
    /// 创建小市值高成长科技股配置
    /// 
    /// 特点：
    /// - 市值范围：20-100亿元（小盘股）
    /// - 高成长：营收增长≥40%，利润增长≥50%
    /// - 高毛利：毛利率≥40%（科技股特征）
    /// - 高净利：净利率≥15%
    /// - 高ROE：≥20%
    /// - 低负债：≤40%
    /// - 允许高估值：PE 20-150
    pub fn small_cap_tech_growth() -> FundamentalConfig {
        FundamentalConfig {
            min_revenue_growth: 40.0,      // 营收高增长
            min_profit_growth: 50.0,       // 利润高增长
            min_gross_margin: 40.0,        // 高毛利率（科技股特征）
            min_net_margin: 15.0,          // 高净利率
            min_roe: 20.0,                 // 高ROE
            max_debt_ratio: 40.0,          // 低负债
            require_positive_cash_flow: true,
            min_pe: Some(20.0),            // 允许高估值
            max_pe: Some(150.0),
            min_pb: Some(3.0),
            max_pb: Some(30.0),
            min_market_cap: Some(20.0),    // 最小市值20亿元
            max_market_cap: Some(100.0),   // 最大市值100亿元（小盘股）
        }
    }
    
    /// 创建中小市值高成长科技股配置（稍微放宽条件）
    /// 
    /// 特点：
    /// - 市值范围：20-200亿元
    /// - 较高成长：营收增长≥30%，利润增长≥35%
    /// - 高毛利：毛利率≥35%
    /// - 较高净利：净利率≥12%
    /// - 较高ROE：≥15%
    /// - 适中负债：≤50%
    pub fn small_mid_cap_tech_growth() -> FundamentalConfig {
        FundamentalConfig {
            min_revenue_growth: 30.0,      // 较高营收增长
            min_profit_growth: 35.0,       // 较高利润增长
            min_gross_margin: 35.0,        // 高毛利率
            min_net_margin: 12.0,          // 较高净利率
            min_roe: 15.0,                 // 较高ROE
            max_debt_ratio: 50.0,          // 适中负债
            require_positive_cash_flow: true,
            min_pe: Some(15.0),
            max_pe: Some(120.0),
            min_pb: Some(2.5),
            max_pb: Some(25.0),
            min_market_cap: Some(20.0),    // 最小市值20亿元
            max_market_cap: Some(200.0),   // 最大市值200亿元
        }
    }
    
    /// 创建潜力科技股配置（条件更宽松，适合早期发现）
    /// 
    /// 特点：
    /// - 市值范围：10-150亿元
    /// - 成长性：营收增长≥25%，利润增长≥30%
    /// - 毛利率：≥30%（科技属性）
    /// - 净利率：≥10%
    /// - ROE：≥12%
    /// - 负债：≤60%
    pub fn potential_tech_growth() -> FundamentalConfig {
        FundamentalConfig {
            min_revenue_growth: 25.0,      // 成长性要求
            min_profit_growth: 30.0,       // 利润增长要求
            min_gross_margin: 30.0,        // 科技属性（高毛利）
            min_net_margin: 10.0,          // 盈利能力
            min_roe: 12.0,                 // ROE要求
            max_debt_ratio: 60.0,          // 负债控制
            require_positive_cash_flow: true,
            min_pe: Some(10.0),
            max_pe: Some(100.0),
            min_pb: Some(2.0),
            max_pb: Some(20.0),
            min_market_cap: Some(10.0),    // 最小市值10亿元
            max_market_cap: Some(150.0),   // 最大市值150亿元
        }
    }
    
    /// 创建成熟科技股配置（已有稳定营收和利润）
    /// 
    /// 特点：
    /// - 市值范围：50-500亿元（已上市一段时间）
    /// - 稳定成长：营收增长≥20%，利润增长≥25%
    /// - 高毛利：毛利率≥35%（科技属性）
    /// - 盈利能力：净利率≥8%
    /// - ROE：≥10%
    /// - 负债：≤50%
    /// - 要求正现金流
    /// 
    /// 注意：此配置适合已经过了初创期、有稳定财务数据的科技公司
    /// 刚成立的公司可能不适用，因为：
    /// 1. 营收基数小，增长率可能失真
    /// 2. 可能还在亏损阶段
    /// 3. 财务数据不完整
    pub fn mature_tech_growth() -> FundamentalConfig {
        FundamentalConfig {
            min_revenue_growth: 20.0,      // 稳定增长
            min_profit_growth: 25.0,       // 利润增长
            min_gross_margin: 35.0,        // 科技属性
            min_net_margin: 8.0,           // 盈利能力
            min_roe: 10.0,                 // ROE要求
            max_debt_ratio: 50.0,          // 负债控制
            require_positive_cash_flow: true,
            min_pe: Some(15.0),
            max_pe: Some(80.0),
            min_pb: Some(2.0),
            max_pb: Some(15.0),
            min_market_cap: Some(50.0),    // 最小市值50亿元
            max_market_cap: Some(500.0),   // 最大市值500亿元
        }
    }
}

impl TradingStrategy for FundamentalStrategy {
    type Config = FundamentalConfig;
    
    fn name(&self) -> &str {
        "基本面选股策略"
    }
    
    fn description(&self) -> &str {
        "基于财务指标筛选优质股票，关注盈利能力、成长性、现金流等核心指标"
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
        Ok(StrategyResult::Fundamental(result))
    }
}

impl FundamentalStrategy {
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<FundamentalResult> {
        if data.is_empty() {
            bail!("数据不足: 需要至少 1 个数据点");
        }
        
        // 获取最新数据
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 检查是否有财务数据
        let financial_data = latest.financial_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("缺少财务数据"))?;
        
        debug!("分析股票 {} 的基本面数据", symbol);
        
        // 计算各项指标
        let revenue = financial_data.revenue.unwrap_or(0.0);
        let net_profit = financial_data.net_profit.unwrap_or(0.0);
        let gross_margin = financial_data.gross_profit_margin.unwrap_or(0.0);
        let operating_cash_flow = financial_data.operating_cash_flow.unwrap_or(0.0);
        
        // 计算净利率
        let net_margin = if revenue > 0.0 {
            (net_profit / revenue) * 100.0
        } else {
            0.0
        };
        
        // 这里需要从历史数据计算增长率，暂时使用模拟值
        // 实际应用中需要对比上一期的数据
        let revenue_growth = self.calculate_growth_rate(data, "revenue");
        let profit_growth = self.calculate_growth_rate(data, "profit");
        
        // 计算ROE（需要净资产数据，这里简化处理）
        let roe = 0.0; // 实际需要从财务数据中获取
        
        // 计算负债率（需要资产负债数据，这里简化处理）
        let debt_ratio = 0.0; // 实际需要从财务数据中获取
        
        // 计算PE和PB（需要市值和每股数据，这里简化处理）
        let pe_ratio = None;
        let pb_ratio = None;
        
        // 计算市值（需要总股本和当前价格，这里简化处理）
        // 实际应用中：市值 = 总股本 × 当前价格 / 100000000（转换为亿元）
        let market_cap = None; // 实际需要从股票基本信息中获取总股本
        
        // 市值筛选
        if let Some(market_cap_value) = market_cap {
            if let Some(min_cap) = self.config.min_market_cap {
                if market_cap_value < min_cap {
                    debug!("股票 {} 市值{:.2}亿元，低于最小市值要求{:.2}亿元", 
                           symbol, market_cap_value, min_cap);
                    return Ok(FundamentalResult {
                        stock_code: symbol.to_string(),
                        analysis_date,
                        current_price,
                        strategy_signal: StrategySignal::Hold,
                        signal_strength: 0,
                        analysis_description: format!("市值{:.2}亿元，低于最小市值要求", market_cap_value),
                        risk_level: 3,
                        report_period: financial_data.report_period.clone(),
                        revenue,
                        revenue_growth,
                        net_profit,
                        profit_growth,
                        gross_margin,
                        net_margin,
                        roe,
                        debt_ratio,
                        operating_cash_flow,
                        pe_ratio,
                        pb_ratio,
                        market_cap,
                    });
                }
            }
            
            if let Some(max_cap) = self.config.max_market_cap {
                if market_cap_value > max_cap {
                    debug!("股票 {} 市值{:.2}亿元，高于最大市值要求{:.2}亿元", 
                           symbol, market_cap_value, max_cap);
                    return Ok(FundamentalResult {
                        stock_code: symbol.to_string(),
                        analysis_date,
                        current_price,
                        strategy_signal: StrategySignal::Hold,
                        signal_strength: 0,
                        analysis_description: format!("市值{:.2}亿元，高于最大市值要求", market_cap_value),
                        risk_level: 3,
                        report_period: financial_data.report_period.clone(),
                        revenue,
                        revenue_growth,
                        net_profit,
                        profit_growth,
                        gross_margin,
                        net_margin,
                        roe,
                        debt_ratio,
                        operating_cash_flow,
                        pe_ratio,
                        pb_ratio,
                        market_cap,
                    });
                }
            }
        }
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            revenue_growth,
            profit_growth,
            gross_margin,
            net_margin,
            roe,
            debt_ratio,
            operating_cash_flow > 0.0,
            pe_ratio,
            pb_ratio,
            market_cap,
        );
        
        // 生成分析说明
        let analysis_description = self.generate_description(
            revenue_growth,
            profit_growth,
            gross_margin,
            net_margin,
            roe,
            debt_ratio,
            operating_cash_flow,
        );
        
        info!(
            "股票 {}: 营收增长{:.2}%, 利润增长{:.2}%, 毛利率{:.2}%, 信号={:?}",
            symbol, revenue_growth, profit_growth, gross_margin, strategy_signal
        );
        
        Ok(FundamentalResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            report_period: financial_data.report_period.clone(),
            revenue,
            revenue_growth,
            net_profit,
            profit_growth,
            gross_margin,
            net_margin,
            roe,
            debt_ratio,
            operating_cash_flow,
            pe_ratio,
            pb_ratio,
            market_cap,
        })
    }
    
    /// 计算增长率（简化版本，实际需要对比历史数据）
    fn calculate_growth_rate(&self, _data: &[SecurityData], _metric: &str) -> f64 {
        // TODO: 实际实现需要对比上一期的财务数据
        // 这里返回模拟值
        15.0
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        revenue_growth: f64,
        profit_growth: f64,
        gross_margin: f64,
        net_margin: f64,
        roe: f64,
        debt_ratio: f64,
        positive_cash_flow: bool,
        pe_ratio: Option<f64>,
        _pb_ratio: Option<f64>,
        _market_cap: Option<f64>,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 营收增长评分（20分）
        if revenue_growth >= self.config.min_revenue_growth * 2.0 {
            signal_strength += 20;
        } else if revenue_growth >= self.config.min_revenue_growth * 1.5 {
            signal_strength += 15;
        } else if revenue_growth >= self.config.min_revenue_growth {
            signal_strength += 10;
        }
        
        // 利润增长评分（25分）
        if profit_growth >= self.config.min_profit_growth * 2.0 {
            signal_strength += 25;
        } else if profit_growth >= self.config.min_profit_growth * 1.5 {
            signal_strength += 18;
        } else if profit_growth >= self.config.min_profit_growth {
            signal_strength += 12;
        }
        
        // 毛利率评分（15分）
        if gross_margin >= self.config.min_gross_margin * 1.5 {
            signal_strength += 15;
        } else if gross_margin >= self.config.min_gross_margin {
            signal_strength += 10;
        }
        
        // 净利率评分（15分）
        if net_margin >= self.config.min_net_margin * 2.0 {
            signal_strength += 15;
        } else if net_margin >= self.config.min_net_margin {
            signal_strength += 10;
        }
        
        // ROE评分（10分）
        if roe >= self.config.min_roe * 1.5 {
            signal_strength += 10;
        } else if roe >= self.config.min_roe {
            signal_strength += 6;
        }
        
        // 现金流评分（10分）
        if self.config.require_positive_cash_flow && positive_cash_flow {
            signal_strength += 10;
        } else if !self.config.require_positive_cash_flow {
            signal_strength += 5;
        }
        
        // 负债率风险调整
        if debt_ratio > self.config.max_debt_ratio {
            risk_level = 4;
            signal_strength = signal_strength.saturating_sub(20);
        } else if debt_ratio > self.config.max_debt_ratio * 0.8 {
            risk_level = 3;
        } else {
            risk_level = 2;
        }
        
        // PE估值调整（5分）
        if let Some(pe) = pe_ratio {
            if let (Some(min), Some(max)) = (self.config.min_pe, self.config.max_pe) {
                if pe >= min && pe <= max {
                    signal_strength += 5;
                } else if pe < min || pe > max * 1.5 {
                    signal_strength = signal_strength.saturating_sub(10);
                }
            }
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
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        revenue_growth: f64,
        profit_growth: f64,
        gross_margin: f64,
        net_margin: f64,
        roe: f64,
        debt_ratio: f64,
        operating_cash_flow: f64,
    ) -> String {
        let mut desc = Vec::new();
        
        // 营收增长
        if revenue_growth >= self.config.min_revenue_growth {
            desc.push(format!(
                "✓ 营收增长率{:.2}%，成长性良好",
                revenue_growth
            ));
        } else {
            desc.push(format!(
                "✗ 营收增长率仅{:.2}%，低于预期",
                revenue_growth
            ));
        }
        
        // 利润增长
        if profit_growth >= self.config.min_profit_growth {
            desc.push(format!(
                "✓ 净利润增长率{:.2}%，盈利能力强",
                profit_growth
            ));
        } else {
            desc.push(format!(
                "✗ 净利润增长率仅{:.2}%，盈利增长不足",
                profit_growth
            ));
        }
        
        // 毛利率
        if gross_margin >= self.config.min_gross_margin {
            desc.push(format!(
                "✓ 毛利率{:.2}%，产品竞争力强",
                gross_margin
            ));
        } else {
            desc.push(format!(
                "✗ 毛利率仅{:.2}%，盈利空间有限",
                gross_margin
            ));
        }
        
        // 净利率
        if net_margin >= self.config.min_net_margin {
            desc.push(format!(
                "✓ 净利率{:.2}%，成本控制良好",
                net_margin
            ));
        } else {
            desc.push(format!(
                "✗ 净利率仅{:.2}%，成本压力大",
                net_margin
            ));
        }
        
        // 现金流
        if operating_cash_flow > 0.0 {
            desc.push(format!(
                "✓ 经营现金流{:.2}亿元，现金流健康",
                operating_cash_flow / 100_000_000.0
            ));
        } else {
            desc.push(format!(
                "⚠ 经营现金流{:.2}亿元，需关注资金状况",
                operating_cash_flow / 100_000_000.0
            ));
        }
        
        // 负债率
        if debt_ratio <= self.config.max_debt_ratio {
            desc.push(format!(
                "✓ 负债率{:.2}%，财务风险可控",
                debt_ratio
            ));
        } else {
            desc.push(format!(
                "⚠ 负债率{:.2}%，财务风险较高",
                debt_ratio
            ));
        }
        
        desc.join("；")
    }
}

/// 基本面策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalResult {
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
    /// 报告期
    pub report_period: String,
    /// 营业收入（元）
    pub revenue: f64,
    /// 营收增长率（%）
    pub revenue_growth: f64,
    /// 净利润（元）
    pub net_profit: f64,
    /// 净利润增长率（%）
    pub profit_growth: f64,
    /// 毛利率（%）
    pub gross_margin: f64,
    /// 净利率（%）
    pub net_margin: f64,
    /// ROE（%）
    pub roe: f64,
    /// 负债率（%）
    pub debt_ratio: f64,
    /// 经营现金流（元）
    pub operating_cash_flow: f64,
    /// 市盈率（PE）
    pub pe_ratio: Option<f64>,
    /// 市净率（PB）
    pub pb_ratio: Option<f64>,
    /// 市值（亿元）
    pub market_cap: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data_with_financials() -> Vec<SecurityData> {
        vec![SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20241118".to_string(),
            open: 10.0,
            high: 11.0,
            low: 9.5,
            close: 10.5,
            pre_close: Some(10.0),
            change: Some(0.5),
            pct_change: Some(5.0),
            volume: 1000000.0,
            amount: 10500000.0,
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
            financial_data: Some(FinancialData {
                report_period: "2024Q3".to_string(),
                revenue: Some(1_000_000_000.0),
                net_profit: Some(100_000_000.0),
                gross_profit_margin: Some(30.0),
                selling_expense_ratio: Some(10.0),
                admin_expense_ratio: Some(5.0),
                financial_expense_ratio: Some(2.0),
                operating_cash_flow: Some(120_000_000.0),
                inventory: Some(50_000_000.0),
                accounts_receivable: Some(80_000_000.0),
                advances_from_customers: Some(30_000_000.0),
                accounts_payable: Some(60_000_000.0),
            }),
        }]
    }
    
    #[test]
    fn test_fundamental_strategy_basic() {
        let config = FundamentalConfig::default();
        let mut strategy = FundamentalStrategy::new(config);
        
        let data = create_test_data_with_financials();
        let result = strategy.analyze("000001.SZ", &data);
        
        assert!(result.is_ok());
        let result = result.unwrap();
        
        if let StrategyResult::Fundamental(fund_result) = result {
            assert_eq!(fund_result.stock_code, "000001.SZ");
            assert!(fund_result.signal_strength > 0);
            println!("信号强度: {}", fund_result.signal_strength);
            println!("分析说明: {}", fund_result.analysis_description);
        } else {
            panic!("Expected Fundamental result");
        }
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = FundamentalConfig::default();
        assert!(config.validate().is_ok());
        
        config.min_gross_margin = 150.0; // 无效值
        assert!(config.validate().is_err());
        
        config.min_gross_margin = 20.0;
        config.min_pe = Some(100.0);
        config.max_pe = Some(50.0); // min > max
        assert!(config.validate().is_err());
    }
}
