# 基本面选股策略使用指南

## 策略概述

基本面选股策略（Fundamental Strategy）通过分析公司的财务指标来筛选优质股票，关注盈利能力、成长性、现金流等核心指标。

### 核心逻辑

1. **成长性分析**
   - 营收增长率：评估公司业务扩张能力
   - 净利润增长率：评估盈利增长能力

2. **盈利能力分析**
   - 毛利率：反映产品竞争力和定价能力
   - 净利率：反映成本控制和整体盈利能力
   - ROE（净资产收益率）：评估股东回报

3. **财务健康度分析**
   - 负债率：评估财务风险
   - 经营现金流：评估现金创造能力

4. **估值分析**（可选）
   - PE（市盈率）：评估估值水平
   - PB（市净率）：评估资产价值

### 策略优势

- ✅ **价值投资导向** - 关注公司内在价值
- ✅ **多维度评估** - 综合考虑成长性、盈利能力、财务健康度
- ✅ **风险控制** - 通过负债率等指标控制风险
- ✅ **灵活配置** - 可自定义各项指标阈值

## 配置参数

```rust
use service::strategy::FundamentalConfig;

let config = FundamentalConfig {
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
    min_market_cap: Some(50.0),    // 最小市值50亿元
    max_market_cap: Some(1000.0),  // 最大市值1000亿元
};

let mut strategy = FundamentalStrategy::new(config);
```

### 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `min_revenue_growth` | f64 | 10.0 | 最小营收增长率（%） |
| `min_profit_growth` | f64 | 15.0 | 最小净利润增长率（%） |
| `min_gross_margin` | f64 | 20.0 | 最小毛利率（%） |
| `min_net_margin` | f64 | 5.0 | 最小净利率（%） |
| `min_roe` | f64 | 10.0 | 最小ROE（%） |
| `max_debt_ratio` | f64 | 70.0 | 最大负债率（%） |
| `require_positive_cash_flow` | bool | true | 是否要求经营现金流为正 |
| `min_pe` | Option<f64> | Some(5.0) | 最小市盈率 |
| `max_pe` | Option<f64> | Some(50.0) | 最大市盈率 |
| `min_pb` | Option<f64> | Some(1.0) | 最小市净率 |
| `max_pb` | Option<f64> | Some(10.0) | 最大市净率 |
| `min_market_cap` | Option<f64> | None | 最小市值（亿元） |
| `max_market_cap` | Option<f64> | None | 最大市值（亿元） |

## 使用示例

### 基础用法

```rust
use service::strategy::{
    FundamentalStrategy, 
    FundamentalConfig,
    TradingStrategy,
    SecurityData,
    FinancialData,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建策略
    let mut strategy = FundamentalStrategy::new(FundamentalConfig::default());
    
    // 准备数据（包含财务数据）
    let data = fetch_stock_data_with_financials("000001.SZ").await?;
    
    // 分析
    let result = strategy.analyze("000001.SZ", &data)?;
    
    // 查看结果
    println!("股票代码: {}", result.stock_code());
    println!("策略信号: {:?}", result.strategy_signal());
    println!("信号强度: {}", result.signal_strength());
    println!("分析说明: {}", result.analysis_description());
    
    Ok(())
}
```

### 批量分析

```rust
use service::strategy::{FundamentalStrategy, TradingStrategy};

async fn batch_analyze_stocks(stock_codes: Vec<String>) -> anyhow::Result<()> {
    let mut strategy = FundamentalStrategy::new(FundamentalConfig::default());
    
    // 准备数据
    let mut securities_data = Vec::new();
    for code in stock_codes {
        let data = fetch_stock_data_with_financials(&code).await?;
        securities_data.push((code, data));
    }
    
    // 批量分析
    let results = strategy.batch_analyze(&securities_data);
    
    // 筛选强买入信号
    let strong_buy: Vec<_> = results.iter()
        .filter(|r| r.signal_strength() >= 80)
        .collect();
    
    println!("找到 {} 只强买入信号的股票", strong_buy.len());
    for result in strong_buy {
        println!("{}: {}", result.stock_code(), result.analysis_description());
    }
    
    Ok(())
}
```

### 自定义配置 - 高成长股筛选

```rust
// 高成长股配置
let growth_config = FundamentalConfig {
    min_revenue_growth: 30.0,      // 营收增长至少30%
    min_profit_growth: 40.0,       // 净利润增长至少40%
    min_gross_margin: 30.0,        // 毛利率至少30%
    min_net_margin: 10.0,          // 净利率至少10%
    min_roe: 15.0,                 // ROE至少15%
    max_debt_ratio: 50.0,          // 负债率不超过50%
    require_positive_cash_flow: true,
    min_pe: Some(10.0),
    max_pe: Some(100.0),           // 允许较高估值
    min_pb: Some(2.0),
    max_pb: Some(20.0),
    min_market_cap: Some(100.0),   // 最小市值100亿元（中大盘）
    max_market_cap: None,          // 不限制最大市值
};

let mut strategy = FundamentalStrategy::new(growth_config);
```

### 自定义配置 - 价值股筛选

```rust
// 价值股配置
let value_config = FundamentalConfig {
    min_revenue_growth: 5.0,       // 营收增长要求较低
    min_profit_growth: 8.0,        // 净利润增长要求较低
    min_gross_margin: 15.0,        // 毛利率要求较低
    min_net_margin: 3.0,           // 净利率要求较低
    min_roe: 8.0,                  // ROE要求较低
    max_debt_ratio: 60.0,          // 负债率适中
    require_positive_cash_flow: true,
    min_pe: Some(3.0),             // 低估值
    max_pe: Some(15.0),            // 低估值
    min_pb: Some(0.5),             // 破净也可以
    max_pb: Some(3.0),             // 低市净率
    min_market_cap: Some(50.0),    // 最小市值50亿元
    max_market_cap: Some(500.0),   // 最大市值500亿元（中小盘）
};

let mut strategy = FundamentalStrategy::new(value_config);
```

## 结果解读

### FundamentalResult 字段

```rust
pub struct FundamentalResult {
    pub stock_code: String,           // 股票代码
    pub analysis_date: NaiveDate,     // 分析日期
    pub current_price: f64,           // 当前价格
    pub strategy_signal: StrategySignal,  // 策略信号
    pub signal_strength: u8,          // 信号强度 (0-100)
    pub analysis_description: String, // 分析说明
    pub risk_level: u8,               // 风险等级 (1-5)
    
    // 策略特有字段
    pub report_period: String,        // 报告期（如 "2024Q3"）
    pub revenue: f64,                 // 营业收入（元）
    pub revenue_growth: f64,          // 营收增长率（%）
    pub net_profit: f64,              // 净利润（元）
    pub profit_growth: f64,           // 净利润增长率（%）
    pub gross_margin: f64,            // 毛利率（%）
    pub net_margin: f64,              // 净利率（%）
    pub roe: f64,                     // ROE（%）
    pub debt_ratio: f64,              // 负债率（%）
    pub operating_cash_flow: f64,     // 经营现金流（元）
    pub pe_ratio: Option<f64>,        // 市盈率（PE）
    pub pb_ratio: Option<f64>,        // 市净率（PB）
    pub market_cap: Option<f64>,      // 市值（亿元）
}
```

### 信号强度评分规则

总分100分，由以下部分组成：

1. **营收增长评分（20分）**
   - 增长率 ≥ 2倍最低要求：20分
   - 增长率 ≥ 1.5倍最低要求：15分
   - 增长率 ≥ 最低要求：10分

2. **利润增长评分（25分）**
   - 增长率 ≥ 2倍最低要求：25分
   - 增长率 ≥ 1.5倍最低要求：18分
   - 增长率 ≥ 最低要求：12分

3. **毛利率评分（15分）**
   - 毛利率 ≥ 1.5倍最低要求：15分
   - 毛利率 ≥ 最低要求：10分

4. **净利率评分（15分）**
   - 净利率 ≥ 2倍最低要求：15分
   - 净利率 ≥ 最低要求：10分

5. **ROE评分（10分）**
   - ROE ≥ 1.5倍最低要求：10分
   - ROE ≥ 最低要求：6分

6. **现金流评分（10分）**
   - 经营现金流为正：10分
   - 不要求现金流为正：5分

7. **PE估值调整（5分）**
   - PE在合理区间：+5分
   - PE过低或过高：-10分

8. **负债率风险调整**
   - 负债率 > 最大要求：-20分，风险等级4
   - 负债率 > 80%最大要求：风险等级3
   - 负债率 ≤ 80%最大要求：风险等级2

### 评分示例

#### 示例1：优质成长股

```
营收增长: 35% (≥ 2倍要求) → 20分
利润增长: 45% (≥ 2倍要求) → 25分
毛利率: 35% (≥ 1.5倍要求) → 15分
净利率: 12% (≥ 2倍要求) → 15分
ROE: 18% (≥ 1.5倍要求) → 10分
现金流: 正 → 10分
PE: 25 (合理区间) → 5分
负债率: 45% (良好) → 风险等级2

总分: 100分 → 强烈买入
```

#### 示例2：稳健价值股

```
营收增长: 12% (≥ 要求) → 10分
利润增长: 18% (≥ 要求) → 12分
毛利率: 22% (≥ 要求) → 10分
净利率: 6% (≥ 要求) → 10分
ROE: 11% (≥ 要求) → 6分
现金流: 正 → 10分
PE: 10 (合理区间) → 5分
负债率: 55% (良好) → 风险等级2

总分: 63分 → 买入
```

## 使用场景

### 场景1：筛选白马股

```rust
let config = FundamentalConfig {
    min_revenue_growth: 15.0,
    min_profit_growth: 20.0,
    min_gross_margin: 30.0,
    min_net_margin: 10.0,
    min_roe: 15.0,
    max_debt_ratio: 50.0,
    require_positive_cash_flow: true,
    min_pe: Some(10.0),
    max_pe: Some(40.0),
    min_pb: Some(2.0),
    max_pb: Some(8.0),
};
```

### 场景2：筛选困境反转股

```rust
let config = FundamentalConfig {
    min_revenue_growth: -10.0,     // 允许营收下降
    min_profit_growth: 50.0,       // 但利润要大幅增长
    min_gross_margin: 15.0,
    min_net_margin: 3.0,
    min_roe: 5.0,
    max_debt_ratio: 80.0,          // 允许较高负债
    require_positive_cash_flow: false, // 不强制要求正现金流
    min_pe: Some(5.0),
    max_pe: Some(30.0),
    min_pb: Some(0.5),
    max_pb: Some(5.0),
};
```

### 场景3：筛选科技成长股

```rust
let config = FundamentalConfig {
    min_revenue_growth: 40.0,      // 高增长
    min_profit_growth: 50.0,       // 高增长
    min_gross_margin: 40.0,        // 高毛利
    min_net_margin: 15.0,          // 高净利
    min_roe: 20.0,                 // 高ROE
    max_debt_ratio: 40.0,          // 低负债
    require_positive_cash_flow: true,
    min_pe: Some(20.0),            // 允许高估值
    max_pe: Some(150.0),
    min_pb: Some(3.0),
    max_pb: Some(30.0),
    min_market_cap: Some(100.0),   // 最小市值100亿元
    max_market_cap: None,          // 不限制最大市值
};
```

### 场景4：按市值筛选

#### 小盘股策略（市值 < 100亿）
```rust
let small_cap_config = FundamentalConfig {
    min_revenue_growth: 20.0,
    min_profit_growth: 25.0,
    min_gross_margin: 25.0,
    min_net_margin: 8.0,
    min_roe: 12.0,
    max_debt_ratio: 60.0,
    require_positive_cash_flow: true,
    min_pe: Some(10.0),
    max_pe: Some(50.0),
    min_pb: Some(1.5),
    max_pb: Some(10.0),
    min_market_cap: Some(20.0),    // 最小市值20亿元
    max_market_cap: Some(100.0),   // 最大市值100亿元
};
```

#### 中盘股策略（100亿 ≤ 市值 < 500亿）
```rust
let mid_cap_config = FundamentalConfig {
    min_revenue_growth: 15.0,
    min_profit_growth: 20.0,
    min_gross_margin: 20.0,
    min_net_margin: 6.0,
    min_roe: 10.0,
    max_debt_ratio: 65.0,
    require_positive_cash_flow: true,
    min_pe: Some(8.0),
    max_pe: Some(40.0),
    min_pb: Some(1.2),
    max_pb: Some(8.0),
    min_market_cap: Some(100.0),   // 最小市值100亿元
    max_market_cap: Some(500.0),   // 最大市值500亿元
};
```

#### 大盘股策略（市值 ≥ 500亿）
```rust
let large_cap_config = FundamentalConfig {
    min_revenue_growth: 10.0,
    min_profit_growth: 15.0,
    min_gross_margin: 18.0,
    min_net_margin: 5.0,
    min_roe: 8.0,
    max_debt_ratio: 70.0,
    require_positive_cash_flow: true,
    min_pe: Some(5.0),
    max_pe: Some(30.0),
    min_pb: Some(1.0),
    max_pb: Some(5.0),
    min_market_cap: Some(500.0),   // 最小市值500亿元
    max_market_cap: None,          // 不限制最大市值
};
```

## 注意事项

1. **财务数据要求**
   - 策略需要 `SecurityData` 中包含 `financial_data` 字段
   - 确保财务数据是最新的季度或年度数据

2. **增长率计算**
   - 当前版本的增长率计算需要对比历史数据
   - 实际使用时需要提供至少两期财务数据

3. **估值指标**
   - PE和PB需要额外的市值和每股数据
   - 如果没有这些数据，可以将 `min_pe`、`max_pe`、`min_pb`、`max_pb` 设为 `None`

4. **市值筛选**
   - 市值需要从股票基本信息中获取总股本数据
   - 市值计算公式：市值（亿元） = 总股本 × 当前价格 / 100000000
   - 如果不需要市值筛选，可以将 `min_market_cap`、`max_market_cap` 设为 `None`
   - 不同市值区间的股票特点：
     - 小盘股（< 100亿）：成长性强，波动大，流动性相对较弱
     - 中盘股（100-500亿）：成长与稳定兼顾，流动性适中
     - 大盘股（≥ 500亿）：稳定性强，波动小，流动性好

5. **行业差异**
   - 不同行业的财务指标差异很大
   - 建议根据行业特点调整配置参数

6. **数据质量**
   - 财务数据的准确性直接影响策略效果
   - 建议使用经过审计的正式财报数据

## 与StockPickerService集成

```rust
use service::stock_picker_service::StockPickerService;
use chrono::NaiveDate;
use serde_json::json;

async fn pick_fundamental_stocks(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 18).unwrap();
    
    // 使用默认配置
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "fundamental",
        None
    ).await?;
    
    // 或使用自定义配置
    let custom_config = json!({
        "min_revenue_growth": 20.0,
        "min_profit_growth": 25.0,
        "min_gross_margin": 25.0,
        "min_net_margin": 8.0,
        "min_roe": 12.0,
        "max_debt_ratio": 60.0,
        "require_positive_cash_flow": true,
        "min_pe": 5.0,
        "max_pe": 50.0,
        "min_pb": 1.0,
        "max_pb": 10.0
    });
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "fundamental",
        Some(custom_config)
    ).await?;
    
    println!("找到 {} 只符合基本面条件的股票", results.len());
    
    Ok(())
}
```

## 完整示例

```rust
use service::strategy::{
    FundamentalStrategy,
    FundamentalConfig,
    TradingStrategy,
    StrategyResult,
};

async fn comprehensive_fundamental_analysis() -> anyhow::Result<()> {
    // 创建策略
    let config = FundamentalConfig {
        min_revenue_growth: 15.0,
        min_profit_growth: 20.0,
        min_gross_margin: 25.0,
        min_net_margin: 8.0,
        min_roe: 12.0,
        max_debt_ratio: 60.0,
        require_positive_cash_flow: true,
        min_pe: Some(10.0),
        max_pe: Some(50.0),
        min_pb: Some(1.5),
        max_pb: Some(8.0),
    };
    
    let mut strategy = FundamentalStrategy::new(config);
    
    // 分析股票
    let data = fetch_stock_data_with_financials("000001.SZ").await?;
    let result = strategy.analyze("000001.SZ", &data)?;
    
    // 提取详细结果
    if let StrategyResult::Fundamental(fund_result) = result {
        println!("=== 基本面分析报告 ===");
        println!("股票代码: {}", fund_result.stock_code);
        println!("报告期: {}", fund_result.report_period);
        println!("当前价格: {:.2}", fund_result.current_price);
        println!("\n=== 成长性指标 ===");
        println!("营业收入: {:.2}亿元", fund_result.revenue / 100_000_000.0);
        println!("营收增长率: {:.2}%", fund_result.revenue_growth);
        println!("净利润: {:.2}亿元", fund_result.net_profit / 100_000_000.0);
        println!("利润增长率: {:.2}%", fund_result.profit_growth);
        println!("\n=== 盈利能力指标 ===");
        println!("毛利率: {:.2}%", fund_result.gross_margin);
        println!("净利率: {:.2}%", fund_result.net_margin);
        println!("ROE: {:.2}%", fund_result.roe);
        println!("\n=== 财务健康度 ===");
        println!("负债率: {:.2}%", fund_result.debt_ratio);
        println!("经营现金流: {:.2}亿元", fund_result.operating_cash_flow / 100_000_000.0);
        println!("\n=== 估值指标 ===");
        if let Some(pe) = fund_result.pe_ratio {
            println!("PE: {:.2}", pe);
        }
        if let Some(pb) = fund_result.pb_ratio {
            println!("PB: {:.2}", pb);
        }
        println!("\n=== 策略评估 ===");
        println!("策略信号: {:?}", fund_result.strategy_signal);
        println!("信号强度: {}/100", fund_result.signal_strength);
        println!("风险等级: {}/5", fund_result.risk_level);
        println!("分析说明: {}", fund_result.analysis_description);
    }
    
    Ok(())
}
```
