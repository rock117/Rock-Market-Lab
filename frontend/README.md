# Rock Market Lab Frontend

专业的股票数据分析和投资决策平台前端应用。

## 功能特性

### 🇺🇸 美股模块
- **美股列表展示**：展示美股主要公司信息
- **基本信息**：公司名称、交易代码、交易所、行业分类
- **财务指标**：市值、PE比率、ROE、员工数量
- **公司详情**：成立时间、上市时间、主营业务、官网等
- **筛选功能**：支持按交易所、行业、财务指标筛选

### 🇨🇳 A股大盘数据汇总
- **市场概览**：涨跌家数、成交量、涨停跌停统计
- **主要指数**：上证指数、深证成指、创业板指实时行情
- **涨跌分布**：不同涨跌幅区间的股票数量分布
- **可视化图表**：饼图、柱状图展示市场数据
- **实时更新**：2分钟自动刷新数据

## 技术栈

- **框架**: Next.js 14 + React 18 + TypeScript
- **样式**: TailwindCSS + shadcn/ui
- **状态管理**: TanStack Query (React Query) + Zustand
- **图表**: Recharts
- **HTTP客户端**: Axios
- **图标**: Lucide React

## 项目结构

```
src/
├── app/                    # Next.js App Router
│   ├── layout.tsx         # 根布局
│   ├── page.tsx           # 首页
│   └── globals.css        # 全局样式
├── components/            # 组件
│   ├── ui/                # 基础UI组件
│   ├── market/            # A股市场组件
│   └── us-stocks/         # 美股组件
├── services/              # API服务
├── types/                 # TypeScript类型定义
├── lib/                   # 工具函数
└── providers/             # Context提供者
```

## 快速开始

### 安装依赖
```bash
npm install
```

### 启动开发服务器
```bash
npm run dev
```

应用将在 http://localhost:3000 启动

### 构建生产版本
```bash
npm run build
npm start
```

## API集成

前端通过代理方式连接到Rust后端API：
- 开发环境：`http://localhost:3000/api` -> `http://localhost:8080/api`
- 生产环境：需要配置相应的API网关或反向代理

### 主要API端点

#### 美股相关
- `GET /api/us-stocks` - 获取美股列表
- `GET /api/us-stocks/{symbol}` - 获取美股详情
- `GET /api/us-stocks/{symbol}/company-info` - 获取公司信息

#### A股相关
- `GET /api/market/summary` - 获取市场概览
- `GET /api/market/indices` - 获取主要指数
- `GET /api/market/price-distribution` - 获取涨跌分布

## 组件说明

### MarketSummary 组件
A股大盘数据汇总组件，包含：
- 涨跌家数统计卡片
- 成交量和成交额显示
- 涨停跌停数量
- 主要指数实时行情
- 涨跌分布饼图和柱状图

### UsStockList 组件
美股列表展示组件，包含：
- 股票基本信息表格
- 财务指标展示
- 筛选和搜索功能
- 分页导航
- 响应式设计

## 样式系统

使用TailwindCSS + CSS变量的主题系统：
- 支持深色/浅色主题
- 股票专用颜色：`text-bull`(上涨)、`text-bear`(下跌)、`text-neutral`(平盘)
- 响应式设计，支持移动端

## 数据缓存

使用TanStack Query进行数据缓存：
- 市场数据：2分钟缓存
- 美股数据：5分钟缓存
- 自动后台刷新
- 错误重试机制

## 开发规范

- 使用TypeScript严格模式
- 组件采用函数式组件 + Hooks
- 遵循ESLint规则
- 使用Prettier格式化代码
- 组件和函数添加适当的类型注解

## 部署

### Docker部署
```bash
# 构建镜像
docker build -t rock-market-frontend .

# 运行容器
docker run -p 3000:3000 rock-market-frontend
```

### 静态部署
```bash
npm run build
# 将 .next 目录部署到静态服务器
```

## 贡献指南

1. Fork项目
2. 创建功能分支
3. 提交代码
4. 创建Pull Request

## 许可证

MIT License
