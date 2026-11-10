// 键盘猫 花色生成器 v3 —— 统一用椭圆列表描述白斑/斑块，便于按解剖坐标调
// 母图(奶油团子 loaf) → 6-slot 分类 → 白斑/斑块/条纹 → 多档透明 PNG
// 用法: node recolor.js [源图] [尺寸列表]
const P='C:/Users/Administrator/.dsh/skills/agent-sprite-forge/scripts/';
const {readPNG,writePNG,canvas}=require(P+'png0.js');
const OUT='H:/Users/Administrator/Documents/deepseek-harness/default-workspace/keyboard-cat/asf/';
const M='C:/Users/Administrator/AppData/Local/Programs/DshShell/maki-out/';
const src=process.argv[2]||(M+'kbcat-loaf3-20261105-203720-301.png');
const sizes=(process.argv[3]||'56,64,84,110').split(',').map(Number);
const base=readPNG(src);

// ---------- 抠底 + bbox ----------
function px(x,y){const i=(y*base.width+x)*4;return [base.data[i],base.data[i+1],base.data[i+2]]}
const cs=[px(3,3),px(base.width-4,3),px(3,base.height-4),px(base.width-4,base.height-4)];
const bg=[0,1,2].map(k=>Math.round(cs.reduce((a,c)=>a+c[k],0)/cs.length));
for(let i=0;i<base.width*base.height;i++){
  const r=base.data[i*4],g=base.data[i*4+1],b=base.data[i*4+2];
  if(Math.hypot(r-bg[0],g-bg[1],b-bg[2])<110) base.data[i*4+3]=0;
}
let x0=1e9,y0=1e9,x1=-1,y1=-1;
for(let y=0;y<base.height;y++)for(let x=0;x<base.width;x++)
  if(base.data[(y*base.width+x)*4+3]>90){
    if(x<x0)x0=x; if(x>x1)x1=x; if(y<y0)y0=y; if(y>y1)y1=y;
  }
if(x1<0) throw new Error('bbox fail');
const bb={x0,y0,w:x1-x0+1,h:y1-y0+1};

// ---------- 6-slot 分类 ----------
function classify(r,g,b){
  const lum=r*0.299+g*0.587+b*0.114;
  const mx=Math.max(r,g,b),mn=Math.min(r,g,b);
  const sat=mx===0?0:(mx-mn)/mx;
  if(lum<70)return 0;                                  // 描边
  if(sat>0.30&&r>=g&&r>b)return lum>200?4:5;            // 腮红 / 耳内
  if(lum>232)return 2;                                 // 高光
  if(lum<200)return 3;                                 // 暗部
  return 1;                                            // 主体
}
const slot=new Uint8Array(base.width*base.height);
for(let i=0;i<base.width*base.height;i++)
  slot[i]=base.data[i*4+3]<10?255:classify(base.data[i*4],base.data[i*4+1],base.data[i*4+2]);

const hex2rgb=h=>[parseInt(h.slice(1,3),16),parseInt(h.slice(3,5),16),parseInt(h.slice(5,7),16)];
const shade=(c,k)=>c.map(v=>Math.max(0,Math.round(v*k)));

// ---------- 解剖坐标（实测 grid）----------
// 耳尖 v .00-.12 / 眼 v .30 / 口鼻 v .36 / 下巴 v .43 / 胸前 v .45+
// 头 u .03-.50，脸心 u .24；尾卷 u .75-.92 v .80-.92
const isHead=(u,v)=>((u-0.245)/0.290)**2+((v-0.265)/0.270)**2<1;

function inEll(u,v,e){
  return ((u-e.u)/e.ru)**2+((v-e.v)/e.rv)**2 < 1;
}
function inAny(list,u,v){ for(const e of list){if(inEll(u,v,e))return e} return null }

// 虎斑：沿身体走向的横纹（侧躺时表现为横向弧线）
function inStripe(u,v,cfg){
  if(isHead(u,v)){
    if(v>0.26)return false;                            // 只打额头
    const m=Math.sin((u*3.2+0.06)*Math.PI*4.2);
    return m>0.55;
  }
  if(v>0.90)return false;
  const bend=Math.sin((u*1.8+0.35)*Math.PI)*0.035;
  const f=cfg.freq||6.0;
  const cur=Math.sin(((v+bend)*f)*Math.PI);
  return cur>0.62;
}

// ---------- 花色表 ----------
// white: 白斑椭圆（baseWhite 时作为底色，否则作为花纹叠加）
const SP=[
 // 橘猫：橘底 + 大白胸腹（下巴→前爪）+ 背部横纹
 {id:'orange',name:'橘猫',baseWhite:false,
  main:'#f9c279',dark:'#e8b56a',light:'#ffd9ac',out:'#462515',blush:'#ffb0b8',ear:'#f2a09a',
  white:[{u:0.238,v:0.372,ru:0.100,rv:0.062},{u:0.150,v:0.745,ru:0.290,rv:0.315}],
  patches:[],stripe:{col:'#e2ac63',freq:6.4}},

 // 老头：全身深橘虎斑，无白
 {id:'old',name:'老头',baseWhite:false,
  main:'#f4b96a',dark:'#dda555',light:'#ffd49c',out:'#47240b',blush:'#ffb0b8',ear:'#ef9e98',
  white:[],patches:[],stripe:{col:'#d9a052',freq:6.8}},

 // 狸花：灰褐底 + 深灰横纹 + 白口鼻胸腹
 {id:'tabby',name:'狸花',baseWhite:false,
  main:'#bdb1a9',dark:'#a99c94',light:'#d9cfc8',out:'#462616',blush:'#f0a8b0',ear:'#e5a0a4',
  white:[{u:0.238,v:0.372,ru:0.092,rv:0.056},{u:0.135,v:0.760,ru:0.250,rv:0.270}],
  patches:[],stripe:{col:'#8a7b78',freq:7.4}},

 // 三花：白底 + 左头顶橘 + 背褐 + 臀橘
 {id:'calico',name:'三花',baseWhite:true,
  main:'#fffdf7',dark:'#f2ecdf',light:'#ffffff',out:'#462615',blush:'#ffa8bb',ear:'#ffa8bb',
  white:[],
  patches:[
    {u:0.180,v:0.055,ru:0.250,rv:0.135,col:'#f9c279'},
    {u:0.885,v:0.360,ru:0.230,rv:0.290,col:'#937857'},
    {u:0.540,v:0.905,ru:0.235,rv:0.110,col:'#f9c279'},
  ],stripe:null},

 // 奶牛：白底 + 右头顶灰 + 背灰 + 臀灰
 {id:'cow',name:'奶牛',baseWhite:true,
  main:'#fffdf7',dark:'#f2ecdf',light:'#ffffff',out:'#452616',blush:'#ffb0be',ear:'#ffb0be',
  white:[],
  patches:[
    {u:0.370,v:0.050,ru:0.235,rv:0.140,col:'#7e7671'},
    {u:0.895,v:0.410,ru:0.270,rv:0.330,col:'#7e7671'},
    {u:0.590,v:0.915,ru:0.205,rv:0.100,col:'#7e7671'},
  ],stripe:null},

 // 警长：燕尾服 —— 灰帽子（含双耳）+ 背臀连片
 {id:'sheriff',name:'警长',baseWhite:true,
  main:'#fffdf7',dark:'#f2ecdf',light:'#ffffff',out:'#452716',blush:'#ffb0be',ear:'#ffb0be',
  white:[],
  patches:[
    {u:0.245,v:-0.010,ru:0.360,rv:0.135,col:'#7d7672'},
    {u:0.930,v:0.470,ru:0.300,rv:0.430,col:'#7d7672'},
  ],stripe:null},

 // 起司：白底 + 浅灰虎斑块
 {id:'cheese',name:'起司',baseWhite:true,
  main:'#fffdf7',dark:'#f2ecdf',light:'#ffffff',out:'#462616',blush:'#ffb0ba',ear:'#e5a0a4',
  white:[],
  patches:[
    {u:0.200,v:0.060,ru:0.235,rv:0.125,col:'#c8b7ad'},
    {u:0.865,v:0.390,ru:0.250,rv:0.300,col:'#c8b7ad'},
  ],stripe:{col:'#b8a9a2',freq:6.0,onlyIn:'patch'}},

 // 暹罗：奶油底 + 耳尖/口鼻/前爪/尾
 {id:'siamese',name:'暹罗',baseWhite:false,
  main:'#fff5e1',dark:'#f4e8d4',light:'#fffaf0',out:'#462615',blush:'#e8a0a6',ear:'#e8a0a6',
  white:[],
  patches:[
    {u:0.235,v:0.300,ru:0.175,rv:0.125,col:'#5c4030'},
    {u:0.130,v:0.010,ru:0.095,rv:0.085,col:'#4b3125'},
    {u:0.420,v:0.008,ru:0.095,rv:0.082,col:'#4b3125'},
    {u:0.905,v:0.855,ru:0.140,rv:0.090,col:'#5c4030'},
    {u:0.070,v:0.875,ru:0.090,rv:0.070,col:'#5c4030'},
  ],stripe:null},

 // 白猫 / 黑猫 / 幽灵猫
 {id:'white',name:'白猫',baseWhite:false,
  main:'#fbf8f2',dark:'#e6dfd2',light:'#ffffff',out:'#4a4238',blush:'#ffb3c2',ear:'#ffb3c2',
  white:[],patches:[],stripe:null},
 {id:'black',name:'黑猫',baseWhite:false,
  main:'#45444d',dark:'#35343b',light:'#5e5d67',out:'#0d0b0a',blush:'#e08595',ear:'#e08595',
  white:[],patches:[],stripe:null},
 {id:'ghost',name:'幽灵猫',baseWhite:false,
  main:'#d3e3f2',dark:'#bacfe2',light:'#eaf5ff',out:'#4a5a70',blush:'#c8b3ff',ear:'#b9a4ff',
  white:[],patches:[],stripe:null},
];

function render(sp){
  const main=hex2rgb(sp.main),dark=hex2rgb(sp.dark),light=hex2rgb(sp.light),
        outC=hex2rgb(sp.out),blush=hex2rgb(sp.blush),ear=hex2rgb(sp.ear),
        whiteC=hex2rgb('#fffdf7');
  const o=canvas(base.width,base.height);
  for(let y=0;y<base.height;y++)for(let x=0;x<base.width;x++){
    const si=y*base.width+x,k=slot[si];
    if(k===255)continue;
    const u=(x-bb.x0)/bb.w, v=(y-bb.y0)/bb.h;
    const j=si*4,a=base.data[j+3];
    let c = k===0?outC : k===1?main : k===2?light : k===3?dark : k===4?blush : ear;

    if(k===1||k===3||k===5){
      const dim = k===3?0.90:1;
      let col=null;
      // 1) 白斑
      if(inAny(sp.white,u,v)) col=whiteC;
      // 2) 斑块
      const pt=inAny(sp.patches,u,v);
      if(pt) col=hex2rgb(pt.col);
      else if(k===5 && sp.earOver) col=ear;   // 未被斑块覆盖的耳内保留 ear 色
      // 3) 条纹
      if(sp.stripe && k!==5){
        const inW=inAny(sp.white,u,v)!=null;
        const inP=pt!=null;
        const allow = sp.stripe.onlyIn==='patch' ? inP : !inW;
        if(allow && inStripe(u,v,sp.stripe)) col=hex2rgb(sp.stripe.col);
      }
      if(col) c=shade(col,dim);
    }
    o.data[j]=c[0];o.data[j+1]=c[1];o.data[j+2]=c[2];o.data[j+3]=a;
  }
  return o;
}

function resize(img,L){
  const sc=Math.max(bb.w,bb.h),tw=Math.round(L*bb.w/sc),th=Math.round(L*bb.h/sc);
  const kx=bb.w/tw,ky=bb.h/th,out=canvas(tw,th);
  for(let y=0;y<th;y++)for(let x=0;x<tw;x++){
    let r=0,g=0,b=0,a=0,n=0;
    for(let sy=Math.floor(bb.y0+y*ky);sy<Math.ceil(bb.y0+(y+1)*ky);sy++)
    for(let sx=Math.floor(bb.x0+x*kx);sx<Math.ceil(bb.x0+(x+1)*kx);sx++){
      if(sx<0||sy<0||sx>=img.width||sy>=img.height)continue;
      const i=(sy*img.width+sx)*4,al=img.data[i+3]/255;
      r+=img.data[i]*al;g+=img.data[i+1]*al;b+=img.data[i+2]*al;a+=al;n++;}
    const j=(y*tw+x)*4;
    if(a>0.06){out.data[j]=r/a;out.data[j+1]=g/a;out.data[j+2]=b/a}
    out.data[j+3]=Math.round(Math.min(1,a/n*1.25)*255);}
  return out;
}

for(const sp of SP){
  const img=render(sp);
  for(const L of sizes) writePNG(OUT+'sp_'+sp.id+'_'+L+'.png',resize(img,L));
  console.log('  '+sp.id+' '+sp.name);
}
console.log('done',SP.length+' species');
