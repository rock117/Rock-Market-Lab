# 选取财务指标表中的roe字段，按照roe字段降序排列，选取股票代码、roe字段和报告期字段
select s.name, f.ts_code, f.roe, f.end_date from finance_indicator f left join stock s on f.ts_code = s.ts_code where f.end_date = '20240930' order by roe desc
