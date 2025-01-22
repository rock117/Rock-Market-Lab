<template>
  <el-date-picker
    v-model="date"
    type="date"
    @change="onStartChanged"
    placeholder="日期"
    :size="size"
  />
  <el-select
    v-model="sectorType"
    placeholder="Select"
    @change="onSectorChanged"
    style="width: 240px"
  >
    <el-option
      v-for="item in options"
      :key="item.value"
      :label="item.label"
      :value="item.value"
    />
  </el-select>

  <el-table :data="tableData" stripe style="width: 100%" v-loading="loading">
    <el-table-column prop="symbol" label="股票代码" width="100" />
    <el-table-column sortable prop="name" label="股票名称" width="150" />
    <el-table-column sortable prop="ann_date" label="公告日期" width="160" />
    <el-table-column sortable prop="end_date" label="截止日期" width="160" />
    <!-- <el-table-column sortable prop="management" label="管理人" width="110" />
      <el-table-column sortable prop="custodian" label="托管人" width="180" /> -->
    <el-table-column sortable prop="mkv" label="持有股票市值(元)" width="180" />
    <el-table-column
      sortable
      prop="amount"
      label="持有股票数量（股）"
      width="180"
    />
    <el-table-column
      sortable
      prop="stk_mkv_ratio"
      label="占股票市值比"
      width="150"
    />
    <!-- <el-table-column sortable prop="stk_float_ratio" label="占流通股本比例" width="130" /> -->
    <el-table-column
      sortable
      :sort-method="(a, b) => sortNum(a, b, 'market_value')"
      prop="market_value"
      label="市值(亿元)"
      width="130"
    />
  </el-table>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import {
  getStockDaily,
  searchSecurity,
  getSectorTradeStastics,
  getFundHoldings,
} from "@/service/index.js";
import {
  getNowStr,
  formateDate,
  toPercent,
  toFixedNum,
  amountTo100Million2,
  sortNum,
} from "@/util/util.ts";
import * as dayjs from "dayjs";
import { defineProps } from "vue";
const loading = ref(true);
const tableData = ref([]);
const date = ref(getDate());

const props = defineProps({
  fundCode: String,
});

const options = [
  {
    value: "industry",
    label: "按行业",
  },
  {
    value: "area",
    label: "按地区",
  },
];
const sectorType = ref("industry");
onMounted(async () => {
  await loadData();
});

const loadData = async () => {
  // 002747.SZ 埃斯顿, 002222 福晶科技
  loading.value = true;
  try {
    let fundCode = props.fundCode;
    let data = await getFundHoldings(fundCode);
    genTableData(data);
  } catch (e) {
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data;
  const trades = market_datas.map((data) => {
    return {
      ...data,
      market_value: amountTo100Million2(data.market_value, true),
    };
  });
  tableData.value = trades;
};

function getDate() {
  let now = dayjs();
  return formateDate(now.toDate());
}

const onStartChanged = async (d) => {
  date.value = formateDate(d);
  await loadData();
};

const onSectorChanged = async (v) => {
  sectorType.value = v;
  await loadData();
};

const onNavClick = (e) => {
  console.log("nav click", e);
};
</script>

<style scoped>
.nav {
  cursor: pointer;
}
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
