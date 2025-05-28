# 选取财务指标表中的roe字段，按照roe字段降序排列，选取股票代码、roe字段和报告期字段

select sd.total_mv, s.name, f.ts_code, f.roe, f.grossprofit_margin, f.gross_margin, f.ocf_to_debt, f.end_date from finance_indicator f
left join stock s on f.ts_code = s.ts_code
left join stock_daily_basic sd on f.ts_code = sd.ts_code
where f.end_date = '20240930' and sd.trade_date = '20250509' and roe > 5 and sd.total_mv < 1005000 and f.grossprofit_margin >= 30 and f.ocf_to_debt > 0.3
order by total_mv asc, roe desc