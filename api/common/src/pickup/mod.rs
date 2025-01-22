mod sideway;

use std::error::Error;
use itertools::Itertools;
use serde::Deserialize;


/// 判断股票是否满足横盘条件
/// # Arguments
/// - `data` - 股票数据
/// - `days` - 最近几天
/// - `price_range_threshold` - 价格波动范围阈值 默认值 0.05
/// - `price_stddev_threshold` - 收盘价标准差阈值 默认值 0.02
/// - `volume_stddev_threshold` - 成交量标准差与均值比率阈值 默认值 0.3
/// - `volume_spike_threshold` - 异常放量的阈值（2倍均量为异常） 默认值 2
pub fn is_sideways(
    data: &[StockRecord],
    days: usize,
    price_range_threshold: f64,
    price_stddev_threshold: f64,
    volume_stddev_threshold: f64,
    volume_spike_threshold: f64,
) -> bool {
    let recent_data = &data[data.len() - days..];

    // 提取收盘价和成交量
    let close_prices: Vec<f64> = recent_data.iter().map(|record| record.close).collect();
    let volumes: Vec<f64> = recent_data.iter().map(|record| record.volume).collect();

    let max_close = close_prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_close = close_prices.iter().cloned().fold(f64::INFINITY, f64::min);

    let price_range = (max_close - min_close) / min_close;
    let price_stddev = standard_deviation(&close_prices);
    let volume_avg = mean(&volumes);
    let volume_stddev = standard_deviation(&volumes) / volume_avg;

    let volume_spike_days = volumes.iter()
        .filter(|&&v| v > volume_avg * volume_spike_threshold)
        .count();

    let ma5 = mean(&close_prices[days.saturating_sub(5)..]);
    let ma10 = mean(&close_prices[days.saturating_sub(10)..]);
    let ma20 = mean(&close_prices);

    let ma_diff = ((ma5 - ma10).abs().max((ma10 - ma20).abs())).max((ma5 - ma20).abs()) / ma20;

    println!("价格波动: {:.4}, {}", price_range, price_range_threshold);
    println!("价格标准差: {:.4}, {}", price_stddev, price_stddev_threshold);
    println!("均线差异: {:.4}, 0.01", ma_diff);
    println!("平均成交量: {:.0}", volume_avg);
    println!("成交量标准差: {:.4}, {}", volume_stddev, volume_stddev_threshold);
    println!("异常放量天数: {}, {:?}", volume_spike_days, volume_spike_days);

    price_range <= price_range_threshold &&
        price_stddev <= price_stddev_threshold &&
        ma_diff <= 0.01 &&
        volume_stddev <= volume_stddev_threshold &&
        volume_spike_days == 0
}

/// 计算平均值
fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

/// 计算标准差
fn standard_deviation(data: &[f64]) -> f64 {
    let mean = mean(data);
    let variance: f64 = data.iter().map(|&value| (value - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

#[derive(Debug, Clone)]
struct StockRecord {
    date: String,
    close: f64,
    volume: f64,
}