import { API_BASE_URL } from './config'

export type FinanceMainBusinessType = 'P' | 'D' | 'I'

export interface FinanceMainBusinessItem {
  tsCode: string
  stockName?: string | null
  endDate: string
  bzItem: string
  bzSales?: string | null
  bzProfit?: string | null
  bzCost?: string | null
}

export interface FinanceMainBusinessListResponse {
  data: FinanceMainBusinessItem[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

export interface FinanceMainBusinessQuery {
  type: FinanceMainBusinessType
  page?: number
  pageSize?: number
  sortBy?: string
  sortDir?: 'asc' | 'desc'
}

export const financeApi = {
  async getMainBusinessList(query: FinanceMainBusinessQuery): Promise<FinanceMainBusinessListResponse> {
    const params = new URLSearchParams()
    params.set('type', query.type)
    if (query.page) params.set('page', String(query.page))
    if (query.pageSize) params.set('page_size', String(query.pageSize))
    if (query.sortBy) params.set('sort_by', query.sortBy)
    if (query.sortDir) params.set('sort_dir', query.sortDir)

    const resp = await fetch(`${API_BASE_URL}/api/finance/main-business?${params.toString()}`)
    if (!resp.ok) throw new Error(`Request failed: ${resp.status}`)
    const json = await resp.json()
    return json.data
  },
}
