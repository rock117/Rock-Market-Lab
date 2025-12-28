// 所有模拟数据集中管理
import { 
  UsStock, 
  MarketSummary, 
  IndexData, 
  VolumeDistribution,
  StockDetail,
  Security,
  StrategyStock
} from '@/types'

// 美股模拟数据
export const mockUsStocks = [
  {
    symbol: 'AAPL',
    name: 'Apple Inc.',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'Consumer Electronics',
    market_cap: 30000000,
    pe_ratio: 28.5,
    roe: 15.6,
    list_date: '1980-12-12',
    description: 'Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.',
    website: 'https://www.apple.com',
    employee_count: 164000,
    founded_date: '1976-04-01',
    address: 'One Apple Park Way, Cupertino, CA 95014'
  },
  {
    symbol: 'MSFT',
    name: 'Microsoft Corporation',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'Software',
    market_cap: 28000000,
    pe_ratio: 32.1,
    roe: 18.2,
    list_date: '1986-03-13',
    description: 'Microsoft Corporation develops, licenses, and supports software, services, devices, and solutions worldwide.',
    website: 'https://www.microsoft.com',
    employee_count: 221000,
    founded_date: '1975-04-04',
    address: 'One Microsoft Way, Redmond, WA 98052'
  },
  {
    symbol: 'GOOGL',
    name: 'Alphabet Inc.',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'Internet Services',
    market_cap: 17000000,
    pe_ratio: 25.8,
    roe: 14.3,
    list_date: '2004-08-19',
    description: 'Alphabet Inc. provides online advertising services in the United States, Europe, the Middle East, Africa, the Asia-Pacific, Canada, and Latin America.',
    website: 'https://www.alphabet.com',
    employee_count: 190000,
    founded_date: '1998-09-04',
    address: '1600 Amphitheatre Parkway, Mountain View, CA 94043'
  },
  {
    symbol: 'AMZN',
    name: 'Amazon.com Inc.',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'E-commerce',
    market_cap: 15000000,
    pe_ratio: 45.2,
    roe: 12.8,
    list_date: '1997-05-15',
    description: 'Amazon.com, Inc. engages in the retail sale of consumer products and subscriptions in North America and internationally.',
    website: 'https://www.amazon.com',
    employee_count: 1540000,
    founded_date: '1994-07-05',
    address: '410 Terry Avenue North, Seattle, WA 98109'
  },
  {
    symbol: 'TSLA',
    name: 'Tesla Inc.',
    exchange: 'NASDAQ',
    industry: 'Automotive',
    sector: 'Electric Vehicles',
    market_cap: 8000000,
    pe_ratio: 65.4,
    roe: 19.3,
    list_date: '2010-06-29',
    description: 'Tesla, Inc. designs, develops, manufactures, leases, and sells electric vehicles, and energy generation and storage systems.',
    website: 'https://www.tesla.com',
    employee_count: 140000,
    founded_date: '2003-07-01',
    address: '1 Tesla Road, Austin, TX 78725'
  },
  {
    symbol: 'NVDA',
    name: 'NVIDIA Corporation',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'Semiconductors',
    market_cap: 12000000,
    pe_ratio: 75.3,
    roe: 22.1,
    list_date: '1999-01-22',
    description: 'NVIDIA Corporation operates as a computing company in the United States, Taiwan, China, Hong Kong, and internationally.',
    website: 'https://www.nvidia.com',
    employee_count: 26196,
    founded_date: '1993-04-05',
    address: '2788 San Tomas Expressway, Santa Clara, CA 95051'
  },
  {
    symbol: 'META',
    name: 'Meta Platforms Inc.',
    exchange: 'NASDAQ',
    industry: 'Technology',
    sector: 'Social Media',
    market_cap: 8500000,
    pe_ratio: 23.4,
    roe: 16.8,
    list_date: '2012-05-18',
    description: 'Meta Platforms, Inc. develops products that enable people to connect and share with friends and family through mobile devices, personal computers, virtual reality headsets, and wearables worldwide.',
    website: 'https://www.meta.com',
    employee_count: 67317,
    founded_date: '2004-02-04',
    address: '1 Meta Way, Menlo Park, CA 94025'
  },
  {
    symbol: 'BRK.A',
    name: 'Berkshire Hathaway Inc.',
    exchange: 'NYSE',
    industry: 'Financial',
    sector: 'Diversified Investments',
    market_cap: 7800000,
    pe_ratio: 8.9,
    roe: 11.2,
    list_date: '1980-03-17',
    description: 'Berkshire Hathaway Inc., through its subsidiaries, engages in the insurance, freight rail transportation, and utility businesses worldwide.',
    website: 'https://www.berkshirehathaway.com',
    employee_count: 383000,
    founded_date: '1839-01-01',
    address: '3555 Farnam Street, Omaha, NE 68131'
  },
  {
    symbol: 'UNH',
    name: 'UnitedHealth Group Inc.',
    exchange: 'NYSE',
    industry: 'Healthcare',
    sector: 'Health Insurance',
    market_cap: 5200000,
    pe_ratio: 24.7,
    roe: 25.3,
    list_date: '1984-10-17',
    description: 'UnitedHealth Group Incorporated operates as a diversified health care company in the United States.',
    website: 'https://www.unitedhealthgroup.com',
    employee_count: 400000,
    founded_date: '1977-01-01',
    address: '9900 Bren Road East, Minnetonka, MN 55343'
  },
  {
    symbol: 'JNJ',
    name: 'Johnson & Johnson',
    exchange: 'NYSE',
    industry: 'Healthcare',
    sector: 'Pharmaceuticals',
    market_cap: 4600000,
    pe_ratio: 15.2,
    roe: 18.7,
    list_date: '1944-09-25',
    description: 'Johnson & Johnson researches and develops, manufactures, and sells a range of products in the health care field worldwide.',
    website: 'https://www.jnj.com',
    employee_count: 152700,
    founded_date: '1886-01-01',
    address: 'One Johnson & Johnson Plaza, New Brunswick, NJ 08933'
  },
  {
    symbol: 'V',
    name: 'Visa Inc.',
    exchange: 'NYSE',
    industry: 'Financial',
    sector: 'Payment Processing',
    market_cap: 5100000,
    pe_ratio: 32.8,
    roe: 38.2,
    list_date: '2008-03-19',
    description: 'Visa Inc. operates as a payments technology company worldwide.',
    website: 'https://www.visa.com',
    employee_count: 26500,
    founded_date: '1958-09-18',
    address: '900 Metro Center Boulevard, Foster City, CA 94404'
  },
  {
    symbol: 'JPM',
    name: 'JPMorgan Chase & Co.',
    exchange: 'NYSE',
    industry: 'Financial',
    sector: 'Banking',
    market_cap: 4800000,
    pe_ratio: 12.3,
    roe: 15.4,
    list_date: '1969-03-05',
    description: 'JPMorgan Chase & Co. operates as a financial services company worldwide.',
    website: 'https://www.jpmorganchase.com',
    employee_count: 296877,
    founded_date: '1799-01-01',
    address: '383 Madison Avenue, New York, NY 10017'
  },
  {
    symbol: 'WMT',
    name: 'Walmart Inc.',
    exchange: 'NYSE',
    industry: 'Consumer Discretionary',
    sector: 'Retail',
    market_cap: 4300000,
    pe_ratio: 26.1,
    roe: 19.8,
    list_date: '1972-08-25',
    description: 'Walmart Inc. engages in the operation of retail, wholesale, and other units worldwide.',
    website: 'https://www.walmart.com',
    employee_count: 2300000,
    founded_date: '1962-07-02',
    address: '702 SW 8th Street, Bentonville, AR 72716'
  },
  {
    symbol: 'PG',
    name: 'Procter & Gamble Co.',
    exchange: 'NYSE',
    industry: 'Consumer Staples',
    sector: 'Personal Care',
    market_cap: 3700000,
    pe_ratio: 24.5,
    roe: 31.2,
    list_date: '1891-01-01',
    description: 'The Procter & Gamble Company provides branded consumer packaged goods to consumers in North and Latin America, Europe, the Asia Pacific, Greater China, India, the Middle East, and Africa.',
    website: 'https://www.pg.com',
    employee_count: 101000,
    founded_date: '1837-01-01',
    address: 'One Procter & Gamble Plaza, Cincinnati, OH 45202'
  },
  {
    symbol: 'MA',
    name: 'Mastercard Inc.',
    exchange: 'NYSE',
    industry: 'Financial',
    sector: 'Payment Processing',
    market_cap: 3900000,
    pe_ratio: 33.7,
    roe: 148.2,
    list_date: '2006-05-25',
    description: 'Mastercard Incorporated, a technology company, provides transaction processing and other payment-related products and services in the United States and internationally.',
    website: 'https://www.mastercard.com',
    employee_count: 24000,
    founded_date: '1966-01-01',
    address: '2000 Purchase Street, Purchase, NY 10577'
  }
] as any as UsStock[]

// A股市场摘要模拟数据
export const mockMarketSummary: MarketSummary = {
  trade_date: '2024-12-08',
  total_stocks: 5234,
  up_count: 2156,
  down_count: 2834,
  flat_count: 244,
  limit_up_count: 45,
  limit_down_count: 12,
  total_volume: 1234567890,
  total_amount: 987654321000,
  avg_pct_chg: -0.85
}

// A股指数模拟数据
export const mockIndexData: IndexData[] = [
  {
    ts_code: '000001.SH',
    name: '上证指数',
    trade_date: '2024-12-08',
    close: 3245.67,
    open: 3258.12,
    high: 3268.45,
    low: 3238.90,
    pre_close: 3258.12,
    change: -12.45,
    pct_chg: -0.38,
    vol: 234567890,
    amount: 345678901234
  },
  {
    ts_code: '399001.SZ',
    name: '深证成指',
    trade_date: '2024-12-08',
    close: 10456.78,
    open: 10523.45,
    high: 10567.89,
    low: 10398.12,
    pre_close: 10523.45,
    change: -66.67,
    pct_chg: -0.63,
    vol: 345678901,
    amount: 456789012345
  },
  {
    ts_code: '399006.SZ',
    name: '创业板指',
    trade_date: '2024-12-08',
    close: 2134.56,
    open: 2156.78,
    high: 2167.89,
    low: 2123.45,
    pre_close: 2156.78,
    change: -22.22,
    pct_chg: -1.03,
    vol: 456789012,
    amount: 567890123456
  }
]

// 成交量分布模拟数据
export const mockVolumeDistribution: VolumeDistribution = {
  trade_date: '2024-12-08',
  total_volume: 1234567890,
  total_stocks: 5234,
  top10_volume: 456789012,
  top10_percentage: 37.0,
  top30_volume: 678901234,
  top30_percentage: 55.0,
  top50_volume: 802345678,
  top50_percentage: 65.0,
  top100_volume: 925678901,
  top100_percentage: 75.0,
  concentration_index: 0.65, // CR指数，表示前N家企业的市场份额
  herfindahl_index: 0.08, // HHI指数，0.08表示市场较为分散
  gini_coefficient: 0.72 // 基尼系数，0.72表示成交量分布不均衡
}

// 股票涨跌幅分布模拟数据
export const mockDistributionData = [
  { range: '>9%', count: 45, percentage: 0.86 },
  { range: '7-9%', count: 123, percentage: 2.35 },
  { range: '5-7%', count: 234, percentage: 4.47 },
  { range: '3-5%', count: 456, percentage: 8.71 },
  { range: '1-3%', count: 789, percentage: 15.08 },
  { range: '0-1%', count: 509, percentage: 9.73 },
  { range: '0%', count: 244, percentage: 4.66 },
  { range: '0~-1%', count: 567, percentage: 10.84 },
  { range: '-1~-3%', count: 890, percentage: 17.01 },
  { range: '-3~-5%', count: 678, percentage: 12.95 },
  { range: '-5~-7%', count: 345, percentage: 6.59 },
  { range: '-7~-9%', count: 234, percentage: 4.47 },
  { range: '<-9%', count: 120, percentage: 2.29 }
]

// 股票详情模拟数据
export const mockStockDetail: StockDetail = {
  ts_code: '000001.SZ',
  name: '平安银行',
  current_price: 12.58,
  change: -0.15,
  pct_chg: -1.18,
  pe_ratio: 5.8,
  pb_ratio: 0.72,
  five_day_return: 2.35,
  fundamentals: {
    roe: 11.2,
    gross_margin: 68.5,
    net_margin: 32.1,
    debt_ratio: 92.3,
    current_ratio: 1.15,
    revenue_growth: 8.7,
    net_profit_growth: 12.4
  },
  block_trades: [
    {
      trade_date: '2024-12-08',
      price: 12.50,
      volume: 5000000,
      amount: 62500000,
      buyer: '机构专用',
      seller: '招商证券深圳益田路',
      premium_rate: -0.63
    },
    {
      trade_date: '2024-12-07',
      price: 12.75,
      volume: 3200000,
      amount: 40800000,
      buyer: '中信证券上海淮海中路',
      seller: '机构专用',
      premium_rate: 1.2
    },
    {
      trade_date: '2024-12-06',
      price: 12.60,
      volume: 8000000,
      amount: 100800000,
      buyer: '华泰证券深圳益田路荣超商务中心',
      seller: '国泰君安深圳益田路',
      premium_rate: -0.32
    }
  ],
  concepts: ['银行', '金融科技', 'MSCI中国', '深港通', '融资融券', '转融券标的', 'ESG概念'],
  sectors: ['银行', '金融服务', '商业银行'],
  shareholding_changes: [
    {
      holder_name: '中国平安保险(集团)股份有限公司',
      change_type: 'increase',
      change_shares: 50000000,
      change_ratio: 0.26,
      change_date: '2024-11-15',
      current_ratio: 56.72
    },
    {
      holder_name: '香港中央结算有限公司',
      change_type: 'decrease',
      change_shares: -12000000,
      change_ratio: -0.06,
      change_date: '2024-11-20',
      current_ratio: 8.45
    },
    {
      holder_name: '全国社保基金一零一组合',
      change_type: 'increase',
      change_shares: 8500000,
      change_ratio: 0.04,
      change_date: '2024-11-25',
      current_ratio: 1.23
    }
  ],
  margin_trading: {
    margin_balance: 1580000000,
    margin_buy: 45000000,
    short_balance: 12000000,
    short_sell: 850000,
    margin_ratio: 7.8
  },
  shareholder_count: {
    holder_count: 1256789,
    avg_holding: 15420,
    change_ratio: -2.3,
    report_date: '2024-09-30'
  }
}

// 股票搜索模拟数据
export const mockStockList = [
  { ts_code: '000001.SZ', name: '平安银行', market: 'SZ' },
  { ts_code: '000002.SZ', name: '万科A', market: 'SZ' },
  { ts_code: '000858.SZ', name: '五粮液', market: 'SZ' },
  { ts_code: '300750.SZ', name: '宁德时代', market: 'SZ' },
  { ts_code: '600036.SH', name: '招商银行', market: 'SH' },
  { ts_code: '600519.SH', name: '贵州茅台', market: 'SH' },
  { ts_code: '600887.SH', name: '伊利股份', market: 'SH' },
  { ts_code: '002415.SZ', name: '海康威视', market: 'SZ' },
  { ts_code: '000725.SZ', name: '京东方A', market: 'SZ' },
  { ts_code: '601318.SH', name: '中国平安', market: 'SH' },
  { ts_code: '600276.SH', name: '恒瑞医药', market: 'SH' },
  { ts_code: '000063.SZ', name: '中兴通讯', market: 'SZ' }
]

// 证券列表模拟数据
export const mockSecurities: Security[] = [
  // 股票
  { code: '000001.SZ', name: '平安银行', type: 'stock', market: 'SZ' },
  { code: '000002.SZ', name: '万科A', type: 'stock', market: 'SZ' },
  { code: '600036.SH', name: '招商银行', type: 'stock', market: 'SH' },
  { code: '600519.SH', name: '贵州茅台', type: 'stock', market: 'SH' },
  { code: '000858.SZ', name: '五粮液', type: 'stock', market: 'SZ' },
  { code: '300750.SZ', name: '宁德时代', type: 'stock', market: 'SZ' },
  
  // 指数
  { code: '000001.SH', name: '上证指数', type: 'index', market: 'SH' },
  { code: '399001.SZ', name: '深证成指', type: 'index', market: 'SZ' },
  { code: '399006.SZ', name: '创业板指', type: 'index', market: 'SZ' },
  { code: '000300.SH', name: '沪深300', type: 'index', market: 'SH' },
  
  // 基金
  { code: '110022.OF', name: '易方达消费行业', type: 'fund' },
  { code: '161725.OF', name: '招商中证白酒', type: 'fund' },
  { code: '000001.OF', name: '华夏成长混合', type: 'fund' },
  { code: '519674.OF', name: '银河创新成长', type: 'fund' }
]

// 策略模拟数据
export const mockStrategyStocks: StrategyStock[] = [
  {
    ts_code: '000001.SZ',
    name: '平安银行',
    current_price: 12.58,
    change_percent: 0.0235,
    signal: 'BUY',
    signal_strength: 0.85,
    updated_at: new Date().toISOString()
  },
  {
    ts_code: '000002.SZ',
    name: '万科A',
    current_price: 8.45,
    change_percent: -0.0123,
    signal: 'HOLD',
    signal_strength: 0.62,
    updated_at: new Date().toISOString()
  },
  {
    ts_code: '600036.SH',
    name: '招商银行',
    current_price: 38.76,
    change_percent: 0.0456,
    signal: 'BUY',
    signal_strength: 0.91,
    updated_at: new Date().toISOString()
  },
  {
    ts_code: '600519.SH',
    name: '贵州茅台',
    current_price: 1680.50,
    change_percent: 0.0189,
    signal: 'HOLD',
    signal_strength: 0.73,
    updated_at: new Date().toISOString()
  },
  {
    ts_code: '000858.SZ',
    name: '五粮液',
    current_price: 158.32,
    change_percent: 0.0298,
    signal: 'BUY',
    signal_strength: 0.78,
    updated_at: new Date().toISOString()
  },
  {
    ts_code: '300750.SZ',
    name: '宁德时代',
    current_price: 185.67,
    change_percent: -0.0267,
    signal: 'SELL',
    signal_strength: 0.68,
    updated_at: new Date().toISOString()
  }
]
