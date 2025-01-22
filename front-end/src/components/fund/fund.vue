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
   
    <el-table :data="tableData" stripe style="width: 100%" v-loading="loading" @cell-click="onCellClick">
      <el-table-column  prop="ts_code" label="基金代码" width="100" />
      <el-table-column sortable prop="name" label="基金名称" width="180" />
  
      <!-- <el-table-column sortable prop="management" label="管理人" width="110" />
      <el-table-column sortable prop="custodian" label="托管人" width="180" /> -->
      <el-table-column sortable prop="benchmark" label="比较基准" width="180" />
      <el-table-column sortable prop="found_date" label="成立时间" width="130" />
      <el-table-column sortable prop="list_date" label="上市日期" width="130" />
      <el-table-column sortable prop="due_date" label="到期日期" width="130" />
      <el-table-column sortable prop="issue_amount" label="发行份额(亿)" width="130" />
      <el-table-column sortable prop="m_fee" label="管理费" width="130" />
      <el-table-column sortable prop="c_fee" label="托管费" width="130" />
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
    getFunds
  } from "@/service/index.js";
  import {
    getNowStr,
    formateDate,
    toPercent,
    toFixedNum,
    amountTo100Million,
  } from "@/util/util.ts";
  import * as dayjs from "dayjs";
  
  import { defineEmits } from 'vue';

// 定义 emit 函数
const emit = defineEmits(['selected:fund']);

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
      let data = await getFunds();
      genTableData(data);
    } catch (e) {
    } finally {
      loading.value = false;
    }
  };
  const genTableData = (data) => {
    const market_datas = data.data;
    const trades =  market_datas
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
  
  function sortNum(a, b)  {
    return parseFloat(a) - parseFloat(b)
  }
  const onNavClick = e => {
    console.log("nav click", e)
  }


  const onCellClick = (row, column, cell, event) => {
       // console.log(row)
        const data = { name: row.name, tsCode: row.ts_code };
         emit('selected:fund', data);
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
  