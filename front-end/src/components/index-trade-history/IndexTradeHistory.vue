<template>
  <div>
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
  </div>
  <div>交易汇总</div>
  <el-table :data="tableData" style="width: 100%" v-loading="loading">
    <el-table-column prop="date" label="日期" width="180" />
    <el-table-column prop="vol" label="成交量(亿元)" width="180" />
    <el-table-column prop="inc" label="上涨家数" />
    <el-table-column prop="dec" label="下跌家数" />
    <el-table-column prop="eq" label="无涨跌家数" />
  </el-table>
</template>

<script setup>
import { ref, provide, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import { getTradeStastics } from "@/service/index.js";
import { getNowStr, formateDate, toPercent, toFixedNum, amountTo100Million } from "@/util/util.ts";
import * as dayjs from "dayjs";

const periodType = ref("custom");
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

const startEnd = getStartEnd();
const startDate = ref(startEnd.start);
const endDate = ref(startEnd.end);
const isCustom = ref(true);

const loading = ref(false)
const tableData = ref([]);

onMounted(async () => {
  await loadData();
});

const loadData = async () => {
  loading.value = true
  const dateType = getDateType(periodType.value)
  let data = await getTradeStastics(startDate.value, endDate.value, dateType);
  loading.value = false
  console.log("data", data);
  genTableData(data);
};
const genTableData = (data) => {
  const trades = data.data.map((v) => {
    return {
      date: v.date,
      vol:  amountTo100Million(v.info.total_amount, true),
      inc: v.info.inc_num,
      dec: v.info.dec_num,
      eq: v.info.eq_num,
    };
  });
  tableData.value = trades;
};

function getStartEnd() {
  let now = dayjs();
  let before5 = now.subtract(5, "day");
  return {
    start: formateDate(before5.toDate()),
    end: formateDate(now.toDate()),
  };
}

const getDateType = type => {
  if(type == 'custom') {
    return "Custom"
  } else {
    return "Days" + type
  }
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

</script>

<style scoped></style>
