/**
 * Normalizes multiple arrays of stock price data to have matching trade dates
 * @param {...Array} priceArrays Arrays of stock price objects
 * @returns {Object} Object containing normalized arrays: {trade_dates, normalizedPrices}
 */
export function normalizeStockPrices(...priceArrays) {
    // Sort each input array by trade_date
    const sortedArrays = priceArrays.map(prices => 
        [...prices].sort((a, b) => a.trade_date.localeCompare(b.trade_date))
    );

    // Extract all trade dates and convert to MMDD format
    const tradeDatesSet = new Set(
        sortedArrays.flatMap(prices => 
            prices.map(price => price.trade_date.slice(-4))
        )
    );

    // Convert to array and sort
    const trade_dates = Array.from(tradeDatesSet).sort();

    // Create maps for quick lookup for each array, using MMDD as key
    const priceMaps = sortedArrays.map(prices =>
        new Map(prices.map(price => [price.trade_date.slice(-4), price]))
    );

    // Create normalized arrays
    const normalizedPrices = priceMaps.map(priceMap =>
        trade_dates.map(date => priceMap.get(date) || {})
    );

    return {
        trade_dates,
        normalizedPrices
    };
}

/**
 * Legacy support for exactly two arrays
 * @param {Array} prePrices1 First array of stock price objects
 * @param {Array} prePrices2 Second array of stock price objects
 * @returns {Object} Object containing normalized arrays: {trade_dates, newPrices1, newPrices2}
 */
export function normalizeTwoStockPrices(prePrices1, prePrices2) {
    const { trade_dates, normalizedPrices } = normalizeStockPrices(prePrices1, prePrices2);
    const [newPrices1, newPrices2] = normalizedPrices;
    return {
        trade_dates,
        newPrices1,
        newPrices2
    };
}
