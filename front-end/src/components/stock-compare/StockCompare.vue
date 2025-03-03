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

      <el-radio-group v-model="displayMode" style="margin-left: 16px" :size="size">
        <el-radio-button label="table">table</el-radio-button>
        <el-radio-button label="chart">chart</el-radio-button>
      </el-radio-group>
    </div>

    <!-- 表格视图 -->
    <div v-if="displayMode === 'table'" class="table-container">
      <el-table :data="stockData" style="width: 100%" border :size="size">
        <el-table-column prop="date" label="日期" width="120" sortable />
        <el-table-column prop="ts_code" label="股票代码" width="120" />
        <el-table-column prop="name" label="股票名称" width="120" />
        <el-table-column prop="price" label="价格" width="100" sortable>
          <template #default="scope">
            {{ scope.row.price.toFixed(2) }}
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 图表视图 -->
    <div v-else class="chart-container">
      <div ref="chartRef" style="width: 100%; height: 600px"></div>
    </div>
  </div>
</template>

<script setup>
import * as echarts from 'echarts';
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import dayjs from 'dayjs';

import {
  searchSecurity, getStockPrice
} from "@/service/index.js";


// 初始化日期：结束日期为今天，开始日期为5天前
const today = dayjs()
const startDate = ref(today.subtract(5, 'day').format('YYYY-MM-DD'))
const endDate = ref(today.format('YYYY-MM-DD'))

const selectedStocks = ref([])
const displayMode = ref('table')
const chartRef = ref(null)
const size = ref('default')
let chart = null

// 股票列表数据
const stockList = ref([])
const loading = ref(false)

// 搜索股票
const handleSearch = async (query) => {
  if (query) {
    loading.value = true
    try {
      let stocks = await searchSecurity(query)
      stockList.value = stocks.data
    } catch (error) {
      console.error('搜索股票失败:', error)
    } finally {
      loading.value = false
    }
  } else {
    stockList.value = []
  }
}

const stockData = ref([])

// 初始化图表
const initChart = () => {
  if (chart) {
    chart.dispose()
  }
  chart = echarts.init(chartRef.value)
}

// 更新图表数据
const updateChart = () => {
  if (!chart || !stockData.value.length) return

  const series = selectedStocks.value.map(ts_code => {
    const stockInfo = stockList.value.find(s => s.ts_code === ts_code)
    const data = stockData.value
      .filter(item => item.ts_code === ts_code)
      .map(item => [item.date, item.price])

    return {
      name: stockInfo.name,
      type: 'line',
      data: data,
      showSymbol: false,
    }
  })

  const option = {
    title: {
      text: '股票价格走势'
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: selectedStocks.value.map(ts_code =>
        stockList.value.find(s => s.ts_code === ts_code).name
      )
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'time',
      boundaryGap: false
    },
    yAxis: {
      type: 'value',
      scale: true,
      splitLine: {
        show: true
      }
    },
    series
  }

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

  const mockData = []
  for (const ts_code of selectedStocks.value) {
    let data = await getStockPrice(ts_code, startDate.value, endDate.value)
    console.log("data = ", data)
  }
   
  stockData.value = mockData

  if (displayMode.value === 'chart') {
    nextTick(() => {
      updateChart()
    })
  }
}

// 监听选中股票和日期范围变化
watch([selectedStocks, startDate, endDate], ([newStocks, newStartDate, newEndDate]) => {
  if (newStocks?.length > 0 && newStartDate && newEndDate) {
    loadStockData()
  }
}, { deep: true })

// 监听显示模式变化
watch(displayMode, (newMode) => {
  if (newMode === 'chart' && stockData.value.length > 0) {
    nextTick(() => {
      initChart()
      updateChart()
    })
  }
})

onMounted(() => {
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
