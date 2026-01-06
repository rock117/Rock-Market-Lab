import { API_BASE_URL } from './config'
import type { ApiTaskListItem } from '@/types'

export const taskManagerApi = {
  listTasks: async (): Promise<ApiTaskListItem[]> => {
    const response = await fetch(`${API_BASE_URL}/api/tasks`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: ApiTaskListItem[]; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof (apiResponse as any).data === 'string' ? (apiResponse as any).data : '获取任务列表失败'
      throw new Error(errorMessage)
    }

    return apiResponse.data
  },

  runTask: async (taskName: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/api/tasks/${encodeURIComponent(taskName)}/run`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' ? apiResponse.data : '运行任务失败'
      throw new Error(errorMessage)
    }
  },

  pauseTask: async (taskName: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/api/tasks/${encodeURIComponent(taskName)}/pause`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' ? apiResponse.data : '暂停任务失败'
      throw new Error(errorMessage)
    }
  },

  resumeTask: async (taskName: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/api/tasks/${encodeURIComponent(taskName)}/resume`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' ? apiResponse.data : '恢复任务失败'
      throw new Error(errorMessage)
    }
  },

  stopTask: async (taskName: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/api/tasks/${encodeURIComponent(taskName)}/stop`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP错误! 状态码: ${response.status}`)
    }

    const apiResponse: { data: any; success: boolean } = await response.json()

    if (!apiResponse.success) {
      const errorMessage = typeof apiResponse.data === 'string' ? apiResponse.data : '停止任务失败'
      throw new Error(errorMessage)
    }
  },
}
