'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Button } from '@/components/ui/button'
import { strategyApi } from '@/services/api'
import { StrategyType } from '@/types'
import { formatNumber } from '@/lib/utils'
import { 
  Target, 
  Settings, 
  Play, 
  BarChart3, 
  AlertTriangle,
  Plus,
  Pencil,
  Trash2,
  ArrowLeft
} from 'lucide-react'
import { useToast } from '@/components/ui/toast'

interface StockSelectionStrategyProps {
  className?: string
}

interface StrategyTemplateParam {
  key: string
  label: string
  type: string
  required: boolean
  default_value?: any
  description?: string
  min?: number
  max?: number
  options?: any[]
}

interface StrategyTemplateDto {
  template: string
  label: string
  description: string
  params: StrategyTemplateParam[]
}

interface StrategyProfileDto {
  id: number
  name: string
  description?: string
  template: string
  settings?: any
  created_at: string
  updated_at: string
}

// 策略类型映射
const STRATEGY_TYPES = [
  { value: 'price_volume_candlestick', label: '价量K线策略', description: '基于价格和成交量的K线形态分析' },
  { value: 'bottom_volume_surge', label: '底部放量上涨策略', description: '识别底部区域的放量上涨信号' },
  { value: 'long_term_bottom_reversal', label: '长期底部反转策略', description: '寻找长期底部的反转机会' },
  { value: 'yearly_high', label: '年内新高策略', description: '筛选创年内新高的强势股' },
  { value: 'price_strength', label: '价格强弱策略', description: '基于相对强弱指标的选股' },
  { value: 'distressed_reversal', label: '困境反转策略', description: '寻找困境中的反转机会' },
  { value: 'single_limit_up', label: '单次涨停策略', description: '识别单次涨停后的机会' },
  { value: 'fundamental', label: '基本面策略', description: '基于财务指标的价值投资策略' },
  { value: 'consecutive_strong', label: '连续强势股策略', description: '筛选连续强势表现的股票' },
  { value: 'turtle', label: '海龟交易策略', description: '经典的趋势跟踪策略' },
  { value: 'limit_up_pullback', label: '涨停回调策略', description: '涨停后回调的买入机会' },
  { value: 'strong_close', label: '强势收盘策略', description: '基于收盘强势的选股策略' },
  { value: 'quality_value', label: '优质价值策略', description: '寻找优质且被低估的股票' },
  { value: 'turnover_ma_bullish', label: '换手率均线多头策略', description: '基于换手率和均线的多头策略' },
  { value: 'turnover_rise', label: '换手率区间涨幅策略', description: '过去N天每日换手率高于阈值，且区间累计涨幅高于阈值' },
  { value: 'daily_rise_turnover', label: '连续上涨且换手率达标策略', description: '过去N天每天涨幅>=阈值，且每天换手率>=阈值' },
  { value: 'ma_divergence_volume', label: '均线向上发散放量策略', description: '日均线向上发散，K线站上5日线(3-4天)，成交量连续放量>=2天' },
  { value: 'low_shadow', label: '低位下影线策略', description: '识别低位长下影线的反转信号' },
  { value: 'similarity', label: '股价走势相似策略', description: '股价走势相似策略' },
  { value: 'ma_convergence', label: '均线粘合策略', description: '识别均线粘合形态，筛选下跌后的粘合机会' },
  { value: 'consecutive_bullish', label: '日/周/月连阳策略', description: '识别连续阳线形态，捕捉上升趋势的持续信号' }
]

// 默认参数示例
const DEFAULT_PARAMS: Record<string, any> = {
  price_volume_candlestick: {
    volume_threshold: 1.5,
    price_change_threshold: 0.03,
    lookback_days: 20
  },
  bottom_volume_surge: {
    volume_surge_ratio: 2.0,
    price_bottom_threshold: 0.9,
    surge_days: 3
  },
  turnover_rise: {
    preset: 'standard',
    lookback_days: 5,
    min_turnover_rate: 3.0,
    min_price_rise_pct: 5.0
  },
  daily_rise_turnover: {
    lookback_days: 5,
    min_daily_rise_pct: 3.0,
    min_turnover_rate: 10.0
  },
  ma_divergence_volume: {
    preset: 'standard',
    ma5_period: 5,
    ma10_period: 10,
    ma20_period: 20,
    gap_lookback_days: 3,
    min_above_ma5_days: 3,
    max_above_ma5_days: 4,
    volume_ma_period: 20,
    volume_surge_ratio: 1.5,
    min_volume_surge_days: 2
  },
  fundamental: {
    min_roe: 0.15,
    max_pe: 25,
    min_revenue_growth: 0.1,
    max_debt_ratio: 0.6
  },
  turtle: {
    entry_period: 20,
    exit_period: 10,
    atr_period: 20,
    risk_per_trade: 0.02
  },
  ma_convergence: {
    ma_types: ["MA5", "MA10", "MA20"],
    convergence_threshold: 0.05,
    min_convergence_days: 3,
    decline_check_period: 20,
    min_decline_pct: 0.10,
    time_frame: "daily",
    max_convergence_days: 20,
    recent_turnover_rate_min: 5,
    recent_turnover_rate_max: 100
  },
  consecutive_bullish: {
    time_period: "daily",
    min_consecutive_days: 3,
    min_rise_pct: 0.0,
    require_volume_surge: false,
    volume_surge_ratio: 1.2,
    analysis_period: 20
  }
}

export default function StockSelectionStrategy({ className }: StockSelectionStrategyProps) {
  const { showToast } = useToast()

  const queryClient = useQueryClient()

  const [selectedProfile, setSelectedProfile] = useState<StrategyProfileDto | null>(null)
  const [isEditing, setIsEditing] = useState(false)
  const [showCreate, setShowCreate] = useState(false)

  const [draftName, setDraftName] = useState('')
  const [draftDescription, setDraftDescription] = useState('')
  const [draftTemplate, setDraftTemplate] = useState('')
  const [draftSettingsText, setDraftSettingsText] = useState('')

  const [runSettingsText, setRunSettingsText] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [hasRun, setHasRun] = useState(false)
  const [executionTime, setExecutionTime] = useState<number>(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  const templatesQuery = useQuery({
    queryKey: ['strategy-templates'],
    queryFn: () => strategyApi.listStrategyTemplates(),
  })

  const profilesQuery = useQuery({
    queryKey: ['strategy-profiles'],
    queryFn: () => strategyApi.listStrategyProfiles(),
  })

  const templates = (templatesQuery.data || []) as StrategyTemplateDto[]
  const profiles = (profilesQuery.data || []) as StrategyProfileDto[]

  const selectedTemplate = useMemo(() => {
    const t = (selectedProfile?.template || draftTemplate || '').trim()
    if (!t) return null
    return templates.find((x) => x.template === t) || null
  }, [draftTemplate, selectedProfile?.template, templates])

  const runQuery = useQuery({
    queryKey: ['strategy-result', selectedProfile?.id, runSettingsText],
    queryFn: () => {
      const template = selectedProfile?.template as StrategyType
      return strategyApi.runStrategy(template, JSON.parse(runSettingsText || '{}'))
    },
    enabled: false,
    staleTime: 5 * 60 * 1000,
  })

  const createMutation = useMutation({
    mutationFn: (payload: { name: string; description?: string; template: string; settings?: any }) =>
      strategyApi.createStrategyProfile(payload),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['strategy-profiles'] })
      setShowCreate(false)
      setDraftName('')
      setDraftDescription('')
      setDraftTemplate('')
      setDraftSettingsText('')
      showToast('创建成功', 'success')
    },
    onError: (e: any) => {
      showToast(e?.message || '创建失败', 'error')
    },
  })

  const updateMutation = useMutation({
    mutationFn: (payload: { id: number; body: { name?: string; description?: string; template?: string; settings?: any } }) =>
      strategyApi.updateStrategyProfile(payload.id, payload.body),
    onSuccess: async (data: any) => {
      await queryClient.invalidateQueries({ queryKey: ['strategy-profiles'] })
      setIsEditing(false)
      setShowCreate(false)
      setSelectedProfile(data as StrategyProfileDto)
      setIsEditing(false)
      showToast('更新成功', 'success')
    },
    onError: (e: any) => {
      showToast(e?.message || '更新失败', 'error')
    },
  })

  const deleteMutation = useMutation({
    mutationFn: (id: number) => strategyApi.deleteStrategyProfile(id),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ['strategy-profiles'] })
      if (selectedProfile) setSelectedProfile(null)
      showToast('删除成功', 'success')
    },
    onError: (e: any) => {
      showToast(e?.message || '删除失败', 'error')
    },
  })

  // 从API响应中提取数据
  const allResults = Array.isArray(runQuery.data?.data) ? runQuery.data?.data : []

  // 调试：打印第一条数据查看结构
  if (allResults.length > 0) {
    console.log('📊 策略结果第一条数据:', allResults[0])
    console.log('📊 concepts字段:', allResults[0].concepts)
  }

  // 计算分页数据
  const totalItems = allResults.length
  const totalPages = Math.ceil(totalItems / pageSize)
  const startIndex = (page - 1) * pageSize
  const endIndex = startIndex + pageSize
  const strategyResult = allResults.slice(startIndex, endIndex)

  const runStrategy = async () => {
    if (!selectedProfile) {
      showToast('请选择一个策略实例', 'warning')
      return
    }

    try {
      JSON.parse(runSettingsText || '{}')
    } catch (error) {
      showToast('参数格式错误，请输入有效的JSON格式', 'error')
      return
    }

    try {
      setHasRun(true)
      setIsRunning(true)
      setPage(1)
      const startTime = Date.now()
      const result = await runQuery.refetch()
      const endTime = Date.now()
      setExecutionTime(endTime - startTime)
      setIsRunning(false)

      if (result.error) {
        showToast(`策略运行失败：${(result.error as any).message}`, 'error')
      } else {
        showToast(`策略运行成功，找到 ${Array.isArray(result.data?.data) ? result.data?.data.length : 0} 只股票`, 'success')
      }
    } catch (error: any) {
      setIsRunning(false)
      showToast(`策略运行失败：${error.message || '未知错误'}`, 'error')
    }
  }

  const resetDraft = () => {
    setDraftName('')
    setDraftDescription('')
    setDraftTemplate('')
    setDraftSettingsText('')
  }

  const fillSettingsFromTemplate = (tpl: StrategyTemplateDto | null) => {
    if (!tpl) return
    const obj: Record<string, any> = {}
    tpl.params.forEach((p) => {
      if (p.default_value !== undefined) obj[p.key] = p.default_value
    })
    setDraftSettingsText(JSON.stringify(obj, null, 2))
  }

  const fillRunSettingsFromProfile = (profile: StrategyProfileDto) => {
    const settingsObj = profile.settings || DEFAULT_PARAMS[profile.template] || {}
    setRunSettingsText(JSON.stringify(settingsObj, null, 2))
  }

  useEffect(() => {
    if (!showCreate) return
    if (!draftTemplate) return
    const tpl = templates.find((t) => t.template === draftTemplate) || null
    if (!tpl) return
    if (draftSettingsText.trim()) return
    fillSettingsFromTemplate(tpl)
  }, [draftSettingsText, draftTemplate, showCreate, templates])

  useEffect(() => {
    if (!selectedProfile) return
    setHasRun(false)
    fillRunSettingsFromProfile(selectedProfile)
  }, [selectedProfile?.id])

  return (
    <div className={className}>
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            选股策略
          </CardTitle>
          <CardDescription>
            {selectedProfile ? (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <button
                  className="hover:underline"
                  onClick={() => {
                    setSelectedProfile(null)
                    setIsEditing(false)
                    setShowCreate(false)
                    setExecutionTime(0)
                    setHasRun(false)
                  }}
                >
                  策略列表
                </button>
                <span>/</span>
                <span className="text-foreground">{selectedProfile.name}</span>
              </div>
            ) : (
              <div className="text-sm text-muted-foreground">策略列表</div>
            )}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {selectedProfile === null ? (
            <>
              <div className="flex flex-wrap items-center justify-between gap-2 mb-3">
                <div className="text-sm text-muted-foreground">
                  共 {profiles.length} 条
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    onClick={() => {
                      resetDraft()
                      setShowCreate((v) => !v)
                      setIsEditing(false)
                    }}
                  >
                    <Plus className="h-4 w-4 mr-2" />
                    新建策略
                  </Button>
                </div>
              </div>

              {showCreate ? (
                <div className="border rounded-md p-4 mb-4 space-y-3">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div>
                      <div className="text-sm font-medium mb-1">名称</div>
                      <Input value={draftName} onChange={(e) => setDraftName(e.target.value)} placeholder="例如：换手率涨幅-标准" />
                    </div>
                    <div>
                      <div className="text-sm font-medium mb-1">模板</div>
                      <select
                        value={draftTemplate}
                        onChange={(e) => {
                          const v = e.target.value
                          setDraftTemplate(v)
                          const tpl = templates.find((t) => t.template === v) || null
                          setDraftSettingsText('')
                          fillSettingsFromTemplate(tpl)
                        }}
                        className="w-full px-3 py-2 border rounded-md text-sm"
                      >
                        <option value="">请选择</option>
                        {templates.map((t) => (
                          <option key={t.template} value={t.template}>
                            {t.label}
                          </option>
                        ))}
                      </select>
                      {draftTemplate ? (
                        <div className="text-xs text-muted-foreground mt-1">
                          {templates.find((t) => t.template === draftTemplate)?.description}
                        </div>
                      ) : null}
                    </div>
                    <div>
                      <div className="text-sm font-medium mb-1">描述</div>
                      <Input value={draftDescription} onChange={(e) => setDraftDescription(e.target.value)} placeholder="可选：简单说明策略用途/场景" />
                    </div>
                  </div>

                  <div>
                    <div className="text-sm font-medium mb-1">参数（JSON）</div>
                    <Textarea
                      value={draftSettingsText}
                      onChange={(e) => setDraftSettingsText(e.target.value)}
                      placeholder="请输入 JSON"
                      className="font-mono"
                      rows={8}
                    />
                  </div>

                  <div className="flex items-center gap-2">
                    <Button
                      onClick={() => {
                        try {
                          const settings = draftSettingsText.trim() ? JSON.parse(draftSettingsText) : undefined
                          if (!draftName.trim()) {
                            showToast('请填写名称', 'warning')
                            return
                          }
                          if (!draftTemplate.trim()) {
                            showToast('请选择模板', 'warning')
                            return
                          }
                          createMutation.mutate({
                            name: draftName.trim(),
                            description: draftDescription.trim() ? draftDescription.trim() : undefined,
                            template: draftTemplate.trim(),
                            settings,
                          })
                        } catch (e) {
                          showToast('参数格式错误，请输入有效的JSON格式', 'error')
                        }
                      }}
                      disabled={createMutation.isPending}
                    >
                      保存
                    </Button>
                    <Button
                      variant="outline"
                      onClick={() => {
                        setShowCreate(false)
                        resetDraft()
                      }}
                    >
                      取消
                    </Button>
                  </div>
                </div>
              ) : null}

              <div className="overflow-x-auto">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead className="whitespace-nowrap min-w-[80px]">ID</TableHead>
                      <TableHead className="whitespace-nowrap min-w-[200px]">名称</TableHead>
                      <TableHead className="whitespace-nowrap min-w-[260px]">描述</TableHead>
                      <TableHead className="whitespace-nowrap min-w-[220px]">模板</TableHead>
                      <TableHead className="whitespace-nowrap min-w-[160px]">更新时间</TableHead>
                      <TableHead className="whitespace-nowrap min-w-[200px]">操作</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {profiles.map((p) => (
                      <TableRow key={p.id}>
                        <TableCell className="font-mono">{p.id}</TableCell>
                        <TableCell>
                          <button
                            className="hover:underline"
                            onClick={() => {
                              setSelectedProfile(p)
                              setIsEditing(false)
                              setShowCreate(false)
                              fillRunSettingsFromProfile(p)
                            }}
                          >
                            {p.name}
                          </button>
                        </TableCell>
                        <TableCell className="text-sm text-muted-foreground">{p.description || '-'}</TableCell>
                        <TableCell className="text-sm text-muted-foreground">
                          {templates.find((t) => t.template === p.template)?.label || p.template}
                        </TableCell>
                        <TableCell className="text-sm text-muted-foreground">{p.updated_at}</TableCell>
                        <TableCell>
                          <div className="flex items-center gap-2">
                            <Button
                              variant="outline"
                              size="sm"
                              onClick={() => {
                                setSelectedProfile(p)
                                setShowCreate(false)
                                setIsEditing(true)
                                setDraftName(p.name)
                                setDraftDescription(p.description || '')
                                setDraftTemplate(p.template)
                                setDraftSettingsText(JSON.stringify(p.settings || {}, null, 2))
                              }}
                            >
                              <Pencil className="h-4 w-4 mr-1" />
                              编辑
                            </Button>
                            <Button
                              variant="outline"
                              size="sm"
                              onClick={() => {
                                if (window.confirm(`确认删除策略：${p.name} ?`)) {
                                  deleteMutation.mutate(p.id)
                                }
                              }}
                              disabled={deleteMutation.isPending}
                            >
                              <Trash2 className="h-4 w-4 mr-1" />
                              删除
                            </Button>
                          </div>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            </>
          ) : (
            <>
              <div className="flex items-center justify-between gap-2 mb-3">
                <div className="text-sm text-muted-foreground">
                  模板：{templates.find((t) => t.template === selectedProfile.template)?.label || selectedProfile.template}
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    variant="outline"
                    onClick={() => {
                      setSelectedProfile(null)
                      setIsEditing(false)
                      setShowCreate(false)
                      setExecutionTime(0)
                      setHasRun(false)
                    }}
                  >
                    <ArrowLeft className="h-4 w-4 mr-2" />
                    返回列表
                  </Button>
                  <Button
                    variant="outline"
                    onClick={() => {
                      setShowCreate(false)
                      setIsEditing((v) => !v)
                      setDraftName(selectedProfile.name)
                      setDraftDescription(selectedProfile.description || '')
                      setDraftTemplate(selectedProfile.template)
                      setDraftSettingsText(JSON.stringify(selectedProfile.settings || {}, null, 2))
                    }}
                  >
                    <Pencil className="h-4 w-4 mr-2" />
                    {isEditing ? '取消编辑' : '编辑'}
                  </Button>
                </div>
              </div>

              {isEditing ? (
                <div className="border rounded-md p-4 mb-4 space-y-3">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div>
                      <div className="text-sm font-medium mb-1">名称</div>
                      <Input value={draftName} onChange={(e) => setDraftName(e.target.value)} />
                    </div>
                    <div>
                      <div className="text-sm font-medium mb-1">模板</div>
                      <select
                        value={draftTemplate}
                        onChange={(e) => {
                          const v = e.target.value
                          setDraftTemplate(v)
                        }}
                        className="w-full px-3 py-2 border rounded-md text-sm"
                      >
                        <option value="">请选择</option>
                        {templates.map((t) => (
                          <option key={t.template} value={t.template}>
                            {t.label}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div>
                      <div className="text-sm font-medium mb-1">描述</div>
                      <Input value={draftDescription} onChange={(e) => setDraftDescription(e.target.value)} />
                    </div>
                  </div>

                  <div>
                    <div className="text-sm font-medium mb-1">参数（JSON）</div>
                    <Textarea value={draftSettingsText} onChange={(e) => setDraftSettingsText(e.target.value)} className="font-mono" rows={8} />
                  </div>

                  <div className="flex items-center gap-2">
                    <Button
                      onClick={() => {
                        try {
                          const settings = draftSettingsText.trim() ? JSON.parse(draftSettingsText) : undefined
                          if (!draftName.trim()) {
                            showToast('请填写名称', 'warning')
                            return
                          }
                          if (!draftTemplate.trim()) {
                            showToast('请选择模板', 'warning')
                            return
                          }
                          if (!selectedProfile) return
                          updateMutation.mutate({
                            id: selectedProfile.id,
                            body: {
                              name: draftName.trim(),
                              description: draftDescription.trim() ? draftDescription.trim() : undefined,
                              template: draftTemplate.trim(),
                              settings,
                            },
                          })
                        } catch (e) {
                          showToast('参数格式错误，请输入有效的JSON格式', 'error')
                        }
                      }}
                      disabled={updateMutation.isPending}
                    >
                      保存修改
                    </Button>
                  </div>
                </div>
              ) : null}

              {!isEditing ? (
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <div className="space-y-4">
                    <div>
                      <div className="text-sm font-medium mb-2">运行参数（JSON）</div>
                      <Textarea
                        value={runSettingsText}
                        readOnly
                        className="font-mono"
                        rows={10}
                      />
                    </div>
                    <div className="flex items-center gap-2">
                      <Button onClick={runStrategy} disabled={isRunning}>
                        <Play className="h-4 w-4 mr-2" />
                        {isRunning ? '运行中...' : '运行策略'}
                      </Button>
                      <Button
                        variant="outline"
                        onClick={() => {
                          fillRunSettingsFromProfile(selectedProfile)
                        }}
                      >
                        重置参数
                      </Button>
                    </div>
                  </div>

                  <div className="space-y-4">
                    <div className="border rounded-md p-4">
                      <div className="text-sm font-medium mb-2">模板参数提示</div>
                      {selectedTemplate ? (
                        <div className="space-y-2">
                          {selectedTemplate.params.map((p) => (
                            <div key={p.key} className="text-sm">
                              <span className="font-medium">{p.label}</span>
                              <span className="text-muted-foreground">（{p.key}）</span>
                              {p.description ? <div className="text-xs text-muted-foreground">{p.description}</div> : null}
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-sm text-muted-foreground">暂无</div>
                      )}
                    </div>
                  </div>
                </div>
              ) : null}
            </>
          )}
        </CardContent>
      </Card>

      {/* 策略结果 */}
      {runQuery.isFetching && selectedProfile && !isEditing && hasRun && (
        <Card>
          <CardContent className="py-8">
            <div className="flex items-center justify-center">
              <div className="text-muted-foreground">策略运行中，请稍候...</div>
            </div>
          </CardContent>
        </Card>
      )}

      {runQuery.error && selectedProfile && !isEditing && hasRun && (
        <Card>
          <CardContent className="py-8">
            <div className="text-center">
              <AlertTriangle className="h-8 w-8 text-destructive mx-auto mb-2" />
              <p className="text-destructive mb-4">策略运行失败</p>
              <button 
                onClick={runStrategy}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                重新运行
              </button>
            </div>
          </CardContent>
        </Card>
      )}

      {selectedProfile && allResults.length > 0 && !isEditing && hasRun && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              策略运行结果
            </CardTitle>
            <CardDescription>
              {selectedProfile.name} - 共找到 {totalItems} 只符合条件的股票
              {executionTime > 0 && ` · 运行耗时 ${(executionTime / 1000).toFixed(2)}s`}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {/* 股票列表 */}
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="whitespace-nowrap min-w-[100px]">股票代码</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">股票名称</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">当前价格</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[100px]">涨跌幅</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[200px]">核心概念</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">信号强度</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[350px]">分析结果</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {strategyResult.map((item: any, index: number) => (
                    <TableRow key={item.ts_code || index}>
                      <TableCell className="font-medium font-mono whitespace-nowrap min-w-[100px]">{item.ts_code}</TableCell>
                      <TableCell className="whitespace-nowrap min-w-[120px]">{item.stock_name}</TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[120px]">
                        ¥{formatNumber(item.strategy_result?.current_price || 0, 2)}
                      </TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[100px]">
                        <span className={`font-medium ${
                          (item.strategy_result?.pct_chg || 0) > 0 ? 'text-red-600' :
                          (item.strategy_result?.pct_chg || 0) < 0 ? 'text-green-600' :
                          'text-gray-600'
                        }`}>
                          {(item.strategy_result?.pct_chg || 0) > 0 ? '+' : ''}
                          {formatNumber(item.strategy_result?.pct_chg || 0, 2)}%
                        </span>
                      </TableCell>
                      <TableCell className="text-sm whitespace-nowrap min-w-[200px]">
                        {item.concepts || 'N/A'}
                      </TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[120px]">
                        <div className="flex items-center gap-2">
                          <div className="flex-1 bg-gray-200 rounded-full h-2 min-w-[60px]">
                            <div
                              className={`h-2 rounded-full ${
                                item.strategy_result?.signal_strength >= 100 ? 'bg-green-600' :
                                item.strategy_result?.signal_strength >= 80 ? 'bg-blue-600' :
                                'bg-yellow-600'
                              }`}
                              style={{ width: `${Math.min((item.strategy_result?.signal_strength || 0), 100)}%` }}
                            ></div>
                          </div>
                          <span className="text-sm font-medium min-w-[40px]">
                            {item.strategy_result?.signal_strength || 0}
                          </span>
                        </div>
                      </TableCell>
                      <TableCell className="text-sm whitespace-nowrap min-w-[350px]">
                        {item.strategy_result?.analysis_description || 'N/A'}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>

            {/* 分页控件 */}
            {totalPages > 1 && (
              <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mt-6 pt-4 border-t">
                <div className="flex items-center gap-4">
                  <div className="text-sm text-muted-foreground">
                    显示 {startIndex + 1} - {Math.min(endIndex, totalItems)} 条，共 {totalItems} 条
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-sm text-muted-foreground">每页显示</span>
                    <select
                      value={pageSize}
                      onChange={(e) => {
                        setPageSize(Number(e.target.value))
                        setPage(1)
                      }}
                      className="px-2 py-1 border rounded text-sm"
                    >
                      <option value={10}>10</option>
                      <option value={20}>20</option>
                      <option value={50}>50</option>
                      <option value={100}>100</option>
                    </select>
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setPage(1)}
                    disabled={page === 1}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    首页
                  </button>
                  <button
                    onClick={() => setPage(page - 1)}
                    disabled={page === 1}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    上一页
                  </button>
                  <span className="text-sm text-muted-foreground">
                    第 {page} / {totalPages} 页
                  </span>
                  <button
                    onClick={() => setPage(page + 1)}
                    disabled={page === totalPages}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    下一页
                  </button>
                  <button
                    onClick={() => setPage(totalPages)}
                    disabled={page === totalPages}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    末页
                  </button>
                </div>
              </div>
            )}

          </CardContent>
        </Card>
      )}

      {/* 无结果提示 */}
      {runQuery.data && selectedProfile && allResults.length === 0 && (
        <Card>
          <CardContent className="py-8">
            <div className="text-center">
              <Settings className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <p className="text-muted-foreground mb-2">当前策略参数下未找到符合条件的股票</p>
              <p className="text-sm text-muted-foreground">请尝试调整策略参数或选择其他策略类型</p>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
