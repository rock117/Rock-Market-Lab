# StockPickerService 使用单次涨停策略示例

## 概述

`StockPickerService` 现已支持 `single_limit_up`（单次涨停）策略类型，可以批量筛选符合单次涨停条件的股票。

## 使用方法

### 1. 基础用法 - 使用默认配置

```rust
use service::stock_picker_service::StockPickerService;
use chrono::NaiveDate;
use entity::sea_orm::DatabaseConnection;

async fn pick_single_limit_up_stocks(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    // 设置日期范围
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    // 使用默认配置筛选股票
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",  // 策略类型
        None                // 使用默认配置
    ).await?;
    
    // 输出结果
    println!("找到 {} 只符合单次涨停条件的股票", results.len());
    for result in results.iter().take(10) {
        println!("股票: {} - {}", 
            result.ts_code, 
            result.stock_name.as_deref().unwrap_or("未知")
        );
        println!("  信号: {:?}", result.strategy_result.strategy_signal());
        println!("  信号强度: {}", result.strategy_result.signal_strength());
        println!("  描述: {}", result.strategy_result.analysis_description());
        println!();
    }
    
    Ok(())
}
```

### 2. 自定义配置

```rust
use serde_json::json;

async fn pick_with_custom_config(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    // 自定义配置 - 更严格的筛选条件
    let custom_config = json!({
        "analysis_period": 30,       // 分析过去30天
        "limit_up_tolerance": 0.3,   // 更小的容差
        "min_up_days": 15,           // 至少15天上涨
        "min_total_gain": 30.0       // 累计涨幅至少30%
    });
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",
        Some(custom_config)
    ).await?;
    
    println!("严格筛选后找到 {} 只股票", results.len());
    
    Ok(())
}
```

### 3. 不同板块的筛选

```rust
async fn pick_by_board(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    // 筛选所有股票（策略会自动识别不同板块的涨停阈值）
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",
        None
    ).await?;
    
    // 按板块分类
    let mut main_board = Vec::new();
    let mut chinext = Vec::new();
    let mut star_market = Vec::new();
    let mut bse = Vec::new();
    
    for result in results {
        let code = &result.ts_code;
        if code.starts_with("688") {
            star_market.push(result);  // 科创板 - 20%涨停
        } else if code.starts_with("300") {
            chinext.push(result);      // 创业板 - 20%涨停
        } else if code.starts_with("920") {
            bse.push(result);          // 北交所 - 30%涨停
        } else {
            main_board.push(result);   // 主板 - 10%涨停
        }
    }
    
    println!("主板: {} 只", main_board.len());
    println!("创业板: {} 只", chinext.len());
    println!("科创板: {} 只", star_market.len());
    println!("北交所: {} 只", bse.len());
    
    Ok(())
}
```

### 4. 结合信号强度筛选

```rust
async fn pick_strong_signals(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",
        None
    ).await?;
    
    // 只保留强买入信号（信号强度 >= 80）
    let strong_buy: Vec<_> = results.into_iter()
        .filter(|r| r.strategy_result.signal_strength() >= 80)
        .collect();
    
    println!("强买入信号: {} 只", strong_buy.len());
    
    for result in strong_buy.iter().take(5) {
        println!("\n股票: {} - {}", 
            result.ts_code, 
            result.stock_name.as_deref().unwrap_or("未知")
        );
        
        // 获取详细信息
        if let service::strategy::StrategyResult::SingleLimitUp(detail) = &result.strategy_result {
            println!("  涨停日期: {}", detail.limit_up_date);
            println!("  上涨天数: {}/{}", detail.up_days, detail.analysis_period);
            println!("  累计涨幅: {:.2}%", detail.total_gain_pct);
            println!("  风险等级: {}/5", detail.risk_level);
        }
    }
    
    Ok(())
}
```

### 5. 导出到 CSV

```rust
use std::fs::File;
use std::io::Write;

async fn export_to_csv(db: DatabaseConnection) -> anyhow::Result<()> {
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",
        None
    ).await?;
    
    // 创建 CSV 文件
    let mut file = File::create("single_limit_up_stocks.csv")?;
    
    // 写入表头
    writeln!(file, "股票代码,股票名称,信号,信号强度,涨停次数,上涨天数,累计涨幅(%),风险等级,描述")?;
    
    // 写入数据
    for result in results {
        if let service::strategy::StrategyResult::SingleLimitUp(detail) = &result.strategy_result {
            writeln!(
                file,
                "{},{},{:?},{},{},{},{:.2},{},\"{}\"",
                result.ts_code,
                result.stock_name.as_deref().unwrap_or("未知"),
                detail.strategy_signal,
                detail.signal_strength,
                detail.limit_up_count,
                detail.up_days,
                detail.total_gain_pct,
                detail.risk_level,
                detail.analysis_description.replace("\"", "\"\"")  // 转义引号
            )?;
        }
    }
    
    println!("结果已导出到 single_limit_up_stocks.csv");
    
    Ok(())
}
```

## 配置参数说明

### 默认配置

```json
{
  "analysis_period": 20,        // 分析周期（天）
  "limit_up_tolerance": 0.5,    // 涨停容差（%）
  "min_up_days": 5,             // 最小上涨天数
  "min_total_gain": 20.0        // 最小累计涨幅（%）
}
```

### 参数调整建议

#### 保守型（稳健上涨）
```json
{
  "analysis_period": 30,
  "limit_up_tolerance": 0.3,
  "min_up_days": 20,
  "min_total_gain": 15.0
}
```

#### 激进型（强势突破）
```json
{
  "analysis_period": 10,
  "limit_up_tolerance": 0.5,
  "min_up_days": 6,
  "min_total_gain": 30.0
}
```

#### 创业板/科创板专用
```json
{
  "analysis_period": 20,
  "limit_up_tolerance": 0.5,
  "min_up_days": 10,
  "min_total_gain": 30.0
}
```

## 返回结果说明

### StockPickResult 结构

```rust
pub struct StockPickResult {
    pub ts_code: String,              // 股票代码
    pub stock_name: Option<String>,   // 股票名称
    pub strategy_result: StrategyResult,  // 策略分析结果
}
```

### SingleLimitUpResult 字段

通过 `StrategyResult::SingleLimitUp` 变体访问：

- `stock_code`: 股票代码
- `analysis_date`: 分析日期
- `current_price`: 当前价格
- `strategy_signal`: 策略信号（StrongBuy/Buy/Hold/Sell/StrongSell）
- `signal_strength`: 信号强度（0-100）
- `analysis_description`: 分析说明
- `risk_level`: 风险等级（1-5）
- `limit_up_count`: 涨停次数
- `limit_up_date`: 涨停日期
- `up_days`: 上涨天数
- `total_gain_pct`: 累计涨幅（%）
- `analysis_period`: 分析周期

## 完整示例

```rust
use service::stock_picker_service::StockPickerService;
use service::strategy::StrategyResult;
use chrono::NaiveDate;
use entity::sea_orm::DatabaseConnection;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 假设已经建立数据库连接
    let db: DatabaseConnection = todo!();
    
    let service = StockPickerService::new(db);
    
    // 设置日期范围
    let start_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();
    
    // 自定义配置
    let config = json!({
        "analysis_period": 20,
        "limit_up_tolerance": 0.5,
        "min_up_days": 5,
        "min_total_gain": 20.0
    });
    
    // 执行选股
    println!("开始筛选单次涨停股票...");
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "single_limit_up",
        Some(config)
    ).await?;
    
    println!("\n=== 单次涨停策略选股结果 ===");
    println!("共找到 {} 只股票\n", results.len());
    
    // 按信号强度排序（已经在内部排序）
    for (i, result) in results.iter().enumerate().take(20) {
        println!("{}. {} - {}", 
            i + 1,
            result.ts_code,
            result.stock_name.as_deref().unwrap_or("未知")
        );
        
        println!("   信号: {:?} (强度: {})", 
            result.strategy_result.strategy_signal(),
            result.strategy_result.signal_strength()
        );
        
        // 获取详细信息
        if let StrategyResult::SingleLimitUp(detail) = &result.strategy_result {
            println!("   涨停: {} 次 ({})", 
                detail.limit_up_count,
                if detail.limit_up_date.is_empty() { 
                    "无".to_string() 
                } else { 
                    detail.limit_up_date.clone() 
                }
            );
            println!("   上涨: {}/{} 天 ({:.1}%)", 
                detail.up_days,
                detail.analysis_period,
                (detail.up_days as f64 / detail.analysis_period as f64) * 100.0
            );
            println!("   累计涨幅: {:.2}%", detail.total_gain_pct);
            println!("   风险等级: {}/5", detail.risk_level);
            println!("   {}", detail.analysis_description);
        }
        println!();
    }
    
    // 统计信息
    let strong_buy_count = results.iter()
        .filter(|r| matches!(r.strategy_result.strategy_signal(), 
            service::strategy::StrategySignal::StrongBuy))
        .count();
    
    let buy_count = results.iter()
        .filter(|r| matches!(r.strategy_result.strategy_signal(), 
            service::strategy::StrategySignal::Buy))
        .count();
    
    println!("\n=== 统计信息 ===");
    println!("强烈买入: {} 只", strong_buy_count);
    println!("买入: {} 只", buy_count);
    println!("总计: {} 只", results.len());
    
    Ok(())
}
```

## 注意事项

1. **数据要求**：确保数据库中有足够的历史数据（至少覆盖配置的 `analysis_period`）
2. **性能考虑**：批量筛选可能需要较长时间，建议合理设置日期范围
3. **涨停识别**：策略会自动识别不同板块的涨停阈值（10%/20%/30%）
4. **结果排序**：返回结果已按信号强度降序排列
5. **风险提示**：策略结果仅供参考，实际投资需结合基本面和市场环境

## 支持的所有策略类型

- `"price_volume_candlestick"` - 价量K线策略
- `"bottom_volume_surge"` - 底部放量上涨策略
- `"long_term_bottom_reversal"` - 长期底部反转策略
- `"yearly_high"` - 年内新高策略
- `"price_strength"` - 价格强弱策略
- `"distressed_reversal"` - 困境反转策略
- `"single_limit_up"` - 单次涨停策略 ✨ 新增
