import { PagedResponse, UsStock, MarketSummary, IndexData, StockDetail, Security, SecurityKLineData, KLineData, SecuritySearchResult, TrendAnalysis } from '@/types'

// 模拟延迟
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

// 美股模拟数据
const mockUsStocks: UsStock[] = [
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
  }
]

// A股市场模拟数据
const mockMarketSummary: MarketSummary = {
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

const mockIndexData: IndexData[] = [
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

const mockDistributionData = [
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

// 美股相关API（使用假数据）
export const usStockApi = {
  getUsStocks: async (params?: {
    page?: number
    page_size?: number
    exchange?: string
    industry?: string
  }): Promise<PagedResponse<UsStock>> => {
    await delay(500) // 模拟网络延迟
    
    let filteredStocks = [...mockUsStocks]
    
    // 简单筛选逻辑
    if (params?.exchange) {
      filteredStocks = filteredStocks.filter(stock => stock.exchange === params.exchange)
    }
    if (params?.industry) {
      filteredStocks = filteredStocks.filter(stock => stock.industry === params.industry)
    }
    
    return {
      items: filteredStocks,
      total: filteredStocks.length,
      page: params?.page || 1,
      page_size: params?.page_size || 50,
      total_pages: 1
    }
  }
}

// A股相关API（使用假数据）
export const stockApi = {
  getMarketSummary: async (): Promise<MarketSummary> => {
    await delay(300)
    return mockMarketSummary
  },

  getIndexData: async (): Promise<IndexData[]> => {
    await delay(300)
    return mockIndexData
  },

  getPriceDistribution: async () => {
    await delay(300)
    return mockDistributionData
  }
}

// 股票详情模拟数据
const mockStockDetail: StockDetail = {
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

// 股票详情API
export const stockDetailApi = {
  getStockDetail: async (ts_code: string): Promise<StockDetail> => {
    await delay(400)
    // 根据不同股票代码返回不同数据，这里简化为同一个模拟数据
    return {
      ...mockStockDetail,
      ts_code,
      name: ts_code === '000001.SZ' ? '平安银行' : 
            ts_code === '000002.SZ' ? '万科A' :
            ts_code === '600036.SH' ? '招商银行' : '示例股票'
    }
  }
}

// 生成模拟K线数据的工具函数
function generateKLineData(startPrice: number, days: number = 60): KLineData[] {
  const data: KLineData[] = []
  let currentPrice = startPrice
  const startDate = new Date()
  startDate.setDate(startDate.getDate() - days)
  
  for (let i = 0; i < days; i++) {
    const date = new Date(startDate)
    date.setDate(date.getDate() + i)
    
    // 模拟价格波动
    const volatility = 0.03 // 3%波动率
    const change = (Math.random() - 0.5) * 2 * volatility
    const open = currentPrice
    const close = open * (1 + change)
    const high = Math.max(open, close) * (1 + Math.random() * 0.02)
    const low = Math.min(open, close) * (1 - Math.random() * 0.02)
    const volume = Math.floor(Math.random() * 10000000) + 1000000
    
    data.push({
      date: date.toISOString().split('T')[0],
      open: Number(open.toFixed(2)),
      high: Number(high.toFixed(2)),
      low: Number(low.toFixed(2)),
      close: Number(close.toFixed(2)),
      volume,
      amount: volume * close
    })
    
    currentPrice = close
  }
  
  return data
}

// 证券列表模拟数据
const mockSecurities: Security[] = [
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

// K线图表颜色配置
const chartColors = [
  '#3b82f6', // 蓝色
  '#ef4444', // 红色
  '#10b981', // 绿色
  '#f59e0b', // 橙色
  '#8b5cf6', // 紫色
  '#06b6d4', // 青色
  '#84cc16', // 柠檬绿
  '#f97316'  // 橙红色
]

// K线比较API
export const klineApi = {
  // 搜索证券
  searchSecurities: async (keyword: string): Promise<SecuritySearchResult> => {
    await delay(200)
    const filtered = mockSecurities.filter(security => 
      security.code.toLowerCase().includes(keyword.toLowerCase()) ||
      security.name.toLowerCase().includes(keyword.toLowerCase())
    )
    return { securities: filtered.slice(0, 10) }
  },

  // 获取证券K线数据
  getSecurityKLineData: async (securities: Security[]): Promise<SecurityKLineData[]> => {
    await delay(600)
    
    return securities.map((security, index) => {
      // 根据证券类型设置不同的起始价格
      let startPrice = 10
      if (security.type === 'stock') {
        startPrice = security.code.includes('600519') ? 1800 : // 贵州茅台
                    security.code.includes('000858') ? 150 :  // 五粮液
                    security.code.includes('300750') ? 200 :  // 宁德时代
                    12 // 其他股票
      } else if (security.type === 'index') {
        startPrice = security.code.includes('000001.SH') ? 3200 : // 上证指数
                    security.code.includes('399001') ? 10400 :    // 深证成指
                    security.code.includes('399006') ? 2100 :     // 创业板指
                    4800 // 沪深300
      } else if (security.type === 'fund') {
        startPrice = Math.random() * 2 + 1 // 基金净值1-3之间
      }
      
      return {
        security,
        data: generateKLineData(startPrice, 60),
        color: chartColors[index % chartColors.length]
      }
    })
  },

  // 分析趋势相关性
  analyzeTrendCorrelation: async (securities: Security[]): Promise<TrendAnalysis> => {
    await delay(300)
    
    // 模拟相关性分析结果
    const correlation = Math.random() * 2 - 1 // -1 到 1
    const absCorrelation = Math.abs(correlation)
    
    let trend_consistency: 'high' | 'medium' | 'low'
    if (absCorrelation > 0.7) trend_consistency = 'high'
    else if (absCorrelation > 0.4) trend_consistency = 'medium'
    else trend_consistency = 'low'
    
    const sync_rate = absCorrelation * 100
    
    return {
      correlation: Number(correlation.toFixed(3)),
      trend_consistency,
      sync_rate: Number(sync_rate.toFixed(1)),
      analysis_period: '近60个交易日'
    }
  }
}
