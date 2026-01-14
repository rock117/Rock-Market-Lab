// 策略相关 API
import { StrategyType } from '@/types'
import { delay } from './config'
import { API_BASE_URL } from './config'

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

  listStrategyTemplates: async () => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-templates`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取策略模板失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  listStrategyProfiles: async () => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-profiles`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取策略列表失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  getStrategyProfile: async (id: number) => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-profiles/${encodeURIComponent(String(id))}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取策略失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  createStrategyProfile: async (payload: { name: string; description?: string; template: string; settings?: any; enabled?: boolean }) => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-profiles`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '创建策略失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  updateStrategyProfile: async (id: number, payload: { name?: string; description?: string; template?: string; settings?: any; enabled?: boolean }) => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-profiles/${encodeURIComponent(String(id))}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '更新策略失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  deleteStrategyProfile: async (id: number) => {
    const response = await fetch(`${API_BASE_URL}/api/strategy-profiles/${encodeURIComponent(String(id))}`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '删除策略失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
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
