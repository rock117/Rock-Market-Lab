pub struct InvestmentPrice {
    pub ts_code: String,
    pub pct_chg: f64,
    pub high: f64,
    pub close: f64,
}

pub fn is_price_limitup(stock: &InvestmentPrice) -> bool {
    let tscode = &stock.ts_code;
    let pct_chg = stock.pct_chg;
    let limitup: f64 = if tscode.ends_with("BJ") {
        30f64
    } else if tscode.starts_with("688") {
        20f64
    } else if tscode.starts_with("300") {
        20f64
    } else {
        10f64
    };
    let delta = pct_chg - limitup;
    delta.abs() < 0.01 && stock.close == stock.high
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_price_limitup() {
        // Arrange
        let stock = InvestmentPrice {
            ts_code: "000001.SZ".to_string(),
            pct_chg: 9.99,
            high: 10.0,
            close: 10.0,
        };

        // Act
        let result = is_price_limitup(&stock);

        // Assert
        assert_eq!(result, true);

        // Arrange
        let stock = InvestmentPrice {
            ts_code: "000001.SZ".to_string(),
            pct_chg: 9f64,
            high: 10.0,
            close: 10.0,
        };

        // Act
        let result = is_price_limitup(&stock);

        // Assert
        assert_eq!(result, false);
    }
}