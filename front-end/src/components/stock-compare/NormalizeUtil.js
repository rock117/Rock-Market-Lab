const normalize = prices => {
    let basePrice = prices[0] // 以第一个价格为基准
    // return prices.map(price => ((price - basePrice) / basePrice) * 100)
    return prices
}

export {
    normalize
}