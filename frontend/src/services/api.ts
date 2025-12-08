import axios from 'axios'
import { ApiResponse, PagedResponse, UsStock, UsCompanyInfo, MarketSummary, IndexData, StockDaily } from '@/types'

// 创建axios实例
const api = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    // 可以在这里添加认证token等
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    return response.data
  },
  (error) => {
    console.error('API Error:', error)
    return Promise.reject(error)
  }
)

// 美股相关API
export const usStockApi = {
  // 获取美股列表
  getUsStocks: async (params?: {
    page?: number
    page_size?: number
    exchange?: string
    industry?: string
  }): Promise<PagedResponse<UsStock>> => {
    const response = await api.get('/us-stocks', { params })
    return response
  },

  // 获取美股详情
  getUsStock: async (symbol: string): Promise<UsStock> => {
    const response = await api.get(`/us-stocks/${symbol}`)
    return response
  },

  // 获取美股公司信息
  getUsCompanyInfo: async (symbol: string): Promise<UsCompanyInfo> => {
    const response = await api.get(`/us-stocks/${symbol}/company-info`)
    return response
  },

  // 获取美股日线数据
  getUsDaily: async (symbol: string, params?: {
    start_date?: string
    end_date?: string
    limit?: number
  }) => {
    const response = await api.get(`/us-stocks/${symbol}/daily`, { params })
    return response
  },
}

// A股相关API
export const stockApi = {
  // 获取市场概览数据
  getMarketSummary: async (date?: string): Promise<MarketSummary> => {
    const response = await api.get('/market/summary', { 
      params: date ? { date } : {} 
    })
    return response
  },

  // 获取主要指数数据
  getIndexData: async (params?: {
    ts_codes?: string[]
    date?: string
  }): Promise<IndexData[]> => {
    const response = await api.get('/market/indices', { params })
    return response
  },

  // 获取涨跌分布数据
  getPriceDistribution: async (date?: string) => {
    const response = await api.get('/market/price-distribution', {
      params: date ? { date } : {}
    })
    return response
  },

  // 获取股票日线数据
  getStockDaily: async (ts_code: string, params?: {
    start_date?: string
    end_date?: string
    limit?: number
  }): Promise<StockDaily[]> => {
    const response = await api.get(`/stocks/${ts_code}/daily`, { params })
    return response
  },

  // 获取均线数据
  getMovingAverages: async (ts_code: string, date?: string) => {
    const response = await api.get(`/stocks/${ts_code}/ma`, {
      params: date ? { date } : {}
    })
    return response
  },
}

export default api
