// A股市场相关 API
import type { AStockOverview, ApiPagedData, ApiResponse, IndexData, MarketSummary, PagedResponse, VolumeDistribution } from '@/types'
import { API_BASE_URL } from './config'
import { delay } from './config'
import { mockMarketSummary, mockIndexData, mockDistributionData, mockVolumeDistribution } from './mock-data'

// A股市场摘要相关API（使用模拟数据）
export const stockApi = {
  // 获取全量A股列表（用于前端本地排序/分页）
  getAllAStocks: async (): Promise<AStockOverview[]> => {
    const resp = await fetch(`${API_BASE_URL}/api/a-stocks`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!resp.ok) {
      throw new Error(`HTTP error! status: ${resp.status}`)
    }

    const raw: ApiResponse<AStockOverview[]> = await resp.json()
    if (raw?.success === false) {
      throw new Error((raw as any)?.data || '获取A股列表失败')
    }

    const payload = raw?.data as unknown
    const all: AStockOverview[] = Array.isArray(payload)
      ? (payload as AStockOverview[])
      : ((payload as any)?.data as AStockOverview[]) || []

    return all
  },

  // 获取A股列表（后端真实数据）
  getAStockOverviews: async (params: {
    page: number
    page_size: number
    order_by: string
    order: string
    market: string
    area: string
    industry: string
  }): Promise<PagedResponse<AStockOverview>> => {
    const resp = await fetch(`${API_BASE_URL}/api/a-stocks`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!resp.ok) {
      throw new Error(`HTTP error! status: ${resp.status}`)
    }

    const raw: ApiResponse<AStockOverview[]> = await resp.json()
    if (raw?.success === false) {
      throw new Error((raw as any)?.data || '获取A股列表失败')
    }

    const payload = raw?.data as unknown
    const all: AStockOverview[] = Array.isArray(payload)
      ? (payload as AStockOverview[])
      : ((payload as any)?.data as AStockOverview[]) || []
    const toNumber = (v: any) => {
      if (v === null || v === undefined) return NaN
      const n = typeof v === 'number' ? v : Number(v)
      return Number.isFinite(n) ? n : NaN
    }

    const orderBy = params.order_by
    const desc = params.order === 'descending' || params.order === 'desc'

    const sortedAll = [...all]
    if (orderBy && orderBy !== 'none') {
      sortedAll.sort((a: any, b: any) => {
        const av = toNumber(a?.[orderBy])
        const bv = toNumber(b?.[orderBy])

        const aValid = Number.isFinite(av)
        const bValid = Number.isFinite(bv)
        if (!aValid && !bValid) return 0
        if (!aValid) return 1
        if (!bValid) return -1

        return desc ? bv - av : av - bv
      })
    }

    const total = sortedAll.length
    const pageSize = Math.max(1, params.page_size)
    const totalPages = Math.max(1, Math.ceil(total / pageSize))
    const page = Math.min(Math.max(1, params.page), totalPages)
    const start = (page - 1) * pageSize
    const end = start + pageSize
    const items = sortedAll.slice(start, end)

    return {
      items,
      total,
      page,
      page_size: pageSize,
      total_pages: totalPages,
    }
  },

  // 获取市场摘要
  getMarketSummary: async (): Promise<MarketSummary> => {
    await delay(300)
    return mockMarketSummary
  },

  // 获取指数数据
  getIndexData: async (): Promise<IndexData[]> => {
    await delay(300)
    return mockIndexData
  },

  // 获取价格分布数据
  getPriceDistribution: async () => {
    await delay(300)
    return mockDistributionData
  },

  // 获取成交量分布
  getVolumeDistribution: async (): Promise<VolumeDistribution> => {
    await delay(300)
    return mockVolumeDistribution
  }
}
