//! 交易策略模块
//! 
//! 包含各种股票交易策略的实现，基于 trait 设计以支持多种策略

pub mod traits;
pub mod price_volume_candlestick_strategy;
pub mod bottom_volume_surge_strategy;
pub mod long_term_bottom_reversal_strategy;
pub mod yearly_high_strategy;
pub mod price_strength_strategy;
pub mod distressed_reversal_strategy;
pub mod single_limit_up_strategy;
pub mod fundamental_strategy;
pub mod consecutive_strong_strategy;
pub mod turtle_strategy;
pub mod limit_up_pullback_strategy;
pub mod strong_close_strategy;
pub mod quality_value_strategy;
pub mod turnover_ma_bullish_strategy;
pub mod turnover_rise_strategy;
pub mod daily_rise_turnover_strategy;
pub mod low_shadow_strategy;
pub mod ma_convergence_strategy;
pub mod consecutive_bullish_strategy;
pub mod ma_divergence_volume_strategy;

// 重新导出主要 traits 和类型
pub use traits::{
    TradingStrategy, 
    StrategyConfig, 
    StrategyResult, 
    StrategySignal,
    BottomVolumeSurgeResult,
    PriceVolumeCandlestickResult,
    StrategyInfo,
    StrategyType,
    RiskLevel,
    StrategyPerformance,
    BacktestResult,
    TradeRecord,
    TradeType,
    SecurityData,
    SecurityType,
    TimeFrame,
    FinancialData,
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
};

// 重新导出长期底部反转策略相关类型
pub use long_term_bottom_reversal_strategy::{
    LongTermBottomReversalStrategy,
    LongTermBottomReversalConfig,
    LongTermBottomReversalResult,
};

// 重新导出年内新高策略相关类型
pub use yearly_high_strategy::{
    YearlyHighStrategy,
    YearlyHighConfig,
    YearlyHighResult,
};

// 重新导出价格强弱策略相关类型
pub use price_strength_strategy::{
    PriceStrengthStrategy,
    PriceStrengthConfig,
    PriceStrengthResult,
};

// 重新导出困境反转策略相关类型
pub use distressed_reversal_strategy::{
    DistressedReversalStrategy,
    DistressedReversalConfig,
    DistressedReversalResult,
};

// 重新导出单次涨停策略相关类型
pub use single_limit_up_strategy::{
    SingleLimitUpStrategy,
    SingleLimitUpConfig,
    SingleLimitUpResult,
};

// 重新导出基本面策略相关类型
pub use fundamental_strategy::{
    FundamentalStrategy,
    FundamentalConfig,
    FundamentalResult,
};

// 重新导出连续强势股策略相关类型
pub use consecutive_strong_strategy::{
    ConsecutiveStrongStrategy,
    ConsecutiveStrongConfig,
    ConsecutiveStrongResult,
};

// 重新导出海龟交易策略相关类型
pub use turtle_strategy::{
    TurtleStrategy,
    TurtleConfig,
    TurtleResult,
};

// 重新导出涨停回调策略相关类型
pub use limit_up_pullback_strategy::{
    LimitUpPullbackStrategy,
    LimitUpPullbackConfig,
    LimitUpPullbackResult,
};

// 重新导出强势收盘策略相关类型
pub use strong_close_strategy::{
    StrongCloseStrategy,
    StrongCloseConfig,
    StrongCloseResult,
};

// 重新导出优质价值策略相关类型
pub use quality_value_strategy::{
    QualityValueStrategy,
    QualityValueConfig,
    QualityValueResult,
};

// 重新导出换手率均线多头策略相关类型
pub use turnover_ma_bullish_strategy::{
    TurnoverMaBullishStrategy,
    TurnoverMaBullishConfig,
    TurnoverMaBullishResult,
};

pub use turnover_rise_strategy::{
    TurnoverRiseStrategy,
    TurnoverRiseConfig,
    TurnoverRiseResult,
};

pub use daily_rise_turnover_strategy::{
    DailyRiseTurnoverStrategy,
    DailyRiseTurnoverConfig,
    DailyRiseTurnoverResult,
};

// 重新导出低位下影线策略相关类型
pub use low_shadow_strategy::{
    LowShadowStrategy,
    LowShadowConfig,
    LowShadowResult,
};

// 重新导出均线粘合策略相关类型
pub use ma_convergence_strategy::{
    MaConvergenceStrategy,
    MaConvergenceConfig,
    MaConvergenceResult,
};

// 重新导出日/周/月连阳策略相关类型
pub use consecutive_bullish_strategy::{
    ConsecutiveBullishStrategy,
    ConsecutiveBullishConfig,
    ConsecutiveBullishResult,
};

pub use ma_divergence_volume_strategy::{
    MaDivergenceVolumeStrategy,
    MaDivergenceVolumeConfig,
    MaDivergenceVolumeResult,
};

