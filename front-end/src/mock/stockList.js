// Generate 6000 mock stock data
const generateMockStocks = () => {
  const stocks = [];
  const areas = ['深圳', '上海', '北京', '广州', '杭州', '成都'];
  const industries = ['银行', '科技', '医药', '房地产', '制造', '新能源', '消费', '农业', '教育', '互联网'];
  
  // Generate SZ stocks (000001-003999)
  for (let i = 1; i <= 3999; i++) {
    const code = i.toString().padStart(6, '0');
    stocks.push({
      ts_code: `${code}.SZ`,
      symbol: code,
      name: `${industries[Math.floor(Math.random() * industries.length)]}${Math.floor(Math.random() * 100)}股份`,
      area: areas[Math.floor(Math.random() * areas.length)]
    });
  }

  // Generate SH stocks (600000-603999)
  for (let i = 600000; i <= 603999; i++) {
    stocks.push({
      ts_code: `${i}.SH`,
      symbol: i.toString(),
      name: `${industries[Math.floor(Math.random() * industries.length)]}${Math.floor(Math.random() * 100)}股份`,
      area: areas[Math.floor(Math.random() * areas.length)]
    });
  }

  return stocks;
};

export const mockStocks = generateMockStocks();

// Function to search stocks by code or name
export const searchStocks = (query) => {
  if (!query) return [];
  
  query = query.toLowerCase();
  return mockStocks.filter(stock => 
    stock.symbol.toLowerCase().includes(query) || 
    stock.name.toLowerCase().includes(query) ||
    stock.ts_code.toLowerCase().includes(query)
  ).slice(0, 10); // Return max 10 results for better performance
};
