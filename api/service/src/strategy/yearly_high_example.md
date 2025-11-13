# 年内新高策略使用示例

## 策略说明

年内新高策略用于识别创出年内新高的股票。**年内**指从当年1月1日到当天的时间段。

## 核心特点

- **股票代码** - 识别的股票代码
- **股票名称** - 股票名称（可选，需外部传入）
- **当前价格** - 最新收盘价
- **前高价格** - 年内前期最高价
- **前高日期** - 前期高点出现的日期
- **是否年内新高** - 当天是否创出年内新高
- **年初日期** - 当年1月1日
- **年内交易天数** - 从年初到当天的交易天数

## 使用示例

```rust
use service::strategy::{
    YearlyHighStrategy, YearlyHighConfig, 
    TradingStrategy, SecurityData
};

// 1. 使用默认配置（只检查当天是否新高）
let mut strategy = YearlyHighStrategy::default();

// 2. 自定义配置（检查最近N天内是否创新高）
let config = YearlyHighConfig {
    check_today_only: false,  // 检查最近N天
    recent_days: 3,           // 最近3天内创新高
};
let mut custom = YearlyHighStrategy::new(config);

// 3. 分析单只股票
let stock_data: Vec<SecurityData> = load_stock_data("000001.SZ");
match strategy.analyze("000001.SZ", &stock_data) {
    Ok(result) => {
        if let StrategyResult::YearlyHigh(r) = result {
            println!("股票代码: {}", r.stock_code);
            println!("当前价格: {:.2}", r.current_price);
            println!("前高价格: {:.2}", r.previous_high);
            println!("前高日期: {}", r.previous_high_date);
            println!("是否年内新高: {}", r.is_yearly_high);
            println!("年初日期: {}", r.year_start_date);
            println!("年内交易天数: {}", r.trading_days_in_year);
            println!("信号强度: {}/100", r.signal_strength);
            println!("策略信号: {:?}", r.strategy_signal);
            println!("分析说明: {}", r.analysis_description);
        }
    }
    Err(e) => eprintln!("分析失败: {}", e),
}

// 4. 批量分析多只股票
let stocks = vec![
    ("000001.SZ".to_string(), load_stock_data("000001.SZ")),
    ("000002.SZ".to_string(), load_stock_data("000002.SZ")),
    ("600000.SH".to_string(), load_stock_data("600000.SH")),
];

let results = strategy.batch_analyze(&stocks);
for result in results {
    if let StrategyResult::YearlyHigh(r) = result {
        if r.is_yearly_high {
            println!("{}: 创年内新高，当前价 {:.2}，前高 {:.2} ({})", 
                r.stock_code, r.current_price, r.previous_high, r.previous_high_date);
        }
    }
}
```

## 配置参数说明

### 默认配置
```rust
YearlyHighConfig {
    check_today_only: true,  // 只检查当天是否新高
    recent_days: 1,          // 检查天数（当 check_today_only=false 时使用）
}
```

### 检查最近N天配置
```rust
YearlyHighConfig {
    check_today_only: false, // 检查最近N天
    recent_days: 3,          // 最近3天内创新高
}
```

## 信号评分规则

非常简单的评分规则：

- **创年内新高**：买入信号，信号强度 100 分
- **未创新高**：卖出信号，信号强度 0 分

## 风险等级

- **3级**：中等风险（创新高存在追高风险）

## 结果字段说明

```rust
pub struct YearlyHighResult {
    pub stock_code: String,              // 股票代码
    pub stock_name: Option<String>,      // 股票名称（可选）
    pub analysis_date: NaiveDate,        // 分析日期
    pub current_price: f64,              // 当前价格
    pub previous_high: f64,              // 年内前期高点价格
    pub previous_high_date: NaiveDate,   // 前期高点日期
    pub days_since_previous_high: i64,   // 距离前高的天数
    pub is_yearly_high: bool,            // 是否为年内新高
    pub year_start_date: NaiveDate,      // 年初日期（1月1日）
    pub trading_days_in_year: usize,     // 年内交易天数
    pub strategy_signal: StrategySignal, // 策略信号
    pub signal_strength: u8,             // 信号强度(0-100)
    pub analysis_description: String,    // 分析说明
    pub risk_level: u8,                  // 风险等级(1-5)
}
```

## 使用场景

### 1. 年内新高股票筛选
```rust
let mut strategy = YearlyHighStrategy::default();
let results = strategy.batch_analyze(&all_stocks);

// 筛选年内新高股票
let yearly_high_stocks: Vec<_> = results.iter()
    .filter_map(|r| {
        if let StrategyResult::YearlyHigh(yh) = r {
            if yh.is_yearly_high {
                Some(yh)
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect();

println!("找到 {} 只年内新高股票", yearly_high_stocks.len());
```

### 2. 新高监控
```rust
// 监控是否创出新高
let mut strategy = YearlyHighStrategy::default();
for (symbol, data) in stocks {
    if let Ok(StrategyResult::YearlyHigh(r)) = strategy.analyze(&symbol, &data) {
        if r.is_yearly_high {
            println!("{} 创年内新高！当前价 {:.2}，前高 {:.2} ({})", 
                symbol, r.current_price, r.previous_high, r.previous_high_date);
        }
    }
}
```

### 3. 配合其他策略
```rust
// 结合底部反转策略，寻找底部反转后创新高的股票
let mut bottom_strategy = LongTermBottomReversalStrategy::default();
let mut high_strategy = YearlyHighStrategy::default();

// 先筛选底部反转
let bottom_results = bottom_strategy.batch_analyze(&stocks);
let bottom_stocks: Vec<_> = bottom_results.iter()
    .filter(|r| r.signal_strength() >= 70)
    .map(|r| r.stock_code())
    .collect();

// 再检查是否创新高
for code in bottom_stocks {
    if let Ok(StrategyResult::YearlyHigh(r)) = high_strategy.analyze(code, &data) {
        if r.is_yearly_high {
            println!("{}: 底部反转后创年内新高！", code);
        }
    }
}
```

## 注意事项

1. **年内定义**：年内指从当年1月1日到当天，不是过去365天
2. **追高风险**：创新高的股票可能已经涨幅较大，注意控制仓位
3. **趋势延续**：新高往往是趋势延续的信号，但也要警惕顶部风险
4. **止损设置**：建议在前高位置设置止损
5. **数据充足性**：需要从年初到当天的完整数据

## 最佳实践

1. **结合成交量**：虽然策略不判断成交量，但建议人工确认成交量是否放大
2. **结合基本面**：技术突破配合基本面改善效果更佳
3. **分批建仓**：突破初期可小仓位试探，确认后加仓
4. **设置止盈止损**：突破后设置合理的止盈止损位
5. **关注市场环境**：牛市中新高更有效，熊市中需谨慎
