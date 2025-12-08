import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { StockTrend } from "@/types"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// 格式化数字
export function formatNumber(num: number | null | undefined, decimals: number = 2): string {
  if (num === null || num === undefined || isNaN(num)) {
    return '--'
  }
  return num.toLocaleString('zh-CN', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals
  })
}

// 格式化大数字（万、亿）
export function formatLargeNumber(num: number | null | undefined): string {
  if (num === null || num === undefined || isNaN(num)) {
    return '--'
  }
  
  if (num >= 100000000) { // 亿
    return `${(num / 100000000).toFixed(2)}亿`
  } else if (num >= 10000) { // 万
    return `${(num / 10000).toFixed(2)}万`
  } else {
    return formatNumber(num, 0)
  }
}

// 格式化百分比
export function formatPercent(num: number | null | undefined, decimals: number = 2): string {
  if (num === null || num === undefined || isNaN(num)) {
    return '--'
  }
  return `${num.toFixed(decimals)}%`
}

// 格式化市值
export function formatMarketCap(marketCap: number | null | undefined): string {
  if (marketCap === null || marketCap === undefined || isNaN(marketCap)) {
    return '--'
  }
  
  // 假设市值单位是万元
  if (marketCap >= 100000000) { // 万亿
    return `${(marketCap / 100000000).toFixed(2)}万亿`
  } else if (marketCap >= 10000) { // 亿
    return `${(marketCap / 10000).toFixed(2)}亿`
  } else {
    return `${marketCap.toFixed(2)}万`
  }
}

// 获取股票趋势
export function getStockTrend(change: number | null | undefined): StockTrend {
  if (change === null || change === undefined || isNaN(change)) {
    return 'neutral'
  }
  if (change > 0) return 'up'
  if (change < 0) return 'down'
  return 'neutral'
}

// 获取趋势颜色类名
export function getTrendColorClass(trend: StockTrend): string {
  switch (trend) {
    case 'up':
      return 'text-bull'
    case 'down':
      return 'text-bear'
    default:
      return 'text-neutral'
  }
}

// 获取趋势颜色值
export function getTrendColor(trend: StockTrend): string {
  switch (trend) {
    case 'up':
      return '#10b981'
    case 'down':
      return '#ef4444'
    default:
      return '#6b7280'
  }
}

// 格式化日期
export function formatDate(dateStr: string | null | undefined, format: 'YYYY-MM-DD' | 'MM-DD' | 'YYYY/MM/DD' = 'YYYY-MM-DD'): string {
  if (!dateStr) return '--'
  
  try {
    const date = new Date(dateStr)
    if (isNaN(date.getTime())) return '--'
    
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    
    switch (format) {
      case 'MM-DD':
        return `${month}-${day}`
      case 'YYYY/MM/DD':
        return `${year}/${month}/${day}`
      default:
        return `${year}-${month}-${day}`
    }
  } catch {
    return '--'
  }
}

// 计算涨跌幅
export function calculatePctChange(current: number, previous: number): number {
  if (previous === 0) return 0
  return ((current - previous) / previous) * 100
}

// 防抖函数
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null
  
  return (...args: Parameters<T>) => {
    if (timeout) clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), wait)
  }
}

// 节流函数
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean = false
  
  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args)
      inThrottle = true
      setTimeout(() => inThrottle = false, limit)
    }
  }
}

// 生成随机ID
export function generateId(): string {
  return Math.random().toString(36).substr(2, 9)
}

// 深拷贝
export function deepClone<T>(obj: T): T {
  if (obj === null || typeof obj !== 'object') return obj
  if (obj instanceof Date) return new Date(obj.getTime()) as unknown as T
  if (obj instanceof Array) return obj.map(item => deepClone(item)) as unknown as T
  if (typeof obj === 'object') {
    const clonedObj = {} as { [key: string]: any }
    for (const key in obj) {
      if (obj.hasOwnProperty(key)) {
        clonedObj[key] = deepClone(obj[key])
      }
    }
    return clonedObj as T
  }
  return obj
}
