<template>
  <div class="history-compare">
    <div class="search-section">
      <el-select v-model="selectedStock" filterable remote :remote-method="handleSearch" :loading="searching"
        placeholder="搜索股票" style="width: 300px; margin-right: 16px">
        <el-option v-for="stock in stockOptions" :key="stock.ts_code" :label="`${stock.ts_code} - ${stock.name}`"
          :value="stock.ts_code" />
      </el-select>

      <el-select v-model="selectedYears" multiple placeholder="选择年份" style="width: 300px" :disabled="!selectedStock">
        <el-option v-for="year in yearOptions" :key="year.value" :label="year.label" :value="year.value" />
      </el-select>

      <el-select v-model="selectedPeriod" placeholder="选择周期" style="width: 300px" :disabled="!selectedStock">
        <el-option v-for="period in periodOptions" :key="period.value" :label="period.label" :value="period.value" />
      </el-select>
    </div>

    <div class="chart-container">
      <!-- <div v-if="!selectedStock" class="empty-hint">请先选择股票</div> -->
      <!-- <div v-else-if="loading" class="loading">
        <el-loading :fullscreen="false" />
      </div> -->
      <div class="chart" ref="chartContainer" style="width: 100%; height: 600px"></div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'
import axios from 'axios'
import { ElMessage } from 'element-plus'
import { getSecurityHistoryCompare } from '@/service/index.js'
import { normalizeStockPrices } from '@/components/stock-compare/stockPriceNormalizer.js'
const searching = ref(false)
const loading = ref(false)
const selectedPeriod = ref('Day')
const selectedStock = ref('')
const selectedYears = ref([])
const stockOptions = ref([])
const chartContainer = ref(null)
let chart = null
// 生成年份选项，从2015年到当前年份
const yearOptions = [
  {
    value: 2020,
    label: '2020年'
  },
  {
    value: 2021,
    label: '2021年'
  },
  {
    value: 2022,
    label: '2022年'
  },
  {
    value: 2023,
    label: '2023年'
  },
  {
    value: 2024,
    label: '2024年'
  },
  {
    value: 2025,
    label: '2025年'
  }
]

const periodOptions = [
  {
    value: 'Day',
    label: '日线'
  },
  {
    value: 'Week',
    label: '周线'
  },
  {
    value: 'Month',
    label: '月线'
  }
]

// 股票搜索
const handleSearch = async (value) => {
  if (!value) {
    stockOptions.value = []
    return
  }

  searching.value = true
  try {
    const response = await axios.get(`/api/securities/search?keyword=${encodeURIComponent(value)}`)
    stockOptions.value = response.data.data
  } catch (error) {
    console.error('搜索股票失败:', error)
    ElMessage.error('搜索股票失败，请重试')
  } finally {
    searching.value = false
  }
}

// 处理股票选择变化
const handleStockChange = (value) => {
  selectedStock.value = value
  selectedYears.value = [] // 清空已选年份
  if (value) {
    initChart() // 初始化图表
  }
}

// 处理年份选择变化
const handleYearChange = async (years) => {
  if (!selectedStock.value || !years.length) return

  loading.value = true
  try {
    const response = await getSecurityHistoryCompare('Stock', selectedStock.value, years, selectedPeriod.value)
    const { trade_dates, yearDataMap } = buildData(response, selectedYears.value)
    updateChart(selectedYears.value, trade_dates, yearDataMap)
  } catch (error) {
    console.error('获取历史数据失败:', error)
    ElMessage.error('获取历史数据失败，请重试')
  } finally {
    loading.value = false
  }
}

const buildData = (data, years) => {
  const datas = data.data
  const yearDatas = years.map(year => datas[year])
  const { trade_dates, normalizedPrices } = normalizeStockPrices(...yearDatas)
  const yearDataMap = {}
  for (let i = 0; i < normalizedPrices.length; i++) {
    let year = parseYear(normalizedPrices[i])
    yearDataMap[year] = normalizedPrices[i]
  }
  return { trade_dates, yearDataMap }
}

const parseYear = (normalizedPrices) => {
  const targets = normalizedPrices.filter(price => price && price.trade_date)
  const date = targets[0].trade_date.substring(0, 4)
  return date
}

// 初始化图表
const initChart = () => {
  if (chart) {
    chart.dispose()
  }
  chart = echarts.init(chartContainer.value)
}

// 更新图表数据
const updateChart = (years, dates, yearDatas) => {
  dates = dates.map(date => date.substring(0, 2) + "/" + date.substring(2, 4))
  if (!chart) return
  const series = years.map(year => ({
    name: year + "年",
    type: 'line',
    data: yearDatas[year].map(item => item.close),

  }))

  let option = {
    title: {
      text: '股票价格走势'
    },
    legend: {
      data: years.map(y => y + "年")
    },
    tooltip: {
      trigger: 'axis'
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: dates
    },
    yAxis: {
      type: 'value',
      name: "股价"
    },
    series
  }

  console.log("history compare option = ", option)
  chart.setOption(option)
}

// 监听窗口大小变化
window.addEventListener('resize', () => {
  chart.resize()
})

// 监听股票和年份变化
watch([() => selectedStock.value, () => selectedYears.value], ([stock, years]) => {
  if (stock && years?.length) {
    handleYearChange(years)
  }
})

onMounted(() => {
  // 组件加载完成后的初始化工作
  initChart()
})
</script>

<style scoped>
.history-compare {
  padding: 20px;
}

.search-section {
  margin-bottom: 20px;
  display: flex;
  gap: 16px;
}

.chart-container {
  margin-top: 20px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 20px;
}

.empty-hint {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: var(--el-text-color-secondary);
}

.loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>