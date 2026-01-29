import request from '@/utils/request'

export const taskApi = {
  // 获取任务类型列表
  getTaskTypes() {
    return request({
      url: '/api/task-types',
      method: 'get'
    })
  },

  // 获取任务列表
  getTasks(params) {
    return request({
      url: '/api/tasks',
      method: 'get',
      params
    })
  },

  // 获取任务详情
  getTask(id) {
    return request({
      url: `/api/tasks/${id}`,
      method: 'get'
    })
  },

  // 创建任务
  createTask(data) {
    return request({
      url: '/api/tasks',
      method: 'post',
      data
    })
  },

  // 更新任务
  updateTask(id, data) {
    return request({
      url: `/api/tasks/${id}`,
      method: 'put',
      data
    })
  },

  // 删除任务
  deleteTask(id) {
    return request({
      url: `/api/tasks/${id}`,
      method: 'delete'
    })
  },

  // 手动执行任务
  runTask(id) {
    return request({
      url: `/api/tasks/${id}/run`,
      method: 'post'
    })
  },

  // 暂停任务
  pauseTask(id) {
    return request({
      url: `/api/tasks/${id}/pause`,
      method: 'post'
    })
  },

  // 恢复任务
  resumeTask(id) {
    return request({
      url: `/api/tasks/${id}/resume`,
      method: 'post'
    })
  },

  // 获取执行记录
  getExecutions(params) {
    return request({
      url: '/api/task-executions',
      method: 'get',
      params
    })
  }
}
