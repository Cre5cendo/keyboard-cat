// 把 agnes_image 出的单品红底猫 → 抠底 → 裁 bbox → 多档降采样 → 透明 PNG
// 用法: node make_sprite.js <源图绝对路径> <id> [尺寸列表,逗号分隔]
const P='C:/Users/Administrator/.dsh/skills/agent-sprite-forge/scripts/';
const {readPNG,writePNG}=require(P+'png0.js');
const OUT='H:/Users/Administrator/Documents/deepseek-harness/default-workspace/keyboard-cat/asf/';

const src=process.argv[2], id=process.argv[3]||'sprite';
const sizes=(process.argv[4]||'48,56,64,84').split(',').map(Number);
const img=readPNG(src);

// 1) 四角采样背景色（agnes 的品红不固定）
function px(x,y){const i=(y*img.width+x)*4;return [img.data[i],img.data[i+1],img.data[i+2]]}
const corners=[px(3,3),px(img.width-4,3),px(3,img.height-4),px(img.width-4,img.height-4)];
const bg=[0,1,2].map(k=>Math.round(corners.reduce((a,c)=>a+c[k],0)/corners.length));
const dist=(r,g,b)=>Math.hypot(r-bg[0],g-bg[1],b-bg[2]);
for(let i=0;i<img.width*img.height;i++){
  const r=img.data[i*4],g=img.data[i*4+1],b=img.data[i*4+2];
  if(dist(r,g,b)<110) img.data[i*4+3]=0;
}
// 2) bbox
let x0=1e9,y0=1e9,x1=-1,y1=-1;
for(let y=0;y<img.height;y++)for(let x=0;x<img.width;x++)
  if(img.data[(y*img.width+x)*4+3]>90){
    if(x<x0)x0=x; if(x>x1)x1=x; if(y<y0)y0=y; if(y>y1)y1=y;
  }
if(x1<0) throw new Error('bbox fail: 背景没抠干净');
const bw=x1-x0+1, bh=y1-y0+1;

// 3) 面积平均降采样（按 alpha 加权），输出高度 = size
function resize(size){
  const k=size/bh;
  const w=Math.max(1,Math.round(bw*k)), h=size;
  const out={width:w,height:h,data:Buffer.alloc(w*h*4)};
  for(let oy=0;oy<h;oy++)for(let ox=0;ox<w;ox++){
    const sx0=x0+ox*bw/w, sx1=x0+(ox+1)*bw/w;
    const sy0=y0+oy*bh/h, sy1=y0+(oy+1)*bh/h;
    let ra=0,ga=0,ba=0,aa=0,n=0;
    for(let sy=Math.floor(sy0);sy<Math.ceil(sy1);sy++)
      for(let sx=Math.floor(sx0);sx<Math.ceil(sx1);sx++){
        if(sx<0||sy<0||sx>=img.width||sy>=img.height)continue;
        const i=(sy*img.width+sx)*4, a=img.data[i+3]/255;
        ra+=img.data[i]*a; ga+=img.data[i+1]*a; ba+=img.data[i+2]*a; aa+=a; n++;
      }
    const o=(oy*w+ox)*4, cov=n?aa/n:0;
    if(cov<0.42){out.data[o+3]=0;continue}
    out.data[o]=Math.round(ra/aa); out.data[o+1]=Math.round(ga/aa);
    out.data[o+2]=Math.round(ba/aa); out.data[o+3]=Math.round(Math.min(1,cov*1.25)*255);
  }
  return out;
}
// 4) 白描边：让猫在深色键盘上浮起来
function outline(o,w=1,color=[252,248,240]){
  const a=o.data, W=o.width,H=o.height;
  const src=new Uint8Array(W*H); for(let i=0;i<W*H;i++) src[i]=a[i*4+3];
  for(let y=0;y<H;y++)for(let x=0;x<W;x++){
    const i=y*W+x; if(src[i]>=90) continue;
    let near=false;
    for(let dy=-w;dy<=w&&!near;dy++)for(let dx=-w;dx<=w;dx++){
      const nx=x+dx,ny=y+dy; if(nx<0||ny<0||nx>=W||ny>=H)continue;
      if(src[ny*W+nx]>=170){near=true;break}
    }
    if(near){const o4=i*4;a[o4]=color[0];a[o4+1]=color[1];a[o4+2]=color[2];a[o4+3]=200}
  }
  return o;
}
for(const s of sizes){
  const img2=outline(resize(s));
  writePNG(OUT+'sp_'+id+'_'+s+'.png',img2);
  console.log('  ->', 'sp_'+id+'_'+s+'.png', img2.width+'x'+img2.height);
}
console.log('done', id, 'bbox', bw+'x'+bh, 'bg', bg.join(','));
