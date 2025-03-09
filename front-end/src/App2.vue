<template>
  <div class="container">
    <div class="left">
      <!-- <div><a>大盘</a></div>
    <div><a>股票</a></div>
    <div><a>融资融券</a></div>
    <div><a>行情</a></div>
    <div><a>板块</a></div>
    <div><a>选股</a></div>
    <div><a>基准比较</a></div> -->
      <el-row class="tac">
        <el-col :span="24">
          <h5 class="mb-2">Default colors</h5>
          <el-menu
            default-active="2"
            class="el-menu-vertical-demo"
            @open="handleOpen"
            @close="handleClose"
            @select="indexClick"
          >
            <el-sub-menu index="1">
              <template #title>
                <el-icon><location /></el-icon>
                <span>行情</span>
              </template>

              <el-menu-item index="1-index">大盘</el-menu-item>
              <el-menu-item index="1-sector">板块</el-menu-item>
              <el-menu-item index="1-concept">概念</el-menu-item>
              <el-menu-item
                index="1-concept-detail"
                v-if="viewType == 'concept-detail'"
                >{{ conceptName }}</el-menu-item
              >
              <el-sub-menu index="1-1-hot">
                <template #title>
                  <el-icon><location /></el-icon>
                  <span>热门</span>
                </template>
                <el-menu-item index="1-1-hot-stocks">热门股</el-menu-item>
                <el-menu-item index="1-1-hot-concepts">热门概念</el-menu-item>
              </el-sub-menu>

              <el-menu-item index="1-stock">股票</el-menu-item>
              <el-menu-item index="1-fund">基金</el-menu-item>
              <el-menu-item index="1-stock-compare">股票对比</el-menu-item>
              <el-menu-item index="1-history-compare">历史比较</el-menu-item>
              <el-menu-item
                index="1-fund-holding"
                v-if="viewType == 'fund-holding'"
                >{{ fundName }}</el-menu-item
              >

              <el-menu-item index="1-investment">投资组合</el-menu-item>
              <el-menu-item index="1-stock-bmk">基准比较</el-menu-item>
              <el-menu-item index="1-main-business">主营业务</el-menu-item>
              <el-menu-item index="1-limit-up">涨停板</el-menu-item>
            </el-sub-menu>
            <el-menu-item index="2">
              <el-icon><icon-menu /></el-icon>
              <span>选股</span>
            </el-menu-item>
            <el-menu-item index="3">
              <el-icon><icon-menu /></el-icon>
              <span>基准比较</span>
            </el-menu-item>
            <el-menu-item index="4">
              <el-icon><icon-menu /></el-icon>
              <span>融资融券</span>
            </el-menu-item>

            <el-menu-item index="5">
              <el-icon><icon-menu /></el-icon>
              <span>测试</span>
            </el-menu-item>
          </el-menu>
        </el-col>
      </el-row>
    </div>
    <div class="right">
      <IndexTradeHistory v-if="viewType == 'index'" />
      <StockOverview v-if="viewType == 'stock'" />
      <Sector v-if="viewType == 'sector'" />
      <StockConcept
        v-if="viewType == 'concept'"
        @selected:concept="handleConceptClick"
      />
      <StockConceptDetail
        v-if="viewType == 'concept-detail'"
        :conceptCode="conceptCode"
      />
      <Investment v-if="viewType == 'investment'" />
      <Fund v-if="viewType == 'fund'" @selected:fund="handleFundClick" />
      <FundHolding v-if="viewType == 'fund-holding'" :fundCode="fundCode" />
      <MainBusiness v-if="viewType == 'main-business'" />
      <StockCompare v-if="viewType == 'stock-compare'" />
      <HistoryCompare v-if="viewType == 'history-compare'" />
      <LimitUp v-if="viewType == 'limit-up'" />
      <TestUi v-if="viewType == 'test'" />
    </div>
  </div>
</template>

<script setup>
import { ref, provide, onMounted } from "vue";
import {
  Menu as IconMenu,
  Message,
  Document,
  Location,
  Setting,
} from "@element-plus/icons-vue";
import { getTradeStastics } from "@/service/index.js";
import { getNowStr } from "./util/util.ts";
import {
  IndexTradeHistory,
  Sector,
  StockConcept,
  StockConceptDetail,
  MainBusiness,
  LimitUp,
  StockOverview,
  TestUi,
  StockCompare,
  HistoryCompare
} from "@/components/index.js";
import Investment from "./components/investment/Investment.vue";
import Fund from "./components/fund/fund.vue";
import FundHolding from "./components/fund/fund-holding.vue";

const showHolding = ref(false); // funds, holdings
const fundName = ref("");
const fundCode = ref("");
const conceptCode = ref("");
const conceptName = ref("");

// import Sector from "./components/sector/Sector.vue";
const viewType = ref("stock"); //
const indexViewMapping = {
  "1-index": "index",
  "1-stock": "stock",
  "1-history-compare": "history-compare",
  "1-stock-compare": "stock-compare",
  "1-sector": "sector",
  "1-concept": "concept",
  "1-investment": "investment",
  "1-fund": "fund",
  "1-fund-holding": "fund-holding",
  "1-concept-detail": "concept-detail",
  "1-main-business": "main-business",
  "1-limit-up": "limit-up",
  "5": "test"
};

const indexClick = (index, indexPath, item, routeResult) => {
  console.log(`indexClick: ${index}, ${indexPath}, ${item}, ${routeResult}`);
  viewType.value = indexViewMapping[index] || "index";
  if (viewType.value != "fund-holding") {
    showHolding.value = false;
  }
};

const handleOpen = () => {
  //  alert('handleOpen click')
};

const handleFundClick = (data) => {
  console.log("emit fund receive", data);
  viewType.value = "fund-holding";
  fundName.value = data.name;
  fundCode.value = data.tsCode;
  showHolding.value = true;
};

const handleConceptClick = (data) => {
  console.log("emit concept receive", data);
  viewType.value = "concept-detail";
  conceptName.value = data.name;
  conceptCode.value = data.tsCode;
};
</script>

<style scoped>
.container {
  display: flex;
}
.left {
  text-align: left;
  min-width: 130px;
  border: solid red 1px;
  padding: 3px;
}
.right {
  margin-left: 10px;
  min-width: 1200px;
  width: auto;
}
</style>
