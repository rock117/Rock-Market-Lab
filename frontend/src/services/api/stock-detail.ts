// 股票详情相关 API
import { StockDetail, StockHistoryResponse, StockHistoryData } from '@/types'
import { API_BASE_URL, delay, normalizeDate } from './config'
import { mockStockDetail, mockStockList } from './mock-data'

// 股票详情API
export const stockDetailApi = {
  // 搜索股票
  searchStocks: async (keyword: string) => {
    if (!keyword || keyword.length < 1) {
      return { stocks: [] as Array<{ ts_code: string; name: string }> }
    }

    const query = new URLSearchParams()
    query.set('keyword', keyword)

    const response = await fetch(`${API_BASE_URL}/api/stocks/search?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    
    // 检查 API 是否返回错误格式 {data: string, success: false}
    if (raw?.success === false) {
      throw new Error(raw?.data || '搜索股票失败')
    }

    const rows: Array<{ ts_code: string; name: string }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : raw?.ts_code && raw?.name
          ? [raw]
          : []

    return {
      stocks: rows.filter(r => r?.ts_code && r?.name),
    }
  },

  // 获取股票详情
  getStockDetail: async (ts_code: string): Promise<StockDetail> => {
    if (!ts_code) {
      throw new Error('股票代码不能为空')
    }

    try {
      const response = await fetch(`${API_BASE_URL}/api/stocks/detail?ts_code=${ts_code}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      })

      if (response.ok) {
        const raw = await response.json()
        
        // 检查 API 是否返回错误格式 {data: string, success: false}
        if (raw?.success === false) {
          throw new Error(raw?.data || '获取股票详情失败')
        }

        // 如果成功获取真实数据，直接返回
        if (raw && typeof raw === 'object') {
          return raw as StockDetail
        }
      }
    } catch (error) {
      // 如果 API 调用失败，继续使用模拟数据
      if (error instanceof Error && error.message.includes('success')) {
        throw error
      }
    }

    // 降级到模拟数据
    await delay(400)
    
    // 根据不同股票代码返回不同数据
    const stockInfo = mockStockList.find(stock => stock.ts_code === ts_code)
    const stockName = stockInfo?.name || '示例股票'
    
    // 为不同股票生成不同的模拟数据
    const basePrice = ts_code === '600519.SH' ? 1680 : // 贵州茅台
                     ts_code === '300750.SZ' ? 185 :   // 宁德时代
                     ts_code === '000858.SZ' ? 158 :   // 五粮液
                     ts_code === '600036.SH' ? 38 :    // 招商银行
                     ts_code === '000002.SZ' ? 8.5 :   // 万科A
                     12.58 // 平安银行等其他
    
    const changePercent = (Math.random() - 0.5) * 6 // -3% 到 3%
    const change = basePrice * (changePercent / 100)
    
    return {
      ...mockStockDetail,
      ts_code,
      name: stockName,
      current_price: Number((basePrice + change).toFixed(2)),
      change: Number(change.toFixed(2)),
      pct_chg: Number(changePercent.toFixed(2)),
      pe_ratio: Number((Math.random() * 30 + 5).toFixed(1)),
      pb_ratio: Number((Math.random() * 3 + 0.5).toFixed(2)),
      five_day_return: Number((Math.random() * 10 - 5).toFixed(2))
    }
  },

  // 获取股票历史价格
  getStockHistory: async (
    ts_code: string,
    params: {
      timeMode: 'custom' | 'quick'
      startDate?: string
      endDate?: string
      timePeriod?: string
    }
  ): Promise<StockHistoryResponse> => {
    const query = new URLSearchParams()
    query.set('ts_code', ts_code)
    if (params.timeMode === 'custom') {
      if (params.startDate) query.set('start_date', params.startDate)
      if (params.endDate) query.set('end_date', params.endDate)
    } else {
      if (params.timePeriod) query.set('time_period', params.timePeriod)
    }

    const response = await fetch(`${API_BASE_URL}/api/stocks/history?${query.toString()}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const raw = await response.json()
    
    // 检查 API 是否返回错误格式 {data: string, success: false}
    if (raw?.success === false) {
      throw new Error(raw?.data || '获取历史数据失败')
    }

    const rows: Array<{
      open?: string | number
      high?: string | number
      low?: string | number
      close?: string | number
      amount?: string | number
      vol?: string | number
      volume?: string | number
      pct_chg?: string | number
      pctChg?: string | number
      date: string
      turnover_rate?: string | number
      turnoverRate?: string | number
    }> = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.data)
        ? raw.data
        : []

    const historyData: StockHistoryData[] = rows
      .filter(r => r?.date)
      .map(r => {
        const open = Number(r.open)
        const high = Number(r.high)
        const low = Number(r.low)
        const close = Number(r.close)
        const amount = Number(r.amount ?? r.vol ?? r.volume)
        const pctChg = Number(r.pct_chg ?? r.pctChg)
        const turnoverRate = Number(r.turnover_rate ?? r.turnoverRate)

        return {
          trade_date: normalizeDate(r.date),
          open: Number.isFinite(open) ? open : 0,
          high: Number.isFinite(high) ? high : 0,
          low: Number.isFinite(low) ? low : 0,
          close: Number.isFinite(close) ? close : 0,
          volume: 0,
          amount: Number.isFinite(amount) ? amount : 0,
          turnover_rate: Number.isFinite(turnoverRate) ? turnoverRate : 0,
          pct_chg: Number.isFinite(pctChg) ? pctChg : 0,
          change: Number.isFinite(open) && Number.isFinite(close) ? Number((close - open).toFixed(2)) : 0,
        }
      })

    historyData.sort((a, b) => new Date(b.trade_date).getTime() - new Date(a.trade_date).getTime())

    return {
      ts_code,
      name: '',
      data: historyData,
      total: historyData.length,
    }
  }
}
