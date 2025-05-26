# 根据 净资产收益率，毛利率
select s.name, s.name_py, ann_date, s.ts_code, roe, grossprofit_margin from finance_indicator f left join stock s on f.ts_code = s.ts_code where (end_date='20230331' or end_date='20240331' or end_date='20250331') and grossprofit_margin > 30 and roe > 5
