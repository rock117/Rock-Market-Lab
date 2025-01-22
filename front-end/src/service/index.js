import {http} from "@/util/index.js";

async function getStockList(){
    return (await http.get('/api/stock-list')).data
}

async function getStockDaily(code, start, end, dateType, period = 'Daily') {
    return (await http.get(`/api/market-data?code=${code}&start_date=${start}&end_date=${end}&type=Stock&period=${period}&date_type=${dateType}`)).data
}

const getTradeStastics = async(start, end, date_type) => {
    return (await http.get(`/api/trade-stastics?start_date=${start}&end_date=${end}&date_type=${date_type}`)).data
}

const searchSecurity = async(code) => {
    return (await http.get(`/api/securities/search?code=${code}`)).data
}


const searchSecurityByPrice = async(data) => {
    return (await http.post(`/api/securities/price/search`, data)).data
}

const getSectorTradeStastics = async(date, type) => {
    return (await http.get(`/api/sector/trade-stastics?date=${date}&sector_type=${type}`)).data
}

const getStockConcepts = async() => {
    return (await http.get(`/api/stock/concepts`)).data
}

const getStockConceptsByCode = async(tsCode) => {
    console.log('getStockConceptsByCode tsCode = ', tsCode)
    return (await http.get(`/api/stock/concepts/${tsCode}`)).data
}


const getInvestments = async(date) => {
    return (await http.get(`/api/investments?date=${date}`)).data
}

const getFunds = async() => {
    return (await http.get(`/api/funds`)).data
}

const getFundHoldings = async(fundCode) => {
    return (await http.get(`/api/funds/${fundCode}/holdings`)).data
}

const findConsecutiveLimitupStocks = async(days) => {
    return (await http.get(`api/stock-picking/consecutive-limitup?days=${days}`)).data
}


export {
    getStockList, getStockDaily, getTradeStastics, searchSecurity, getSectorTradeStastics, 
    getStockConcepts, getStockConceptsByCode, getInvestments, getFunds, getFundHoldings,
    findConsecutiveLimitupStocks
}