<template>
  <el-date-picker
    v-model="date"
    type="date"
    @change="onStartChanged"
    placeholder="日期"
    :size="size"
  />

 
  <el-select
    v-model="value"
    multiple
    clearable
    collapse-tags
    placeholder="Select"
    popper-class="custom-header"
    :max-collapse-tags="1"
    style="width: 240px"
  >
    <template #header>
      <el-checkbox
        v-model="checkAll"
        :indeterminate="indeterminate"
        @change="handleCheckAll"
      >
        All
      </el-checkbox>
    </template>
    <el-option
      v-for="item in exchanges"
      :key="item.value"
      :label="item.label"
      :value="item.value"
    />
  </el-select>
 

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
<div>
  <el-table
    :data="tableData"
    stripe
    style="width: 100%"
    v-loading="loading"
    @sort-change="handleSortChange"
    height="550"
  >
    <el-table-column fixed prop="ts_code" label="股票代码" width="100" />
    <el-table-column fixed sortable prop="name" label="股票名称" width="130" />
    <el-table-column sortable="custom" prop="close" label="收盘价"  width="100"/>
    <el-table-column sortable="custom" prop="pct_chg" label="涨跌幅" width="100"/>
    <el-table-column sortable prop="low" label="最低" width="80" />
    <el-table-column sortable prop="high" label="最高" width="80" />
    <el-table-column  sortable="custom"   prop="amount" label="成交量(亿元)" width="150" />
    <el-table-column v-if="fields['turnover_rate']" sortable="custom"  prop="turnover_rate" label="换手率" width="100" />

    <el-table-column  sortable="custom"  prop="pct_chg5" label="5日涨幅"  width="120" />
    <el-table-column sortable="custom"  prop="pct_chg10" label="10日涨幅" width="120"/>
    <el-table-column  sortable="custom"  prop="pct_chg20" label="20日涨幅"  width="120"/>
    <el-table-column  sortable="custom"  prop="pct_chg60"  label="60日涨幅"  width="120" />
    <el-table-column  sortable="custom"  prop="pct_chg250" label="250日涨幅" width="150"/>

    <el-table-column sortable="custom"  prop="area" label="地区" width="180" />
    <el-table-column sortable="custom"  prop="market" label="交易所" width="120" />
    <el-table-column sortable="custom"  prop="list_date" label="上市日期" width="120" />
    <el-table-column sortable="custom"  prop="gross_margin" label="毛利率" width="120" />
    <el-table-column  sortable="custom"  prop="roe" label="净资产收益率" width="180" />
    <el-table-column sortable="custom"  prop="total_mv" label="总市值(亿元)" width="180" />

    <el-table-column  sortable="custom"  prop="ma5" label="MA5" width="100" />
    <el-table-column  sortable="custom"   prop="ma10" label="MA10"  width="100"/>
    <el-table-column  sortable="custom"  prop="ma20" label="MA20" width="100" />
    <el-table-column  sortable="custom"  prop="ma60" label="MA60"  width="100"/>
    <el-table-column sortable="custom"  prop="ma250"  label="MA250" width="100" />
  </el-table>
  <el-pagination
      style="padding-top: 10px;"
      background
      layout="prev, pager, next"
      :total="total"
      :page-size="pageSize"
      :current-page="currentPage"
      @current-change="handlePageChange"
    />
  </div>
</template>

<script setup>
// https://element-plus.org/zh-CN/component/autocomplete.html
import { ref, provide, watch, onMounted } from "vue";
import { Message, Document } from "@element-plus/icons-vue";
import { getStockList, searchSecurity } from "@/service/index.js";
import {
  getNowStr,
  formateDate,
  toPercent,
  toFixedNum,
  amountTo100Million,
} from "@/util/util.ts";
import * as dayjs from "dayjs";
import { getAreas, getIndustries } from "@/service/index.js";

// 自定义下拉菜单的头部
// https://element-plus.org/zh-CN/component/select.html
const fields = ref({turnover_rate: false})
const loading = ref(true);
const tableData = ref([]);
const date = ref(getDate());

const checkAll = ref(false)
const indeterminate = ref(false)
const value = ref([])
const exchanges = ref([
  {
    value: 'all',
    label: '全部',
  },
  {
    value: '主板',
    label: '主板',
  },
  {
    value: '创业板',
    label: '创业板',
  },
  {
    value: '科创板',
    label: '科创板',
  },
  {
    value: '北交所',
    label: '北交所',
  },
])

watch(value, (val) => {
  if (val.length === 0) {
    checkAll.value = false
    indeterminate.value = false
  } else if (val.length === exchanges.value.length) {
    checkAll.value = true
    indeterminate.value = false
  } else {
    indeterminate.value = true
  }
})

const handleCheckAll = (val) => {
  indeterminate.value = false
  if (val) {
    value.value = exchanges.value.map((_) => _.value)
  } else {
    value.value = []
  }
}


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

const value1 = ref([]);
const value2 = ref([]);
const value3 = ref([]);
const value4 = ref([]);
const options2 = [
  {
    value: "Option1",
    label: "Option1",
  },
  {
    value: "Option2",
    label: "Option2",
  },
  {
    value: "Option3",
    label: "Option3",
  },
  {
    value: "Option4",
    label: "Option4",
  },
  {
    value: "Option5",
    label: "Option5",
  },
];
const total = ref(0);
const currentPage = ref(1);
const pageSize = ref(100);
const orderBy = ref("pct_chg");
const order = ref("desc");

const sectorType = ref("industry");
onMounted(async () => {
  await loadData();
});

const loadData = async () => {
  // 002747.SZ 埃斯顿, 002222 福晶科技
  loading.value = true;
  try {
    let data = await getData();
    genTableData(data);
  } catch (e) {
    console.log("failed to getstock list", e);
  } finally {
    loading.value = false;
  }
};
const genTableData = (data) => {
  const market_datas = data.data.data;
  const trades = market_datas.map((v) => {
    return {
      ...v,
      //  amount: amountTo100Million(v.amount, true),
      pct_chg: toFixedNum(v.pct_chg),
      pct_chg5: toFixedNum(v.pct_chg5),
      pct_chg10: toFixedNum(v.pct_chg10),
      pct_chg20: toFixedNum(v.pct_chg20),
      pct_chg60: toFixedNum(v.pct_chg60),
      pct_chg250: toFixedNum(v.pct_chg250),
      roe: toFixedNum(v.roe),
      ma5: toFixedNum(v.ma5),
      ma10: toFixedNum(v.ma10),
      ma20: toFixedNum(v.ma20),
      ma60: toFixedNum(v.ma60),
      ma250: toFixedNum(v.ma250),
      turnover_rate: toFixedNum(v.turnover_rate),
      total_mv: toFixedNum(v.total_mv / 10000),
      amount: toFixedNum((v.amount * 1000) / (10000 * 10000)),
    };
  });
  total.value = data.data.total
  tableData.value = trades;
};

function getDate() {
  let now = dayjs();
  return formateDate(now.toDate());
}

const getData = async () =>  {
   return await getStockList(currentPage.value, pageSize.value, orderBy.value, order.value)
}

const onStartChanged = async (d) => {
  date.value = formateDate(d);
  //  await loadData();
};

const onSectorChanged = async (v) => {
  sectorType.value = v;
  //  await loadData();
};

function sortNum(a, b, field) {
  return parseFloat(a[field]) - parseFloat(b[field]);
}
const onNavClick = (e) => {
  console.log("nav click", e);
};

const handleSortChange = (prop) => {
      order.value = prop.order;
      orderBy.value = prop.prop;
      currentPage.value = 1; // 排序后重置到第一页
      loadData(); // 重新获取数据
}

const handlePageChange = (page) => {
    currentPage.value = page;
    loadData(); // 重新获取数据
}

const loadAreasAndIndustries = async () => {
  const areas = await getAreas();
  const industries = await getIndustries();
  areas.value = areas;
  industries.value = industries;
}

</script>

<style scoped>
.custom-header {
  .el-checkbox {
    display: flex;
    height: unset;
  }
}
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
