<template>
  <el-autocomplete
    v-model="stockDisplay"
    value-key="label"
    :fetch-suggestions="querySearch"
    clearable
    class="inline-input w-50"
    placeholder="Please Input"
    @select="handleSelect"
    width="40px"
  />
  <el-select
    v-model="periodType"
    placeholder="Select"
    @change="onPeriodChanged"
    style="width: 240px"
  >
    <el-option
      v-for="item in options"
      :key="item.value"
      :label="item.label"
      :value="item.value"
    />
  </el-select>
  <el-date-picker
    v-if="isCustom"
    v-model="startDate"
    type="date"
    @change="onStartChanged"
    placeholder="开始时间"
    :size="size"
  />
  <el-date-picker
    v-if="isCustom"
    v-model="endDate"
    @change="onEndChanged"
    type="date"
    placeholder="结束时间"
    :size="size"
  />
  <div>
    <span>均线:</span>
    <span>MA5: {{ toFixedNum(summary_info?.price_macd?.m5) }}</span>
    <span>MA10: {{ toFixedNum(summary_info?.price_macd?.m10) }}</span>
    <span>MA20: {{ toFixedNum(summary_info?.price_macd?.m20) }}</span>
    <span>MA60: {{ toFixedNum(summary_info?.price_macd?.m60) }}</span>
    <span>MA120: {{ toFixedNum(summary_info?.price_macd?.m120) }}</span>
    <span>MA250: {{ toFixedNum(summary_info?.price_macd?.m250) }}</span>
  </div>

  <div>
    <span>统计:</span>
    <span>上涨天数: {{ summary_info?.inc_num }}</span>
    <span>下跌天数: {{ summary_info?.dec_num }}</span>
    <span>平盘天数: {{ summary_info?.eq_num }}</span>
    <span>涨跌幅: {{ toPercent(summary_info?.inc_percent) }}</span>
  </div>

  <div>
    <span>价格:</span>
    <span>平均: {{ toFixedNum(summary_info?.price_avg) }}</span>
    <span>最大: {{ toFixedNum(summary_info?.price_max) }}</span>
    <span>最小: {{ toFixedNum(summary_info?.price_min) }}</span>
    <span>标准差: {{ toFixedNum(summary_info?.price_stdv) }}</span>
  </div>

  <div>
    <span>成交量(亿元):</span>
    <span>平均: {{ amountTo100Million(summary_info?.amount_avg, true) }}</span>
    <span>最大: {{ amountTo100Million(summary_info?.amount_max, true) }}</span>
    <span>最小: {{ amountTo100Million(summary_info?.amount_min, true) }}</span>
    <span>标准差: {{ amountTo100Million(summary_info?.amount_stdv, true) }}</span>
  </div>
  <StockChart v-if="viewType == 'chart'" :stockData="stockData"/>
  <el-table v-if="viewType == 'table'" :data="tableData" stripe  style="width: 100%" v-loading="loading">
    <el-table-column prop="date" label="日期" width="180" />
    <el-table-column prop="price" label="价格" />
    <el-table-column prop="low" label="最低" />
    <el-table-column prop="high" label="最高" />
    <el-table-column prop="percent" label="涨跌%">
      <template #default="scope">
          <span :class=" parseFloat(scope.row.percent) >= 0? 'red': 'green'" >{{ scope.row.percent + '%' }}</span>
      </template>
      </el-table-column>
    <el-table-column prop="range" label="波动%" />
    <el-table-column prop="amount" label="成交量(亿元)" width="180" />
    <el-table-column prop="turnover_rate" label="换手率" width="180" />
  </el-table>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import { getStockDaily, searchSecurity } from "@/service/index.js";
import { getNowStr, formateDate, toPercent, toFixedNum, amountTo100Million, calcIncPercent, calcRangePercent } from "@/util/util.ts";
import StockChart from "../chart/chart.vue"

import * as dayjs from "dayjs";
const viewType = ref('table')
const loading = ref(true);
const startEnd = getStartEnd();
const tableData = ref([]);
const value = ref(null);
const stocks = ref([]);
const allStocks = ref([]);
const stockDisplay = ref("002747.SZ");
const secCode = ref("000963.SZ");
const startDate = ref(startEnd.start);
const endDate = ref(startEnd.end);
const isCustom = ref(true);
const summary_info = ref({});
const periodType = ref("custom");
const stockData = ref({})
const options = [
  {
    value: "5",
    label: "近5日",
  },
  {
    value: "10",
    label: "近10日",
  },
  {
    value: "20",
    label: "近20日",
  },
  {
    value: "60",
    label: "近60日",
  },
  {
    value: "250",
    label: "近250日",
  },
  {
    value: "custom",
    label: "自定义",
  },
];

onMounted(async () => {
  await loadAllSecurities();
  await loadData();
});

const loadAllSecurities = async () => {
  const data = await searchSecurity("");
  const stockOptions = data.data.map((stock) => {
    return {
      data: stock,
      value: stock.ts_code,
      label: `${stock.symbol} ${stock.name}`,
    };
  });
  allStocks.value = stockOptions;
};

const loadData = async () => {
  // 002747.SZ 埃斯顿, 002222 福晶科技
  loading.value = true;
  try {
    const dateType = getDateType(periodType.value)
    let data = await getStockDaily(
      secCode.value,
      startDate.value,
      endDate.value,
      dateType
    );
    summary_info.value = data.data.summary_info;
    stockData.value = {prices: data.data.history_datas};
    genTableData(data);
  } catch (e) {
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data.history_datas;
  const trades = market_datas.map((v) => {
   
    return {
      ... v,
      date: v.trade_date,
      amount: amountTo100Million(v.amount, true),
      price: v.close,
      percent: toFixedNum(v.pct_chg),
      low: `${v.low}(${calcIncPercent(v.pre_close, v.low)})`,
      high: `${v.high}(${calcIncPercent(v.pre_close, v.high)})`,
      range: `${calcRangePercent(v.pre_close, v.low, v.high)}`
    }

  });
  tableData.value = trades;
};

const contains = (s, s1) => s && s.indexOf(s1) >= 0;

function debounce(func, timeout = 300) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);
    }, timeout);
  };
}

const handleSelect = async (item) => {
  console.log("you select: ", item.data.ts_code, JSON.stringify(item));
  secCode.value = item.data.ts_code;
  stockData.value.name = item.data.name;
  await loadData();
};
const querySearch = (queryString, cb) => {
  const results = queryString
    ? allStocks.value.filter((stock) => filterStock(stock.data, queryString))
    : allStocks.value;
  // call callback function to return suggestions
  cb(results);
};

const filterStock = (stock, code) => {
  const res =
    contains(stock.symbol, code) ||
    contains(stock.name, code) ||
    contains(stock.simple_name, code);
  return res;
};

function getStartEnd() {
  let now = dayjs();
  let before5 = now.subtract(5, "day");
  return {
    start: formateDate(before5.toDate()),
    end: formateDate(now.toDate()),
  };
}

const onStartChanged = async (date) => {
  startDate.value = formateDate(date);
  await loadData();
};

const onEndChanged = async (date) => {
  endDate.value = formateDate(date);
  await loadData();
};

const onPeriodChanged = async (period) => {
  isCustom.value = period === "custom";
  periodType.value = period
  await loadData()
};

const getDateType = type => {
  if(type == 'custom') {
    return "Custom"
  } else {
    return "Days" + type
  }
}

</script>

<style scoped>
.red {
  color: red;
}
.green {
  color: rgb(4, 169, 4);
}
span {
  display: inline-block;
  padding: 5px;
}
</style>
