# 策略预设配置使用指南

## 概述

从现在开始，`StockPickerService` 支持三种配置方式：

1. **默认配置** - `settings` 为 `None`
2. **预设配置** - `settings` 中包含 `"preset"` 字段
3. **自定义配置** - `settings` 中直接提供完整参数

## 配置字段名

使用 **`"preset"`** 字段来指定预设配置名称。

## 支持预设的策略

### 1. 海龟交易策略 (turtle)

**可用预设：**
- `"system1"` - 标准配置（20天突破）
- `"system2"` - 保守配置（55天突破）
- `"conservative"` - 更大止损空间
- `"aggressive"` - 更激进配置

### 2. 涨停回调策略 (limit_up_pullback)

**可用预设：**
- `"standard"` - 标准配置（回调到5日或10日线）
- `"aggressive"` - 激进配置（只看5日线）
- `"conservative"` - 稳健配置（只看10日线，要求更严格）
- `"strong_stock"` - 强势股配置（至少2次涨停）

## 使用示例

### 方式1：使用默认配置

```rust
use service::StockPickerService;
use chrono::NaiveDate;

let service = StockPickerService::new(db);

// 使用默认配置
let results = service.pick_stocks(
    &NaiveDate::from_ymd(2024, 1, 1),
    &NaiveDate::from_ymd(2024, 12, 31),
    "turtle",
    None  // 使用默认配置
).await?;
```

### 方式2：使用预设配置

```rust
// 海龟策略 - 使用 system1 预设
let preset = serde_json::json!({
    "preset": "system1"
});

let results = service.pick_stocks(
    &start_date,
    &end_date,
    "turtle",
    Some(preset)
).await?;

// 涨停回调策略 - 使用 aggressive 预设
let preset = serde_json::json!({
    "preset": "aggressive"
});

let results = service.pick_stocks(
    &start_date,
    &end_date,
    "limit_up_pullback",
    Some(preset)
).await?;
```

### 方式3：使用自定义配置

```rust
// 海龟策略 - 自定义参数
let custom = serde_json::json!({
    "entry_breakout_period": 30,
    "exit_breakout_period": 15,
    "atr_period": 20,
    "stop_loss_atr_multiple": 2.5,
    "pyramid_atr_multiple": 0.5,
    "max_pyramid_units": 3,
    "use_system2": false
});

let results = service.pick_stocks(
    &start_date,
    &end_date,
    "turtle",
    Some(custom)
).await?;

// 涨停回调策略 - 自定义参数
let custom = serde_json::json!({
    "lookback_days": 7,
    "limit_up_threshold": 9.9,
    "ma_type": "MA5",
    "pullback_tolerance": 2.5,
    "min_limit_up_count": 1,
    "require_volume_shrink": true,
    "volume_shrink_ratio": 0.6,
    "require_ma_bullish": false
});

let results = service.pick_stocks(
    &start_date,
    &end_date,
    "limit_up_pullback",
    Some(custom)
).await?;
```

## 配置优先级

配置的处理逻辑如下：

```
1. settings 是 None
   ↓
   使用 Config::default()

2. settings 有值
   ↓
   检查是否有 "preset" 字段
   ↓
   有 "preset" → 调用对应的预设函数（如 TurtleStrategy::system1()）
   ↓
   无 "preset" → 使用 JSON 反序列化整个 settings
```

## 错误处理

### 预设名称不存在

如果指定的预设名称不存在，会返回错误：

```rust
// 错误示例：不存在的预设
let preset = serde_json::json!({
    "preset": "invalid_preset"
});

// 会返回错误：
// "海龟策略不支持预设 'invalid_preset', 可用预设: system1, system2, conservative, aggressive"
```

### 策略不支持预设

对于不支持预设的策略（如 `price_strength`），如果指定了 `preset` 字段，会返回错误：

```rust
let preset = serde_json::json!({
    "preset": "aggressive"
});

let results = service.pick_stocks(
    &start_date,
    &end_date,
    "price_strength",  // 不支持预设
    Some(preset)
).await?;

// 会返回错误：
// "策略配置不支持预设 'aggressive', 请使用 JSON 配置或不指定 preset 字段"
```

## 在 HTTP API 中使用

如果你的 API 接受 JSON 请求，可以这样使用：

### 请求示例 1：使用预设

```json
POST /api/stocks/pick
{
  "start_date": "2024-01-01",
  "end_date": "2024-12-31",
  "strategy_type": "turtle",
  "settings": {
    "preset": "aggressive"
  }
}
```

### 请求示例 2：使用自定义配置

```json
POST /api/stocks/pick
{
  "start_date": "2024-01-01",
  "end_date": "2024-12-31",
  "strategy_type": "limit_up_pullback",
  "settings": {
    "lookback_days": 7,
    "ma_type": "MA5",
    "pullback_tolerance": 3.0,
    "min_limit_up_count": 2
  }
}
```

### 请求示例 3：使用默认配置

```json
POST /api/stocks/pick
{
  "start_date": "2024-01-01",
  "end_date": "2024-12-31",
  "strategy_type": "turtle"
  // settings 字段省略，使用默认配置
}
```

## 添加新的预设配置

如果你想为其他策略添加预设支持，需要：

1. 在策略的 `impl` 块中添加预设函数：

```rust
impl MyStrategy {
    pub fn preset1() -> MyConfig {
        MyConfig {
            // ... 预设参数
        }
    }
    
    pub fn preset2() -> MyConfig {
        MyConfig {
            // ... 预设参数
        }
    }
}
```

2. 在 `stock_picker_service.rs` 的 `match` 语句中添加处理：

```rust
"my_strategy" => create_strategy!(MyConfig, MyStrategy, |preset: &str| {
    Ok(match preset {
        "preset1" => MyStrategy::preset1(),
        "preset2" => MyStrategy::preset2(),
        _ => bail!("不支持的预设 '{}', 可用: preset1, preset2", preset),
    })
}),
```

## 总结

- **字段名**：`"preset"`
- **三种方式**：默认、预设、自定义
- **灵活性**：可以根据需要选择最合适的配置方式
- **易用性**：预设配置让常用场景更简单
- **扩展性**：可以轻松为任何策略添加预设支持
