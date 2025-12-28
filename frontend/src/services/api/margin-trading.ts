// 融资融券相关 API
import { 
  MarginTradingKLineRequest, 
  MarginTradingKLineResponse,
  StockMarginTradingKLineRequest,
  StockMarginTradingKLineResponse,
  StockHistoryData
} from '@/types'
import { API_BASE_URL, normalizeDate, toYiYuan, toWanYuan } from './config'

// 融资融券API
export const marginTradingApi = {
  // 获取市场融资融券K线数据
  getMarginTradingKLine: async (request: MarginTradingKLineRequest): Promise<MarginTradingKLineResponse> => {
    const query = new URLSearchParams()
    query.set('exchange', request.exchange)
    query.set('start_date', request.startDate)
    query.set('end_date', request.endDate)

    const response = await fetch(`${API_BASE_URL}/api/margin/balance?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    const rows: Array<{ date: string; marginBalance: string | number }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const sorted = rows
      .filter(r => r?.date)
      .map(r => ({
        date: normalizeDate(r.date),
        marginBalance: typeof r.marginBalance === 'number' ? r.marginBalance : Number(r.marginBalance),
      }))
      .filter(r => Number.isFinite(r.marginBalance))
      .map(r => ({
        ...r,
        marginBalance: toYiYuan(r.marginBalance),
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

    const data: StockHistoryData[] = sorted.map((r, idx) => {
      const close = r.marginBalance
      const open = idx === 0 ? close : sorted[idx - 1].marginBalance
      const high = Math.max(open, close)
      const low = Math.min(open, close)

      const change = close - open
      const pctChg = open === 0 ? 0 : (change / open) * 100

      return {
        trade_date: r.date,
        open: Number(open.toFixed(2)),
        high: Number(high.toFixed(2)),
        low: Number(low.toFixed(2)),
        close: Number(close.toFixed(2)),
        volume: 0,
        amount: 0,
        turnover_rate: 0,
        pct_chg: Number(pctChg.toFixed(2)),
        change: Number(change.toFixed(2)),
      }
    })

    return {
      exchange: request.exchange,
      startDate: request.startDate,
      endDate: request.endDate,
      data,
    }
  },

  // 获取个股融资融券K线数据
  getStockMarginTradingKLine: async (request: StockMarginTradingKLineRequest): Promise<StockMarginTradingKLineResponse> => {
    const query = new URLSearchParams()
    query.set('stock', request.stock)
    query.set('start_date', request.startDate)
    query.set('end_date', request.endDate)

    const response = await fetch(`${API_BASE_URL}/api/margin/balance?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    const rows: Array<{ date: string; marginBalance: string | number }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const sorted = rows
      .filter(r => r?.date)
      .map(r => ({
        date: normalizeDate(r.date),
        marginBalance: typeof r.marginBalance === 'number' ? r.marginBalance : Number(r.marginBalance),
      }))
      .filter(r => Number.isFinite(r.marginBalance))
      .map(r => ({
        ...r,
        marginBalance: toWanYuan(r.marginBalance),
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

    const data: StockHistoryData[] = sorted.map((r, idx) => {
      const close = r.marginBalance
      const open = idx === 0 ? close : sorted[idx - 1].marginBalance
      const high = Math.max(open, close)
      const low = Math.min(open, close)

      const change = close - open
      const pctChg = open === 0 ? 0 : (change / open) * 100

      return {
        trade_date: r.date,
        open: Number(open.toFixed(2)),
        high: Number(high.toFixed(2)),
        low: Number(low.toFixed(2)),
        close: Number(close.toFixed(2)),
        volume: 0,
        amount: 0,
        turnover_rate: 0,
        pct_chg: Number(pctChg.toFixed(2)),
        change: Number(change.toFixed(2)),
      }
    })

    return {
      stock: request.stock,
      startDate: request.startDate,
      endDate: request.endDate,
      data,
    }
  },
}
