import type { ApiResponse, StockSimilarityResponse } from '@/types'
import { API_BASE_URL } from './config'

export const stockSimilarityApi = {
  getSimilarity: async (params: {
    ts_code: string
    days: number
    top: number
    algo?: string
    freq?: 'day' | 'week' | 'month'
  }): Promise<StockSimilarityResponse> => {
    const query = new URLSearchParams()
    query.set('ts_code', params.ts_code)
    query.set('days', String(params.days))
    query.set('top', String(params.top))
    if (params.algo != null) {
      query.set('algo', params.algo)
    }
    if (params.freq != null) {
      query.set('freq', params.freq)
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

    const raw: ApiResponse<StockSimilarityResponse> = await resp.json()
    if ((raw as any)?.success === false) {
      throw new Error((raw as any)?.data || '获取相似度列表失败')
    }

    const payload = (raw as any)?.data
    const items = Array.isArray(payload?.items) ? payload.items : []
    const kline = payload?.kline && typeof payload.kline === 'object' ? payload.kline : {}
    return { items, kline }
  },
}
