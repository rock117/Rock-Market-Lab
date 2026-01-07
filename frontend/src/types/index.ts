// 投资组合相关类型定义

// 后端API返回的持仓数据格式
export interface ApiHolding {
  id: number; // 持仓ID
  symbol: string; // 股票代码
  name: string; // 股票名称
  exchange_id?: string; // 交易所ID
  portfolio_id: number; // 投资组合ID
  desc?: string; // 描述/备注
  added_date?: string; // 添加日期

  order?: number

  current_price?: number | null

  pct_chg?: number | null
  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null
}

// 任务管理 API 类型
export interface ApiTaskInfo {
  name: string
  schedule?: string | null
}

export interface ApiTaskStateView {
  task_name: string
  status: string
  paused: boolean
  stopped: boolean
  last_started_at?: string | null
  last_ended_at?: string | null
  last_success_count: number
  last_fail_count: number
  updated_at?: string | null
}

export interface ApiTaskListItem {
  info: ApiTaskInfo
  state: ApiTaskStateView
}

// 概念板块（东方财富）API 类型
export interface ApiDcIndex {
  ts_code: string
  trade_date: string
  name?: string | null
  leading?: string | null
  leading_code?: string | null
  pct_change?: string | number | null
  leading_pct?: string | number | null
  total_mv?: string | number | null
  turnover_rate?: string | number | null
  up_num?: number | null
  down_num?: number | null
}

export interface ApiDcMember {
  ts_code: string
  trade_date: string
  con_code: string
  name?: string | null
}

export interface ApiDcMemberEnriched {
  ts_code: string
  trade_date: string
  con_code: string
  name?: string | null

  pct_chg_day?: number | null
  pct_chg_latest?: number | null

  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null
}

// 后端API返回的投资组合格式（列表接口）
export interface ApiPortfolio {
  id: number; // 组合ID
  name: string; // 组合名称
  holdings_num: number; // 持仓数量
  description?: string; // 组合描述
  created_date?: string; // 创建日期
  updated_date?: string; // 更新日期
}

// 后端API返回的投资组合详情格式
export interface ApiPortfolioDetail {
  id: number; // 组合ID
  name: string; // 组合名称
  description?: string; // 组合描述
  created_date?: string; // 创建日期
  updated_date?: string; // 更新日期
  holdings?: ApiHolding[]; // 持仓列表（可选，创建时可能不返回）
}

// 前端使用的持仓数据格式（保持向后兼容）
export interface PortfolioStock {
  id: string; // 唯一标识
  symbol: string; // 股票代码
  name: string; // 股票名称
  exchange_id?: string; // 交易所ID
  portfolio_id: string; // 投资组合ID
  desc?: string; // 描述/备注
  added_date: string; // 添加日期
  tags?: string[]; // 标签ID列表

  order?: number

  current_price?: number | null

  pct_chg?: number | null
  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null
}

// 前端使用的投资组合格式（保持向后兼容）
export interface Portfolio {
  id: string; // 组合ID
  name: string; // 组合名称
  description?: string; // 组合描述
  created_date: string; // 创建日期
  updated_date: string; // 更新日期
  stocks: PortfolioStock[]; // 成分股列表
  holdings_num?: number; // 持仓数量（从列表接口获取）
}

// ETF 相关类型
export interface EtfItem {
  tsCode?: string
  listDate?: string | null
  etfType?: string | null
  cname?: string | null
  exchange?: string | null

  close?: number | null
  vol?: number | null
  amount?: number | null
  pct_chg?: number | null
  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null

  ts_code?: string
  csname?: string
  extname?: string | null
  index_code?: string | null
  index_name?: string | null
  setup_date?: string | null
  list_date?: string | null
  list_status?: string | null
  custod_name?: string | null
  mgr_name?: string | null
  mgt_fee?: string | null
  etf_type?: string | null
}

export interface EtfHolding {
  ts_code: string
  ann_date: string
  end_date: string
  symbol: string
  name?: string | null
  mkv: number
  amount: number
  stk_mkv_ratio?: number | null
  stk_float_ratio?: number | null
}

export interface StockSimilarityItem {
  ts_code: string
  name?: string | null
  similarity: number
  current_price?: number | null
  turnover_rate?: number | null
  pct_chg?: number | null
  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null
}

export interface StockSimilarityKLinePoint {
  date: string
  open: number
  high: number
  low: number
  close: number
  pct_chg: number
  turnover_rate: number
  amount?: number | null
}

export interface StockSimilarityResponse {
  items: StockSimilarityItem[]
  kline: Record<string, StockSimilarityKLinePoint[]>
}

// 美股相关类型定义
export interface UsStock {
  tsCode: string; // 股票代码
  name: string; // 公司名称
  exchangeId: string; // 交易所ID
  businessDescription?: string; // 业务描述（英文）
  businessDescriptionCn?: string; // 业务描述（中文）
  businessCountry?: string; // 业务国家
  sectorName?: string; // 行业名称（英文）
  sectorNameCn?: string; // 行业名称（中文）
  industryName?: string; // 细分行业名称（英文）
  industryNameCn?: string; // 细分行业名称（中文）
  webAddress?: string; // 官网地址
  // 保留旧字段以兼容现有代码
  symbol?: string;
  exchange?: string;
  industry?: string;
  sector?: string;
  market_cap?: number;
  pe_ratio?: number;
  roe?: number;
  list_date?: string;
  description?: string;
  website?: string;
  employee_count?: number;
  founded_date?: string;
  address?: string;
}

// 美股元数据
export interface UsStockMeta {
  sectors: string[];
  industries: string[];
}

// A股相关类型定义
export interface MarketSummary {
  trade_date: string;
  total_stocks: number;
  up_count: number;
  down_count: number;
  flat_count: number;
  limit_up_count: number;
  limit_down_count: number;
  total_volume: number;
  total_amount: number;
  avg_pct_chg: number;
}

export interface IndexData {
  ts_code: string;
  name: string;
  trade_date: string;
  close: number;
  open: number;
  high: number;
  low: number;
  pre_close: number;
  change: number;
  pct_chg: number;
  vol: number;
  amount: number;
}

// 分页响应
export interface PagedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

// 真实API响应格式
export interface ApiResponse<T> {
  data: T;
  success: boolean;
}

export interface ApiPagedData<T> {
  data: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

// 颜色工具类型
export type StockTrend = 'up' | 'down' | 'neutral';

// 图表数据类型
export interface PieChartData {
  name: string;
  value: number;
  color?: string;
}

// 成交量分布统计
export interface VolumeDistribution {
  trade_date: string;
  total_volume: number;
  total_stocks: number;
  top10_volume: number;
  top10_percentage: number;
  top30_volume: number;
  top30_percentage: number;
  top50_volume: number;
  top50_percentage: number;
  top100_volume: number;
  top100_percentage: number;
  concentration_index: number; // 集中度指数 (0-1，越接近1越集中)
  herfindahl_index: number; // 赫芬达尔指数 (0-1，越接近1越集中)
  gini_coefficient: number; // 基尼系数 (0-1，越接近1越不均衡)
}

// 股票详情相关类型定义
export interface StockDetail {
  ts_code: string;
  name: string;
  current_price: number;
  change: number;
  pct_chg: number;
  pe_ratio?: number;
  pb_ratio?: number;
  five_day_return: number;
  fundamentals: StockFundamentals;
  block_trades: BlockTrade[];
  concepts: string[];
  sectors: string[];
  shareholding_changes: ShareholdingChange[];
  margin_trading: MarginTradingData;
  shareholder_count: ShareholderData;
}

// 基本面数据
export interface StockFundamentals {
  roe: number; // 净资产收益率
  gross_margin: number; // 毛利率
  net_margin: number; // 净利率
  debt_ratio: number; // 资产负债率
  current_ratio: number; // 流动比率
  revenue_growth: number; // 营收增长率
  net_profit_growth: number; // 净利润增长率
}

// 大宗交易
export interface BlockTrade {
  trade_date: string;
  price: number;
  volume: number;
  amount: number;
  buyer: string;
  seller: string;
  premium_rate: number; // 溢价率
}

// 增减持数据
export interface ShareholdingChange {
  holder_name: string;
  change_type: 'increase' | 'decrease'; // 增持/减持
  change_shares: number; // 变动股数
  change_ratio: number; // 变动比例
  change_date: string;
  current_ratio: number; // 变动后持股比例
}

// 融资融券数据
export interface MarginTradingData {
  margin_balance: number; // 融资余额
  margin_buy: number; // 融资买入额
  short_balance: number; // 融券余额
  short_sell: number; // 融券卖出量
  margin_ratio: number; // 融资融券比例
}

export type ExchangeCode = 'SSE' | 'SZSE' | 'BSE' | 'ALL';

export interface MarginTradingKLineRequest {
  startDate: string;
  endDate: string;
  exchange: ExchangeCode;
}

export interface MarginTradingKLineResponse {
  exchange: ExchangeCode;
  startDate: string;
  endDate: string;
  data: StockHistoryData[];
}

export interface StockMarginTradingKLineRequest {
  startDate: string;
  endDate: string;
  stock: string;
}

export interface StockMarginTradingKLineResponse {
  stock: string;
  startDate: string;
  endDate: string;
  data: StockHistoryData[];
}

// 股东人数数据
export interface ShareholderData {
  holder_count: number; // 股东人数
  avg_holding: number; // 户均持股
  change_ratio: number; // 股东人数变化率
  report_date: string;
}

// K线比较相关类型定义
export type SecurityType = 'stock' | 'fund' | 'index';

export interface Security {
  code: string;
  name: string;
  type: SecurityType;
  market?: string; // 市场：SH/SZ
}

export interface KLineData {
  date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  amount?: number;
}

export interface SecurityKLineData {
  security: Security;
  data: KLineData[];
  color: string; // 图表显示颜色
}

// 证券搜索结果
export interface SecuritySearchResult {
  securities: Security[];
}

// K线图表配置
export interface KLineChartConfig {
  showVolume: boolean; // 是否显示成交量
  period: '1D' | '1W' | '1M'; // 时间周期
  indicators: string[]; // 技术指标
  dateRange: {
    start: string;
    end: string;
  };
}

// 涨跌趋势分析结果
export interface TrendAnalysis {
  correlation: number; // 相关系数 (-1 到 1)
  trend_consistency: 'high' | 'medium' | 'low'; // 趋势一致性
  sync_rate: number; // 同步率 (0-100%)
  analysis_period: string;
}

// 策略相关类型定义
export type StrategyType =
  | 'price_volume_candlestick'
  | 'bottom_volume_surge'
  | 'long_term_bottom_reversal'
  | 'yearly_high'
  | 'price_strength'
  | 'distressed_reversal'
  | 'single_limit_up'
  | 'fundamental'
  | 'consecutive_strong'
  | 'turtle'
  | 'limit_up_pullback'
  | 'strong_close'
  | 'quality_value'
  | 'turnover_ma_bullish'
  | 'low_shadow'
  | 'ma_convergence'
  | 'consecutive_bullish';

export type StrategySignal = 'BUY' | 'SELL' | 'HOLD';

export type RiskLevel = 'LOW' | 'MEDIUM' | 'HIGH';

// 策略结果中的股票信息
export interface StrategyStock {
  ts_code: string;
  name: string;
  current_price: number;
  change_percent: number;
  signal: StrategySignal;
  signal_strength: number; // 0-1之间的信号强度
  updated_at?: string;
}

// 策略性能指标
export interface StrategyPerformance {
  success_rate: number; // 成功率 0-1
  avg_return: number; // 平均收益率
  max_drawdown: number; // 最大回撤
  sharpe_ratio: number; // 夏普比率
  total_trades: number; // 总交易次数
}

// 策略运行结果
export interface StrategyResult {
  strategy_type: StrategyType;
  stocks: StrategyStock[];
  performance?: StrategyPerformance;
  risk_level: RiskLevel;
  execution_time: number; // 执行时间(毫秒)
  parameters: Record<string, any>; // 策略参数
  created_at: string;
}

// 策略配置
export interface StrategyConfig {
  strategy_type: StrategyType;
  parameters: Record<string, any>;
  enabled: boolean;
  description?: string;
}

// 股票历史价格数据
export interface StockHistoryData {
  trade_date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  amount: number;
  turnover_rate: number; // 换手率
  pct_chg: number; // 涨跌幅
  change: number; // 涨跌额
}

// 股票历史价格响应
export interface StockHistoryResponse {
  ts_code: string;
  name: string;
  data: StockHistoryData[];
  total: number;
}

// A股列表/概览
export interface AStockOverview {
  name: string
  ts_code: string
  name_py?: string | null
  list_date?: string | null
  market?: string | null
  area?: string | null
  industry?: string | null

  concepts?: string | null

  close?: number | null
  pct_chg?: number | null

  pct5?: number | null
  pct10?: number | null
  pct20?: number | null
  pct60?: number | null

  pe?: number | null
  dv_ratio?: number | null
  total_mv?: number | null
}
