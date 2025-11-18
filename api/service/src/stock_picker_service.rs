//! 选股服务
//! 
//! 利用交易策略筛选符合买入条件的股票

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use entity::sea_orm::{DatabaseConnection, EntityTrait, JsonValue, QueryFilter, QueryOrder};
use entity::{stock, stock_daily, finance_indicator, income, cashflow, balancesheet};
use entity::sea_orm::ColumnTrait;
use std::collections::HashMap;
use tracing::{info, warn};
use crate::strategy::{
    PriceVolumeCandlestickStrategy, PriceVolumeStrategyConfig,
    BottomVolumeSurgeStrategy, BottomVolumeSurgeConfig,
    LongTermBottomReversalStrategy, LongTermBottomReversalConfig,
    YearlyHighStrategy, YearlyHighConfig,
    PriceStrengthStrategy, PriceStrengthConfig,
    DistressedReversalStrategy, DistressedReversalConfig,
    SingleLimitUpStrategy, SingleLimitUpConfig,
};

use crate::strategy::traits::{SecurityData, StrategyResult, StrategySignal, TradingStrategy, FinancialData};

/// 选股结果
#[derive(Debug, Clone, Serialize)]
pub struct StockPickResult {
    /// 股票代码
    pub ts_code: String,
    /// 股票名称
    pub stock_name: Option<String>,
    /// 策略分析结果
    pub strategy_result: StrategyResult,
}

/// 选股服务
pub struct StockPickerService {
    db: DatabaseConnection,
}

impl StockPickerService {
    /// 创建选股服务实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 使用动态策略筛选股票
    /// 
    /// # 参数
    /// - `start_date`: 开始日期
    /// - `end_date`: 结束日期
    /// - `strategy_type`: 策略类型（"price_volume_candlestick", "bottom_volume_surge", "long_term_bottom_reversal", "yearly_high", "price_strength", "distressed_reversal", "single_limit_up"）
    /// - `settings`: 策略配置的 JSON 对象
    /// 
    /// # 返回
    /// 符合条件的股票列表
    pub async fn pick_stocks(
        &self,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        strategy_type: &str,
        settings: Option<JsonValue>
    ) -> Result<Vec<StockPickResult>> {
        // 宏：简化策略创建和执行
        macro_rules! create_strategy {
            ($config:ty, $strategy:ty) => {{
                let config: $config = match settings {
                    Some(json) => serde_json::from_value(json)?,
                    None => <$config>::default(),
                };
                let mut strategy = <$strategy>::new(config);
                self.pick_stocks_internal(&mut strategy, strategy_type, start_date, end_date, None).await
            }};
        }

        match strategy_type {
            "price_volume_candlestick" => create_strategy!(PriceVolumeStrategyConfig, PriceVolumeCandlestickStrategy),
            "bottom_volume_surge" => create_strategy!(BottomVolumeSurgeConfig, BottomVolumeSurgeStrategy),
            "long_term_bottom_reversal" => create_strategy!(LongTermBottomReversalConfig, LongTermBottomReversalStrategy),
            "yearly_high" => create_strategy!(YearlyHighConfig, YearlyHighStrategy),
            "price_strength" => create_strategy!(PriceStrengthConfig, PriceStrengthStrategy),
            "distressed_reversal" => create_strategy!(DistressedReversalConfig, DistressedReversalStrategy),
            "single_limit_up" => create_strategy!(SingleLimitUpConfig, SingleLimitUpStrategy),
            _ => bail!("不支持的策略类型: {}。支持的类型: price_volume_candlestick, bottom_volume_surge, long_term_bottom_reversal, yearly_high, price_strength, distressed_reversal, single_limit_up", strategy_type)
        }
    }

    /// 使用策略筛选股票
    /// 
    /// # 参数
    /// - `strategy`: 交易策略实例
    /// - `start_date`: 数据开始日期
    /// - `end_date`: 数据结束日期
    /// - `min_signal`: 最小信号等级（默认为 Buy）
    /// 
    /// # 返回
    /// 返回符合条件的股票列表，按信号强度降序排列
    async fn pick_stocks_internal<S: TradingStrategy>(
        &self,
        strategy: &mut S,
        strategy_type: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        min_signal: Option<StrategySignal>,
    ) -> Result<Vec<StockPickResult>> {
        let min_signal = min_signal.unwrap_or(StrategySignal::Buy);
        
        info!(
            "开始选股，策略: {}, 日期范围: {} - {}, 最小信号: {:?}",
            strategy.name(),
            start_date,
            end_date,
            min_signal
        );

        // 获取所有股票列表
        let stocks = stock::Entity::find().all(&self.db).await?;
        info!("共获取 {} 只股票", stocks.len());

        let mut results = Vec::new();
        let mut processed = 0;
        let total = stocks.len();

        // 遍历所有股票进行分析
        for stock_model in stocks {
            processed += 1;
            
            // 准备股票分析数据
            let security_data = match self
                .prepare_stock_data(
                    &stock_model.ts_code,
                    strategy_type,
                    start_date,
                    end_date,
                    strategy.required_data_points(),
                )
                .await?
            {
                Some(data) => data,
                None => continue, // 数据不足，跳过
            };

            // 使用策略分析
            match strategy.analyze(&stock_model.ts_code, &security_data) {
                Ok(result) => {
                    // 筛选符合信号条件的股票
                    if self.meets_signal_criteria(&result.strategy_signal(), &min_signal) {
                        results.push(StockPickResult {
                            ts_code: stock_model.ts_code.clone(),
                            stock_name: stock_model.name.clone(),
                            strategy_result: result,
                        });
                        
                        info!(
                            "找到符合条件的股票: {} ({}), 信号: {:?}, 强度: {}",
                            stock_model.ts_code,
                            stock_model.name.as_deref().unwrap_or("未知"),
                            results.last().unwrap().strategy_result.strategy_signal(),
                            results.last().unwrap().strategy_result.signal_strength()
                        );
                    }
                }
                Err(e) => {
                    warn!("分析股票 {} 失败: {}", stock_model.ts_code, e);
                }
            }

            // 每处理100只股票输出进度
            if processed % 100 == 0 {
                info!("选股进度: {}/{} ({:.1}%)", processed, total, (processed as f64 / total as f64) * 100.0);
            }
        }

        // 按信号强度降序排序
        results.sort_by(|a, b| {
            b.strategy_result
                .signal_strength()
                .cmp(&a.strategy_result.signal_strength())
        });

        info!(
            "选股完成，共筛选出 {} 只符合条件的股票",
            results.len()
        );

        Ok(results)
    }



    /// 准备股票分析数据
    /// 
    /// 获取股票日线数据并转换为策略所需的 SecurityData 格式
    /// 
    /// # 参数
    /// - `ts_code`: 股票代码
    /// - `start_date`: 开始日期
    /// - `end_date`: 结束日期
    /// - `required_points`: 策略所需的最少数据点数
    /// 
    /// # 返回
    /// - `Ok(Some(Vec<SecurityData>))`: 数据充足，返回转换后的数据
    /// - `Ok(None)`: 数据不足，无法进行分析
    /// - `Err`: 数据库查询错误
    async fn prepare_stock_data(
        &self,
        ts_code: &str,
        strategy_type: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        required_points: usize,
    ) -> Result<Option<Vec<SecurityData>>> {
        // 获取股票日线数据
        if strategy_type == "" {
            self.get_financial_data(ts_code).await
        } else {
            let daily_data = self.get_stock_daily_data(ts_code, start_date, end_date).await?;
            // 检查数据是否足够
            if daily_data.len() < required_points {
                warn!(
                "股票 {} 数据不足: 需要 {} 个数据点，实际 {} 个",
                ts_code,
                required_points,
                daily_data.len()
            );
                return Ok(None);
            }

            // 转换为 SecurityData
            let security_data: Vec<SecurityData> = daily_data
                .iter()
                .map(SecurityData::from_stock_daily)
                .collect();

            Ok(Some(security_data))
        }
    }

    /// 获取股票日线数据
    async fn get_stock_daily_data(
        &self,
        ts_code: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<stock_daily::Model>> {
        let start = start_date.format("%Y%m%d").to_string();
        let end = end_date.format("%Y%m%d").to_string();

        let data = stock_daily::Entity::find()
            .filter(stock_daily::Column::TsCode.eq(ts_code))
            .filter(stock_daily::Column::TradeDate.gte(&start))
            .filter(stock_daily::Column::TradeDate.lte(&end))
            .order_by_asc(stock_daily::Column::TradeDate) // 按日期升序，策略需要从旧到新
            .all(&self.db)
            .await?;

        Ok(data)
    }

    /// 判断信号是否符合条件
    /// 
    /// 信号强度：StrongBuy > Buy > Hold > Sell > StrongSell
    /// 只要实际信号 >= 最小信号要求，就符合条件
    fn meets_signal_criteria(&self, signal: &StrategySignal, min_signal: &StrategySignal) -> bool {
        signal >= min_signal
    }

    /// 获取股票的财务数据
    /// 
    /// 从 fina_indicator, income, cashflow, balancesheet 四张表查询数据并组装成 FinancialData
    /// 
    /// # 参数
    /// - `ts_code`: 股票代码
    /// 
    /// # 返回
    /// 返回按报告期排序的财务数据数组（从旧到新）
    /// 
    /// # 字段映射
    /// - `gross_profit_margin` <- fina_indicator.grossprofit_margin
    /// - `revenue` <- income.revenue
    /// - `net_profit` <- income.n_income
    /// - `selling_expense_ratio` <- income.sell_exp (需要计算比率)
    /// - `admin_expense_ratio` <- income.admin_exp (需要计算比率)
    /// - `financial_expense_ratio` <- income.fin_exp (需要计算比率)
    /// - `operating_cash_flow` <- cashflow.n_cashflow_act
    /// - `inventory` <- balancesheet.inventories
    /// - `accounts_receivable` <- balancesheet.accounts_receiv
    /// - `advances_from_customers` <- balancesheet.adv_receipts
    /// - `accounts_payable` <- balancesheet.acct_payable
    pub async fn get_financial_data(&self, ts_code: &str) -> Result<Option<Vec<SecurityData>>> {
        let report_type = "1"; //合并报表
        // 1. 查询财务指标表（毛利率）
        let indicators = finance_indicator::Entity::find()
            .filter(finance_indicator::Column::TsCode.eq(ts_code))
            .order_by_asc(finance_indicator::Column::EndDate)
            .all(&self.db)
            .await?;
        
        // 2. 查询利润表（营收、净利润、三费）
        let incomes = income::Entity::find()
            .filter(ColumnTrait::eq(&income::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&income::Column::ReportType, report_type))
            .filter(income::Column::EndDate.is_not_null())
            .order_by_asc(income::Column::EndDate)
            .all(&self.db)
            .await?;
        
        // 3. 查询现金流量表（经营现金流）
        let cashflows = cashflow::Entity::find()
            .filter(ColumnTrait::eq(&cashflow::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&cashflow::Column::ReportType, ts_code))
            .order_by_asc(cashflow::Column::EndDate)
            .all(&self.db)
            .await?;
        
        // 4. 查询资产负债表（存货、应收、预收、应付）
        let balancesheets = balancesheet::Entity::find()
            .filter(ColumnTrait::eq(&balancesheet::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&balancesheet::Column::ReportType, ts_code))
            .order_by_asc(balancesheet::Column::EndDate)
            .all(&self.db)
            .await?;
        
        // 构建 end_date -> 各表数据的映射
        let mut indicator_map: HashMap<String, &finance_indicator::Model> = HashMap::new();
        for item in &indicators {
            indicator_map.insert(item.end_date.clone(), item);
        }
        
        let mut income_map: HashMap<String, &income::Model> = HashMap::new();
        for item in &incomes {
            if let Some(ref end_date) = item.end_date {
                income_map.insert(end_date.clone(), item);
            }
        }
        
        let mut cashflow_map: HashMap<String, &cashflow::Model> = HashMap::new();
        for item in &cashflows {
            cashflow_map.insert(item.end_date.clone(), item);
        }
        
        let mut balancesheet_map: HashMap<String, &balancesheet::Model> = HashMap::new();
        for item in &balancesheets {
            balancesheet_map.insert(item.end_date.clone(), item);
        }
        
        // 收集所有唯一的报告期
        let mut all_periods: Vec<String> = indicator_map.keys()
            .chain(income_map.keys())
            .chain(cashflow_map.keys())
            .chain(balancesheet_map.keys())
            .cloned()
            .collect();
        all_periods.sort();
        all_periods.dedup();
        
        // 组装 FinancialData
        let mut sec_data_list = Vec::new();
        
        for end_date in all_periods {
            let indicator = indicator_map.get(&end_date);
            let income_data = income_map.get(&end_date);
            let cashflow_data = cashflow_map.get(&end_date);
            let balance_data = balancesheet_map.get(&end_date);
            
            // 转换报告期格式：20240930 -> 2024Q3
            let report_period = end_date;// Self::format_report_period(&end_date);
            
            // 计算费用率（费用 / 营收 * 100）
            let revenue_decimal = income_data.and_then(|i| i.revenue);
            let revenue_f64 = revenue_decimal.and_then(|r| r.to_string().parse::<f64>().ok());
            
            let selling_expense_ratio = if let (Some(income), Some(rev)) = (income_data, revenue_f64) {
                income.sell_exp
                    .and_then(|exp| exp.to_string().parse::<f64>().ok())
                    .map(|exp| (exp / rev) * 100.0)
            } else {
                None
            };
            
            let admin_expense_ratio = if let (Some(income), Some(rev)) = (income_data, revenue_f64) {
                income.admin_exp
                    .and_then(|exp| exp.to_string().parse::<f64>().ok())
                    .map(|exp| (exp / rev) * 100.0)
            } else {
                None
            };
            
            let financial_expense_ratio = if let (Some(income), Some(rev)) = (income_data, revenue_f64) {
                income.fin_exp
                    .and_then(|exp| exp.to_string().parse::<f64>().ok())
                    .map(|exp| (exp / rev) * 100.0)
            } else {
                None
            };

            let financial_data = FinancialData {
                report_period,
                revenue: revenue_decimal.and_then(|v| v.to_string().parse().ok()),
                net_profit: income_data
                    .and_then(|i| i.n_income)
                    .and_then(|v| v.to_string().parse().ok()),
                gross_profit_margin: indicator
                    .and_then(|i| i.grossprofit_margin)
                    .and_then(|v| v.to_string().parse().ok()),
                selling_expense_ratio,
                admin_expense_ratio,
                financial_expense_ratio,
                operating_cash_flow: cashflow_data
                    .and_then(|c| c.n_cashflow_act)
                    .and_then(|v| v.to_string().parse().ok()),
                inventory: balance_data
                    .and_then(|b| b.inventories)
                    .and_then(|v| v.to_string().parse().ok()),
                accounts_receivable: balance_data
                    .and_then(|b| b.accounts_receiv)
                    .and_then(|v| v.to_string().parse().ok()),
                advances_from_customers: balance_data
                    .and_then(|b| b.adv_receipts)
                    .and_then(|v| v.to_string().parse().ok()),
                accounts_payable: balance_data
                    .and_then(|b| b.acct_payable)
                    .and_then(|v| v.to_string().parse().ok()),
            };
            let mut sec_data = SecurityData::default();
            sec_data.financial_data = Some(financial_data);
            sec_data_list.push(sec_data);
        }
        Ok(Some(sec_data_list))
    }
    
    /// 格式化报告期：20240930 -> 2024Q3
    fn format_report_period(end_date: &str) -> String {
        if end_date.len() != 8 {
            return end_date.to_string();
        }
        
        let year = &end_date[0..4];
        let month = &end_date[4..6];
        
        let quarter = match month {
            "03" => "Q1",
            "06" => "Q2",
            "09" => "Q3",
            "12" => "Q4",
            _ => return end_date.to_string(),
        };
        
        format!("{}{}", year, quarter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_criteria() {
        let service = StockPickerService {
            db: todo!(), // 测试时需要 mock
        };

        // 测试 StrongBuy 条件
        assert!(service.meets_signal_criteria(&StrategySignal::StrongBuy, &StrategySignal::StrongBuy));
        assert!(!service.meets_signal_criteria(&StrategySignal::Buy, &StrategySignal::StrongBuy));

        // 测试 Buy 条件
        assert!(service.meets_signal_criteria(&StrategySignal::StrongBuy, &StrategySignal::Buy));
        assert!(service.meets_signal_criteria(&StrategySignal::Buy, &StrategySignal::Buy));
        assert!(!service.meets_signal_criteria(&StrategySignal::Hold, &StrategySignal::Buy));

        // 测试 Hold 条件
        assert!(service.meets_signal_criteria(&StrategySignal::StrongBuy, &StrategySignal::Hold));
        assert!(service.meets_signal_criteria(&StrategySignal::Buy, &StrategySignal::Hold));
        assert!(service.meets_signal_criteria(&StrategySignal::Hold, &StrategySignal::Hold));
        assert!(!service.meets_signal_criteria(&StrategySignal::Sell, &StrategySignal::Hold));
    }
}
