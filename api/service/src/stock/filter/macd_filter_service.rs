use entity::sea_orm::DatabaseConnection;
use entity::stock_daily;
use common::finance::ma;

/// Filters stocks that have broken above their 5-day moving average for the first time in the past 60 days.
///
/// # Arguments
///
/// * `conn` - A reference to the database connection.
///
/// # Returns
///
/// * `anyhow::Result<()>` - Returns an Ok result if successful, otherwise returns an error.
pub async fn filter_continue_price_limit(conn: &DatabaseConnection) -> anyhow::Result<()> {
    // Initialize an empty vector to store stock prices.
    let prices: Vec<stock_daily::Model> = vec![];

    // Convert stock_daily models to a vector of f64 prices.
    let prices = to_prices(&prices);

    // Assume the first price for processing.
    let price = prices[0];

    // Calculate different moving averages for the given prices.
    let ma5 = ma::<5>(&prices);
    let ma10 = ma::<10>(&prices);
    let ma20 = ma::<20>(&prices);
    let ma60 = ma::<60>(&prices);
    let ma120 = ma::<120>(&prices);

    // Return Ok result, indicating successful execution.
    Ok(())
}


/// Converts a slice of `stock_daily::Model` to a vector of `f64` prices.
///
/// # Arguments
///
/// * `prices` - A slice of `stock_daily::Model` representing the stock data.
///
/// # Returns
///
/// * `Vec<f64>` - A vector of `f64` representing the converted stock prices.
fn to_prices(prices: &[stock_daily::Model]) -> Vec<f64> {
    todo!()
}
