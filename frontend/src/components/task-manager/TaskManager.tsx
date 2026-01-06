'use client'

import React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { taskManagerApi } from '@/services/api/task-manager'
import type { ApiTaskListItem } from '@/types'
import { useToast } from '@/components/ui/toast'

function statusBadge(status: string) {
  const s = String(status || '').toLowerCase()
  const variant = s === 'success' ? 'default' : s === 'error' ? 'destructive' : 'secondary'
  return <Badge variant={variant as any}>{status || '--'}</Badge>
}

function formatMaybeTime(v?: string | null) {
  if (!v) return '--'
  return v
}

export default function TaskManager() {
  const queryClient = useQueryClient()
  const { showToast } = useToast()

  const { data = [], isLoading, error, refetch, isFetching } = useQuery({
    queryKey: ['tasks'],
    queryFn: () => taskManagerApi.listTasks(),
    refetchInterval: 5000,
  })

  const runMutation = useMutation({
    mutationFn: (taskName: string) => taskManagerApi.runTask(taskName),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['tasks'] })
      showToast('任务已触发运行', 'success')
    },
    onError: (e: Error) => {
      showToast(e.message, 'error')
    },
  })

  const pauseMutation = useMutation({
    mutationFn: (taskName: string) => taskManagerApi.pauseTask(taskName),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['tasks'] })
      showToast('任务已暂停', 'success')
    },
    onError: (e: Error) => {
      showToast(e.message, 'error')
    },
  })

  const resumeMutation = useMutation({
    mutationFn: (taskName: string) => taskManagerApi.resumeTask(taskName),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['tasks'] })
      showToast('任务已恢复', 'success')
    },
    onError: (e: Error) => {
      showToast(e.message, 'error')
    },
  })

  const stopMutation = useMutation({
    mutationFn: (taskName: string) => taskManagerApi.stopTask(taskName),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['tasks'] })
      showToast('任务已停止', 'success')
    },
    onError: (e: Error) => {
      showToast(e.message, 'error')
    },
  })

  const isMutating = runMutation.isPending || pauseMutation.isPending || resumeMutation.isPending || stopMutation.isPending

  const rows = (data || []) as ApiTaskListItem[]

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <div className="flex items-start justify-between gap-4">
            <div>
              <CardTitle>任务管理</CardTitle>
              <CardDescription>查看定时任务状态，并执行 run / pause / resume / stop</CardDescription>
            </div>
            <div className="flex items-center gap-2">
              <Button variant="outline" size="sm" onClick={() => refetch()} disabled={isFetching || isLoading || isMutating}>
                刷新
              </Button>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          {error ? (
            <div className="text-sm text-destructive">{(error as any)?.message || '加载失败'}</div>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>任务名</TableHead>
                  <TableHead>状态</TableHead>
                  <TableHead>Paused</TableHead>
                  <TableHead>Stopped</TableHead>
                  <TableHead>上次开始</TableHead>
                  <TableHead>上次结束</TableHead>
                  <TableHead>成功/失败</TableHead>
                  <TableHead className="text-right">操作</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {isLoading ? (
                  <TableRow>
                    <TableCell colSpan={8} className="text-center py-8 text-muted-foreground">
                      加载中...
                    </TableCell>
                  </TableRow>
                ) : rows.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={8} className="text-center py-8 text-muted-foreground">
                      暂无任务
                    </TableCell>
                  </TableRow>
                ) : (
                  rows.map((item) => {
                    const name = item.info?.name
                    const state = item.state
                    const paused = Boolean(state?.paused)
                    const stopped = Boolean(state?.stopped)
                    const disabled = isMutating

                    return (
                      <TableRow key={name}>
                        <TableCell className="font-mono font-medium">{name}</TableCell>
                        <TableCell>{statusBadge(state?.status || '--')}</TableCell>
                        <TableCell>{paused ? <Badge variant="secondary">yes</Badge> : <span className="text-sm text-muted-foreground">no</span>}</TableCell>
                        <TableCell>{stopped ? <Badge variant="secondary">yes</Badge> : <span className="text-sm text-muted-foreground">no</span>}</TableCell>
                        <TableCell className="font-mono text-xs">{formatMaybeTime(state?.last_started_at)}</TableCell>
                        <TableCell className="font-mono text-xs">{formatMaybeTime(state?.last_ended_at)}</TableCell>
                        <TableCell className="font-mono text-xs">{state?.last_success_count ?? 0}/{state?.last_fail_count ?? 0}</TableCell>
                        <TableCell className="text-right">
                          <div className="flex items-center justify-end gap-2">
                            <Button size="sm" onClick={() => runMutation.mutate(name)} disabled={disabled || paused || stopped}>
                              Run
                            </Button>
                            <Button variant="outline" size="sm" onClick={() => pauseMutation.mutate(name)} disabled={disabled || paused || stopped}>
                              Pause
                            </Button>
                            <Button variant="outline" size="sm" onClick={() => resumeMutation.mutate(name)} disabled={disabled || !paused || stopped}>
                              Resume
                            </Button>
                            <Button variant="destructive" size="sm" onClick={() => stopMutation.mutate(name)} disabled={disabled || stopped}>
                              Stop
                            </Button>
                          </div>
                        </TableCell>
                      </TableRow>
                    )
                  })
                )}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
