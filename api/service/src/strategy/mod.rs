//! 交易策略模块
//! 
//! 包含各种股票交易策略的实现，基于 trait 设计以支持多种策略

pub mod traits;
pub mod price_volume_candlestick_strategy;
pub mod bottom_volume_surge_strategy;
pub mod examples;
pub mod simple_usage;

// 重新导出主要 traits 和类型
pub use traits::{
    TradingStrategy, 
    StrategyConfig, 
    StrategyResult, 
    StrategySignal,
    StrategyResultTrait,
    GenericStrategyResult,
    StrategyInfo,
    StrategyType,
    RiskLevel,
    StrategyPerformance,
    BacktestResult,
    TradeRecord,
    TradeType,
    SecurityData,
    SecurityType,
    TimeFrame
};

// 重新导出价量K线策略相关类型
pub use price_volume_candlestick_strategy::{
    PriceVolumeCandlestickStrategy,
    PriceVolumeStrategyConfig,
    PriceVolumeAnalysisResult,
    CandlestickPattern, 
    VolumeSignal
};

// 重新导出底部放量上涨策略相关类型
pub use bottom_volume_surge_strategy::{
    BottomVolumeSurgeStrategy,
    BottomVolumeSurgeConfig,
    BottomVolumeSurgeResult,
};
