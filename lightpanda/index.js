const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });
  const page = await browser.newPage();

  // 监听响应
  page.on('response', async response => {
    const url = response.url();
    
    // 只打印文本类型的响应
    const contentType = response.headers()['content-type'] || '';
    const isText = contentType.includes('text/') || 
                   contentType.includes('application/json') || 
                   contentType.includes('application/javascript') ||
                   contentType.includes('application/xml');

    if(!isText) {
         try {
            let t = await response.text()
            isText = true   
         }catch(e) {

         }
    }

    if (isText) {
    //   console.log('=== 拦截到响应 ===');
    //   console.log('状态码:', response.status());
    //   console.log('响应头:', response.headers());
      
      try {
        // 获取响应体（文本）
        const body = await response.text();
        if (body.includes('商业航天')) {
          console.log('URL:', url);
           console.log('body ===>:', body);
        } else {
             console.log('URL:', url);
        }
      } catch (e) {
        console.log('无法读取响应体:', e.message);
      }
    }
  });

  // https://emweb.securities.eastmoney.com/pc_hsf10/pages/index.html?type=web&code=SZ300620&color=b#/hxtc
  // https://emweb.securities.eastmoney.com/pc_hsf10/pages/index.html?type=web&code=SZ300620&color=b#/gsgk
  await page.goto('https://emweb.securities.eastmoney.com/pc_hsf10/pages/index.html?type=web&code=SZ300620&color=b#/gsgk', { timeout: 160000 });
  await page.waitForTimeout(20000);
  await browser.close();
})();