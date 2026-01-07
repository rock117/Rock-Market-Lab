import { API_BASE_URL } from './config'
import type { ApiDcIndex, ApiDcMember, ApiDcMemberEnriched } from '@/types'

export const dcConceptApi = {
  listConcepts: async (): Promise<ApiDcIndex[]> => {
    const response = await fetch(`${API_BASE_URL}/api/dc_index`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: ApiDcIndex[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取概念列表失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  listTradeDates: async (): Promise<string[]> => {
    const response = await fetch(`${API_BASE_URL}/api/dc_index/trade_dates`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: string[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取日期列表失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  queryConcepts: async (tradeDates: string[]): Promise<ApiDcIndex[]> => {
    const response = await fetch(`${API_BASE_URL}/api/dc_index/query`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ trade_dates: tradeDates }),
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: ApiDcIndex[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取概念列表失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  listMembers: async (tsCode: string, tradeDate: string): Promise<ApiDcMember[]> => {
    const response = await fetch(
      `${API_BASE_URL}/api/dc_index/${encodeURIComponent(tsCode)}/members?trade_date=${encodeURIComponent(tradeDate)}`,
      {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      }
    )

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: ApiDcMember[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取概念成分股失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  listMembersEnriched: async (tsCode: string, tradeDate: string): Promise<ApiDcMemberEnriched[]> => {
    const response = await fetch(
      `${API_BASE_URL}/api/dc_index/${encodeURIComponent(tsCode)}/members_enriched?trade_date=${encodeURIComponent(tradeDate)}`,
      {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      }
    )

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: ApiDcMemberEnriched[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage =
        typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取概念成分股失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },
}
