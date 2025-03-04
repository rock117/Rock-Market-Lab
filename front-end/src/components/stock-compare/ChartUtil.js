/**
 * 
 * @param {*} tsCodes ["000001.SZ"]
 * @param {*} stocksNames {ts_code: "", name: ""}
 * @param {*} stocksPrices [{ts_code: "", close: ""}]
 * @returns 
 */
const buildChartOption = (tsCodes, stocksNames, stocksPrices) => {
    let firstPrice = stocksPrices[tsCodes[0]]
    const opt = {
        title: {
            text: '股票价格走势'
        },
        tooltip: {
            trigger: 'axis'
        },
        legend: {
            data: tsCodes.map(tscode => stocksNames[tscode].name)
        },
        grid: {
            left: '3%',
            right: '4%',
            bottom: '3%',
            containLabel: true
        },
        toolbox: {
            feature: {
                saveAsImage: {}
            }
        },
        xAxis: {
            type: 'category',
            boundaryGap: false,
            data: firstPrice.map(item => item.trade_date)
        },
        yAxis: {
            type: 'value'
        },
        series: createSeries(tsCodes, stocksNames, stocksPrices)
    };
    return opt
}

const createSeries = (tsCodes, stocksNames, stocksPrices) => {
    const series = tsCodes.map((tscode) => {
        return {
            name: stocksNames[tscode].name,
            type: 'line',
            stack: 'Total',
            data: stocksPrices[tscode].map(v => v.close)
        }
    })
    return series
}

export {
    buildChartOption
}