const { chromium } = require('playwright');

(async () => {
  try {
    const raw = process.argv[2] || '{}';
    const args = JSON.parse(raw);

    const url = args.url;
    if (!url) {
      throw new Error('missing args.url');
    }

    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();

    await page.goto(url, { timeout: 160000, waitUntil: 'domcontentloaded' });
    await page.waitForTimeout(1500);

    const content = await page.content();
    const finalUrl = page.url();

    await browser.close();

    process.stdout.write(JSON.stringify({ final_url: finalUrl, content }));
  } catch (e) {
    // Print a json-ish error to stderr, but keep stdout clean.
    process.stderr.write(String(e && e.stack ? e.stack : e));
    process.exit(1);
  }
})();
