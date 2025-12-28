const { chromium } = require('playwright');

const url = "https://emweb.eastmoney.com/PC_USF10/pages/index.html?code=TSLA&type=web&color=w#/hxbd";
(async () => {
  await find_word(url, '1.58万亿');
})();


async function find_word(url, word) {

   const browser = await chromium.launch({ headless: false });
  const page = await browser.newPage();

  // 监听响应
  page.on('response', async response => {
    const url = response.url();

    // 只打印文本类型的响应
    // const contentType = response.headers()['content-type'] || '';
    // const isText = contentType.includes('text/') ||
    //   contentType.includes('application/json') ||
    //   contentType.includes('application/javascript') ||
    //   contentType.includes('application/xml');

    const isText = true
    if (isText) {
      try {
        // 获取响应体（文本）
        const body = await response.text();
        if (body.includes(word)) {
          console.log('URL:', url);
          console.log('body ===>:', body);
        } else {
          // console.log('URL:', url);
        }
      } catch (e) {
        console.log('无法读取响应体:', e.message, url);
      }
    }
  });
  await page.goto(url, { timeout: 160000 });
  await page.waitForTimeout(20000);
  await browser.close();
}