<template>
  <el-table :data="tableData" stripe style="width: 100%" v-loading="loading">
    <el-table-column prop="code" label="股票代码" width="180" />
    <el-table-column sortable prop="name" label="股票名称" width="180" />
    <el-table-column sortable prop="market_value" label="市值(亿元)" width="180" />
  </el-table>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import {
  getStockDaily,
  searchSecurity,
  getStockConcepts,
  getStockConceptsByCode,
} from "@/service/index.js";
import {
  getNowStr,
  formateDate,
  toPercent,
  toFixedNum,
  amountTo100Million,
  amountTo100Million2,
} from "@/util/util.ts";
import * as dayjs from "dayjs";
import { defineProps } from "vue";

const loading = ref(true);
const tableData = ref([]);
const date = ref(getDate());
const props = defineProps({
  conceptCode: String,
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
    let data = await getStockConceptsByCode(props.conceptCode);
    genTableData(data);
  } catch (e) {
    console.error("error in getStockConceptsByCode", e)
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data;
  const trades = market_datas.map(data => {
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

function sortNum(a, b) {
  return parseFloat(a) - parseFloat(b);
}
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
