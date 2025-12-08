// 美股相关类型定义
export interface UsStock {
  symbol: string;
  name: string;
  exchange: string;
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

export interface UsCompanyInfo {
  symbol: string;
  company_name: string;
  exchange: string;
  founded_date?: string;
  employee_count?: number;
  address?: string;
  website?: string;
  description?: string;
}

export interface UsDaily {
  symbol: string;
  trade_date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  amount?: number;
}

// A股相关类型定义
export interface StockDaily {
  ts_code: string;
  trade_date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  pre_close: number;
  change: number;
  pct_chg: number;
  vol: number;
  amount: number;
}

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

// 涨跌分布数据
export interface PriceDistribution {
  range: string; // 如 ">9%", "7-9%", "5-7%" 等
  count: number;
  percentage: number;
}

// 均线数据
export interface MovingAverageData {
  ma5: number;
  ma10: number;
  ma20: number;
  ma60: number;
  ma120: number;
  ma250: number;
}

// API响应类型
export interface ApiResponse<T> {
  code: number;
  message: string;
  data: T;
}

// 分页响应
export interface PagedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

// 通用筛选参数
export interface FilterParams {
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

// 美股筛选参数
export interface UsStockFilterParams extends FilterParams {
  exchange?: string;
  industry?: string;
  sector?: string;
  min_market_cap?: number;
  max_market_cap?: number;
  min_pe?: number;
  max_pe?: number;
  min_roe?: number;
  max_roe?: number;
}

// 颜色工具类型
export type StockTrend = 'up' | 'down' | 'neutral';

// 图表数据类型
export interface ChartDataPoint {
  date: string;
  value: number;
  label?: string;
}

export interface PieChartData {
  name: string;
  value: number;
  color?: string;
}
