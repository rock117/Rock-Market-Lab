// 人均持股相关 API
import type { ApiResponse } from '@/types'
import { API_BASE_URL } from './config'

export interface HolderPerCapitaItem {
  ts_code: string
  name: string
  holder_num: number | null
  total_mv: number | null
  circ_mv: number | null
  per_capita_mv: number | null
  per_capita_share: number | null
  end_date: string | null
  close: number | null
  total_share: number | null
}

export const holderPerCapitaApi = {
  getAll: async (): Promise<HolderPerCapitaItem[]> => {
    const resp = await fetch(`${API_BASE_URL}/api/holder-per-capita`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!resp.ok) {
      throw new Error(`HTTP error! status: ${resp.status}`)
    }

    const raw: ApiResponse<HolderPerCapitaItem[]> = await resp.json()
    if (raw?.success === false) {
      throw new Error((raw as any)?.data || '获取人均持股列表失败')
    }

    const payload = raw?.data as unknown
    const all: HolderPerCapitaItem[] = Array.isArray(payload)
      ? (payload as HolderPerCapitaItem[])
      : ((payload as any)?.data as HolderPerCapitaItem[]) || []

    return all
  },
}
