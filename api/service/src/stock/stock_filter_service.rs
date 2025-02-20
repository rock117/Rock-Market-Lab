use anyhow::anyhow;

struct StockInfo {
    close: f64,
    open: f64,
    high: f64,
    low: f64,
}

/// 20天内涨幅超过20%
/// 10天内涨幅超过10%
///
pub async fn filter_stocks(days: u64) {

}

pub async fn filter_by_macd(ma5: f64, ma10: f64, ma20: f64, ma60: f64) -> anyhow::Result<()>{
    let ma:Vec<f64> = vec![ma5, ma10, ma20, ma60];
    let max = *ma.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).ok_or(anyhow!("max is none"))?;
    let min = *ma.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).ok_or(anyhow!("max is none"))?;
    if lte_pct(min, max, 0.02) {

    }
    Ok(())
}

fn lte_pct(n1: f64, n2: f64, pct: f64) -> bool {
    (n2 - n1)/n1 <= pct
}

fn gte_pct(n1: f64, n2: f64, pct: f64) -> bool {
    (n2 - n1)/n1 >= pct
}