// ETF相关 API
import { EtfItem, EtfHolding } from '@/types'
import { API_BASE_URL } from './config'

// ETF API（后端返回全部，前端搜索过滤）
export const etfApi = {
  getEtfList: async (): Promise<EtfItem[]> => {
    const response = await fetch(`${API_BASE_URL}/api/etf/list`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const result: { data: EtfItem[]; success: boolean } = await response.json()
    if (!result.success) {
      throw new Error((result as any)?.data || '获取ETF列表失败')
    }

    return Array.isArray(result.data) ? result.data : []
  },

  getEtfHoldings: async (ts_code: string): Promise<EtfHolding[]> => {
    const query = new URLSearchParams()
    query.set('ts_code', ts_code)

    const response = await fetch(`${API_BASE_URL}/api/etf/holdings?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const result: { data: any; success: boolean } = await response.json()
    if (!result.success) {
      throw new Error((result as any)?.data || '获取ETF持仓失败')
    }

    const rows = Array.isArray(result.data) ? result.data : []
    return rows.map((r: any) => ({
      ts_code: String(r.ts_code ?? ts_code),
      ann_date: String(r.ann_date ?? ''),
      end_date: String(r.end_date ?? ''),
      symbol: String(r.symbol ?? ''),
      mkv: Number(r.mkv),
      amount: Number(r.amount),
      stk_mkv_ratio: r.stk_mkv_ratio == null ? null : Number(r.stk_mkv_ratio),
      stk_float_ratio: r.stk_float_ratio == null ? null : Number(r.stk_float_ratio),
    }))
  },
}
