//! 选股模块使用示例

use anyhow::Result;
use tracing::info;

use super::{
    StockPickingService, StockPickingRequest, StockPickingStrategy,
    StockPickingCriteria, DataProviderConfig
};

/// 基础选股示例
pub async fn basic_stock_picking_example() -> Result<()> {
    info!("=== 基础选股示例 ===");
    
    // 创建选股服务
    let service = StockPickingService::new();
    
    // 1. 获取强烈看涨的股票
    info!("1. 获取强烈看涨的股票");
    let strong_bullish = service.get_strong_bullish_stocks(10).await?;
    
    for (i, stock) in strong_bullish.iter().enumerate() {
        info!(
            "  {}. {} - 当前价格: {:.2}, 评分: {}, 分析: {}",
            i + 1,
            stock.stock_code,
            stock.current_price,
            stock.score,
            stock.analysis_result
        );
    }
    
    // 2. 获取超卖反弹机会
    info!("2. 获取超卖反弹机会");
    let oversold_opportunities = service.get_oversold_opportunities(5).await?;
    
    for (i, stock) in oversold_opportunities.iter().enumerate() {
        info!(
            "  {}. {} - RSI: {:.1}, 评分: {}, 分析: {}",
            i + 1,
            stock.stock_code,
            stock.rsi14.unwrap_or(0.0),
            stock.score,
            stock.analysis_result
        );
    }
    
    Ok(())
}

/// 自定义条件选股示例
pub async fn custom_criteria_example() -> Result<()> {
    info!("=== 自定义条件选股示例 ===");
    
    let service = StockPickingService::new();
    
    // 自定义筛选条件：
    // - RSI在30-70之间（避免超买超卖）
    // - 要求短期趋势向上
    // - 要求成交量放大
    // - 最小评分70分
    let custom_criteria = StockPickingCriteria {
        min_rsi: Some(30.0),
        max_rsi: Some(70.0),
        require_short_trend: true,
        require_long_trend: false,
        require_volume_surge: true,
        min_score: 70,
        max_results: 15,
    };
    
    let request = StockPickingRequest {
        strategy: StockPickingStrategy::Custom,
        max_results: Some(15),
        stock_codes: None,
        market: None,
        custom_criteria: Some(custom_criteria),
    };
    
    let response = service.pick_stocks(request).await?;
    
    info!(
        "自定义条件选股结果：分析了 {} 只股票，筛选出 {} 只股票",
        response.total_analyzed,
        response.filtered_count
    );
    
    for (i, stock) in response.results.iter().enumerate() {
        info!(
            "  {}. {} - 价格: {:.2}, MA5: {:.2}, MA20: {:.2}, RSI: {:.1}, 成交量放大: {}, 评分: {}",
            i + 1,
            stock.stock_code,
            stock.current_price,
            stock.ma5.unwrap_or(0.0),
            stock.ma20.unwrap_or(0.0),
            stock.rsi14.unwrap_or(0.0),
            stock.volume_surge,
            stock.score
        );
    }
    
    Ok(())
}

/// 分析指定股票示例
pub async fn analyze_specific_stocks_example() -> Result<()> {
    info!("=== 分析指定股票示例 ===");
    
    let service = StockPickingService::new();
    
    // 分析指定的股票列表
    let stock_codes = vec![
        "000001.SZ".to_string(), // 平安银行
        "000002.SZ".to_string(), // 万科A
        "600000.SH".to_string(), // 浦发银行
        "600036.SH".to_string(), // 招商银行
    ];
    
    let results = service.analyze_specific_stocks(stock_codes).await?;
    
    info!("指定股票分析结果：");
    
    for stock in results {
        info!("股票代码: {}", stock.stock_code);
        info!("  当前价格: {:.2}", stock.current_price);
        info!("  技术指标:");
        info!("    MA5: {:.2}", stock.ma5.unwrap_or(0.0));
        info!("    MA20: {:.2}", stock.ma20.unwrap_or(0.0));
        info!("    MA60: {:.2}", stock.ma60.unwrap_or(0.0));
        info!("    RSI14: {:.1}", stock.rsi14.unwrap_or(0.0));
        if let (Some(macd), Some(signal)) = (stock.macd_line, stock.macd_signal) {
            info!("    MACD: {:.4}, Signal: {:.4}", macd, signal);
        }
        info!("  趋势分析:");
        info!("    短期趋势: {}", if stock.short_trend { "向上" } else { "向下" });
        info!("    长期趋势: {}", if stock.long_trend { "向上" } else { "向下" });
        info!("    整体看涨: {}", stock.overall_bullish);
        info!("  成交量分析:");
        info!("    成交量: {:.0}", stock.volume);
        info!("    成交量放大: {}", stock.volume_surge);
        info!("  综合评价:");
        info!("    分析结果: {}", stock.analysis_result);
        info!("    评分: {}/100", stock.score);
        info!("  ---");
    }
    
    Ok(())
}

/// 按市场选股示例
pub async fn market_analysis_example() -> Result<()> {
    info!("=== 按市场选股示例 ===");
    
    let service = StockPickingService::new();
    
    // 分析深圳市场的股票
    info!("分析深圳市场股票:");
    let sz_results = service.analyze_market_stocks("SZ".to_string(), 10).await?;
    
    for (i, stock) in sz_results.iter().enumerate() {
        info!(
            "  {}. {} - 评分: {}, 分析: {}",
            i + 1,
            stock.stock_code,
            stock.score,
            stock.analysis_result
        );
    }
    
    // 分析上海市场的股票
    info!("分析上海市场股票:");
    let sh_results = service.analyze_market_stocks("SH".to_string(), 10).await?;
    
    for (i, stock) in sh_results.iter().enumerate() {
        info!(
            "  {}. {} - 评分: {}, 分析: {}",
            i + 1,
            stock.stock_code,
            stock.score,
            stock.analysis_result
        );
    }
    
    Ok(())
}

/// 高级配置示例
pub async fn advanced_configuration_example() -> Result<()> {
    info!("=== 高级配置示例 ===");
    
    // 自定义数据提供者配置
    let config = DataProviderConfig {
        default_days: 200,     // 获取200天数据
        min_volume: 5000000,   // 最小成交量500万
        max_stocks: 500,       // 最大500只股票
    };
    
    let service = StockPickingService::with_config(config);
    
    // 使用多种策略进行选股
    let strategies = vec![
        StockPickingStrategy::StrongBullish,
        StockPickingStrategy::OversoldOpportunity,
        StockPickingStrategy::UptrendNormalRsi,
    ];
    
    for strategy in strategies {
        let request = StockPickingRequest {
            strategy: strategy.clone(),
            max_results: Some(5),
            stock_codes: None,
            market: None,
            custom_criteria: None,
        };
        
        let response = service.pick_stocks(request).await?;
        
        info!(
            "策略 {:?}: 分析 {} 只股票，筛选出 {} 只股票，耗时 {}",
            strategy,
            response.total_analyzed,
            response.filtered_count,
            response.analysis_time
        );
        
        for (i, stock) in response.results.iter().enumerate() {
            info!(
                "  {}. {} - 评分: {}, {}",
                i + 1,
                stock.stock_code,
                stock.score,
                stock.analysis_result
            );
        }
        info!("---");
    }
    
    Ok(())
}

/// 运行所有示例
pub async fn run_all_examples() -> Result<()> {
    info!("开始运行选股模块示例...");
    
    basic_stock_picking_example().await?;
    custom_criteria_example().await?;
    analyze_specific_stocks_example().await?;
    market_analysis_example().await?;
    advanced_configuration_example().await?;
    
    info!("所有示例运行完成！");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_examples() {
        // 注意：这些测试需要实际的数据库连接才能正常工作
        // 在没有数据的情况下，它们会返回空结果但不会出错
        
        let result = basic_stock_picking_example().await;
        assert!(result.is_ok());
        
        let result = custom_criteria_example().await;
        assert!(result.is_ok());
        
        let result = analyze_specific_stocks_example().await;
        assert!(result.is_ok());
    }
}
