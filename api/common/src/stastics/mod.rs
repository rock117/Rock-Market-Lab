use serde::Serialize;
use std::cmp::Ordering;

#[derive(Serialize, Debug, Copy, Clone)]
pub struct Stastics {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub total: f64,
    pub median: f64,
    //中位数
    pub standard_dev: f64, //标准差, 标准差是方差的算术平方根
    pub volatility: f64,   // 波动，(vmax - vmin)/vmax
    pub pct_change: f64,   // 涨跌幅，(v0-vn)/v0
}

#[derive(Serialize, Debug, Copy, Clone)]
pub struct IncDecInfo {
    pub consecutive_inc: u64, // 连涨数
    pub consecutive_dec: u64, // 连跌数
    pub inc: u64,             // 涨数
    pub dec: u64,             // 跌数
}

impl From<&Vec<f64>> for IncDecInfo {
    fn from(datas: &Vec<f64>) -> Self {
        let (mut cinc_num, mut cdec_num, mut inc_num, mut dec_num) = (0, 0, 0, 0);

        if datas.is_empty() {
            return Self {
                consecutive_inc: cinc_num,
                consecutive_dec: cdec_num,
                inc: inc_num,
                dec: dec_num,
            };
        }
        let datas = datas.iter().rev().collect::<Vec<&f64>>();
        let mut current = *datas[0];
        let remains = &datas[1..datas.len()];
        let mut calc_cinc_num = true;
        let mut calc_cdec_num = true;

        for data in remains {
            let data = **data;
            if current >= data {
                inc_num += 1;
                if calc_cinc_num {
                    cinc_num += 1;
                    calc_cdec_num = false;
                }
            }
            if current <= data {
                dec_num += 1;
                if calc_cdec_num {
                    cdec_num += 1;
                    calc_cinc_num = false;
                }
            }
            current = data;
        }
        Self {
            consecutive_inc: cinc_num,
            consecutive_dec: cdec_num,
            inc: inc_num,
            dec: dec_num,
        }
    }
}

pub fn calc_stastics(data: &mut Vec<f64>) -> Option<Stastics> {
    if data.is_empty() {
        return None;
    }
    let first = *data.first().unwrap_or(&0.0);
    let last = *data.last().unwrap_or(&0.0);

    data.sort_by(|v1, v2| v1.partial_cmp(&v2).unwrap_or(Ordering::Equal));
    let min = data.first();
    let max = data.last();
    let total = data.iter().sum::<f64>();
    let avg = total / data.len() as f64;
    let median = calc_median(data);
    let standard_dev = calc_standard_deviation(data.as_slice());
    match (min, max, standard_dev) {
        (Some(min), Some(max), Some(standard_dev)) => Some(Stastics {
            min: *min,
            max: *max,
            avg,
            total,
            median,
            standard_dev,
            volatility: (max - min) / min,
            pct_change: (first - last) / first,
        }),
        _ => None,
    }
}

fn calc_standard_deviation(data: &[f64]) -> Option<f64> {
    let n = data.len() as f64;
    if n <= 1.0 {
        return None;
    }

    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let standard_deviation = variance.sqrt();

    Some(standard_deviation)
}

fn calc_median(data: &Vec<f64>) -> f64 {
    if data.len() % 2 == 0 {
        let m1 = data.len() / 2;
        let m2 = m1 - 1;
        (*data.get(m1).unwrap() + *data.get(m2).unwrap()) / 2f64
    } else {
        *data.get(data.len() / 2).unwrap()
    }
}

mod test {
    use crate::stastics::calc_median;

    #[test]
    fn test_gen_median() {
        assert_eq!(2.0, calc_median(&vec![1.0, 2.0, 3.0]));
        assert_eq!(2.5, calc_median(&vec![1.0, 2.0, 3.0, 4.0]));
    }
}
