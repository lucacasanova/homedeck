// usage: node render.js page.html W H scale out.png [omitBg]
const { chromium } = require('playwright');
const path = require('path');
(async () => {
  const [page_, W, H, scale, out, omit] = process.argv.slice(2);
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: +W, height: +H }, deviceScaleFactor: +scale });
  await page.goto('file://' + path.resolve(__dirname, page_.split('?')[0]) + (page_.includes('?')?'?'+page_.split('?')[1]:''));
  await page.evaluate(() => document.fonts.ready);
  await page.waitForTimeout(400);
  await page.screenshot({ path: out, omitBackground: omit === '1', clip: { x: 0, y: 0, width: +W, height: +H } });
  await browser.close();
})();
