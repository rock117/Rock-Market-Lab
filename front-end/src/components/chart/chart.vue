<template>
  <div id="main-chart" style="width: auto; height: 400px"></div>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, onMounted } from "vue";

import * as echarts from "echarts";

import { defineProps } from "vue";

// 使用 defineProps 定义接收的属性
const props = defineProps({
  stockData: Object,
});

var option;

option = {
  title: {
    text: "Stacked Line",
  },
  tooltip: {
    trigger: "axis",
  },
  legend: {
    data: ["Email", "Union Ads", "Video Ads", "Direct", "Search Engine"],
  },
  grid: {
    left: "3%",
    right: "4%",
    bottom: "3%",
    containLabel: true,
  },
  toolbox: {
    feature: {
      saveAsImage: {},
    },
  },
  xAxis: {
    type: "category",
    boundaryGap: false,
    data: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
  },
  yAxis: {
    type: "value",
  },
  series: [
    {
      name: "Email",
      type: "line",
      stack: "Total",
      data: [120, 132, 101, 134, 90, 230, 210],
    },
    {
      name: "Union Ads",
      type: "line",
      stack: "Total",
      data: [220, 182, 191, 234, 290, 330, 310],
    },
    {
      name: "Video Ads",
      type: "line",
      stack: "Total",
      data: [150, 232, 201, 154, 190, 330, 410],
    },
    {
      name: "Direct",
      type: "line",
      stack: "Total",
      data: [320, 332, 301, 334, 390, 330, 320],
    },
    {
      name: "Search Engine",
      type: "line",
      stack: "Total",
      data: [820, 932, 901, 934, 1290, 1330, 1320],
    },
  ],
};

onMounted(async () => {
  var chartDom = document.getElementById("main-chart");
  var myChart = echarts.init(chartDom);
  option.title.text = '';
  option.legend.data = [props.stockData.name]
  initxAxis();
  initSeries();
  option && myChart.setOption(option);
});

const initxAxis = () => {
  option.xAxis.data = props.stockData.prices.map((p) => p.trade_date);
};

const initSeries = () => {
    // series: [
    // {
    //   name: "Email",
    //   type: "line",
    //   stack: "Total",
    //   data: [120, 132, 101, 134, 90, 230, 210],
    // },
    option.series = [
        {
            name: props.stockData.name,
            type: "line",
            stack: "Total",
            data:  props.stockData.prices.map((p) => p.close)
        }
    ]
};
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
