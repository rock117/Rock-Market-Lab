use num::integer::sqrt;
use num::Zero;
use std::ops::{Add, Div, Mul, MulAssign};
use tracing::info;

pub fn find_max(datas: &Vec<Option<f64>>) -> Option<f64> {
    find_data(datas, |x, y| x > y)
}

pub fn find_min(datas: &Vec<Option<f64>>) -> Option<f64> {
    find_data(datas, |x, y| x < y)
}

/// (second - first)/frist
pub fn calc_inc_percent(first: Option<f64>, second: Option<f64>) -> Option<f64> {
    info!("calc_inc_percent: => first = {first:?}, second = {second:?}");
    match (first, second) {
        (Some(first), Some(second)) if first != 0f64 => Some((second - first) / first),
        (_, _) => None,
    }
}

pub fn avg(datas: &Vec<Option<f64>>) -> Option<f64> {
    let ret = sum_(datas);
    match ret {
        None => None,
        Some(v) => Some(v.1 / v.0 as f64),
    }
}

pub fn sum<T: Add + Zero + Copy>(datas: &Vec<Option<T>>) -> Option<T> {
    return sum_(datas).map(|v| v.1);
}

fn sum_<T: Add + Zero + Copy>(datas: &Vec<Option<T>>) -> Option<(usize, T)> {
    let mut sum = T::zero();
    let mut len = 0usize;
    for x in datas {
        if let Some(x) = x {
            sum = sum + *x;
            len += 1;
        }
    }
    Some((len, sum))
}

pub fn calc_macd(datas: &[f64], n: usize) -> Option<f64> {
    if datas.len() < n {
        return None;
    } else {
        let datas = &datas[datas.len() - n..datas.len()];
        Some(datas.iter().sum::<f64>() / n as f64)
    }
}

pub fn calc_macd_option(datas: &Vec<Option<f64>>, n: usize) -> Option<f64> {
    if datas.len() < n {
        return None;
    }
    let mut params = vec![];
    for data in datas {
        if let Some(v) = data {
            params.push(*v);
        }
    }
    calc_macd(params.as_slice(), n)
}

pub fn calc_stddev(datas: &Vec<Option<f64>>) -> Option<f64> {
    if datas.is_empty() {
        return None;
    }
    let mut total = 0f64;
    for x in datas {
        if let Some(x) = *x {
            total += x * x; // TODO handle 溢出
        }
    }
    let sqrt = (total / (datas.len() - 1) as f64).sqrt();
    Some(sqrt)
}

fn find_data(datas: &Vec<Option<f64>>, f: fn(x: f64, y: f64) -> bool) -> Option<f64> {
    if datas.is_empty() {
        return None;
    }
    let datas = datas
        .iter()
        .filter(|v| v.is_some())
        .collect::<Vec<&Option<f64>>>();
    let mut first = datas.get(0);
    if let Some(&&Some(ref first)) = first {
        let mut ret = *first;
        for x in datas {
            if let Some(x) = x {
                if f(*x, ret) {
                    ret = *x;
                }
            }
        }
        return Some(ret);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        let datas = vec![Some(1f64), Some(2f64), Some(3f64)];
        assert_eq!(Some(3f64), find_max(&datas))
    }

    #[test]
    fn test_find_min() {
        let datas = vec![Some(1f64), Some(2f64), Some(3f64)];
        assert_eq!(Some(1f64), find_min(&datas))
    }

    #[test]
    fn test_calc_macd() {
        let data = vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64];
        assert_eq!(Some(5f64), calc_macd(data.as_slice(), 5));
        assert_eq!(None, calc_macd(data.as_slice(), 10));
    }
}
