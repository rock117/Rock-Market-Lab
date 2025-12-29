import type { ApiResponse, StockSimilarityItem } from '@/types'
import { API_BASE_URL } from './config'

export const stockSimilarityApi = {
  getSimilarity: async (params: { ts_code: string; days: number; top: number; algo?: string }): Promise<StockSimilarityItem[]> => {
    const query = new URLSearchParams()
    query.set('ts_code', params.ts_code)
    query.set('days', String(params.days))
    query.set('top', String(params.top))
    if (params.algo != null) {
      query.set('algo', params.algo)
    }

    const resp = await fetch(`${API_BASE_URL}/api/stocks/similarity?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!resp.ok) {
      throw new Error(`HTTP error! status: ${resp.status}`)
    }

    const raw: ApiResponse<StockSimilarityItem[]> = await resp.json()
    if ((raw as any)?.success === false) {
      throw new Error((raw as any)?.data || '获取相似度列表失败')
    }

    const payload = (raw as any)?.data
    return Array.isArray(payload) ? (payload as StockSimilarityItem[]) : []
  },
}
