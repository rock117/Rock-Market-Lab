<template>
  <div class="stock-price-chart">
    <!-- 控制区域 -->
    <div class="controls">
      <el-date-picker v-model="startDate" type="date" :size="size" value-format="YYYY-MM-DD" />
      <el-date-picker v-model="endDate" type="date" :size="size" value-format="YYYY-MM-DD" />
      <el-select v-model="selectedStocks" multiple filterable remote :remote-method="handleSearch" :loading="loading"
        clearable collapse-tags :max-collapse-tags="1" collapse-tags-tooltip placeholder="请输入股票代码或名称搜索"
        style="width: 300px; margin-left: 16px" :size="size">
        <el-option v-for="stock in stockList" :key="stock.ts_code" :label="`${stock.name} (${stock.ts_code})`"
          :value="stock.ts_code" />
      </el-select>

    </div>

    <!-- 图表视图 -->
    <div class="chart-container">
      <div ref="chartRef" style="width: 100%; height: 600px"></div>
    </div>
  </div>
</template>

<script setup>
import * as echarts from 'echarts';
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import dayjs from 'dayjs';
import * as ChartUtil from './ChartUtil.js';

import {
  searchSecurity, getStockPrice
} from "@/service/index.js";


// 初始化日期：结束日期为今天，开始日期为5天前
const today = dayjs()
const startDate = ref(today.subtract(5, 'day').format('YYYY-MM-DD'))
const endDate = ref(today.format('YYYY-MM-DD'))

const selectedStocks = ref([])
const chartRef = ref(null)
const size = ref('default')
let chart = null

// 股票列表数据
const stockList = ref([])
const loading = ref(false)
const stocksPrices = ref({})
const stocksNames = ref({})
// 搜索股票
const handleSearch = async (query) => {
  if (query) {
    loading.value = true
    try {
      let stocks = await searchSecurity(query)
      stockList.value = stocks.data
      initStockName()
    } catch (error) {
      console.error('搜索股票失败:', error)
    } finally {
      loading.value = false
    }
  } else {
    stockList.value = []
  }
}

const initStockName = () => {
  for (const stock of stockList.value) {
    stocksNames.value[stock.ts_code] = stock
  }
}
 

// 初始化图表
const initChart = () => {
  if (chart) {
    chart.dispose()
  }
  chart = echarts.init(chartRef.value)
}

// 更新图表数据
const updateChart = () => {
  if (!chart) {
    return
  }

  const option = ChartUtil.buildChartOption(selectedStocks.value, stocksNames.value, stocksPrices.value)
  chart.setOption(option)
}

// 处理日期变化
const handleDateChange = () => {
  if (startDate.value && endDate.value && selectedStocks.value.length > 0) {
    loadStockData()
  }
}

// 加载股票数据
const loadStockData = async () => {
  if (!startDate.value || !endDate.value || !selectedStocks.value.length) {
    stockData.value = []
    return
  }

  for (const ts_code of selectedStocks.value) {
    let type = stocksNames.value[ts_code].type
    let data = await getStockPrice(type, ts_code, startDate.value, endDate.value)
    data.data.sort((a, b) => a.trade_date.localeCompare(b.trade_date))
    stocksPrices.value[ts_code] = data.data
  }

  nextTick(() => {
    updateChart()
  })
}

// 监听选中股票和日期范围变化
watch([selectedStocks, startDate, endDate], ([newStocks, newStartDate, newEndDate]) => {
  if (newStocks?.length > 0 && newStartDate && newEndDate) {
    loadStockData()
  }
}, { deep: true })



onMounted(() => {
  initChart()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (chart) {
    chart.dispose()
    chart = null
  }
})

const handleResize = () => {
  if (chart) {
    chart.resize()
  }
}
</script>

<style scoped>
.stock-price-chart {
  padding: 20px;
}

.controls {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
}

.table-container {
  margin-top: 20px;
}

.chart-container {
  margin-top: 20px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 20px;
}
</style>
