// def calculateReturns(index1Prices: Array[Double], index2Prices: Array[Double]): (Array[Double], Array[Double]) = {
// val index1Returns = index1Prices.zip(index1Prices.tail).map { case (current, previous) =>
// math.log(current / previous)
// }
// val index2Returns = index2Prices.zip(index2Prices.tail).map { case (current, previous) =>
// math.log(current / previous)
// }
// println(s"index1Returns = ${index1Returns.toList}, index1Returns.init = ${index1Returns.init.toList}")
// (index1Returns.init, index2Returns.init)
// }
use num_traits::Float;
trait Correlation {
    fn calculate(return1: &Vec<f64>, return2: &Vec<f64>) -> Option<f64>;
}

pub struct PearsonCorrelation;

impl Correlation for PearsonCorrelation {
    fn calculate(return1: &Vec<f64>, return2: &Vec<f64>) -> Option<f64> {
        if return1.len() < 2 || return2.len() < 2 {
            return None;
        }
        let mean_x = return1.iter().sum::<f64>() / return1.len() as f64;
        let mean_y = return2.iter().sum::<f64>() / return2.len() as f64;
        let numerator = return1
            .iter()
            .zip(return2)
            .map(|xy| (*xy.0 - mean_x) * (*xy.1 - mean_y))
            .sum::<f64>();
        let x_square_sum = return1
            .iter()
            .map(|xi| (*xi - mean_x) * (*xi - mean_x))
            .sum::<f64>();
        let y_square_sum = return2
            .iter()
            .map(|yi| (*yi - mean_y) * (*yi - mean_y))
            .sum::<f64>();
        Some(numerator / (x_square_sum.sqrt() * y_square_sum.sqrt()))
    }
}

/// 计算股票/指数/基金的相关性, prices1, prices2 长度至少为2
pub fn calculate_correlation<T: Correlation>(
    prices1: &Vec<f64>,
    prices2: &Vec<f64>,
) -> Option<f64> {
    if prices1.len() < 2 || prices2.len() < 2 {
        return None;
    }
    let (return1, return2) = calculate_returns(prices1, prices2);
    T::calculate(&return1, &return2)
}

fn calculate_returns(index1_prices: &Vec<f64>, index2_prices: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let return1 = index1_prices
        .iter()
        .zip(&index1_prices[1..])
        .map(|v| (v.1 / v.0).ln())
        .collect::<Vec<f64>>();
    let return2 = index2_prices
        .iter()
        .zip(&index2_prices[1..])
        .map(|v| (v.1 / v.0).ln())
        .collect::<Vec<f64>>();
    (return1, return2)
}
