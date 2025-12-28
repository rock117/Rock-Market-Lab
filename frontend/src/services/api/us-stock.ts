// 美股相关 API
import { UsStock, UsStockMeta, PagedResponse, ApiResponse, ApiPagedData } from '@/types'
import { API_BASE_URL } from './config'
import { mockUsStocks } from './mock-data'

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

// 美股数据API
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

  // 获取美股列表（支持分页和筛选）
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
  },
  
  // 提供美股模拟数据（直接导出）
  getMockUsStocks: () => mockUsStocks
}
