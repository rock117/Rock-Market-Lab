// A股市场相关 API
import { MarketSummary, IndexData, VolumeDistribution } from '@/types'
import { delay } from './config'
import { mockMarketSummary, mockIndexData, mockDistributionData, mockVolumeDistribution } from './mock-data'

// A股市场摘要相关API（使用模拟数据）
export const stockApi = {
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
