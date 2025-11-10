# 技术分析选股模块

这个模块提供了基于技术指标的股票筛选和分析功能，实现了你提供的 TKR 基础技术分析逻辑。

## 模块结构

```
analysis/
├── technical_stock_picker.rs  # 技术分析核心逻辑
├── stock_data_provider.rs     # 数据获取层
├── stock_picking_service.rs   # 选股服务（主要接口）
├── examples.rs               # 使用示例
└── README.md                # 说明文档
```

## 主要功能

### 1. 技术指标计算

基于 `common::indicators` 模块计算以下技术指标：

- **移动平均线**: MA5, MA20, MA60
- **指数移动平均**: EMA12, EMA26  
- **相对强弱指数**: RSI14
- **MACD**: MACD线、信号线、柱状图
- **成交量分析**: 成交量5日均线

### 2. 趋势判断

- **短期趋势**: MA5 > MA20
- **长期趋势**: MA20 > MA60
- **整体看涨**: 短期趋势 && 长期趋势

### 3. RSI分析

- **超卖**: RSI < 30
- **超买**: RSI > 70  
- **中性**: 30 ≤ RSI ≤ 70

### 4. 成交量分析

- **成交量放大**: 当前成交量 > 5日均量 × 1.5

### 5. 综合分析结果

根据技术指标组合给出分析结论：

- **强烈看涨**: 趋势向上 + RSI正常 + 成交量放大 (评分90)
- **看涨**: 趋势向上 + RSI未超买 (评分75)
- **看跌**: 趋势向下 + RSI未超卖 (评分25)
- **超卖反弹机会**: RSI超卖 (评分65)
- **超买回调风险**: RSI超买 (评分35)
- **震荡整理**: 其他情况 (评分50)

## 快速开始

### 基础用法

```rust
use service::analysis::{StockPickingService, StockPickingStrategy, StockPickingRequest};

// 创建选股服务
let service = StockPickingService::new();

// 1. 获取强烈看涨的股票
let strong_bullish = service.get_strong_bullish_stocks(10).await?;

// 2. 获取超卖反弹机会
let oversold_opportunities = service.get_oversold_opportunities(5).await?;

// 3. 分析指定股票
let stock_codes = vec!["000001.SZ".to_string(), "600000.SH".to_string()];
let results = service.analyze_specific_stocks(stock_codes).await?;
```

### 自定义条件选股

```rust
use service::analysis::{StockPickingCriteria, StockPickingRequest, StockPickingStrategy};

let criteria = StockPickingCriteria {
    min_rsi: Some(30.0),           // RSI最小值
    max_rsi: Some(70.0),           // RSI最大值  
    require_short_trend: true,      // 要求短期趋势向上
    require_long_trend: false,      // 不要求长期趋势
    require_volume_surge: true,     // 要求成交量放大
    min_score: 70,                 // 最小评分70分
    max_results: 20,               // 最多返回20只股票
};

let request = StockPickingRequest {
    strategy: StockPickingStrategy::Custom,
    custom_criteria: Some(criteria),
    max_results: Some(20),
    stock_codes: None,
    market: None,
};

let response = service.pick_stocks(request).await?;
```

### 按市场选股

```rust
// 分析深圳市场股票
let sz_stocks = service.analyze_market_stocks("SZ".to_string(), 50).await?;

// 分析上海市场股票  
let sh_stocks = service.analyze_market_stocks("SH".to_string(), 50).await?;
```

## 选股策略

### 1. 强烈看涨策略 (StrongBullish)

筛选条件：
- RSI在30-70之间
- 短期和长期趋势都向上
- 成交量放大
- 评分≥80分

适用场景：寻找技术面强劲的上涨股票

### 2. 超卖反弹机会 (OversoldOpportunity)

筛选条件：
- RSI ≤ 30（超卖）
- 评分≥60分

适用场景：寻找超卖后的反弹机会

### 3. 趋势向上且RSI正常 (UptrendNormalRsi)

筛选条件：
- RSI在30-70之间
- 短期趋势向上
- 评分≥70分

适用场景：寻找趋势良好且未超买的股票

### 4. 自定义策略 (Custom)

使用自定义的 `StockPickingCriteria` 进行筛选

### 5. 全面分析 (Comprehensive)

返回所有股票的分析结果，按评分排序

## 配置选项

### DataProviderConfig

```rust
use service::analysis::DataProviderConfig;

let config = DataProviderConfig {
    default_days: 120,        // 默认获取120天数据
    min_volume: 1000000,      // 最小成交量100万
    max_stocks: 1000,         // 最大分析1000只股票
};

let service = StockPickingService::with_config(config);
```

## 数据要求

选股模块需要以下数据：

1. **股票日线数据** (`stock_daily` 表)
   - 交易日期、开盘价、最高价、最低价、收盘价
   - 成交量、成交额等

2. **股票基本信息** (可选，用于获取股票名称)

3. **足够的历史数据**
   - 建议至少120天的数据以计算60日均线
   - 数据越多，技术指标越准确

## 实现数据库查询

当前 `StockDataProvider` 中的数据库查询逻辑需要你根据实际情况实现：

```rust
// 在 stock_data_provider.rs 中实现
pub async fn get_stock_daily_data(
    &self,
    stock_code: &str,
    days: u32,
    end_date: Option<NaiveDate>,
) -> Result<Vec<stock_daily::Model>> {
    // TODO: 使用 sea-orm 或其他 ORM 查询数据库
    // 示例查询逻辑：
    // 
    // use entity::stock_daily;
    // use sea_orm::*;
    // 
    // let db = &self.db_connection;
    // let end_date = end_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    // let start_date = end_date - chrono::Duration::days(days as i64);
    // 
    // let results = stock_daily::Entity::find()
    //     .filter(stock_daily::Column::TsCode.eq(stock_code))
    //     .filter(stock_daily::Column::TradeDate.between(start_date, end_date))
    //     .order_by_asc(stock_daily::Column::TradeDate)
    //     .all(db)
    //     .await?;
    // 
    // Ok(results)
}
```

## 性能优化建议

1. **数据缓存**: 对常用股票的历史数据进行缓存
2. **并行计算**: 使用 `tokio::spawn` 并行分析多只股票
3. **数据库索引**: 在 `ts_code` 和 `trade_date` 字段上建立索引
4. **分页查询**: 对大量股票进行分页处理
5. **增量更新**: 只获取新增的交易日数据

## 扩展功能

可以基于现有框架扩展更多功能：

1. **更多技术指标**: KDJ, BOLL, ATR 等
2. **形态识别**: 头肩顶、双底等经典形态
3. **资金流向分析**: 大单净流入等
4. **行业轮动分析**: 按行业进行技术分析
5. **回测功能**: 验证选股策略的历史表现

## 使用示例

完整的使用示例请参考 `examples.rs` 文件，包含：

- 基础选股示例
- 自定义条件选股
- 指定股票分析
- 按市场选股
- 高级配置使用

运行示例：

```rust
use service::analysis::examples;

// 运行所有示例
examples::run_all_examples().await?;
```
