mod correlation;

/// 计算移动平均线，如 5日线，10日线
/// # Arguments
/// - `N` 5, 10, 20, 60 等
pub fn ma<const N: usize>(prices: &[f64]) -> Option<f64> {
    if prices.len() < N {
        return None
    }
    let total = (&prices[0 .. N]).iter().sum::<f64>();
    Some(total / N as f64)
}

/// 计算涨跌幅, 涨跌幅范围 x%100
///
pub fn pct_chg(begin: f64, end: f64) -> f64 {
    100f64*(end - begin) / begin
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ma() {
        let ma5 = super::ma::<5>(&vec![1f64,2f64,3f64]);
        assert_eq!(None, ma5);

        let ma5 = super::ma::<5>(&vec![1f64,2f64,3f64, 4f64, 5f64]);
        assert_eq!(Some(3f64), ma5);
    }
}