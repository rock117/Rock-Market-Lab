//! 困境反转策略
//! 
//! 基于财务报表数据，识别经历困境但有反转迹象的公司
//! 
//! 核心逻辑：
//! 1. 识别困境特征：连续亏损、ROE下降、负债率上升等
//! 2. 识别反转信号：盈利改善、现金流好转、资产质量提升
//! 3. 估值合理性：PE、PB处于历史低位
//! 4. 技术面配合：价格企稳、成交量放大

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    SecurityData,
};

/// 困境反转策略配置
/// 
/// # 评分体系（总分14分）
/// 
/// ## 盈利能力（4分）
/// - 营收增速：连续2季改善 → 2分
/// - 净利润：连续2季改善或转正 → 2分
/// 
/// ## 盈利质量（4分）
/// - 毛利率：连续上升 → 1分
/// - 费用率：降低 → 1分
/// - 经营现金流：转正或大幅改善 → 2分
/// 
/// ## 营运资本（6分）
/// - 存货：减少 → 2分
/// - 应收账款：减少 → 2分
/// - 预收账款：增加 → 1分
/// - 应付账款：增加 → 1分
/// 
/// ## 信号等级
/// - 11-14分（80%+）：StrongBuy 强烈买入
/// - 8-10分（60-80%）：Buy 买入
/// - 6-7分（40-60%）：Hold 持有
/// - 0-5分（<40%）：Sell 卖出
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DistressedReversalConfig {
    /// 需要的最少财务季度数（用于计算趋势）
    pub min_quarters: usize,
    
    /// 营收大幅改善阈值（百分比）
    pub revenue_improvement_threshold: f64,
    
    /// 现金流大幅改善阈值（倍数）
    pub cashflow_improvement_threshold: f64,
}

impl Default for DistressedReversalConfig {
    fn default() -> Self {
        Self {
            min_quarters: 3,  // 至少需要3个季度数据（当前+前2季）
            revenue_improvement_threshold: 20.0,  // 营收改善20%
            cashflow_improvement_threshold: 2.0,  // 现金流改善2倍
        }
    }
}

impl StrategyConfigTrait for DistressedReversalConfig {
    fn strategy_name(&self) -> &str {
        "DistressedReversal"
    }
    
    fn analysis_period(&self) -> usize {
        60  // 固定返回60天
    }
    
    fn validate(&self) -> Result<()> {
        if self.min_quarters < 2 {
            return Err(anyhow::anyhow!("最少需要2个季度数据"));
        }
        Ok(())
    }
}

/// 困境反转策略分析结果
/// 
/// 基于14分制评分体系，综合评估企业困境反转信号：
/// - 盈利能力改善（4分）
/// - 盈利质量提升（4分）
/// - 营运资本优化（6分）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistressedReversalResult {
    /// 股票代码
    pub stock_code: String,
    /// 分析日期
    pub analysis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 策略信号
    pub strategy_signal: StrategySignal,
    /// 信号强度 (0-10)
    pub signal_strength: u8,
    /// 分析说明
    pub analysis_description: String,
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 各项指标得分明细
    /// 营收增速得分 (0-2)
    pub revenue_score: u8,
    /// 净利润得分 (0-2)
    pub profit_score: u8,
    /// 毛利率得分 (0-1)
    pub margin_score: u8,
    /// 费用率得分 (0-1)
    pub expense_score: u8,
    /// 经营现金流得分 (0-2)
    pub cashflow_score: u8,
    /// 存货周转得分 (0-2)
    pub inventory_score: u8,
    /// 应收账款周转得分 (0-2)
    pub receivable_score: u8,
    /// 预收账款得分 (0-1)
    pub advance_score: u8,
    /// 应付账款得分 (0-1)
    pub payable_score: u8,
    
    // 详细指标数据
    /// 最新季度报告期
    pub latest_period: String,
    /// 营收同比增速（百分比）
    pub revenue_growth: Option<f64>,
    /// 净利润（元）
    pub net_profit: Option<f64>,
    /// 净利润同比增速（百分比）
    pub profit_growth: Option<f64>,
    /// 毛利率（百分比）
    pub gross_margin: Option<f64>,
    /// 三费合计费用率（百分比）
    pub total_expense_ratio: Option<f64>,
    /// 经营现金流（元）
    pub operating_cashflow: Option<f64>,
    /// 存货（元）
    pub inventory: Option<f64>,
    /// 应收账款（元）
    pub accounts_receivable: Option<f64>,
    /// 预收账款（元）
    pub advances_from_customers: Option<f64>,
    /// 应付账款（元）
    pub accounts_payable: Option<f64>,
}

/// 困境反转策略
/// 
/// 通过分析企业季度财务数据，识别困境企业的反转信号。
/// 
/// # 核心逻辑
/// 1. 需要至少3个季度的财务数据（当前季度+前2季度）
/// 2. 对9个财务指标进行评分（总分14分）
/// 3. 根据总分生成买卖信号
/// 
/// # 评分维度
/// - **盈利能力**：营收增速、净利润
/// - **盈利质量**：毛利率、费用率、现金流
/// - **营运资本**：存货、应收账款、预收账款、应付账款
pub struct DistressedReversalStrategy {
    config: DistressedReversalConfig,
}

impl DistressedReversalStrategy {
    /// 创建新的策略实例
    pub fn new(config: DistressedReversalConfig) -> Self {
        Self { config }
    }
    
    /// 创建默认配置的策略
    pub fn default_strategy() -> Self {
        Self::new(DistressedReversalConfig::default())
    }
    
    /// 创建激进配置（更宽松的条件）
    pub fn aggressive() -> Self {
        Self::new(DistressedReversalConfig {
            min_quarters: 2,
            revenue_improvement_threshold: 10.0,
            cashflow_improvement_threshold: 1.5,
        })
    }
    
    /// 创建保守配置（更严格的条件）
    pub fn conservative() -> Self {
        Self::new(DistressedReversalConfig {
            min_quarters: 4,
            revenue_improvement_threshold: 30.0,
            cashflow_improvement_threshold: 3.0,
        })
    }
}

impl TradingStrategy for DistressedReversalStrategy {
    type Config = DistressedReversalConfig;
    
    fn name(&self) -> &str {
        "困境反转策略"
    }
    
    fn description(&self) -> &str {
        "基于财务报表识别困境反转机会，关注盈利改善、现金流好转、估值低位的公司"
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
        info!("开始分析股票 {} 的困境反转信号", symbol);
        
        let result = self.analyze_internal(symbol, data)?;
        
        Ok(StrategyResult::DistressedReversal(result))
    }
}

impl DistressedReversalStrategy {
    /// 内部分析方法
    fn analyze_internal(
        &self,
        symbol: &str,
        data: &[SecurityData],
    ) -> Result<DistressedReversalResult> {
        if data.is_empty() {
            return Ok(self.create_empty_result(symbol, "数据为空"));
        }
        
        // 提取所有有财务数据的季度，并按报告期排序
        let mut quarters: Vec<_> = data.iter()
            .filter_map(|d| d.financial_data.as_ref().map(|fd| (d, fd)))
            .collect();
        
        if quarters.len() < self.config.min_quarters {
            return Ok(self.create_empty_result(symbol, 
                &format!("财务数据不足，需要至少{}个季度", self.config.min_quarters)));
        }
        
        // 按报告期排序（假设 report_period 格式为 "2024Q3"）
        quarters.sort_by(|a, b| a.1.report_period.cmp(&b.1.report_period));
        
        let (latest_data, latest_fin) = quarters.last().unwrap();
        let current_price = latest_data.close;
        let analysis_date = NaiveDate::parse_from_str(&latest_data.trade_date, "%Y%m%d")?;
        
        // 计算各项指标得分
        let revenue_score = self.score_revenue(&quarters)?;
        let profit_score = self.score_profit(&quarters)?;
        let margin_score = self.score_margin(&quarters)?;
        let expense_score = self.score_expense(&quarters)?;
        let cashflow_score = self.score_cashflow(&quarters)?;
        let inventory_score = self.score_inventory(&quarters)?;
        let receivable_score = self.score_receivable(&quarters)?;
        let advance_score = self.score_advance(&quarters)?;
        let payable_score = self.score_payable(&quarters)?;
        
        // 总分（14分：营收2+利润2+毛利1+费用1+现金流2+存货2+应收2+预收1+应付1）
        let total_score = revenue_score + profit_score + margin_score + 
                         expense_score + cashflow_score +
                         inventory_score + receivable_score + advance_score + payable_score;
        
        // 生成信号（总分14分）
        let strategy_signal = match total_score {
            11..=14 => StrategySignal::StrongBuy,  // 80%以上
            8..=10 => StrategySignal::Buy,         // 60-80%
            6..=7 => StrategySignal::Hold,         // 40-60%
            _ => StrategySignal::Sell,             // 40%以下
        };
        
        // 风险等级（基于财务健康度）
        let risk_level = self.calculate_risk_level(&quarters);
        
        // 生成分析说明
        let analysis_description = self.generate_description_new(
            total_score,
            revenue_score,
            profit_score,
            margin_score,
            expense_score,
            cashflow_score,
            inventory_score,
            receivable_score,
            advance_score,
            payable_score,
        );
        
        // 提取详细指标数据
        let revenue_growth = self.calculate_growth(
            quarters.get(quarters.len()-1).and_then(|(_, f)| f.revenue),
            quarters.get(quarters.len()-2).and_then(|(_, f)| f.revenue),
        );
        
        let profit_growth = self.calculate_growth(
            latest_fin.net_profit,
            quarters.get(quarters.len()-2).and_then(|(_, f)| f.net_profit),
        );
        
        let total_expense_ratio = self.calculate_total_expense_ratio(latest_fin);
        
        Ok(DistressedReversalResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            strategy_signal,
            signal_strength: total_score,
            analysis_description,
            risk_level,
            revenue_score,
            profit_score,
            margin_score,
            expense_score,
            cashflow_score,
            inventory_score,
            receivable_score,
            advance_score,
            payable_score,
            latest_period: latest_fin.report_period.clone(),
            revenue_growth,
            net_profit: latest_fin.net_profit,
            profit_growth,
            gross_margin: latest_fin.gross_profit_margin,
            total_expense_ratio,
            operating_cashflow: latest_fin.operating_cash_flow,
            inventory: latest_fin.inventory,
            accounts_receivable: latest_fin.accounts_receivable,
            advances_from_customers: latest_fin.advances_from_customers,
            accounts_payable: latest_fin.accounts_payable,
        })
    }
    
    // ========== 各项指标评分方法 ==========
    // 
    // 评分体系（14分制）：
    // 
    // 【盈利能力 4分】
    // - 营收增速：连续2季改善 → 2分
    // - 净利润：连续2季改善或转正 → 2分
    // 
    // 【盈利质量 4分】
    // - 毛利率：连续上升 → 1分
    // - 费用率：降低 → 1分
    // - 经营现金流：转正或大幅改善 → 2分
    // 
    // 【营运资本 6分】
    // - 存货：减少 → 2分
    // - 应收账款：减少 → 2分
    // - 预收账款：增加 → 1分
    // - 应付账款：增加 → 1分
    
    /// 营收增速评分：连续2季改善 -> 2分
    /// 
    /// 计算逻辑：
    /// 1. 计算最近3个季度的营收同比增速
    /// 2. 如果增速连续2季改善（Q2>Q1 且 Q3>Q2），得2分
    /// 3. 否则得0分
    fn score_revenue(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 3 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let q0_rev = quarters[len-3].1.revenue;
        let q1_rev = quarters[len-2].1.revenue;
        let q2_rev = quarters[len-1].1.revenue;
        
        if let (Some(r0), Some(r1), Some(r2)) = (q0_rev, q1_rev, q2_rev) {
            if r0 > 0.0 && r1 > 0.0 && r2 > 0.0 {
                let growth1 = (r1 - r0) / r0 * 100.0;
                let growth2 = (r2 - r1) / r1 * 100.0;
                
                // 连续2季改善
                if growth2 > growth1 && growth1 > 0.0 {
                    return Ok(2);
                }
            }
        }
        
        Ok(0)
    }
    
    /// 净利润评分：连续2季改善或转正 -> 2分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近3个季度的净利润
    /// 2. 如果连续2季改善（Q2>Q1 且 Q3>Q2），得2分
    /// 3. 或者从负转正（Q1<0 且 Q2>=0 或 Q2<0 且 Q3>=0），得2分
    /// 4. 否则得0分
    fn score_profit(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 3 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let q0_profit = quarters[len-3].1.net_profit;
        let q1_profit = quarters[len-2].1.net_profit;
        let q2_profit = quarters[len-1].1.net_profit;
        
        if let (Some(p0), Some(p1), Some(p2)) = (q0_profit, q1_profit, q2_profit) {
            // 转正：从亏损到盈利
            if p0 < 0.0 && p1 >= 0.0 && p2 >= 0.0 {
                return Ok(2);
            }
            
            // 连续改善
            if p2 > p1 && p1 > p0 {
                return Ok(2);
            }
        }
        
        Ok(0)
    }
    
    /// 毛利率评分：连续上升 -> 1分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的毛利率
    /// 2. 如果当前季度毛利率 > 上季度毛利率，得1分
    /// 3. 否则得0分
    fn score_margin(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 3 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let q0_margin = quarters[len-3].1.gross_profit_margin;
        let q1_margin = quarters[len-2].1.gross_profit_margin;
        let q2_margin = quarters[len-1].1.gross_profit_margin;
        
        if let (Some(m0), Some(m1), Some(m2)) = (q0_margin, q1_margin, q2_margin) {
            if m2 > m1 && m1 > m0 {
                return Ok(1);
            }
        }
        
        Ok(0)
    }
    
    /// 费用率评分：降低 -> 1分
    /// 
    /// 计算逻辑：
    /// 1. 计算最近2个季度的三费合计费用率（销售+管理+财务）
    /// 2. 如果当前季度费用率 < 上季度费用率，得1分
    /// 3. 否则得0分
    fn score_expense(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_expense = self.calculate_total_expense_ratio(quarters[len-2].1);
        let curr_expense = self.calculate_total_expense_ratio(quarters[len-1].1);
        
        if let (Some(prev), Some(curr)) = (prev_expense, curr_expense) {
            if curr < prev {
                return Ok(1);
            }
        }
        
        Ok(0)
    }
    
    /// 经营现金流评分：转正 or 大幅改善 -> 2分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的经营现金流
    /// 2. 如果从负转正（上期<0 且 当期>=0），得2分
    /// 3. 或者大幅改善（当期 > 上期 * 改善倍数阈值），得2分
    /// 4. 否则得0分
    fn score_cashflow(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_cf = quarters[len-2].1.operating_cash_flow;
        let curr_cf = quarters[len-1].1.operating_cash_flow;
        
        if let (Some(prev), Some(curr)) = (prev_cf, curr_cf) {
            // 转正
            if prev < 0.0 && curr > 0.0 {
                return Ok(2);
            }
            
            // 大幅改善
            if prev > 0.0 && curr > prev * self.config.cashflow_improvement_threshold {
                return Ok(2);
            }
        }
        
        Ok(0)
    }
    
    /// 存货评分：存货减少（说明去库存良好）-> 2分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的存货金额
    /// 2. 如果当前季度存货 < 上季度存货，得2分
    /// 3. 否则得0分
    /// 
    /// 财务意义：存货减少说明产品畅销，去库存良好
    fn score_inventory(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_inventory = quarters[len-2].1.inventory;
        let curr_inventory = quarters[len-1].1.inventory;
        
        if let (Some(prev), Some(curr)) = (prev_inventory, curr_inventory) {
            // 存货减少是好的
            if curr < prev {
                return Ok(2);
            }
        }
        
        Ok(0)
    }
    
    /// 应收账款评分：应收账款减少（说明回款良好）-> 2分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的应收账款金额
    /// 2. 如果当前季度应收账款 < 上季度应收账款，得2分
    /// 3. 否则得0分
    /// 
    /// 财务意义：应收账款减少说明回款加快，现金流改善
    fn score_receivable(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_receivable = quarters[len-2].1.accounts_receivable;
        let curr_receivable = quarters[len-1].1.accounts_receivable;
        
        if let (Some(prev), Some(curr)) = (prev_receivable, curr_receivable) {
            // 应收账款减少是好的
            if curr < prev {
                return Ok(2);
            }
        }
        
        Ok(0)
    }
    
    /// 预收账款评分：预收账款增加（说明产品受欢迎，客户愿意预付）-> 1分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的预收账款金额
    /// 2. 如果当前季度预收账款 > 上季度预收账款，得1分
    /// 3. 否则得0分
    /// 
    /// 财务意义：预收账款增加说明产品受欢迎，客户愿意预付款
    fn score_advance(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_advance = quarters[len-2].1.advances_from_customers;
        let curr_advance = quarters[len-1].1.advances_from_customers;
        
        if let (Some(prev), Some(curr)) = (prev_advance, curr_advance) {
            // 预收账款增加
            if curr > prev && prev >= 0.0 {
                return Ok(1);
            }
        }
        
        Ok(0)
    }
    
    /// 应付账款评分：应付账款增加（说明对上游有议价能力，占用上游资金）-> 1分
    /// 
    /// 计算逻辑：
    /// 1. 比较最近2个季度的应付账款金额
    /// 2. 如果当前季度应付账款 > 上季度应付账款，得1分
    /// 3. 否则得0分
    /// 
    /// 财务意义：应付账款增加说明对上游有议价能力，可以占用上游资金改善现金流
    fn score_payable(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> Result<u8> {
        if quarters.len() < 2 {
            return Ok(0);
        }
        
        let len = quarters.len();
        let prev_payable = quarters[len-2].1.accounts_payable;
        let curr_payable = quarters[len-1].1.accounts_payable;
        
        if let (Some(prev), Some(curr)) = (prev_payable, curr_payable) {
            // 应付账款增加是好的（占用上游资金）
            if curr > prev {
                return Ok(1);
            }
        }
        
        Ok(0)
    }
    
    // ========== 辅助方法 ==========
    
    /// 计算增长率
    fn calculate_growth(&self, current: Option<f64>, previous: Option<f64>) -> Option<f64> {
        match (current, previous) {
            (Some(curr), Some(prev)) if prev != 0.0 => {
                Some((curr - prev) / prev.abs() * 100.0)
            }
            _ => None,
        }
    }
    
    /// 计算总费用率（销售+管理+财务）
    fn calculate_total_expense_ratio(&self, fin: &super::traits::FinancialData) -> Option<f64> {
        match (fin.selling_expense_ratio, fin.admin_expense_ratio, fin.financial_expense_ratio) {
            (Some(s), Some(a), Some(f)) => Some(s + a + f),
            _ => None,
        }
    }
    
    /// 计算风险等级
    fn calculate_risk_level(&self, quarters: &[(&SecurityData, &super::traits::FinancialData)]) -> u8 {
        let latest = quarters.last().unwrap().1;
        let mut risk = 3;  // 基础风险
        
        // 现金流为负增加风险
        if let Some(cf) = latest.operating_cash_flow {
            if cf < 0.0 {
                risk += 1;
            }
        }
        
        risk.min(5)
    }
    
    /// 生成分析说明
    fn generate_description_new(
        &self,
        total_score: u8,
        revenue_score: u8,
        profit_score: u8,
        margin_score: u8,
        expense_score: u8,
        cashflow_score: u8,
        inventory_score: u8,
        receivable_score: u8,
        advance_score: u8,
        payable_score: u8,
    ) -> String {
        let mut parts = Vec::new();
        
        if revenue_score > 0 {
            parts.push("营收连续改善");
        }
        if profit_score > 0 {
            parts.push("净利润改善/转正");
        }
        if margin_score > 0 {
            parts.push("毛利率上升");
        }
        if expense_score > 0 {
            parts.push("费用率下降");
        }
        if cashflow_score > 0 {
            parts.push("现金流改善");
        }
        if inventory_score > 0 {
            parts.push("存货减少");
        }
        if receivable_score > 0 {
            parts.push("应收账款减少");
        }
        if advance_score > 0 {
            parts.push("预收账款增加");
        }
        if payable_score > 0 {
            parts.push("应付账款增加");
        }
        
        if parts.is_empty() {
            return "未发现明显的困境反转信号".to_string();
        }
        
        format!(
            "困境反转信号（总分{}/14）：{}。建议关注后续季度财报持续改善情况。",
            total_score,
            parts.join("、")
        )
    }
    
    /// 创建空结果
    fn create_empty_result(&self, symbol: &str, reason: &str) -> DistressedReversalResult {
        DistressedReversalResult {
            stock_code: symbol.to_string(),
            analysis_date: chrono::Local::now().date_naive(),
            current_price: 0.0,
            strategy_signal: StrategySignal::Hold,
            signal_strength: 0,
            analysis_description: reason.to_string(),
            risk_level: 3,
            revenue_score: 0,
            profit_score: 0,
            margin_score: 0,
            expense_score: 0,
            cashflow_score: 0,
            inventory_score: 0,
            receivable_score: 0,
            advance_score: 0,
            payable_score: 0,
            latest_period: String::new(),
            revenue_growth: None,
            net_profit: None,
            profit_growth: None,
            gross_margin: None,
            total_expense_ratio: None,
            operating_cashflow: None,
            inventory: None,
            accounts_receivable: None,
            advances_from_customers: None,
            accounts_payable: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = DistressedReversalConfig::default();
        assert_eq!(config.min_quarters, 3);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_strategy_creation() {
        let strategy = DistressedReversalStrategy::default_strategy();
        assert_eq!(strategy.name(), "困境反转策略");
        
        let aggressive = DistressedReversalStrategy::aggressive();
        assert_eq!(aggressive.config.min_quarters, 2);
        
        let conservative = DistressedReversalStrategy::conservative();
        assert_eq!(conservative.config.min_quarters, 4);
    }
}
