use entity::sea_orm::prelude::Decimal;

#[derive(Debug, Clone, Default)]
pub struct PeriodPctChg {
    pub pct5: Option<Decimal>,
    pub pct10: Option<Decimal>,
    pub pct20: Option<Decimal>,
    pub pct60: Option<Decimal>,
}

impl PeriodPctChg {
    pub fn from_closes_desc(closes_desc: &[Decimal]) -> Self {
        Self {
            pct5: calc_period_pct_chg(closes_desc, 5),
            pct10: calc_period_pct_chg(closes_desc, 10),
            pct20: calc_period_pct_chg(closes_desc, 20),
            pct60: calc_period_pct_chg(closes_desc, 60),
        }
    }

    pub fn to_f64_tuple(&self) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
        (
            self.pct5.and_then(|v| v.to_string().parse::<f64>().ok()),
            self.pct10.and_then(|v| v.to_string().parse::<f64>().ok()),
            self.pct20.and_then(|v| v.to_string().parse::<f64>().ok()),
            self.pct60.and_then(|v| v.to_string().parse::<f64>().ok()),
        )
    }
}

fn calc_period_pct_chg(closes_desc: &[Decimal], days: usize) -> Option<Decimal> {
    if days < 2 {
        return None;
    }
    if closes_desc.len() < days {
        return None;
    }

    let today = closes_desc.get(0).copied()?;
    let past = closes_desc.get(days - 1).copied()?;
    if past.is_zero() {
        return None;
    }

    Some((today - past) / past * Decimal::from(100i64))
}
