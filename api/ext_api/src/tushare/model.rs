use std::collections::HashMap;
use anyhow::bail;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde::ser::SerializeStruct;
use common::data_type::NumOrString;
use common::util::csv_util;


#[derive(Debug, Copy, Clone, Serialize)]
pub enum Api {
    stock_basic,
    fund_basic,
    fund_daily,
    fund_portfolio,
    daily,       // 日线行情
    daily_basic, // 每日指标
    moneyflow_mkt_dc,
    weekly,
    monthly,
    index_daily,
    index_weekly,
    index_monthly,
    trade_cal,
    margin,
    stock_company,
    margin_detail,
    stk_holdernumber,
    ths_index, // 同花顺概念列表
    ths_member,
    ths_daily,
    ths_hot,
    fina_mainbz,
    fina_mainbz_vip,
    fina_indicator,
    balancesheet,
    income,
    cashflow,
    index_basic,
    index_daily_basic,
    moneyflow,
    moneyflow_industry_ths,

    us_basic,
    us_daily
}


#[derive(Debug, Clone)]
pub struct ApiParam<'a> {
    pub api_name: Api,
    pub token: &'a str,
    pub params: &'a HashMap<&'a str, &'a str>,
    pub fields: &'a [&'a str],
}

impl<'a> Serialize for ApiParam<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApiParam", 4)?;
        state.serialize_field("api_name", &self.api_name)?;
        state.serialize_field("token", &self.token)?;
        state.serialize_field("params", &self.params)?;
        state.serialize_field("fields", &self.fields.join(","))?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TushareApiResp {
    pub request_id: Option<String>,
    pub code: u32,
    pub msg: Option<String>,
    pub data: Option<Data>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub has_more: bool,
    pub fields: Vec<String>,
    pub items: Vec<Vec<Option<NumOrString>>>,
}

impl TushareApiResp {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

impl Data {
    pub fn to_structs<T: DeserializeOwned>(&self) -> anyhow::Result<Vec<T>> {
        let field_num = self.fields.len();
        for item in &self.items {
            if item.len() != field_num {
                bail!("item len: {} not equal to field num: {}", item.len(), field_num);
            }
        }
        let items = self.items.iter().map(|item| {
            item.iter().map(|item| {
                match item {
                    Some(v) => v.to_string(),
                    None => "".to_string(),
                }
            }).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();
        let csv = csv_util::to_csv(&self.fields, &items)?;
        let res = csv_util::csv_to_structs::<T>(csv.as_str());
        if res.is_err() {
            panic!("csv: {} to structs error: {}", csv, res.err().unwrap());
        }
        res
    }
}
