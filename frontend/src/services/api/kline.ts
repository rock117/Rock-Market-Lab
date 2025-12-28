// K线比较相关 API
import { SecuritySearchResult, Security, SecurityKLineData, TrendAnalysis, KLineData } from '@/types'
import { delay, chartColors, generateKLineDataWithDateRange } from './config'
import { mockSecurities } from './mock-data'

// K线比较API
export const klineApi = {
  // 搜索证券
  searchSecurities: async (keyword: string): Promise<SecuritySearchResult> => {
    await delay(200)
    const filtered = mockSecurities.filter(security => 
      security.code.toLowerCase().includes(keyword.toLowerCase()) ||
      security.name.toLowerCase().includes(keyword.toLowerCase())
    )
    return { securities: filtered.slice(0, 10) }
  },

  // 获取证券K线数据
  getSecurityKLineData: async (
    securities: Security[], 
    options?: {
      startDate?: string
      endDate?: string
      period?: 'daily' | 'weekly' | 'monthly'
    }
  ): Promise<SecurityKLineData[]> => {
    await delay(600)
    
    // 计算数据天数
    const startDate = options?.startDate ? new Date(options.startDate) : new Date(Date.now() - 90 * 24 * 60 * 60 * 1000)
    const endDate = options?.endDate ? new Date(options.endDate) : new Date()
    
    return securities.map((security, index) => {
      // 根据证券类型设置不同的起始价格
      let startPrice = 10
      if (security.type === 'stock') {
        startPrice = security.code.includes('600519') ? 1800 : // 贵州茅台
                    security.code.includes('000858') ? 150 :  // 五粮液
                    security.code.includes('300750') ? 200 :  // 宁德时代
                    12 // 其他股票
      } else if (security.type === 'index') {
        startPrice = security.code.includes('000001.SH') ? 3200 : // 上证指数
                    security.code.includes('399001') ? 10400 :    // 深证成指
                    security.code.includes('399006') ? 2100 :     // 创业板指
                    4800 // 沪深300
      } else if (security.type === 'fund') {
        startPrice = Math.random() * 2 + 1 // 基金净值1-3之间
      }
      
      return {
        security,
        data: generateKLineDataWithDateRange(startPrice, startDate, endDate, options?.period || 'daily'),
        color: chartColors[index % chartColors.length]
      }
    })
  },

  // 分析趋势相关性
  analyzeTrendCorrelation: async (
    securities: Security[],
    options?: {
      startDate?: string
      endDate?: string
      period?: 'daily' | 'weekly' | 'monthly'
    }
  ): Promise<TrendAnalysis> => {
    await delay(300)
    
    // 模拟相关性分析结果
    const correlation = Math.random() * 2 - 1 // -1 到 1
    const absCorrelation = Math.abs(correlation)
    
    let trend_consistency: 'high' | 'medium' | 'low'
    if (absCorrelation > 0.7) trend_consistency = 'high'
    else if (absCorrelation > 0.4) trend_consistency = 'medium'
    else trend_consistency = 'low'
    
    const sync_rate = absCorrelation * 100
    
    // 根据时间周期生成分析周期描述
    const startDate = options?.startDate ? new Date(options.startDate) : new Date(Date.now() - 90 * 24 * 60 * 60 * 1000)
    const endDate = options?.endDate ? new Date(options.endDate) : new Date()
    const daysDiff = Math.ceil((endDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24))
    
    let periodDesc = ''
    if (options?.period === 'weekly') {
      periodDesc = `近${Math.ceil(daysDiff / 7)}周`
    } else if (options?.period === 'monthly') {
      periodDesc = `近${Math.ceil(daysDiff / 30)}个月`
    } else {
      periodDesc = `近${daysDiff}个交易日`
    }
    
    return {
      correlation: Number(correlation.toFixed(3)),
      trend_consistency,
      sync_rate: Number(sync_rate.toFixed(1)),
      analysis_period: periodDesc
    }
  }
}
