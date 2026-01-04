// 策略相关 API
import { StrategyType } from '@/types'
import { delay } from './config'

// 策略API
export const strategyApi = {
  // 运行策略
  runStrategy: async (strategyType: StrategyType, parameters: Record<string, any>) => {
    // 创建超时控制器（2分钟 = 120秒）
    // 策略分析可能需要较长时间，特别是对大量股票进行复杂分析时
    const controller = new AbortController()
    const timeoutId = setTimeout(() => {
      console.warn('策略运行超时，自动中止请求')
      controller.abort()
    }, 120000) // 120000毫秒 = 2分钟

    try {
      const response = await fetch('/api/stocks/pick', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          strategy: strategyType,
          settings: parameters
        }),
        signal: controller.signal
      })

      clearTimeout(timeoutId)

      const result = await response.json()

      // 检查业务逻辑错误
      if (!result.success) {
        throw new Error(result.data || '策略运行失败')
      }

      // 检查HTTP错误
      if (!response.ok) {
        throw new Error('策略运行失败')
      }

      return result
    } catch (error: any) {
      clearTimeout(timeoutId)
      if (error.name === 'AbortError') {
        throw new Error('请求超时，请稍后重试')
      }
      throw error
    }
  },

  // 获取策略列表
  getStrategies: async () => {
    await delay(300)
    return [
      {
        strategy_type: 'price_volume_candlestick' as StrategyType,
        parameters: { volume_threshold: 1.5, price_change_threshold: 0.03 },
        enabled: true,
        description: '基于价格和成交量的K线形态分析'
      },
      {
        strategy_type: 'fundamental' as StrategyType,
        parameters: { min_roe: 0.15, max_pe: 25 },
        enabled: true,
        description: '基于财务指标的价值投资策略'
      },
      {
        strategy_type: 'turtle' as StrategyType,
        parameters: { entry_period: 20, exit_period: 10 },
        enabled: true,
        description: '经典的趋势跟踪策略'
      }
    ]
  }
}
