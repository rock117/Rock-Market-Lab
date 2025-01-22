const formateDate = (date: Date) => {
    const year = date.getFullYear()
    const month = date.getMonth() + 1
    const day = date.getDate()
    const m = addPadding(month)
    const d = addPadding(day)
    return `${year}-${m}-${d}`
}

const getNowStr = () => {
    return formateDate(new Date())
}
const addPadding = (n: number) => {
    if(n < 10) {
        return `0${n}`
    } else {
        return `${n}`
    }
}

const getDefaultStartEnd = () => {

}

const toPercent = (value: number) => {
    let ret = (value * 100).toFixed(2)
    return `${ret}%`
}

const toFixedNum = (value: number, n: number = 2) => {
    if(value == null || value == undefined) {
        return ""
    } else {
        return value.toFixed(n)
    }
}


/**
 * 将成交额转成亿为单位
 * @param amount 单位 千元
 * @returns 亿元
 */
const amountTo100Million = (amount: number, toFixed: boolean = false, n: number = 2) => {
    if(amount == null || amount == undefined) {
        return null
    }
    const ret =  (amount * 1000.0) / (10000.0 * 10000.0)
    if(!toFixed) {
        return ret
    } else {
        return toFixedNum(ret, n)
    }
}

/**
 * 将成交额转成亿为单位
 * @param amount 单位 万元
 * @returns 亿元
 */
const amountTo100Million2 = (amount: number, toFixed: boolean = false, n: number = 2) => {
    if(amount == null || amount == undefined) {
        return null
    }
    const ret =  (amount * 10000.0) / (10000.0 * 10000.0)
    if(!toFixed) {
        return ret
    } else {
        return toFixedNum(ret, n)
    }
}

const calcIncPercent = (a, b) => {
    let delta = b - a
    let percent = ((b-a)/a) 
    return toPercent(percent)
}

const calcRangePercent = (p, min, max) => {
    let p1 = (min - p)/p
    let p2 = (max - p)/p
    return toPercent(p2-p1)
}

const sortNum = (a, b, field) => {
    return parseFloat(a[field]) - parseFloat(b[field])
  }

export {
    formateDate, getNowStr, toPercent, toFixedNum, amountTo100Million, calcIncPercent, calcRangePercent, amountTo100Million2, sortNum
}

