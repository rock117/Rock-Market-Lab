# request param
{
  "start_date": "2024-10-08",
  "end_date": "2024-11-08",
  "pct_chg_min_date": 1, // 每日最小涨跌额
  "pct_chg_max_date": 2  // 每日最大涨跌额
  "pct_chg_min": 1, // 最小涨跌额
  "pct_chg_max": 2  // 最大涨跌额
}

# response
{
  "data": [
      ｛
        "ts_code": "600000.SH",
        "name": "浦发银行",
        "pct_chg": 1, // 涨跌百分比
        "inc_num": 4, //上涨天数
        "dec_num": 1,
        "consecutive_inc_num": 3, // 连续上涨天数
        "consecutive_dec_num": 3, // 连续下跌天数
        "consecutive_limit_up_num": 3, // 连续涨停天数
        "consecutive_limit_down_num": 3, // 连续跌停天数
      ｝
  ]
}