// ---------- deterministic noise ----------
function mulberry32(a){return function(){a|=0;a=a+0x6D2B79F5|0;let t=Math.imul(a^a>>>15,1|a);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296}}
function makeNoise1D(seed){
  const r=mulberry32(seed), N=4096, v=new Float32Array(N);
  for(let i=0;i<N;i++)v[i]=r()*2-1;
  const s=t=>t*t*(3-2*t);
  return x=>{const i=Math.floor(x),f=x-i;const a=v[((i%N)+N)%N],b=v[(((i+1)%N)+N)%N];return a+(b-a)*s(f)};
}
function fbm(n,x,oct){let sum=0,amp=1,fr=1,norm=0;for(let o=0;o<oct;o++){sum+=n(x*fr)*amp;norm+=amp;amp*=.5;fr*=2.03}return sum/norm}

/*
 opts = {
   W,H, scale (world px per canvas px, default 1),
   offsetX (world offset), worldH,
   glows:[{x,y,r,a}], ridges:[{y,amp,freq,color,seed,mist}],
   stars:bool
 }
*/
function drawScene(canvas,o){
  const W=o.W,H=o.H; canvas.width=W;canvas.height=H;
  const c=canvas.getContext('2d');
  const k=o.k||1; // world units -> px
  const ox=o.offsetX||0, oy=o.offsetY||0;
  const X=x=>(x-ox)*k, Y=y=>(y-oy)*k;

  // sky
  const sky=c.createLinearGradient(0,0,0,H);
  sky.addColorStop(0,'#060607');sky.addColorStop(.55,'#121215');sky.addColorStop(1,'#09090b');
  c.fillStyle=sky;c.fillRect(0,0,W,H);

  // light glows (moonlight)
  for(const g of o.glows||[]){
    const gr=c.createRadialGradient(X(g.x),Y(g.y),0,X(g.x),Y(g.y),g.r*k);
    gr.addColorStop(0,`rgba(235,235,240,${g.a})`);
    gr.addColorStop(.35,`rgba(200,200,208,${g.a*.42})`);
    gr.addColorStop(.7,`rgba(150,150,160,${g.a*.10})`);
    gr.addColorStop(1,'rgba(120,120,130,0)');
    c.fillStyle=gr;c.fillRect(0,0,W,H);
  }
  // moon disc
  if(o.moon){
    const m=o.moon;
    const mg=c.createRadialGradient(X(m.x)-m.r*k*.25,Y(m.y)-m.r*k*.3,0,X(m.x),Y(m.y),m.r*k);
    mg.addColorStop(0,`rgba(255,255,255,${m.a})`);mg.addColorStop(.9,`rgba(220,220,226,${m.a*.8})`);mg.addColorStop(.985,`rgba(210,210,216,${m.a*.6})`);mg.addColorStop(1,`rgba(210,210,215,0)`);
    c.fillStyle=mg;c.beginPath();c.arc(X(m.x),Y(m.y),m.r*k,0,Math.PI*2);c.fill();
  }

  // stars
  if(o.stars){
    const r=mulberry32(7);const total=o.worldW*o.worldH/9000;
    for(let i=0;i<total;i++){
      const x=r()*o.worldW,y=r()*o.worldH*.55,s=r();
      if(s<.6)continue;
      c.fillStyle=`rgba(255,255,255,${(s-.6)*.9*(1-y/(o.worldH*.55))})`;
      const rr=(s>.95?1.6:1)*k;c.beginPath();c.arc(X(x),Y(y),rr,0,7);c.fill();
    }
  }

  // ridges
  for(const L of o.ridges){
    const n=makeNoise1D(L.seed);
    c.beginPath();c.moveTo(0,H);
    for(let px=0;px<=W;px+=2){
      const wx=px/k+ox;
      let y=L.y+fbm(n,wx*L.freq,7)*L.amp;
      // sharpen peaks a bit
      y-=Math.pow(Math.max(0,-fbm(n,wx*L.freq*.5+33,3)),1.6)*L.amp*.6;
      c.lineTo(px,Y(y));
    }
    c.lineTo(W,H);c.closePath();
    const top=Y(L.y-L.amp*1.4), bot=Y(L.y+L.amp*2.4);
    const fg=c.createLinearGradient(0,top,0,bot);
    fg.addColorStop(0,L.colorTop||L.color);fg.addColorStop(1,L.color);
    c.fillStyle=fg;c.fill();
    // rim light on ridge
    if(L.rim){c.save();c.clip();
      c.globalCompositeOperation='lighter';
      for(const g of o.glows||[]){
        const gr=c.createRadialGradient(X(g.x),Y(g.y),0,X(g.x),Y(g.y),g.r*k*1.1);
        gr.addColorStop(0,`rgba(255,255,255,${L.rim})`);gr.addColorStop(1,'rgba(255,255,255,0)');
        c.fillStyle=gr;c.fillRect(0,0,W,H);
      }
      c.restore();}
    // mist above the next layer
    if(L.mist){
      const mg=c.createLinearGradient(0,Y(L.y+L.amp*.2),0,Y(L.y+L.amp*2.8));
      mg.addColorStop(0,'rgba(190,190,198,0)');mg.addColorStop(.55,`rgba(190,190,198,${L.mist})`);mg.addColorStop(1,'rgba(190,190,198,0)');
      c.fillStyle=mg;c.fillRect(0,Y(L.y-L.amp),W,(L.amp*4)*k);
    }
  }
  // vignette
  if(o.vignette!==false){
    const vg=c.createRadialGradient(W/2,H*.45,Math.min(W,H)*.2,W/2,H*.5,Math.max(W,H)*.85);
    vg.addColorStop(0,'rgba(0,0,0,0)');vg.addColorStop(1,`rgba(0,0,0,${o.vignette||.55})`);
    c.fillStyle=vg;c.fillRect(0,0,W,H);
  }
}

function drawGrain(canvas,W,H,amt){
  canvas.width=W;canvas.height=H;const c=canvas.getContext('2d');
  const img=c.createImageData(W,H),d=img.data,r=mulberry32(99);
  for(let i=0;i<d.length;i+=4){const v=128+(r()-.5)*255*amt;d[i]=d[i+1]=d[i+2]=v;d[i+3]=255}
  c.putImageData(img,0,0);
}

// ---------- brand mark ----------
function markSVG(size,{fill='#fff',bar=null,stroke=58}={}){
  const b=bar||fill;
  return `<svg class="mark" width="${size}" height="${size*528/594}" viewBox="243 255 594 528" xmlns="http://www.w3.org/2000/svg">
    <path d="M309 541 L540 321 L771 541" fill="none" stroke="${fill}" stroke-width="${stroke}" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="309" cy="541" r="66" fill="${fill}"/><circle cx="540" cy="321" r="66" fill="${fill}"/><circle cx="771" cy="541" r="66" fill="${fill}"/>
    <rect x="353" y="673" width="374" height="110" rx="55" fill="${b}"/>
  </svg>`;
}

const ICONS={
  tv:'<rect x="3" y="4" width="18" height="12.5" rx="2.5"/><path d="M8.5 20h7"/>',
  pc:'<rect x="4.5" y="5" width="15" height="10" rx="2"/><path d="M2.5 18.5h19"/>',
  phone:'<rect x="7" y="2.5" width="10" height="19" rx="2.6"/><path d="M11 18h2"/>',
  play:'<path d="M8 5.5v13l10.5-6.5z"/>',
  music:'<path d="M9 18V6l10-2v12"/><circle cx="6.5" cy="18" r="2.5"/><circle cx="16.5" cy="16" r="2.5"/>',
  image:'<rect x="3" y="4" width="18" height="16" rx="2.5"/><circle cx="9" cy="10" r="1.8"/><path d="M21 16l-5-5-9 9"/>',
  check:'<path d="M5 12.5l4.5 4.5L19 7.5"/>',
  lock:'<rect x="5" y="11" width="14" height="10" rx="2"/><path d="M8 11V8a4 4 0 0 1 8 0v3"/>',
};
function icon(name,size,sw=1.6){return `<svg class="icon" width="${size}" height="${size}" viewBox="0 0 24 24" style="stroke-width:${sw}">${ICONS[name]}</svg>`}
