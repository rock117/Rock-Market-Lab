<template>
  <div class="history-compare">
    <div class="search-section">
      <el-select
        v-model="selectedStock"
        filterable
        remote
        :remote-method="handleSearch"
        :loading="searching"
        placeholder="搜索股票"
        style="width: 300px; margin-right: 16px"
      >
        <el-option
          v-for="stock in stockOptions"
          :key="stock.ts_code"
          :label="`${stock.ts_code} - ${stock.name}`"
          :value="stock.ts_code"
        />
      </el-select>

      <el-select
        v-model="selectedYears"
        multiple
        placeholder="选择年份"
        style="width: 300px"
        :disabled="!selectedStock"
      >
        <el-option
          v-for="year in yearOptions"
          :key="year.value"
          :label="year.label"
          :value="year.value"
        />
      </el-select>
    </div>

    <div class="chart-container" ref="chartContainer">
      <div v-if="!selectedStock" class="empty-hint">请先选择股票</div>
      <div v-else-if="loading" class="loading">
        <el-loading :fullscreen="false" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'
import axios from 'axios'
import { ElMessage } from 'element-plus'

const searching = ref(false)
const loading = ref(false)
const selectedStock = ref('')
const selectedYears = ref([])
const stockOptions = ref([])
const chartInstance = ref(null)
const chartContainer = ref(null)

// 生成年份选项，从2015年到当前年份
const currentYear = new Date().getFullYear()
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
    const promises = years.map(year => 
      axios.get(`/api/stocks/${selectedStock.value}/history/${year}`)
    )
    const responses = await Promise.all(promises)
    updateChart(responses.map((res, index) => ({
      year: years[index],
      data: res.data
    })))
  } catch (error) {
    console.error('获取历史数据失败:', error)
    ElMessage.error('获取历史数据失败，请重试')
  } finally {
    loading.value = false
  }
}

// 初始化图表
const initChart = () => {
  if (chartInstance.value) {
    chartInstance.value.dispose()
  }
  
  chartInstance.value = echarts.init(chartContainer.value)
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: []
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      data: []
    },
    yAxis: {
      type: 'value',
      scale: true
    },
    series: []
  }
  chartInstance.value.setOption(option)
}

// 更新图表数据
const updateChart = (yearDataList) => {
  if (!chartInstance.value) return

  const series = yearDataList.map(({ year, data }) => ({
    name: `${year}年`,
    type: 'line',
    data: data.map(item => item.price),
    smooth: true
  }))

  const option = {
    legend: {
      data: yearDataList.map(({ year }) => `${year}年`)
    },
    xAxis: {
      data: yearDataList[0].data.map((_, index) => `第${index + 1}个交易日`)
    },
    series
  }

  chartInstance.value.setOption(option)
}

// 监听窗口大小变化
window.addEventListener('resize', () => {
  chartInstance.value?.resize()
})

// 监听股票和年份变化
watch([() => selectedStock.value, () => selectedYears.value], ([stock, years]) => {
  if (stock && years?.length) {
    handleYearChange(years)
  }
})

onMounted(() => {
  // 组件加载完成后的初始化工作
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
  width: 100%;
  height: 600px;
  background: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  position: relative;
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