'use client'

import React, { useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useToast } from '@/components/ui/toast'
import { Plus, Play, Pause, Edit, Trash2, RotateCcw } from 'lucide-react'
import { taskApi } from '@/api/task'
import TaskDialog from './TaskDialog'
import ExecutionDialog from './ExecutionDialog'

interface Task {
  id: number
  name: string
  description?: string
  task_type: string
  schedule_type: string
  status: string
  next_run_time?: string
  created_at: string
  updated_at: string
}

interface TaskFilter {
  task_type: string
  status: string
}

function getStatusBadge(status: string) {
  const variant = status === 'enabled' ? 'default' : 
                  status === 'paused' ? 'secondary' : 
                  status === 'disabled' ? 'destructive' : 'outline'
  return <Badge variant={variant as any}>{getStatusText(status)}</Badge>
}

function getStatusText(status: string) {
  const statusMap: Record<string, string> = {
    'enabled': '启用',
    'paused': '暂停', 
    'disabled': '禁用'
  }
  return statusMap[status] || status
}

function getTaskTypeText(type: string) {
  const typeMap: Record<string, string> = {
    'http_request': 'HTTP 请求',
    'shell_command': 'Shell 命令',
    'rust_function': 'Rust 函数'
  }
  return typeMap[type] || type
}

function getTaskTypeBadge(type: string) {
  const variant = type === 'http_request' ? 'default' :
                  type === 'shell_command' ? 'secondary' :
                  type === 'rust_function' ? 'outline' : 'outline'
  return <Badge variant={variant as any}>{getTaskTypeText(type)}</Badge>
}

function formatDateTime(dateTime?: string) {
  if (!dateTime) return '-'
  return new Date(dateTime).toLocaleString('zh-CN')
}

export default function TaskScheduler() {
  const queryClient = useQueryClient()
  const { showToast } = useToast()
  const [filter, setFilter] = useState<TaskFilter>({ task_type: '', status: '' })
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [showExecutionDialog, setShowExecutionDialog] = useState(false)
  const [currentTask, setCurrentTask] = useState<Task | null>(null)
  const [currentTaskId, setCurrentTaskId] = useState<number | null>(null)

  // 获取任务列表
  const { data: tasksData, isLoading, refetch } = useQuery({
    queryKey: ['scheduled-tasks', filter],
    queryFn: () => taskApi.getTasks({
      task_type: filter.task_type || undefined,
      status: filter.status || undefined,
      page: 0,
      page_size: 100
    }),
    refetchInterval: 10000, // 每10秒刷新
  })

  const tasks = tasksData?.tasks || []

  // 运行任务
  const runMutation = useMutation({
    mutationFn: (taskId: number) => taskApi.runTask(taskId),
    onSuccess: (_, taskId) => {
      showToast('任务已开始执行', 'success')
      setCurrentTaskId(taskId)
      setShowExecutionDialog(true)
      refetch()
    },
    onError: (error: any) => {
      showToast(`执行失败: ${error.message}`, 'error')
    },
  })

  // 暂停任务
  const pauseMutation = useMutation({
    mutationFn: (taskId: number) => taskApi.pauseTask(taskId),
    onSuccess: () => {
      showToast('任务已暂停', 'success')
      refetch()
    },
    onError: (error: any) => {
      showToast(`暂停失败: ${error.message}`, 'error')
    },
  })

  // 恢复任务
  const resumeMutation = useMutation({
    mutationFn: (taskId: number) => taskApi.resumeTask(taskId),
    onSuccess: () => {
      showToast('任务已恢复', 'success')
      refetch()
    },
    onError: (error: any) => {
      showToast(`恢复失败: ${error.message}`, 'error')
    },
  })

  // 删除任务
  const deleteMutation = useMutation({
    mutationFn: (taskId: number) => taskApi.deleteTask(taskId),
    onSuccess: () => {
      showToast('任务已删除', 'success')
      refetch()
    },
    onError: (error: any) => {
      showToast(`删除失败: ${error.message}`, 'error')
    },
  })

  const handleCreateTask = () => {
    setCurrentTask(null)
    setShowCreateDialog(true)
  }

  const handleEditTask = (task: Task) => {
    setCurrentTask(task)
    setShowCreateDialog(true)
  }

  const handleRunTask = (task: Task) => {
    if (task.status !== 'enabled') {
      showToast('只能执行启用状态的任务', 'error')
      return
    }
    runMutation.mutate(task.id)
  }

  const handlePauseTask = (task: Task) => {
    pauseMutation.mutate(task.id)
  }

  const handleResumeTask = (task: Task) => {
    resumeMutation.mutate(task.id)
  }

  const handleDeleteTask = (task: Task) => {
    if (confirm(`确定要删除任务 "${task.name}" 吗？此操作不可恢复！`)) {
      deleteMutation.mutate(task.id)
    }
  }

  const handleTaskSaved = () => {
    setShowCreateDialog(false)
    refetch()
  }

  const resetFilter = () => {
    setFilter({ task_type: '', status: '' })
  }

  return (
    <div className="space-y-6">
      {/* 页面标题 */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">定时任务管理</h1>
          <p className="text-muted-foreground">管理和监控定时任务的执行</p>
        </div>
        <Button onClick={handleCreateTask}>
          <Plus className="w-4 h-4 mr-2" />
          创建任务
        </Button>
      </div>

      {/* 筛选器 */}
      <Card>
        <CardHeader>
          <CardTitle>筛选条件</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex gap-4 items-end">
            <div className="space-y-2">
              <Label>任务类型</Label>
              <Select value={filter.task_type} onValueChange={(value) => setFilter(prev => ({ ...prev, task_type: value }))}>
                <SelectTrigger className="w-40">
                  <SelectValue placeholder="全部类型" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">全部类型</SelectItem>
                  <SelectItem value="http_request">HTTP 请求</SelectItem>
                  <SelectItem value="shell_command">Shell 命令</SelectItem>
                  <SelectItem value="rust_function">Rust 函数</SelectItem>
                </SelectContent>
              </Select>
            </div>
            
            <div className="space-y-2">
              <Label>状态</Label>
              <Select value={filter.status} onValueChange={(value) => setFilter(prev => ({ ...prev, status: value }))}>
                <SelectTrigger className="w-32">
                  <SelectValue placeholder="全部状态" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">全部状态</SelectItem>
                  <SelectItem value="enabled">启用</SelectItem>
                  <SelectItem value="paused">暂停</SelectItem>
                  <SelectItem value="disabled">禁用</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <Button variant="outline" onClick={resetFilter}>
              <RotateCcw className="w-4 h-4 mr-2" />
              重置
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* 任务列表 */}
      <Card>
        <CardHeader>
          <CardTitle>任务列表</CardTitle>
          <CardDescription>
            共 {tasks.length} 个任务
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>任务名称</TableHead>
                <TableHead>类型</TableHead>
                <TableHead>状态</TableHead>
                <TableHead>下次执行</TableHead>
                <TableHead>创建时间</TableHead>
                <TableHead>操作</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={6} className="text-center py-8">
                    加载中...
                  </TableCell>
                </TableRow>
              ) : tasks.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={6} className="text-center py-8 text-muted-foreground">
                    暂无任务数据
                  </TableCell>
                </TableRow>
              ) : (
                tasks.map((task) => (
                  <TableRow key={task.id}>
                    <TableCell className="font-medium">{task.name}</TableCell>
                    <TableCell>{getTaskTypeBadge(task.task_type)}</TableCell>
                    <TableCell>{getStatusBadge(task.status)}</TableCell>
                    <TableCell>{formatDateTime(task.next_run_time)}</TableCell>
                    <TableCell>{formatDateTime(task.created_at)}</TableCell>
                    <TableCell>
                      <div className="flex gap-2">
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => handleRunTask(task)}
                          disabled={task.status !== 'enabled' || runMutation.isPending}
                        >
                          <Play className="w-3 h-3" />
                        </Button>
                        
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => handleEditTask(task)}
                        >
                          <Edit className="w-3 h-3" />
                        </Button>
                        
                        {task.status === 'enabled' ? (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => handlePauseTask(task)}
                            disabled={pauseMutation.isPending}
                          >
                            <Pause className="w-3 h-3" />
                          </Button>
                        ) : task.status === 'paused' ? (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => handleResumeTask(task)}
                            disabled={resumeMutation.isPending}
                          >
                            <Play className="w-3 h-3" />
                          </Button>
                        ) : null}
                        
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => handleDeleteTask(task)}
                          disabled={deleteMutation.isPending}
                        >
                          <Trash2 className="w-3 h-3" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* 对话框 */}
      <TaskDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
        task={currentTask}
        onSaved={handleTaskSaved}
      />

      <ExecutionDialog
        open={showExecutionDialog}
        onOpenChange={setShowExecutionDialog}
        taskId={currentTaskId}
      />
    </div>
  )
}
