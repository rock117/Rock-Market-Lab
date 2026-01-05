// 投资组合相关 API
import { 
  Portfolio, 
  PortfolioStock, 
  ApiPortfolio, 
  ApiPortfolioDetail, 
  ApiHolding 
} from '@/types'
import { API_BASE_URL } from './config'

// 数据转换函数：将后端API格式转换为前端格式
function transformApiHolding(apiHolding: ApiHolding): PortfolioStock {
  return {
    id: apiHolding.id.toString(),
    symbol: apiHolding.symbol,
    name: apiHolding.name,
    exchange_id: apiHolding.exchange_id,
    portfolio_id: apiHolding.portfolio_id.toString(),
    desc: apiHolding.desc,
    added_date: apiHolding.added_date || new Date().toISOString(),
    current_price: apiHolding.current_price ?? null,
    pct_chg: apiHolding.pct_chg ?? null,
    pct5: apiHolding.pct5 ?? null,
    pct10: apiHolding.pct10 ?? null,
    pct20: apiHolding.pct20 ?? null,
    pct60: apiHolding.pct60 ?? null,
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
    const payload: { symbol: string; exchange_id?: string; desc?: string } = {
      symbol: stock.symbol,
      desc: stock.desc,
    }

    if (stock.exchange_id) {
      payload.exchange_id = stock.exchange_id
    }

    const response = await fetch(`${API_BASE_URL}/api/portfolios/${portfolioId}/holdings`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
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
