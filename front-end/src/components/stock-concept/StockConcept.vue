<template>
  <el-table
    :data="tableData"
    stripe
    style="width: 100%"
    v-loading="loading"
    @cell-click="onCellClick"
  >
    <el-table-column prop="ts_code" label="概念代码" width="180" />
    <el-table-column sortable prop="name" label="概念" width="180" />
    <el-table-column sortable prop="count" label="股票数" width="180" />
    <el-table-column sortable prop="list_date" label="上市日期" width="180" />
    <el-table-column sortable prop="inc_percent" label="涨幅%" width="180" />
    <el-table-column label="查看">
      <template #default="scope">
        <el-button
          size="small"
          type="info"
          @click="handleView(scope.$index, scope.row)"
        >
          查看
        </el-button>
      </template>
    </el-table-column>
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
} from "@/util/util.ts";
import * as dayjs from "dayjs";

import { defineEmits } from "vue";

// 定义 emit 函数
const emit = defineEmits(["selected:concept"]);

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
    let data = await getStockConcepts(date.value, sectorType.value);
    genTableData(data);
  } catch (e) {
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data;
  const trades = market_datas.map((v) => {
    return v;
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

const handleView = async (index, row) => {
  console.log(`index = ${index},  ${row}`);
  let tsCode = row.ts_code;
  let data = await getStockConceptsByCode(tsCode);
  console.log("data = ", data);
};

const onCellClick = (row, column, cell, event) => {
  // console.log(row)
  const data = { name: row.name, tsCode: row.ts_code };
  emit("selected:concept", data);
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
