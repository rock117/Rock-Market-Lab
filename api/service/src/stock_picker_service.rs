//! 选股服务
//!
//! 利用交易策略筛选符合买入条件的股票

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use entity::sea_orm::{DatabaseConnection, EntityTrait, JsonValue, QueryFilter, QueryOrder};
use entity::{stock, stock_daily, stock_daily_basic, finance_indicator, income, cashflow, balancesheet, cn_security_info};
use entity::sea_orm::ColumnTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};
use common::task_runner::run_with_limit;
use crate::strategy::{
    PriceVolumeCandlestickStrategy, PriceVolumeStrategyConfig,
    BottomVolumeSurgeStrategy, BottomVolumeSurgeConfig,
    LongTermBottomReversalStrategy, LongTermBottomReversalConfig,
    YearlyHighStrategy, YearlyHighConfig,
    PriceStrengthStrategy, PriceStrengthConfig,
    DistressedReversalStrategy, DistressedReversalConfig,
    SingleLimitUpStrategy, SingleLimitUpConfig,
    FundamentalStrategy, FundamentalConfig,
    ConsecutiveStrongStrategy, ConsecutiveStrongConfig,
    TurtleStrategy, TurtleConfig,
    LimitUpPullbackStrategy, LimitUpPullbackConfig,
    StrongCloseStrategy, StrongCloseConfig,
    QualityValueStrategy, QualityValueConfig,
    TurnoverMaBullishStrategy, TurnoverMaBullishConfig,
    LowShadowStrategy, LowShadowConfig,
    SimilarityStrategy, SimilarityStrategyConfig,
};

use crate::strategy::traits::{SecurityData, StrategyResult, StrategySignal, TradingStrategy, FinancialData};

/// 选股结果
#[derive(Debug, Clone, Serialize)]
pub struct StockPickResult {
    /// 股票代码
    pub ts_code: String,
    /// 股票名称
    pub stock_name: Option<String>,
    pub concepts: Option<String>,
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
    /// - `strategy_type`: 策略类型（"price_volume_candlestick", "bottom_volume_surge", "long_term_bottom_reversal", "yearly_high", "price_strength", "distressed_reversal", "single_limit_up", "fundamental", "consecutive_strong", "turtle", "limit_up_pullback"）
    /// - `settings`: 策略配置的 JSON 对象
    ///   - 可以为 `None`，使用默认配置
    ///   - 可以包含 `"preset"` 字段来指定预设配置（如 `{"preset": "aggressive"}`）
    ///   - 可以直接提供完整的配置参数（如 `{"lookback_days": 10, "ma_type": "MA5"}`）
    ///
    /// # 预设配置支持
    /// - **turtle**: system1, system2, conservative, aggressive
    /// - **limit_up_pullback**: standard, aggressive, conservative, strong_stock
    ///
    /// # 返回
    /// 符合条件的股票列表
    ///
    /// # 示例
    /// ```rust
    /// // 使用默认配置
    /// service.pick_stocks(&start, &end, "turtle", None).await?;
    ///
    /// // 使用预设配置
    /// let preset = serde_json::json!({"preset": "aggressive"});
    /// service.pick_stocks(&start, &end, "turtle", Some(preset)).await?;
    ///
    /// // 使用自定义配置
    /// let custom = serde_json::json!({
    ///     "entry_breakout_period": 30,
    ///     "exit_breakout_period": 15
    /// });
    /// service.pick_stocks(&start, &end, "turtle", Some(custom)).await?;
    /// ```
    pub async fn pick_stocks(
        &self,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        strategy_type: &str,
        settings: Option<JsonValue>,
    ) -> Result<Vec<StockPickResult>> {
        // 宏：简化策略创建
        // 支持三种配置方式：
        // 1. settings 为 None：使用 default()
        // 2. settings 中有 "preset" 字段：调用对应的预设函数（如 standard(), aggressive() 等）
        // 3. settings 中无 "preset" 字段：使用 JSON 反序列化
        macro_rules! create_strategy {
            ($config:ty, $strategy:ty, $preset_handler:expr) => {{
                let config: $config = match settings {
                    Some(json) => {
                        // 检查是否指定了预设配置
                        if let Some(preset_name) = json.get("preset").and_then(|v| v.as_str()) {
                            // 使用预设配置函数
                            $preset_handler(preset_name)?
                        } else {
                            // 没有 preset 字段，使用 JSON 反序列化
                            serde_json::from_value(json)?
                        }
                    },
                    None => <$config>::default(),
                };
                <$strategy>::new(config)
            }};
        }

        // 使用宏创建策略并调用 pick_stocks_internal
        macro_rules! execute_strategy {
            ($config:ty, $strategy:ty, $preset_handler:expr) => {{
                let mut strategy = create_strategy!($config, $strategy, $preset_handler);
                self.pick_stocks_internal(&mut strategy, strategy_type, start_date, end_date, None).await
            }};
        }

        let mut results = match strategy_type {
            "price_volume_candlestick" => execute_strategy!(PriceVolumeStrategyConfig, PriceVolumeCandlestickStrategy, Self::handle_preset),
            "bottom_volume_surge" => execute_strategy!(BottomVolumeSurgeConfig, BottomVolumeSurgeStrategy, Self::handle_preset),
            "long_term_bottom_reversal" => execute_strategy!(LongTermBottomReversalConfig, LongTermBottomReversalStrategy, Self::handle_preset),
            "yearly_high" => execute_strategy!(YearlyHighConfig, YearlyHighStrategy, Self::handle_preset),
            "price_strength" => execute_strategy!(PriceStrengthConfig, PriceStrengthStrategy, Self::handle_preset),
            "distressed_reversal" => execute_strategy!(DistressedReversalConfig, DistressedReversalStrategy, Self::handle_preset),
            "single_limit_up" => execute_strategy!(SingleLimitUpConfig, SingleLimitUpStrategy, Self::handle_preset),
            "fundamental" => execute_strategy!(FundamentalConfig, FundamentalStrategy, Self::handle_preset),
            "consecutive_strong" => execute_strategy!(ConsecutiveStrongConfig, ConsecutiveStrongStrategy, Self::handle_preset),
            "turtle" => execute_strategy!(TurtleConfig, TurtleStrategy, |preset: &str| {
                Ok(match preset {
                    "system1" => TurtleStrategy::system1(),
                    "system2" => TurtleStrategy::system2(),
                    "conservative" => TurtleStrategy::conservative(),
                    "aggressive" => TurtleStrategy::aggressive(),
                    _ => bail!("海龟策略不支持预设 '{}', 可用预设: system1, system2, conservative, aggressive", preset),
                })
            }),
            "limit_up_pullback" => execute_strategy!(LimitUpPullbackConfig, LimitUpPullbackStrategy, |preset: &str| {
                Ok(match preset {
                    "standard" => LimitUpPullbackStrategy::standard(),
                    "aggressive" => LimitUpPullbackStrategy::aggressive(),
                    "conservative" => LimitUpPullbackStrategy::conservative(),
                    "strong_stock" => LimitUpPullbackStrategy::strong_stock(),
                    _ => bail!("涨停回调策略不支持预设 '{}', 可用预设: standard, aggressive, conservative, strong_stock", preset),
                })
            }),
            "strong_close" => execute_strategy!(StrongCloseConfig, StrongCloseStrategy, |preset: &str| {
                Ok(match preset {
                    "standard" => StrongCloseStrategy::standard(),
                    "aggressive" => StrongCloseStrategy::aggressive(),
                    "conservative" => StrongCloseStrategy::conservative(),
                    "super_strong" => StrongCloseStrategy::super_strong(),
                    _ => bail!("强势收盘策略不支持预设 '{}', 可用预设: standard, aggressive, conservative, super_strong", preset),
                })
            }),
            "quality_value" => execute_strategy!(QualityValueConfig, QualityValueStrategy, |preset: &str| {
                Ok(match preset {
                    "standard" => QualityValueStrategy::standard(),
                    "strict" => QualityValueStrategy::strict(),
                    "small_cap_growth" => QualityValueStrategy::small_cap_growth(),
                    "large_cap_blue_chip" => QualityValueStrategy::large_cap_blue_chip(),
                    _ => bail!("优质价值策略不支持预设 '{}', 可用预设: standard, strict, small_cap_growth, large_cap_blue_chip", preset),
                })
            }),
            "turnover_ma_bullish" => execute_strategy!(TurnoverMaBullishConfig, TurnoverMaBullishStrategy, |preset: &str| {
                Ok(match preset {
                    "standard" => TurnoverMaBullishStrategy::standard(),
                    "active" => TurnoverMaBullishStrategy::active(),
                    "conservative" => TurnoverMaBullishStrategy::conservative(),
                    "short_term" => TurnoverMaBullishStrategy::short_term(),
                    _ => bail!("换手率均线多头策略不支持预设 '{}', 可用预设: standard, active, conservative, short_term", preset),
                })
            }),
            "low_shadow" => execute_strategy!(LowShadowConfig, LowShadowStrategy, |preset: &str| {
                Ok(match preset {
                    "standard" => LowShadowConfig::default(),
                    "conservative" => LowShadowConfig {
                        min_lower_shadow_ratio: 0.5,   // 下影线至少50%
                        low_position_threshold: 0.25,  // 价格在下25%
                        require_bullish_close: true,   // 必须阳线
                        min_volume_ratio: 1.5,         // 成交量1.5倍
                        ..Default::default()
                    },
                    "aggressive" => LowShadowConfig {
                        min_lower_shadow_ratio: 0.3,   // 下影线30%即可
                        low_position_threshold: 0.4,   // 价格在下40%
                        require_bullish_close: false,  // 不要求阳线
                        min_volume_ratio: 1.0,         // 成交量正常即可
                        ..Default::default()
                    },
                    _ => bail!("低位下影线策略不支持预设 '{}', 可用预设: standard, conservative, aggressive", preset),
                })
            }),
            "similarity" => execute_strategy!(SimilarityStrategyConfig, SimilarityStrategy, Self::handle_preset),
            _ => bail!("不支持的策略类型: {}。支持的类型: price_volume_candlestick, bottom_volume_surge, long_term_bottom_reversal, yearly_high, price_strength, distressed_reversal, single_limit_up, fundamental, consecutive_strong, turtle, limit_up_pullback, strong_close, quality_value, turnover_ma_bullish, low_shadow, similarity", strategy_type)
        }?;
        for result in &mut results {
            let tscode = &result.ts_code;
            let concepts = cn_security_info::Entity::find_by_id(tscode).one(&self.db).await?.and_then(|entity| entity.concepts);
            result.concepts = concepts;
        }
        Ok(results)
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

        let total = stocks.len();
        let prepared_data = Arc::new(Mutex::new(Vec::new()));
        let processed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let db_conn = Arc::new(self.db.clone());

        // 预先获取策略所需的数据点数
        let required_data_points = strategy.required_data_points();

        // 第一阶段：使用 run_with_limit 并行准备数据
        info!("开始并行准备股票数据...");
        run_with_limit(
            10, // 并发数设为10
            stocks,
            {
                let strategy_type = strategy_type.to_string();
                let db_conn = db_conn.clone();
                let start_date = *start_date;
                let end_date = *end_date;
                move |stock_model| {
                    let strategy_type = strategy_type.clone();
                    let db_conn = db_conn.clone();
                    async move {
                        // 使用静态方法准备股票分析数据
                        match StockPickerService::prepare_stock_data(
                            &*db_conn,
                            &stock_model.ts_code,
                            &strategy_type,
                            &start_date,
                            &end_date,
                            required_data_points,
                        )
                            .await
                        {
                            Ok(Some(data)) => Some((stock_model, data)),
                            Ok(None) => None, // 数据不足，跳过
                            Err(e) => {
                                warn!("准备股票 {} 数据失败: {}", stock_model.ts_code, e);
                                None
                            }
                        }
                    }
                }
            },
            {
                let prepared_data = prepared_data.clone();
                let processed_count = processed_count.clone();
                move |_original_stock, data_result| {
                    let prepared_data = prepared_data.clone();
                    let processed_count = processed_count.clone();
                    async move {
                        let current = processed_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;

                        // 如果数据准备成功，添加到准备好的数据列表
                        if let Some(data) = data_result {
                            let mut prepared_guard = prepared_data.lock().unwrap();
                            prepared_guard.push(data);
                        }

                        // 每处理100只股票输出进度
                        if current % 100 == 0 {
                            info!("数据准备进度: {}/{} ({:.1}%)", current, total, (current as f64 / total as f64) * 100.0);
                        }
                    }
                }
            },
        ).await;

        // 第二阶段：串行进行策略分析
        let prepared_stocks = Arc::try_unwrap(prepared_data).unwrap().into_inner().unwrap();
        let prepared_count = prepared_stocks.len();
        info!("数据准备完成，开始策略分析，共 {} 只股票有效数据", prepared_count);

        let mut results = Vec::new();
        for (i, (stock_model, security_data)) in prepared_stocks.into_iter().enumerate() {
            match strategy.analyze(&stock_model.ts_code, &security_data) {
                Ok(result) => {
                    // 筛选符合信号条件的股票
                    if self.meets_signal_criteria(&result.strategy_signal(), &min_signal) {
                        let pick_result = StockPickResult {
                            ts_code: stock_model.ts_code.clone(),
                            stock_name: stock_model.name.clone(),
                            strategy_result: result,
                            concepts: None,
                        };

                        info!(
                            "找到符合条件的股票: {} ({}), 信号: {:?}, 强度: {}",
                            pick_result.ts_code,
                            pick_result.stock_name.as_deref().unwrap_or("未知"),
                            pick_result.strategy_result.strategy_signal(),
                            pick_result.strategy_result.signal_strength()
                        );
                        results.push(pick_result);
                    }
                }
                Err(e) => {
                    warn!("分析股票 {} 失败: {}", stock_model.ts_code, e);
                }
            }

            // 每分析100只股票输出进度
            if (i + 1) % 100 == 0 {
                info!("策略分析进度: {}/{}", i + 1, prepared_count);
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

    /// 准备股票分析数据（静态方法）
    ///
    /// 获取股票日线数据并转换为策略所需的 SecurityData 格式
    ///
    /// # 参数
    /// - `db`: 数据库连接
    /// - `ts_code`: 股票代码
    /// - `strategy_type`: 策略类型
    /// - `start_date`: 开始日期
    /// - `end_date`: 结束日期
    /// - `required_points`: 策略所需的最少数据点数
    ///
    /// # 返回
    /// - `Ok(Some(Vec<SecurityData>))`: 数据充足，返回转换后的数据
    /// - `Ok(None)`: 数据不足，无法进行分析
    /// - `Err`: 数据库查询错误
    async fn prepare_stock_data(
        db: &DatabaseConnection,
        ts_code: &str,
        strategy_type: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        required_points: usize,
    ) -> Result<Option<Vec<SecurityData>>> {
        // 获取股票日线数据
        if strategy_type == "" {
            Self::get_financial_data(db, ts_code).await
        } else {
            let daily_data = Self::get_stock_daily_data_static(db, ts_code, start_date, end_date).await?;
            // 检查数据是否足够
            if daily_data.len() < required_points {
                // warn!(
                // "股票 {} 数据不足: 需要 {} 个数据点，实际 {} 个",
                // ts_code,
                // required_points,
                // daily_data.len()
                // );
                return Ok(None);
            }

            // 转换为 SecurityData
            let security_data: Vec<SecurityData> = daily_data
                .iter()
                .map(|(daily, basic)| SecurityData::from_daily((daily, basic)))
                .collect();

            Ok(Some(security_data))
        }
    }

    /// 获取股票日线数据（实例方法）
    async fn get_stock_daily_data(
        &self,
        ts_code: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<(stock_daily::Model, stock_daily_basic::Model)>> {
        Self::get_stock_daily_data_static(&self.db, ts_code, start_date, end_date).await
    }

    /// 获取股票日线数据（静态方法，包含基本面数据）
    async fn get_stock_daily_data_static(
        db: &DatabaseConnection,
        ts_code: &str,
        start_date: &NaiveDate,
        end_date: &NaiveDate,
    ) -> Result<Vec<(stock_daily::Model, stock_daily_basic::Model)>> {
        let start = start_date.format("%Y%m%d").to_string();
        let end = end_date.format("%Y%m%d").to_string();

        // 获取日线数据
        let daily_data = stock_daily::Entity::find()
            .filter(ColumnTrait::eq(&stock_daily::Column::TsCode, ts_code))
            .filter(stock_daily::Column::TradeDate.gte(&start))
            .filter(stock_daily::Column::TradeDate.lte(&end))
            .order_by_asc(stock_daily::Column::TradeDate)
            .all(db)
            .await?;

        // 获取基本面数据
        let basic_data = stock_daily_basic::Entity::find()
            .filter(ColumnTrait::eq(&stock_daily_basic::Column::TsCode, ts_code))
            .filter(stock_daily_basic::Column::TradeDate.gte(&start))
            .filter(stock_daily_basic::Column::TradeDate.lte(&end))
            .order_by_asc(stock_daily_basic::Column::TradeDate)
            .all(db)
            .await?;

        // 将两个数据集按日期匹配
        let mut result = Vec::new();
        for daily in daily_data {
            // 查找对应日期的基本面数据
            if let Some(basic) = basic_data.iter().find(|b| b.trade_date == daily.trade_date) {
                result.push((daily, basic.clone()));
            }
        }

        Ok(result)
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
    /// 获取财务数据（静态方法）
    async fn get_financial_data(db: &DatabaseConnection, ts_code: &str) -> Result<Option<Vec<SecurityData>>> {
        let report_type = "1"; //合并报表
        // 1. 查询财务指标表（毛利率）
        let indicators = finance_indicator::Entity::find()
            .filter(finance_indicator::Column::TsCode.eq(ts_code))
            .order_by_asc(finance_indicator::Column::EndDate)
            .all(db)
            .await?;

        // 2. 查询利润表（营收、净利润、三费）
        let incomes = income::Entity::find()
            .filter(ColumnTrait::eq(&income::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&income::Column::ReportType, report_type))
            .filter(income::Column::EndDate.is_not_null())
            .order_by_asc(income::Column::EndDate)
            .all(db)
            .await?;

        // 3. 查询现金流量表（经营现金流）
        let cashflows = cashflow::Entity::find()
            .filter(ColumnTrait::eq(&cashflow::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&cashflow::Column::ReportType, ts_code))
            .order_by_asc(cashflow::Column::EndDate)
            .all(db)
            .await?;

        // 4. 查询资产负债表（存货、应收、预收、应付）
        let balancesheets = balancesheet::Entity::find()
            .filter(ColumnTrait::eq(&balancesheet::Column::TsCode, ts_code))
            .filter(ColumnTrait::eq(&balancesheet::Column::ReportType, ts_code))
            .order_by_asc(balancesheet::Column::EndDate)
            .all(db)
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
            let report_period = end_date; // Self::format_report_period(&end_date);

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
                market_cap: None,  // TODO: 从数据库获取市值数据
                roe: indicator
                    .and_then(|i| i.roe)
                    .and_then(|v| v.to_string().parse().ok()),
            };
            let mut sec_data = SecurityData::default();
            sec_data.financial_data = Some(financial_data);
            sec_data_list.push(sec_data);
        }
        Ok(Some(sec_data_list))
    }

    /// 通用预设配置处理器
    /// 尝试调用配置类型的预设方法，如果不存在则返回错误
    fn handle_preset<T>(preset_name: &str) -> Result<T>
    where
        T: Default,
    {
        bail!("策略配置不支持预设 '{}', 请使用 JSON 配置或不指定 preset 字段", preset_name)
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
