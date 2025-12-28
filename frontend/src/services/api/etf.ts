// ETF相关 API
import { EtfItem, EtfHolding } from '@/types'
import { API_BASE_URL } from './config'

const normalizeEtfItem = (r: any): EtfItem => ({
  tsCode: String(r.tsCode ?? r.ts_code ?? ''),
  cname: r.cname == null ? null : String(r.cname),
  listDate: r.listDate ?? r.list_date ?? null,
  etfType: r.etfType ?? r.etf_type ?? null,
  exchange: r.exchange ?? null,
  ts_code: r.ts_code,
  csname: r.csname,
  extname: r.extname,
  index_code: r.index_code,
  index_name: r.index_name,
  setup_date: r.setup_date,
  list_date: r.list_date,
  list_status: r.list_status,
  custod_name: r.custod_name,
  mgr_name: r.mgr_name,
  mgt_fee: r.mgt_fee,
  etf_type: r.etf_type,
})

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

    const rows = Array.isArray(result.data) ? result.data : []
    return rows.map(normalizeEtfItem)
  },

  searchEtfs: async (keyword: string): Promise<EtfItem[]> => {
    const kw = keyword.trim()
    if (!kw) return []

    const query = new URLSearchParams()
    query.set('keyword', kw)

    const response = await fetch(`${API_BASE_URL}/api/etf/search?${query.toString()}`, {
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
      throw new Error((result as any)?.data || '搜索ETF失败')
    }

    const rows = Array.isArray(result.data) ? result.data : []
    return rows.map(normalizeEtfItem)
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
      name: r.name == null ? null : String(r.name),
      mkv: Number(r.mkv),
      amount: Number(r.amount),
      stk_mkv_ratio: r.stk_mkv_ratio == null ? null : Number(r.stk_mkv_ratio),
      stk_float_ratio: r.stk_float_ratio == null ? null : Number(r.stk_float_ratio),
    }))
  },
}
