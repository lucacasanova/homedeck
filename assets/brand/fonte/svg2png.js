const { chromium } = require('playwright'); const fs=require('fs'); const path=require('path');
(async()=>{const dir=process.argv[2];const b=await chromium.launch();const p=await b.newPage();
for(const f of fs.readdirSync(dir).filter(f=>f.endsWith('.svg'))){
  const svg=fs.readFileSync(path.join(dir,f),'utf8');const m=svg.match(/viewBox="0 0 ([\d.]+) ([\d.]+)"/);
  const w=+m[1],h=+m[2];const W=f.includes('simbolo')?1024:2400;const H=Math.round(W*h/w);
  await p.setViewportSize({width:W,height:H});
  await p.setContent(`<html><body style="margin:0;background:transparent">${svg.replace(/width="[\d.]+" height="[\d.]+"/,`width="${W}" height="${H}"`)}</body></html>`);
  await p.screenshot({path:path.join(dir,'png',f.replace('.svg','.png')),omitBackground:true,clip:{x:0,y:0,width:W,height:H}});
}await b.close();})();
