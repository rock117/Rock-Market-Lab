// API基础配置和共享工具函数

// API基础配置 - 使用相对路径，通过Next.js rewrites转发
export const API_BASE_URL = ''

// 模拟延迟
export const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

// 日期格式化工具
export const normalizeDate = (input: string): string => {
  const raw = String(input).trim()
  if (!raw) return raw

  if (/^\d{8}$/.test(raw)) {
    const y = raw.slice(0, 4)
    const m = raw.slice(4, 6)
    const d = raw.slice(6, 8)
    return `${y}-${m}-${d}`
  }

  if (/^\d{4}-\d{2}-\d{2}$/.test(raw)) return raw

  const t = Date.parse(raw)
  if (Number.isFinite(t)) {
    const dt = new Date(t)
    const y = dt.getFullYear()
    const m = String(dt.getMonth() + 1).padStart(2, '0')
    const d = String(dt.getDate()).padStart(2, '0')
    return `${y}-${m}-${d}`
  }

  return raw
}

// K线图表颜色配置
export const chartColors = [
  '#3b82f6', // 蓝色
  '#ef4444', // 红色
  '#10b981', // 绿色
  '#f59e0b', // 橙色
  '#8b5cf6', // 紫色
  '#06b6d4', // 青色
  '#84cc16', // 柠檬绿
  '#f97316'  // 橙红色
]

// 生成K线数据的工具函数
export function generateKLineData(startPrice: number, days: number = 60) {
  const data = []
  let currentPrice = startPrice
  const startDate = new Date()
  startDate.setDate(startDate.getDate() - days)
  
  for (let i = 0; i < days; i++) {
    const date = new Date(startDate)
    date.setDate(date.getDate() + i)
    
    // 模拟价格波动
    const volatility = 0.03 // 3%波动率
    const change = (Math.random() - 0.5) * 2 * volatility
    const open = currentPrice
    const close = open * (1 + change)
    const high = Math.max(open, close) * (1 + Math.random() * 0.02)
    const low = Math.min(open, close) * (1 - Math.random() * 0.02)
    const volume = Math.floor(Math.random() * 10000000) + 1000000
    
    data.push({
      date: date.toISOString().split('T')[0],
      open: Number(open.toFixed(2)),
      high: Number(high.toFixed(2)),
      low: Number(low.toFixed(2)),
      close: Number(close.toFixed(2)),
      volume,
      amount: volume * close
    })
    
    currentPrice = close
  }
  
  return data
}

// 根据日期范围和周期生成K线数据
export function generateKLineDataWithDateRange(
  startPrice: number, 
  startDate: Date, 
  endDate: Date, 
  period: 'daily' | 'weekly' | 'monthly'
) {
  const data = []
  let currentPrice = startPrice
  let currentDate = new Date(startDate)
  
  // 根据周期设置日期增量
  const getNextDate = (date: Date, period: string): Date => {
    const nextDate = new Date(date)
    switch (period) {
      case 'weekly':
        nextDate.setDate(nextDate.getDate() + 7)
        break
      case 'monthly':
        nextDate.setMonth(nextDate.getMonth() + 1)
        break
      default: // daily
        nextDate.setDate(nextDate.getDate() + 1)
        break
    }
    return nextDate
  }
  
  // 根据周期调整波动率
  const getVolatility = (period: string): number => {
    switch (period) {
      case 'weekly':
        return 0.06 // 6%波动率
      case 'monthly':
        return 0.12 // 12%波动率
      default: // daily
        return 0.03 // 3%波动率
    }
  }
  
  const volatility = getVolatility(period)
  
  while (currentDate <= endDate) {
    // 模拟价格波动
    const change = (Math.random() - 0.5) * 2 * volatility
    const open = currentPrice
    const close = currentPrice * (1 + change)
    const high = Math.max(open, close) * (1 + Math.random() * 0.02)
    const low = Math.min(open, close) * (1 - Math.random() * 0.02)
    const volume = Math.floor(Math.random() * 1000000 + 500000)
    
    data.push({
      date: currentDate.toISOString().split('T')[0],
      open: Number(open.toFixed(2)),
      high: Number(high.toFixed(2)),
      low: Number(low.toFixed(2)),
      close: Number(close.toFixed(2)),
      volume
    })
    
    currentPrice = close
    currentDate = getNextDate(currentDate, period)
  }
  
  return data
}

// 数据转换单位函数
export const toYiYuan = (v: number): number => Number((v / 100000000).toFixed(2))
export const toWanYuan = (v: number): number => Number((v / 10000).toFixed(2))
