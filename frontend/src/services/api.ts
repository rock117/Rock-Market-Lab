import { PagedResponse, UsStock, UsStockMeta, MarketSummary, IndexData, VolumeDistribution, StockDetail, Security, SecurityKLineData, KLineData, SecuritySearchResult, TrendAnalysis, StrategyResult, StrategyType, StrategyStock, StrategyPerformance, StockHistoryData, StockHistoryResponse, ApiResponse, ApiPagedData, Portfolio, PortfolioStock, ApiPortfolio, ApiPortfolioDetail, ApiHolding, ExchangeCode, MarginTradingKLineRequest, MarginTradingKLineResponse, StockMarginTradingKLineRequest, StockMarginTradingKLineResponse } from '@/types'

// 模拟延迟
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

// 美股模拟数据
const mockUsStocks = [
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
] as any as UsStock[];

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

// API基础配置 - 使用相对路径，通过Next.js rewrites转发
const API_BASE_URL = ''

// 数据转换函数：将API数据转换为前端格式
function transformUsStock(apiStock: any): UsStock {
  return {
    // 新API字段
    tsCode: apiStock.tsCode,
    name: apiStock.name,
    exchangeId: apiStock.exchangeId,
    businessDescription: apiStock.businessDescription,
    businessDescriptionCn: apiStock.businessDescriptionCn,
    businessCountry: apiStock.businessCountry,
    sectorName: apiStock.sectorName,
    sectorNameCn: apiStock.sectorNameCn,
    industryName: apiStock.industryName,
    industryNameCn: apiStock.industryNameCn,
    webAddress: apiStock.webAddress,
    // 兼容字段映射
    symbol: apiStock.tsCode,
    exchange: apiStock.exchangeId,
    industry: apiStock.industryName,
    sector: apiStock.sectorName,
    description: apiStock.businessDescription,
    website: apiStock.webAddress,
  }
}

// 美股相关API（使用真实数据）
export const usStockApi = {
  // 获取美股元数据（板块和行业）
  getUsStockMeta: async (): Promise<UsStockMeta> => {
    try {
      const response = await fetch(`${API_BASE_URL}/api/us-company/meta`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      
      const apiResponse: { data: UsStockMeta; success: boolean } = await response.json()
      
      if (!apiResponse.success) {
        throw new Error('API returned unsuccessful response')
      }
      
      return apiResponse.data
    } catch (error) {
      console.error('Error fetching US stock meta:', error)
      // 如果API调用失败，返回空数据
      return {
        industries: [],
        sectors: []
      }
    }
  },

  getUsStocks: async (params?: {
    page?: number
    page_size?: number
    keyword?: string
    sector?: string
    industry?: string
  }): Promise<PagedResponse<UsStock>> => {
    try {
      // 构建查询参数
      const queryParams = new URLSearchParams()
      if (params?.page) queryParams.append('page', params.page.toString())
      if (params?.page_size) queryParams.append('page_size', params.page_size.toString())
      if (params?.keyword) queryParams.append('keyword', params.keyword)
      if (params?.sector) queryParams.append('sector', params.sector)
      if (params?.industry) queryParams.append('industry', params.industry)
      
      const url = `${API_BASE_URL}/api/us-stocks?${queryParams.toString()}`
      
      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      
      const apiResponse: ApiResponse<ApiPagedData<any>> = await response.json()
      
      if (!apiResponse.success) {
        throw new Error('API returned unsuccessful response')
      }
      
      const { data } = apiResponse.data
      const transformedStocks = data.map(transformUsStock)
      
      return {
        items: transformedStocks,
        total: apiResponse.data.total,
        page: apiResponse.data.page,
        page_size: apiResponse.data.page_size,
        total_pages: apiResponse.data.total_pages
      }
    } catch (error) {
      console.error('Error fetching US stocks:', error)
      // 如果API调用失败，返回空结果
      return {
        items: [],
        total: 0,
        page: params?.page || 1,
        page_size: params?.page_size || 20,
        total_pages: 0
      }
    }
  }
}

// 成交量分布模拟数据
const mockVolumeDistribution: VolumeDistribution = {
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
  },

  getVolumeDistribution: async (): Promise<VolumeDistribution> => {
    await delay(300)
    return mockVolumeDistribution
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

// 股票搜索模拟数据
const mockStockList = [
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

// 股票详情API
export const stockDetailApi = {
  // 搜索股票
  searchStocks: async (keyword: string) => {
    if (!keyword || keyword.length < 1) {
      return { stocks: [] as Array<{ ts_code: string; name: string }> }
    }

    const query = new URLSearchParams()
    query.set('keyword', keyword)

    const response = await fetch(`${API_BASE_URL}/api/stocks/search?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    
    // 检查 API 是否返回错误格式 {data: string, success: false}
    if (raw?.success === false) {
      throw new Error(raw?.data || '搜索股票失败')
    }

    const rows: Array<{ ts_code: string; name: string }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : raw?.ts_code && raw?.name
          ? [raw]
          : []

    return {
      stocks: rows.filter(r => r?.ts_code && r?.name),
    }
  },

  // 获取股票详情
  getStockDetail: async (ts_code: string): Promise<StockDetail> => {
    if (!ts_code) {
      throw new Error('股票代码不能为空')
    }

    try {
      const response = await fetch(`${API_BASE_URL}/api/stocks/detail?ts_code=${ts_code}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })

      if (response.ok) {
        const raw = await response.json()
        
        // 检查 API 是否返回错误格式 {data: string, success: false}
        if (raw?.success === false) {
          throw new Error(raw?.data || '获取股票详情失败')
        }

        // 如果成功获取真实数据，直接返回
        if (raw && typeof raw === 'object') {
          return raw as StockDetail
        }
      }
    } catch (error) {
      // 如果 API 调用失败，继续使用模拟数据
      if (error instanceof Error && error.message.includes('success')) {
        throw error
      }
    }

    // 降级到模拟数据
    await delay(400)
    
    // 根据不同股票代码返回不同数据
    const stockInfo = mockStockList.find(stock => stock.ts_code === ts_code)
    const stockName = stockInfo?.name || '示例股票'
    
    // 为不同股票生成不同的模拟数据
    const basePrice = ts_code === '600519.SH' ? 1680 : // 贵州茅台
                     ts_code === '300750.SZ' ? 185 :   // 宁德时代
                     ts_code === '000858.SZ' ? 158 :   // 五粮液
                     ts_code === '600036.SH' ? 38 :    // 招商银行
                     ts_code === '000002.SZ' ? 8.5 :   // 万科A
                     12.58 // 平安银行等其他
    
    const changePercent = (Math.random() - 0.5) * 6 // -3% 到 3%
    const change = basePrice * (changePercent / 100)
    
    return {
      ...mockStockDetail,
      ts_code,
      name: stockName,
      current_price: Number((basePrice + change).toFixed(2)),
      change: Number(change.toFixed(2)),
      pct_chg: Number(changePercent.toFixed(2)),
      pe_ratio: Number((Math.random() * 30 + 5).toFixed(1)),
      pb_ratio: Number((Math.random() * 3 + 0.5).toFixed(2)),
      five_day_return: Number((Math.random() * 10 - 5).toFixed(2))
    }
  },

  // 获取股票历史价格
  getStockHistory: async (
    ts_code: string,
    params: {
      timeMode: 'custom' | 'quick'
      startDate?: string
      endDate?: string
      timePeriod?: string
    }
  ): Promise<StockHistoryResponse> => {
    const normalizeDate = (input: string): string => {
      const raw = String(input).trim()
      if (!raw) return raw

      if (/^\d{8}$/.test(raw)) {
        const y = raw.slice(0, 4)
        const m = raw.slice(4, 6)
        const d = raw.slice(6, 8)
        return `${y}-${m}-${d}`
      }

      if (/^\d{4}-\d{2}-\d{2}$/.test(raw)) return raw

      const t = Date.parse(raw)
      if (Number.isFinite(t)) {
        const dt = new Date(t)
        const y = dt.getFullYear()
        const m = String(dt.getMonth() + 1).padStart(2, '0')
        const d = String(dt.getDate()).padStart(2, '0')
        return `${y}-${m}-${d}`
      }

      return raw
    }

    const query = new URLSearchParams()
    query.set('ts_code', ts_code)
    if (params.timeMode === 'custom') {
      if (params.startDate) query.set('start_date', params.startDate)
      if (params.endDate) query.set('end_date', params.endDate)
    } else {
      if (params.timePeriod) query.set('time_period', params.timePeriod)
    }

    const response = await fetch(`${API_BASE_URL}/api/stocks/history?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    
    // 检查 API 是否返回错误格式 {data: string, success: false}
    if (raw?.success === false) {
      throw new Error(raw?.data || '获取历史数据失败')
    }

    const rows: Array<{
      open?: string | number
      high?: string | number
      low?: string | number
      close?: string | number
      amount?: string | number
      vol?: string | number
      volume?: string | number
      pct_chg?: string | number
      pctChg?: string | number
      date: string
      turnover_rate?: string | number
      turnoverRate?: string | number
    }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const historyData: StockHistoryData[] = rows
      .filter(r => r?.date)
      .map(r => {
        const open = Number(r.open)
        const high = Number(r.high)
        const low = Number(r.low)
        const close = Number(r.close)
        const amount = Number(r.amount ?? r.vol ?? r.volume)
        const pctChg = Number(r.pct_chg ?? r.pctChg)
        const turnoverRate = Number(r.turnover_rate ?? r.turnoverRate)

        return {
          trade_date: normalizeDate(r.date),
          open: Number.isFinite(open) ? open : 0,
          high: Number.isFinite(high) ? high : 0,
          low: Number.isFinite(low) ? low : 0,
          close: Number.isFinite(close) ? close : 0,
          volume: 0,
          amount: Number.isFinite(amount) ? amount : 0,
          turnover_rate: Number.isFinite(turnoverRate) ? turnoverRate : 0,
          pct_chg: Number.isFinite(pctChg) ? pctChg : 0,
          change: Number.isFinite(open) && Number.isFinite(close) ? Number((close - open).toFixed(2)) : 0,
        }
      })

    historyData.sort((a, b) => new Date(b.trade_date).getTime() - new Date(a.trade_date).getTime())

    return {
      ts_code,
      name: '',
      data: historyData,
      total: historyData.length,
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
  getSecurityKLineData: async (
    securities: Security[], 
    options?: {
      startDate?: string
      endDate?: string
      period?: 'daily' | 'weekly' | 'monthly'
    }
  ): Promise<SecurityKLineData[]> => {
    await delay(600)
    
    // 计算数据天数
    const startDate = options?.startDate ? new Date(options.startDate) : new Date(Date.now() - 90 * 24 * 60 * 60 * 1000)
    const endDate = options?.endDate ? new Date(options.endDate) : new Date()
    const daysDiff = Math.ceil((endDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24))
    
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
        data: generateKLineDataWithDateRange(startPrice, startDate, endDate, options?.period || 'daily'),
        color: chartColors[index % chartColors.length]
      }
    })
  },

  // 分析趋势相关性
  analyzeTrendCorrelation: async (
    securities: Security[],
    options?: {
      startDate?: string
      endDate?: string
      period?: 'daily' | 'weekly' | 'monthly'
    }
  ): Promise<TrendAnalysis> => {
    await delay(300)
    
    // 模拟相关性分析结果
    const correlation = Math.random() * 2 - 1 // -1 到 1
    const absCorrelation = Math.abs(correlation)
    
    let trend_consistency: 'high' | 'medium' | 'low'
    if (absCorrelation > 0.7) trend_consistency = 'high'
    else if (absCorrelation > 0.4) trend_consistency = 'medium'
    else trend_consistency = 'low'
    
    const sync_rate = absCorrelation * 100
    
    // 根据时间周期生成分析周期描述
    const startDate = options?.startDate ? new Date(options.startDate) : new Date(Date.now() - 90 * 24 * 60 * 60 * 1000)
    const endDate = options?.endDate ? new Date(options.endDate) : new Date()
    const daysDiff = Math.ceil((endDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24))
    
    let periodDesc = ''
    if (options?.period === 'weekly') {
      periodDesc = `近${Math.ceil(daysDiff / 7)}周`
    } else if (options?.period === 'monthly') {
      periodDesc = `近${Math.ceil(daysDiff / 30)}个月`
    } else {
      periodDesc = `近${daysDiff}个交易日`
    }
    
    return {
      correlation: Number(correlation.toFixed(3)),
      trend_consistency,
      sync_rate: Number(sync_rate.toFixed(1)),
      analysis_period: periodDesc
    }
  }
}

export const marginTradingApi = {
  getMarginTradingKLine: async (request: MarginTradingKLineRequest): Promise<MarginTradingKLineResponse> => {
    const normalizeDate = (input: string): string => {
      const raw = String(input).trim()
      if (!raw) return raw

      if (/^\d{8}$/.test(raw)) {
        const y = raw.slice(0, 4)
        const m = raw.slice(4, 6)
        const d = raw.slice(6, 8)
        return `${y}-${m}-${d}`
      }

      if (/^\d{4}-\d{2}-\d{2}$/.test(raw)) return raw

      const t = Date.parse(raw)
      if (Number.isFinite(t)) {
        const dt = new Date(t)
        const y = dt.getFullYear()
        const m = String(dt.getMonth() + 1).padStart(2, '0')
        const d = String(dt.getDate()).padStart(2, '0')
        return `${y}-${m}-${d}`
      }

      return raw
    }

    const query = new URLSearchParams()
    query.set('exchange', request.exchange)
    query.set('start_date', request.startDate)
    query.set('end_date', request.endDate)

    const response = await fetch(`${API_BASE_URL}/api/margin/balance?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    const rows: Array<{ date: string; marginBalance: string | number }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const toYiYuan = (v: number): number => Number((v / 100000000).toFixed(2))

    const sorted = rows
      .filter(r => r?.date)
      .map(r => ({
        date: normalizeDate(r.date),
        marginBalance: typeof r.marginBalance === 'number' ? r.marginBalance : Number(r.marginBalance),
      }))
      .filter(r => Number.isFinite(r.marginBalance))
      .map(r => ({
        ...r,
        marginBalance: toYiYuan(r.marginBalance),
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

    const data: StockHistoryData[] = sorted.map((r, idx) => {
      const close = r.marginBalance
      const open = idx === 0 ? close : sorted[idx - 1].marginBalance
      const high = Math.max(open, close)
      const low = Math.min(open, close)

      const change = close - open
      const pctChg = open === 0 ? 0 : (change / open) * 100

      return {
        trade_date: r.date,
        open: Number(open.toFixed(2)),
        high: Number(high.toFixed(2)),
        low: Number(low.toFixed(2)),
        close: Number(close.toFixed(2)),
        volume: 0,
        amount: 0,
        turnover_rate: 0,
        pct_chg: Number(pctChg.toFixed(2)),
        change: Number(change.toFixed(2)),
      }
    })

    return {
      exchange: request.exchange,
      startDate: request.startDate,
      endDate: request.endDate,
      data,
    }
  },

  getStockMarginTradingKLine: async (request: StockMarginTradingKLineRequest): Promise<StockMarginTradingKLineResponse> => {
    const normalizeDate = (input: string): string => {
      const raw = String(input).trim()
      if (!raw) return raw

      if (/^\d{8}$/.test(raw)) {
        const y = raw.slice(0, 4)
        const m = raw.slice(4, 6)
        const d = raw.slice(6, 8)
        return `${y}-${m}-${d}`
      }

      if (/^\d{4}-\d{2}-\d{2}$/.test(raw)) return raw

      const t = Date.parse(raw)
      if (Number.isFinite(t)) {
        const dt = new Date(t)
        const y = dt.getFullYear()
        const m = String(dt.getMonth() + 1).padStart(2, '0')
        const d = String(dt.getDate()).padStart(2, '0')
        return `${y}-${m}-${d}`
      }

      return raw
    }

    const query = new URLSearchParams()
    query.set('stock', request.stock)
    query.set('start_date', request.startDate)
    query.set('end_date', request.endDate)

    const response = await fetch(`${API_BASE_URL}/api/margin/balance?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    const rows: Array<{ date: string; marginBalance: string | number }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const toWanYuan = (v: number): number => Number((v / 10000).toFixed(2))

    const sorted = rows
      .filter(r => r?.date)
      .map(r => ({
        date: normalizeDate(r.date),
        marginBalance: typeof r.marginBalance === 'number' ? r.marginBalance : Number(r.marginBalance),
      }))
      .filter(r => Number.isFinite(r.marginBalance))
      .map(r => ({
        ...r,
        marginBalance: toWanYuan(r.marginBalance),
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

    const data: StockHistoryData[] = sorted.map((r, idx) => {
      const close = r.marginBalance
      const open = idx === 0 ? close : sorted[idx - 1].marginBalance
      const high = Math.max(open, close)
      const low = Math.min(open, close)

      const change = close - open
      const pctChg = open === 0 ? 0 : (change / open) * 100

      return {
        trade_date: r.date,
        open: Number(open.toFixed(2)),
        high: Number(high.toFixed(2)),
        low: Number(low.toFixed(2)),
        close: Number(close.toFixed(2)),
        volume: 0,
        amount: 0,
        turnover_rate: 0,
        pct_chg: Number(pctChg.toFixed(2)),
        change: Number(change.toFixed(2)),
      }
    })

    return {
      stock: request.stock,
      startDate: request.startDate,
      endDate: request.endDate,
      data,
    }
  },
}

// 根据日期范围和周期生成K线数据
function generateKLineDataWithDateRange(
  startPrice: number, 
  startDate: Date, 
  endDate: Date, 
  period: 'daily' | 'weekly' | 'monthly'
): KLineData[] {
  const data: KLineData[] = []
  let currentPrice = startPrice
  let currentDate = new Date(startDate)
  
  // 根据周期设置日期增量
  const getNextDate = (date: Date, period: string): Date => {
    const nextDate = new Date(date)
    switch (period) {
      case 'weekly':
        nextDate.setDate(nextDate.getDate() + 7)
        break
      case 'monthly':
        nextDate.setMonth(nextDate.getMonth() + 1)
        break
      default: // daily
        nextDate.setDate(nextDate.getDate() + 1)
        break
    }
    return nextDate
  }
  
  // 根据周期调整波动率
  const getVolatility = (period: string): number => {
    switch (period) {
      case 'weekly':
        return 0.06 // 6%波动率
      case 'monthly':
        return 0.12 // 12%波动率
      default: // daily
        return 0.03 // 3%波动率
    }
  }
  
  const volatility = getVolatility(period)
  
  while (currentDate <= endDate) {
    // 模拟价格波动
    const change = (Math.random() - 0.5) * 2 * volatility
    const open = currentPrice
    const close = currentPrice * (1 + change)
    const high = Math.max(open, close) * (1 + Math.random() * 0.02)
    const low = Math.min(open, close) * (1 - Math.random() * 0.02)
    const volume = Math.floor(Math.random() * 1000000 + 500000)
    
    data.push({
      date: currentDate.toISOString().split('T')[0],
      open: Number(open.toFixed(2)),
      high: Number(high.toFixed(2)),
      low: Number(low.toFixed(2)),
      close: Number(close.toFixed(2)),
      volume
    })
    
    currentPrice = close
    currentDate = getNextDate(currentDate, period)
  }
  
  return data
}

// 策略模拟数据
const mockStrategyStocks: StrategyStock[] = [
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

// 策略API
export const strategyApi = {
  // 运行策略
  runStrategy: async (strategyType: StrategyType, parameters: Record<string, any>) => {
    const response = await fetch('/api/stocks/pick', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        strategy: strategyType,
        settings: parameters
      })
    })
    
    const result = await response.json()
    
    // 检查业务逻辑错误
    if (!result.success) {
      throw new Error(result.data || '策略运行失败')
    }
    
    // 检查HTTP错误
    if (!response.ok) {
      throw new Error('策略运行失败')
    }
    
    return result
  },

  // 获取策略列表
  getStrategies: async () => {
    await delay(300)
    return [
      {
        strategy_type: 'price_volume_candlestick' as StrategyType,
        parameters: { volume_threshold: 1.5, price_change_threshold: 0.03 },
        enabled: true,
        description: '基于价格和成交量的K线形态分析'
      },
      {
        strategy_type: 'fundamental' as StrategyType,
        parameters: { min_roe: 0.15, max_pe: 25 },
        enabled: true,
        description: '基于财务指标的价值投资策略'
      },
      {
        strategy_type: 'turtle' as StrategyType,
        parameters: { entry_period: 20, exit_period: 10 },
        enabled: true,
        description: '经典的趋势跟踪策略'
      }
    ]
  }
}

// 数据转换函数：将后端API格式转换为前端格式
function transformApiHolding(apiHolding: ApiHolding): PortfolioStock {
  return {
    id: apiHolding.id.toString(),
    symbol: apiHolding.symbol,
    name: apiHolding.name,
    exchange_id: apiHolding.exchange_id,
    portfolio_id: apiHolding.portfolio_id.toString(),
    desc: apiHolding.desc,
    added_date: apiHolding.added_date || new Date().toISOString()
  }
}

// 转换列表接口返回的投资组合（不包含holdings）
function transformApiPortfolio(apiPortfolio: ApiPortfolio): Portfolio {
  return {
    id: apiPortfolio.id.toString(),
    name: apiPortfolio.name,
    description: apiPortfolio.description,
    created_date: apiPortfolio.created_date || new Date().toISOString(),
    updated_date: apiPortfolio.updated_date || new Date().toISOString(),
    stocks: [], // 列表接口不返回holdings，初始化为空数组
    holdings_num: apiPortfolio.holdings_num // 保留持仓数量
  }
}

// 转换详情接口返回的投资组合（包含holdings）
function transformApiPortfolioDetail(apiPortfolio: ApiPortfolioDetail): Portfolio {
  return {
    id: apiPortfolio.id.toString(),
    name: apiPortfolio.name,
    description: apiPortfolio.description,
    created_date: apiPortfolio.created_date || new Date().toISOString(),
    updated_date: apiPortfolio.updated_date || new Date().toISOString(),
    stocks: apiPortfolio.holdings ? apiPortfolio.holdings.map(transformApiHolding) : []
  }
}

// 投资组合API（使用真实API）
export const portfolioApi = {
  // 获取所有投资组合
  getPortfolios: async (): Promise<Portfolio[]> => {
    try {
      console.log('📊 正在获取投资组合列表...')
      const response = await fetch(`${API_BASE_URL}/api/portfolios`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })
      
      console.log('📊 API响应状态:', response.status)
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      
      const apiResponse: { data: ApiPortfolio[]; success: boolean } = await response.json()
      console.log('📊 API返回数据:', apiResponse)
      
      if (!apiResponse.success) {
        throw new Error('API returned unsuccessful response')
      }
      
      // 转换为前端格式
      const portfolios = apiResponse.data.map(transformApiPortfolio)
      console.log('📊 转换后的投资组合:', portfolios)
      return portfolios
    } catch (error) {
      console.error('❌ 获取投资组合失败:', error)
      return []
    }
  },

  // 获取单个投资组合（包含持仓列表）
  getPortfolio: async (id: string): Promise<Portfolio | null> => {
    try {
      console.log('📊 正在获取投资组合详情:', id)
      
      // 1. 先获取投资组合基本信息（从列表中找到）
      const portfolios = await portfolioApi.getPortfolios()
      const portfolioInfo = portfolios.find(p => p.id === id)
      
      if (!portfolioInfo) {
        console.error('❌ 投资组合不存在:', id)
        return null
      }
      
      // 2. 获取持仓列表
      console.log('📊 正在获取持仓列表:', id)
      const holdingsResponse = await fetch(`${API_BASE_URL}/api/portfolios/${id}/holdings`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })
      
      if (!holdingsResponse.ok) {
        throw new Error(`HTTP错误! 状态码: ${holdingsResponse.status}`)
      }
      
      const holdingsApiResponse: { data: ApiHolding[]; success: boolean } = await holdingsResponse.json()
      console.log('📊 持仓列表数据:', holdingsApiResponse)
      
      if (!holdingsApiResponse.success) {
        throw new Error('获取持仓列表失败')
      }
      
      // 3. 合并数据
      const portfolio: Portfolio = {
        ...portfolioInfo,
        stocks: holdingsApiResponse.data.map(transformApiHolding)
      }
      
      console.log('📊 完整的投资组合数据:', portfolio)
      return portfolio
    } catch (error) {
      console.error('❌ 获取投资组合详情失败:', error)
      return null
    }
  },

  // 创建投资组合
  createPortfolio: async (name: string, description?: string): Promise<Portfolio> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ name, description }),
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      // 如果success为false，抛出错误信息
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '创建投资组合失败'
      throw new Error(errorMessage)
    }
    
    // 创建成功后返回的是列表格式（包含holdings_num），使用列表转换函数
    return transformApiPortfolio(apiResponse.data as ApiPortfolio)
  },

  // 更新投资组合
  updatePortfolio: async (id: string, updates: { name?: string; description?: string }): Promise<Portfolio> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(updates),
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '更新投资组合失败'
      throw new Error(errorMessage)
    }
    
    // 更新成功后返回的是列表格式
    return transformApiPortfolio(apiResponse.data as ApiPortfolio)
  },

  // 删除投资组合
  deletePortfolio: async (id: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios/${id}`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json',
      },
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '删除投资组合失败'
      throw new Error(errorMessage)
    }
  },

  // 添加股票到组合
  addStock: async (portfolioId: string, stock: Omit<PortfolioStock, 'id' | 'added_date' | 'portfolio_id' | 'name'>): Promise<Portfolio> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios/${portfolioId}/holdings`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        symbol: stock.symbol,
        exchange_id: stock.exchange_id,
        desc: stock.desc,
      }),
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '添加股票失败'
      throw new Error(errorMessage)
    }
    
    // 添加成功后，重新获取完整的投资组合数据
    const portfolio = await portfolioApi.getPortfolio(portfolioId)
    if (!portfolio) {
      throw new Error('投资组合不存在')
    }
    
    return portfolio
  },

  // 从组合中删除股票
  removeStock: async (portfolioId: string, stockId: string): Promise<Portfolio> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios/${portfolioId}/holdings/${stockId}`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json',
      },
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '删除股票失败'
      throw new Error(errorMessage)
    }
    
    // 删除成功后，重新获取完整的投资组合数据
    const portfolio = await portfolioApi.getPortfolio(portfolioId)
    if (!portfolio) {
      throw new Error('投资组合不存在')
    }
    
    return portfolio
  },

  // 更新持仓描述
  updateStock: async (portfolioId: string, stockId: string, desc: string): Promise<Portfolio> => {
    const response = await fetch(`${API_BASE_URL}/api/portfolios/${portfolioId}/holdings/${stockId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ desc }),
    })
    
    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }
    
    const apiResponse: { data: any; success: boolean } = await response.json()
    
    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' 
        ? apiResponse.data 
        : '更新描述失败'
      throw new Error(errorMessage)
    }
    
    // 更新成功后，重新获取完整的投资组合数据
    const portfolio = await portfolioApi.getPortfolio(portfolioId)
    if (!portfolio) {
      throw new Error('投资组合不存在')
    }
    
    return portfolio
  }
}
