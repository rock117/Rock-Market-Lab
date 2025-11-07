### 指数
000001.SH 上证指数
399001.SZ 深证城指
399006.SZ 创业板指

# db
sudo apt install mysql-server
sudo service mysql restart
service mysql start
mysql -u root
mysql -p -u [user] [database] < backup-file.sql
mysql -uroot investmentresearch < investment-research-db.sql 

# 疑难
https://cloud.tencent.com/developer/article/2102020

# task TODO
1. 定时服务接口，周期性抓取股票，指数数据
2. API list
2.1 搜索接口，搜索股票，基金，指数 done
2.2 获取历史交易数据(价格，成交量，汇总)
3. 帅选，通过财务指标，成交信息
4. 融资融券总额 涨跌家数
5. 按市值统计
6. 股票列表API 成交量 换手率 振幅 涨跌
7. 搜索股价在周线，月线，季线，年线上的股票，均线之间大小关系，均线选股
8. 统计 跌幅4000家后 五日内指数涨跌情况。统计年线 60日线的股票数 
9. 统计a股 营收 净利润 盈亏情况占比
10. 根据当期财务报表 预测下期财务报表
11. 美股研发投入



# 功能
股票: 股价，成交量, 涨跌天数，技术指标 5日，20日，60日，120日
大盘: 涨跌数，交易量，板块交易量占比 过去一段时间的统计信息, 技术指标 5日，20日，60日，120日
行业: 行业列表，行业成交量
关键字搜索: 搜索股票名，主营业务，财报
技术指标: MACD,  price > m5, m10, m20, 搜索股价在周线，月线，季线，年线上的股票
**特色指标搜索:** 
- 统计大跌后5日 10日内 股价走势
- 涨/跌时的成交量: 最小，最大，平均 标准差
- 在上涨趋势中 统计 低于 5 10 20日线的天数
- 统计板块成交量历史
- 两年内 横盘 处在低位的股票
- 每个股票以年为基准比较，每年按月线显示
- 净资产收益率 负债率排名 库存排名 应收账款排名 外汇储备
- 统计 收盘价 与 最低点，最高点的关系，如收盘价在最低点/最高点，往后3天各交易日 股价涨的频率
- top 20 成交量股票，且连续涨/跌n天， 涨/跌幅xx, 按行业/概念
- 找每天股价涨跌小于2, 股价波动小于3的股票
- 封成比, 多日波动小于2%, 且股价处于底部
- 统计大盘主力资金出逃时，大盘涨跌，涨跌家数, 个股主力净流入和股价量价关系
- 过去5天涨50% 五天前一个月 涨幅小于10
- 统计10倍股 耗时 及其涨跌数 均线
- 统计单位价格涨跌的成交量
- 查找最近没怎么涨的 板块/行业/概念 ，再在里面查找涨幅居前的个股 或者说 波动大的个股
- 5 10 20 60日线 差值小于5%
- 统计某段时间内的涨停数
- 5日均线形态 小步上升还是 大起大落
- 根据海外营收 排序
- 查找十大流通股股东 持有哪些其他公司
- 根据海外营收 列出历年业务增长情况 排序
- 寻找趋势:  某天之前，五日线小于20日线(连续3天)，之后五日线大于20日线.
  比如今天之前
- 选股
  - 20日内首次涨停
  - 20日内3连板
  - 10日内涨停，且已跌了-5%
  - 今年年内低点或高点 到现在的涨跌幅
  - 早出下跌趋势 识别下跌 日线MA 5< 10  <20<30<60
  - 统计连板


### 分析
1. 股价到达最高点时，偏离5，10，20，60日线的幅度
2. 股价到达最低点时，偏离5，10，20，60日线的幅度

### 自定义筛选
基于表达式，过滤股票，如:  price > m5 && pe > 3 && 营收增速 > 0
搜索: 连涨/跌 天/周/月数，3年内腰斩
社区热点:连板数


### 金融数据
1. 外资持股

https://data.eastmoney.com/hsgtcg/StockHdStatistics/002747.html

https://choice.eastmoney.com/

### 股价
https://www.cnblogs.com/ldlchina/p/5392670.html
https://blog.crackcreed.com/diy-xue-qiu-app-shu-ju-api/

### 国债利率
https://cn.investing.com/rates-bonds/u.s.-10-year-bond-yield-historical-data
https://ycharts.com/indicators/10_year_treasury_rate
https://zh.tradingeconomics.com/united-states/15-year-mortgage-rate

### 美元指数
https://cn.investing.com/indices/usdollar-historical-data


### 其他
基准
宏观数据
外汇
区块链

# log
https://blog.devgenius.io/adding-logging-and-tracing-to-an-axum-app-rust-d7935693bc3c
https://blog.logrocket.com/composing-underpinnings-observable-rust-application/

.route("/image/:spec/:url", get(generate));
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {

# run & test
cargo nextest list # list all test
cargo nextest run  # run all test
cargo nextest run --nocapture # run all test and show output(by invoking println!)
cargo nextest run service::macd::test::test_average # service模块的 macd::test::test_average
set RUSTFLAGS=-Awarnings & cargo nextest run  --nocapture test_get_trade_calendar_list
