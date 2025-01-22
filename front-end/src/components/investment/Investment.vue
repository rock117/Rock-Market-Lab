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
    <el-table-column  prop="ts_code" label="股票代码" width="180" />
    <el-table-column sortable prop="name" label="股票名称" width="180" />
    <el-table-column sortable prop="close" label="收盘价" width="180" />
    <el-table-column sortable prop="low" label="最低" width="180" />
    <el-table-column sortable prop="high" label="最高" width="180" />
    <el-table-column sortable :sort-method="(a, b) => sortNum(a, b, 'pct_chg')" prop="pct_chg" label="涨幅%" width="180" />
    <el-table-column sortable :sort-method="(a, b) => sortNum(a, b, 'amount')" prop="amount" label="成交量(亿元)" width="180" />
  
  </el-table>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import {
  getStockDaily,
  searchSecurity,
  getInvestments,
} from "@/service/index.js";
import {
  getNowStr,
  formateDate,
  toPercent,
  toFixedNum,
  amountTo100Million,
} from "@/util/util.ts";
import * as dayjs from "dayjs";

const loading = ref(true);
const tableData = ref([]);
const date = ref(getDate());

const options = [
  {
    value: "industry",
    label: "按行业",
  },
  {
    value: "area",
    label: "按地区",
  }]
const sectorType = ref("industry")
onMounted(async () => {
  await loadData();
});

const loadData = async () => {
  // 002747.SZ 埃斯顿, 002222 福晶科技
  loading.value = true;
  try {
    let data = await getInvestments(date.value, sectorType.value);
    genTableData(data);
  } catch (e) {
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data;
  const trades = market_datas.map((v) => {
    return {
      ... v,
      amount: amountTo100Million(v.amount, true),
      pct_chg: toFixedNum(v.pct_chg)
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

const onSectorChanged = async v => {
    sectorType.value = v;
    await loadData();
}

function sortNum(a, b, field)  {
  return parseFloat(a[field]) - parseFloat(b[field])
}
const onNavClick = e => {
  console.log("nav click", e)
}
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
