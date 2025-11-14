//! 选股服务
//! 
//! 利用交易策略筛选符合买入条件的股票

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, JsonValue, QueryFilter, QueryOrder};
use entity::{stock, stock_daily};
use tracing::{info, warn};
use crate::strategy::{
    PriceVolumeCandlestickStrategy, PriceVolumeStrategyConfig,
    BottomVolumeSurgeStrategy, BottomVolumeSurgeConfig,
    LongTermBottomReversalStrategy, LongTermBottomReversalConfig,
    YearlyHighStrategy, YearlyHighConfig,
    PriceStrengthStrategy, PriceStrengthConfig,
};

use crate::strategy::traits::{SecurityData, StrategyResult, StrategySignal, TradingStrategy};

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
    /// - `strategy_type`: 策略类型（"price_volume_candlestick", "bottom_volume_surge", "long_term_bottom_reversal", "yearly_high", "price_strength"）
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
                self.pick_stocks_internal(&mut strategy, start_date, end_date, None).await
            }};
        }

        match strategy_type {
            "price_volume_candlestick" => create_strategy!(PriceVolumeStrategyConfig, PriceVolumeCandlestickStrategy),
            "bottom_volume_surge" => create_strategy!(BottomVolumeSurgeConfig, BottomVolumeSurgeStrategy),
            "long_term_bottom_reversal" => create_strategy!(LongTermBottomReversalConfig, LongTermBottomReversalStrategy),
            "yearly_high" => create_strategy!(YearlyHighConfig, YearlyHighStrategy),
            "price_strength" => create_strategy!(PriceStrengthConfig, PriceStrengthStrategy),
            _ => bail!("不支持的策略类型: {}。支持的类型: price_volume_candlestick, bottom_volume_surge, long_term_bottom_reversal, yearly_high, price_strength", strategy_type)
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
            
            // 获取股票日线数据
            let daily_data = self
                .get_stock_daily_data(&stock_model.ts_code, start_date, end_date)
                .await?;

            // 检查数据是否足够
            if daily_data.len() < strategy.required_data_points() {
                warn!(
                    "股票 {} 数据不足: 需要 {} 个数据点，实际 {} 个",
                    stock_model.ts_code,
                    strategy.required_data_points(),
                    daily_data.len()
                );
                continue;
            }

            // 转换为 SecurityData
            let security_data: Vec<SecurityData> = daily_data
                .iter()
                .map(SecurityData::from_stock_daily)
                .collect();

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
