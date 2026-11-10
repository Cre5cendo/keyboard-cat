# 键盘猫 · 交接文档

> 生成时间：2026-11-06 19:08（北京时间）
> 工作目录：`H:/Users/Administrator/Documents/deepseek-harness/default-workspace/keyboard-cat/`
> 主文件：`demo.html`（单文件，1315 行，无依赖，双击即跑）

---

## 一、这个项目是什么

BongoCat 的变体：桌面上显示一整块 87 键（现为 60 键）键盘 + 一只像素猫。猫蹲在某个键上，你用鼠标点它 / 按下它脚下那颗键，猫就被**击飞**到另一颗键上，带旋转、拖尾、落地挤压、连击、鱼干收益。

- 一个程序两个输出：透明置顶桌面挂件 + 本地 http 页面给 OBS Browser Source（OBS 窗口采集会丢 alpha，所以走浏览器源）。
- 正式版计划用 Tauri（比 Electron 省内存），当前是纯 HTML demo，跑在浏览器里。
- 计数只存本地（今日/累计/最高连击），不做键盘记录。

---

## 二、当前状态（一句话）

**面板 UI 换皮已基本完成，87 项自测全 PASS。** 用户提的六条修正 + 第二轮四条修正都已落地，只剩「标签胶囊的立体感」与「正文起始位置」两项在最后一轮调整中（代码已改完并通过自测，等用户验收）。

---

## 三、用户最新需求（未验收的两项）

> 用户原话（m07368）：「这几个按钮还是做成图2的样子。详情按钮文字位置调到居中。所有界面的正文位置，放在按钮底部开始」

1. **标签胶囊要图2的立体感** —— 深色外圈描边 + 顶部内侧亮高光 + 上亮下暗竖向渐变。
   - 参考图已存到 `ref/pill_target.png`（237×160）。
   - **注意**：这与更早的 m06774「顶部和底部的黄色和咖啡色多余，不要」不矛盾 ——
     那次反对的是「顶部整条黄、底部整条咖啡」的粗分割边；这次要的是**细描边 + 内侧渐变高光**。
   - **当前状态**：`ui/tab_on.png`(187×78) / `tab_off.png`(198×78) / `tab_diary.png`(164×78) /
     `tab_badge.png`(161×78) / `tab_set.png`(158×78) 已用 `_gen24g.js` 重建 ——
     保留原图外圈像素（描边与内高光），只抹掉内芯文字。**但实机看立体感仍偏弱**，
     因为 CSS 把 187×78 拉伸到 62px 高、横向 border-image 拉伸，渐变被压平了。
     下一步建议：直接加大图里渐变的对比度（把顶部高光再提亮、底部再压暗 8~12），
     或改用 `background-size:100% 100%` 的整图 + `padding` 撑宽，避免 border-image 拉伸。

2. **详情按钮文字居中** —— 已改：`.card .dt` 与 `.detX` 从 `line-height` 居中改为
   `display:flex;align-items:center;justify-content:center`，并换成新的空白金色底图
   `ui/btn_gold_sm.png`(104×38) 与 `ui/btn_gold.png`(140×76)。

3. **正文位置从按钮底部开始** —— 已改：
   - `#panelHead` 高度 104px → **114px**（胶囊底实测 169px，head 底也到 169）
   - `#panelBody` 上内边距 2px → **0**
   - `.tip2` 上外边距 6px → **0**
   - `.slot` 上外边距 16px → **0**
   - `h4:first-child` 上外边距归零（`#panelBody>div:first-child>h4:first-child`）
   - 实测验证：5 个标签页的第一个子元素 top 全部 = 169 = 胶囊底，已对齐。

---

## 四、代码地图（demo.html）

| 行号 | 内容 |
|---|---|
| 22–45 | 面板外壳 CSS：`#panelBox` `#panelHead` `#tabs` `#tabs button` `#panelClose` `#panelBody` + 滚动条 |
| 46–57 | 卡片 CSS：`.grid` `.card` `.card.ineq` `.card .dt` `.card .n` `.card .s`（两行截断） |
| 66–91 | 槽位 / 详情页 CSS：`.slot` `.sl` `.det` `.detX` `.rw` |
| 93–101 | 通用：`h4` `.tip2` `.sw`（开关）`#panelBody button` |
| 124 | 面板 DOM 骨架（一行） |
| 151–190 | `SPECIES` 11 只猫、`THEMES` 8 套键盘主题、`ITEMS` 饰品（已隐藏）、`SIZES` 四档 |
| 190 | 存档键 `kbcat.v2` |
| 191 | `defaults()` 默认存档 |
| 202–214 | `THEMES` 主题表 |
| 248–260 | 猫咪尺寸：`SIZES`/`SIZEK`/`setCatSize()`/`loadSprites()` |
| 509 | `drawKeys()` —— 樱夜主题走裁切位图分支（`skOk`），其它主题走矢量键帽 |
| 584 | `const ACC_ON=false` —— **饰品总开关，已关** |
| 650 | `draw()` 主绘制 |
| 693–716 | 标签页框架：`TABS` / `TABFN` / `paint()` / `showTab()` / `buildTabs()` |
| 734 | `panelDex()` 图鉴 + 编队 |
| 779 | `showCat()` 详情页 |
| 835 | `panelBadge()` 成就 |
| 894 | `panelGacha()` 扭蛋 |
| 922 | `shopHTML()` 商店（含键盘主题卡） |
| 946 | `panelDiary()` 日记 |
| 984 | `panelSet()` 设置 |
| 1073–1090 | URL hash 入口（见下） |
| 1114 | `selfTest()` 自测 87 项 |

---

## 五、调试入口（URL hash）

```
demo.html#test          跑自测，结果写进 document.title（JSON 数组）
demo.html#sim=0.3       经济模拟 120 秒
demo.html#dex           直接打开图鉴
demo.html#gacha         扭蛋商店
demo.html#diary         日记
demo.html#badge         成就
demo.html#set           设置
demo.html#cat           猫详情页
demo.html#theme=sakura  指定主题
demo.html#th&theme=wood 主题缩略图模式（生成 ui/th_*.png 用）
demo.html#geo           几何探针，把布局坐标写进 title（调试用，可删）
demo.html#<tab>bot      打开该页并滚到底
```

hash 是**前缀匹配**，可以拼接，如 `#dex&geo`。

### 跑自测

```bash
cd keyboard-cat && node _t_list_9131.js    # 打印 87 项逐条结果
cd keyboard-cat && node _t_run_8872.js     # 只打印 PASS/FAIL 计数
```

**关键**：`selfTest()` 返回的是**字符串数组**（`"PASS xxx"` / `"FAIL xxx"`），
不是对象数组。解析要先 `JSON.parse` 再 `startsWith('PASS ')`。
直接 `grep FAIL` 会误命中 `ok()` 函数定义的源码。

---

## 六、资产清单

### `ui/`（56 个 PNG，面板视觉件）
| 文件 | 尺寸 | 用途 |
|---|---|---|
| `panel.png` | 1402×981 | 面板底板（含左上猫耳 + 圆角 + 爪印水印） |
| `plate.png` / `topstrip.png` / `shelf.png` | 1346×945 / 1346×62 / 380×100 | 底板拆分件（中间产物） |
| `close.png` | 80×82 | 右上粉色圆形关闭钮 |
| `tab_on.png` | 187×78 | 选中的奶油金胶囊（空白，文字走 HTML） |
| `tab_off.png` | 198×78 | 未选紫灰胶囊 |
| `tab_diary.png` / `tab_badge.png` / `tab_set.png` | 164/161/158×78 | 同宽的三颗 |
| `ic_on.png` / `ic_off.png` / `ic_diary.png` / `ic_badge.png` / `ic_set.png` | 37~38px | 五个标签图标 |
| `card.png` | 250×210 | 卡片底（紫渐变圆角） |
| `slot_off.png` / `slot_on.png` | 181×169 | 编队槽位（紫填充 / 金边） |
| `btn_gold.png` / `btn_gold_sm.png` | 140×76 / 104×38 | **空白**金色胶囊（详情/返回按钮底） |
| `btn_detail.png` | 88×56 | 旧的带「详情」二字版本（已弃用） |
| `paw_wm.png` | 120×105 | 右下角大爪水印 |
| `th_<id>.png` | 1136×416 | 8 套键盘主题预览图（sakura 1128×408） |

> 未使用但保留：`tab_dex_*.png` / `tab_gacha_*.png` / `ic_*_off.png` 等整颗切图（文字烤死，无法配动态标签）、
> `track.png` / `paw.png` / `star.png` / `ear_*.png`。

### `asf/`（236 个 PNG）
- `sp_<pid>_<size>.png` —— 11 只猫 × 4 档尺寸的立绘
- `cap_<theme>_<l|m|r>.png` —— 各主题键帽裁切件
- `sk_<code>.png`（59 张）+ `sk_board.png` —— 樱夜主题从参考图扣的键帽与底板
- `acc_*.png` —— 饰品位图（4 件，因功能隐藏而闲置）

### `ref/`（参考图，本轮从 attachments 复制进来避免路径失效）
- `mockup.png` —— 面板版式 mockup（1536×1024）
- `panel_base.png` —— 纯底板无内容
- `pill_target.png` —— **用户要的立体胶囊样式（237×160）**
- `kbd_ref.png` / `kbd_ref2.png` —— 樱夜键盘实拍参考

---

## 七、关键脚本（`_*.js`，485 个，绝大部分是一次性试验，可删）

**还在用的几个：**

| 脚本 | 作用 |
|---|---|
| `_gen24g.js` | 重建 5 颗胶囊：保留原外圈，只抹内芯文字（距离场 `dist>2.6` 才替换） |
| `_genbtn2.js` | 生成空白金色按钮 `btn_gold*.png`（先 `cl()` 再赋值，防溢出变绿） |
| `_gen31.js` | 抠 5 个标签图标（按行背景距离 >40 取紧凑框） |
| `_cut2.js` | `rr(w,h,r,fillFn,border,bw)` 圆角矩形生成器（card/slot 来自它） |
| `_shot_v9.js` | 截 4 个标签页 DPR2 截图 |
| `_half9.js` | 把 DPR2 截图面积平均降采样一半（read_image 读不了 >1500px） |
| `_probe_geo.js` | 跑 `#geo` 探针，打印标签/正文坐标 |
| `_t_list_9131.js` | 打印 87 项自测逐条结果 |
| `_t_run_8872.js` | 只打印 PASS/FAIL 计数 |

**改 demo.html 的方式**：不要用 `tools.edit`（old_string 逐字符匹配，缩进差一格就失败），
一律写一个 `_pN.js` 用 `fs.readFileSync` + `String.replace` + `writeFileSync`，并先断言 `indexOf(a)>=0`。

---

## 八、踩过的坑（务必先看，能省几小时）

### Chrome headless
1. **截图必须给绝对路径**，相对路径会写到别处。
2. `--window-size=680,340` 会被缩放到 512×256 —— **宽度必须 ≥600**。
3. DPR2 截图 2360×1760 直接 `read_image` 会报
   `value.image.originalDimensions is not a declared property` 或 `Unsupported or malformed image data`。
   **先面积平均降采样到 ~1180 宽**再用 `areaDown(im, w, h, cov)` —— 注意签名是**目标宽高**，不是比例。
4. 引号里带 `$'` 的 bash 命令会被 TS 解析器吃掉，多行脚本一律写成 `.js` 文件再跑。

### png0.js（`C:/Users/Administrator/.dsh/skills/agent-sprite-forge/scripts/png0.js`）
5. `P.crop(img, box)` 的参数是 **`{x0,y0,w,h}`**，传 `{x,y,w,h}` 会静默产出 NaN。
6. 没有 `P.resizeArea` / `P.copy`，只有 `canvas / blit / crop / scale(整数倍) / bbox / palette / writePNG`。
7. `P.writePNG(path, img)` 参数顺序是 **(路径, 图像)**；路径相对 **cwd**。

### demo.html 自身
8. **同一内联 script 里重复声明同名变量 → 整段 script 不执行**，表现为「页面停在初始态、按钮全哑、title 还是旧标题」。
   排查：把内联 script 抽成 `.mjs` 跑 `node --check`，它直接指行号。
9. `tools.write` 对已存在文件**要求先 read**；脚本名重复用会报 `file has not been read`，换新文件名即可。
10. 删 if/else 链时把条件首行一起删 → 悬空 `else if` → SyntaxError，症状同上。

### 像素重建（血泪）
11. 想把底板「抹掉内容只留底色」，任何**逐行中值/众数/百分位/插值**都会因采样列落在内容上而拉出彩色条纹。
    唯一稳的做法：内部**全部填单一底色**，只保留四边原像素。
12. 胶囊抠字，唯一成功的是 `_gen24g.js` 的「保留外圈、只替换 dist>2.6 的内芯」。
    逐行中值重涂、干净列平铺、unblend 迭代、离群行剔除 **全部失败过**（会留白杠或文字鬼影）。
13. 生成 PNG 时 `P.canvas` 返回全 0 Buffer，**先显式清零**再按 mask 写，否则边缘留脏值。
14. 混色公式直接 `Math.min(255, r*k+...)` 会让三通道一起推爆**偏绿**，必须写成 `cl(v)` 先算后 clamp。

### 关于「立体感」
15. 用户拒绝**拟物立体投影**（盒阴影、文字阴影、"按钮底部一坨黑"），但**接受图片自带的细描边 + 内侧渐变高光**。
    两者别搞混：CSS `box-shadow` 一律不要，立体感全部烤进 PNG。
16. 但 border-image 会把 78px 高的图拉伸到 62px，**渐变会被压平**。要立体感就得把图里的对比度做大，
    或者不用 border-image 拉伸，改用整图 + padding。

---

## 九、自测覆盖（87 项）

| 组 | 项数 | 内容 |
|---|---|---|
| 扭蛋/猫池 | 8 | rollCat 合法、不返回满星、免费次数、门槛解锁、锁死、抽满关闭 |
| 星级奖励 | 3 | 1星特效 / 2星落地 / 3星装饰 |
| 编队 | 2 | 可同时 3 只、同花色只算一只 |
| 标签页 | 8 | 5 页都能渲染、切页、高亮唯一、扭蛋商店合并、尺寸统一、内部滚动 |
| 设置 | 7 | 开关而非复选框、隐私/自动/隐藏写存档、快捷键已取消、关闭重开回原页 |
| 主题 | 12 | 免费换、西瓜已移除、樱夜注册/绘制/深色粉色彩色分类/键帽全覆盖/不叠字 |
| 饰品（隐藏） | 4 | 入口全收、成就数 14→13、日记不提 |
| 饰品底层 | 4 | 旧格式可绘、三层可拆、锚点比例、换尺寸跟放大 |
| 成就 | 3 | 首次达成、不重复发、按数量触发 |
| 物理 | 7 | 按键击飞、鱼干增加、同键连锁、猫不会自己走、四档尺寸切换 |
| 落点范围 | 7 | Fn 键存在、总宽 15U、Fn 有键帽图、Fn 默认不可落点、限定字母区、手动划出、双开关 |
| 编队交互 | 6 | 点图鉴上阵、填第三位、满 3 不显空位、点编队下阵、出现 + 空位、不再弹选择界面 |
| 面板细节 | 7 | 主题卡预览图、按钮无投影、无立体文字、详情按钮与上阵中同高、卡片缩高、描述两行 |

---

## 十、下一步建议（按优先级）

1. **等用户验收**：胶囊立体感 + 正文位置。若用户仍嫌立体不够，按「坑 16」加大图内渐变对比度。
2. 已问但未答的遗留问题：
   - 卡片高度是否统一为固定值（描述不足两行的猫会矮约 17px，网格有起伏）。
   - 是否彻底删除饰品代码（`ITEMS`/`ACCDEF`/`accPicker`/30 多个 `asf/acc_*.png`，当前只是 `ACC_ON=false` 关掉入口）。
   - 非樱夜主题下 Fn 键仍是矢量占位，没单独出键帽图。
3. **收尾清理**：`keyboard-cat/` 现有 485 个 `_*.js`、55 个试验 `.html`、230MB PNG。
   交付前建议把 `_*.js` 与 `_*.png` 全删（只留 `demo.html` / `ui/` / `asf/` / `ref/` / 少量正式截图）。
4. **打包成桌面程序**：Tauri + 透明置顶窗口 + 全局键盘钩子（当前 demo 只在页面有焦点时收键）。
   同一份 `demo.html` 同时充当 OBS 的浏览器源页面。

---

## 十一、怎么跑起来

```bash
cd "H:/Users/Administrator/Documents/deepseek-harness/default-workspace/keyboard-cat"
# 直接双击 demo.html，或起个本地服务（OBS 需要 http 源时）
node -e "require('http').createServer((q,s)=>{const fs=require('fs'),p='.'+decodeURIComponent(q.url.split('?')[0]);fs.readFile(p==='./'?'./demo.html':p,(e,d)=>{if(e){s.writeHead(404);s.end();}else{s.writeHead(200);s.end(d)}})}).listen(8099)"
```

---

## 十二、2026-11-06 晚：进度更新（Tauri 外壳已跑通）

### 已完成
1. **标签胶囊立体感**：按 ref/pill_target.png 重出 ui/tab_*.png（198×62 / 187×62 原生高度），border-image slice 从 0 39 改为 0 31，1:1 映射不再被压扁。生成器 `_tabgen.js`。
2. **右缘米色竖条（根因已定位）**：不是滚动条，是 `ui/paw_wm.png` 右侧被烤进去了 8px 宽奶油色竖条（x=112..119，全高不透明）。已裁成 112×105，CSS 背景尺寸同步改 112px 105px。滚动条本身也从 18px 奶油轨道改成 12px 透明轨道 + 半透明金拇指。
3. **卡片高度统一**：.card 固定 height:132px（.card.theme 保持 auto），五个页面的卡片全部 132，无内容裁切。
4. **标签栏溢出**：#tabs padding-left 214→140、gap 9→8，五枚标签右缘 842 < 关闭键 869，设置页不再被 X 压住。
5. **桌面外壳（Tauri 2）**：透明置顶 + 贴任务栏 + 鼠标穿透 + 全局键盘钩子 + 托盘 + 快捷键都已实机跑通，见 `desktop/README.md`。
   - **全局钩子关键 bug**：在 WH_KEYBOARD_LL 回调里直接 `app.emit`，慢活超过 LowLevelHooksTimeout，系统会**静默摘钩**（表现为收十几条后彻底没反应）。修法：回调只做 `tx.send`，另起线程 `rx.recv()` 再 emit。
6. 自测 87 项全绿（原「猫咪卡片已缩高」断言改成「高度统一 ≤1.5px 差」）。

### 构建/运行
```bash
cd keyboard-cat/desktop
node _sync_web.js            # demo.html -> web/index.html（注入 shell.js），改完 demo 必须重跑
./_build.bat                 # 需要 RUSTUP_HOME=H:\rustup
./src-tauri/target/debug/keyboard-cat.exe
```
改 web 资源后必须 `touch src-tauri/src/main.rs` 再 build，否则 cargo 不会重新嵌入资源；rebuild 前先杀进程（否则 os error 5）。

### 仍未做
- 打包（NSIS 安装包 / release 构建）、开机自启（任务计划）、托盘图标细节。
- Esc 切回穿透后的交互提示；OBS 纯净模式（`.plane`/`.taskbar` 目前由 shell 注入 CSS 隐藏）。
- 饰品代码仍只是 `ACC_ON=false` 关闭入口；非樱夜主题 Fn 键仍是矢量占位；日记 8 句模板；猫间互动 0 代码。


## 十三、2026-11-06 21:00 托盘 + 菜单缩放

1. **托盘图标**（`src-tauri/src/main.rs`）
   - 左键单击：窗口呼出 / 隐藏（`.show_menu_on_left_click(false)` + `on_tray_icon_event` 里判 `TrayIconEvent::Click{button:Left,button_state:Up}`）。
   - 右键菜单：「菜单」「显示 / 隐藏」「切换鼠标穿透」「退出」。
   - 「菜单」= `open_menu()`：show + 置顶 → `set_passthrough_state(app,false)`（让面板可点）→ `emit("kc-open-panel")`。
2. **前端配合**（`desktop/_sync_web.js` 的 SHELL 模板）
   - `#panel{transform:scale(2/3)}`：**只缩菜单**，挂件键盘保持原尺寸（用户 m00935 明确要求）。
   - 监听 `kc-open-panel` → 点 `#bOpen` 打开面板。
   - 包装 `window.closePanel`：关面板即 `set_passthrough {on:true}`；Esc 也走这条。
   - 交互态（`kc-interactive`）下左上角出现红色 ✕（`#kcQuit`），点了 `quit_app`，防止再找不到关闭入口。
3. 实测：`#panelBox` 949×644 → 633×429（=2/3）；`#widget` 仍 560×287（未变）；`_t_run_8872.js` 87 PASS。
4. 注意：Win11 21H2（build 22000）默认把新托盘图标收进 ^ 溢出区，需要在「设置 → 个性化 → 任务栏 → 选择哪些图标显示在任务栏上」里打开，或把图标从 ^ 拖到任务栏。

## 十四、2026-11-06 22:20 托盘真凶找到：exe 放在 H: 盘就 NIM_ADD 失败

1. **根因（已验证）**：同一份 exe，放在 H: 盘运行时 `Shell_NotifyIconW(NIM_ADD)` 一律返回 FALSE + `ERROR_ACCESS_DENIED(5)`；
   复制到 `C:\Users\Administrator\AppData\Local\kctray\kc.exe` 再跑就 `ok=true`，图标真的出现在托盘（`Shell_NotifyIconGetRect` 返回 rect≈(3020,1384,3044,1440)）。
   - 排除项：窗口站 WinSta0 / 桌面 Default / session 1 完全相同；提权与非提权一样；COM(STA) 初始化无关；嵌现代 manifest 无关；
     由 bash / cmd / PowerShell / WMI 启动都一样；主线程与工作线程都一样；结构体 sizeof=976 与 PowerShell 成功用例一致。
   - 结论：**托盘类程序必须放在 C: 盘运行**；H: 盘只用来编译。
2. 托盘已完全自写（`src-tauri/src/tray.rs`，不再用 tray-icon crate）：自建 `KC_TrayWindow` + `Shell_NotifyIconW` + `TrackPopupMenu`。
   - 左键单击 = 显示/隐藏挂件（截图实测）；右键菜单「菜单 / 显示 / 隐藏 / 切换鼠标穿透 / 退出」（截图实测）。
   - 调试日志 `desktop/kc_boot.log`（`dbg()`），含 `tray NIM_ADD[...] ok=... err=...` 与 `rect=...`。
3. 已清理：`tray::probe_main_thread()` 删除；`app.manifest` 恢复 `requireAdministrator`；single-instance 插件恢复（重复启动时 show 已有窗口）。
4. 运行循环（新增硬规矩）：
   ```bash
   taskkill //F //IM keyboard-cat.exe
   cd keyboard-cat/desktop && node _sync_web.js
   touch src-tauri/build.rs        # 改前端后必须，否则不重新嵌入 web 资源
   cmd //c _build.bat              # 需要 RUSTUP_HOME=H:\rustup
   cp src-tauri/target/debug/keyboard-cat.exe /c/Users/Administrator/AppData/Local/kctray/kc.exe
   cd /c/Users/Administrator/AppData/Local/kctray && ./kc.exe
   ```
   requireAdministrator 的 exe 无法从非提权 bash 启动（Permission denied）；调试期可临时把 manifest 改 asInvoker。
5. 仍未做：release 构建 + 安装到 C: 盘 + 开机自启；「菜单」菜单项的端到端点击验证（代码路径与 #bOpen 相同）。



十五、2026-11-06 22:50 面板正文放大 + 真实键盘音 + 拖动
------------------------------------------------------
用户这轮要 5 件事（m02190 原话）：「现在菜单界面有黑色边，这个隐藏掉。键盘音我想要清脆的真实键盘音，
非穿透状态下键盘可以鼠标拖动调整位置。现在UI的长宽大小合适，但是内容太小了看不清，按照上一版的正文尺寸恢复，
你把卡片啊什么的排的紧凑一点，不要动UI的总长总宽。菜单的标题头可以拖动调整整个菜单的位置。」

1. 黑底：`#panel{background:#0a0810}` → `transparent`（demo.html:21）。黑边就是这层铺满整窗的深色底。
2. 正文放大（关键手法）：面板外层有 shell 注入的 `#panel{transform:scale(2/3)}`，字号跟着缩。
   不动面板总长宽、又要正文回到 1:1，用 CSS `zoom` 反向抵消：
   `#panelBody{zoom:1.5}`（ZB=1/S，在 desktop/_sync_web.js 的 SHELL 模板里）。
   面板外框/标签栏/关闭键/猫耳仍是 2/3 外观，只有正文区域回到 13px。
3. 3 列：正文放回 1:1 后可用宽度变窄，`repeat(auto-fill,minmax(186px,1fr))` 只剩 2 列、卡片被拉得很宽。
   shell 里加 `#panelBody .grid{grid-template-columns:repeat(3,1fr)}` 固定 3 列（实测列宽 175px）。
4. 紧凑：`.grid` gap 12→9；`.card` 的 `height:132px` → `min-height:114px`（自适应，行内靠 grid stretch 等高）。
   demo.html 自测里「猫咪卡片高度统一」改成按行分组比较（不同行可以不同高），仍是 87 PASS。
5. 键盘音：`sndKey` 重写——① 白噪声过带通（上排 3.8k/中排 2.9k/下排 2.3k Hz，Q=1.2）+ 1.2k 高通，
   1.5ms 起振、42ms 衰减；② 175→88Hz 正弦"键帽触底"闷响 62ms。松键另有 `sndKeyUp()`（3.4k 带通 28ms，音量 12%）。
   旧的按行给音高的 `tone()` 键音已删除（`keyWave()` 也一并删）。
6. 拖动：shell 里 `mkDrag(sel)`，mousedown（左键、非 button 元素）→ `__TAURI__.window.getCurrentWindow().startDragging()`。
   挂在 `#panelHead` 和 `#widget` 上。穿透状态下收不到鼠标事件，所以天然"只在非穿透时能拖"。

验证（实机，临时 asInvoker + 临时关掉 single-instance 的 debug 构建）：
- 面板在 WebView2 里排版与 Chrome 预览一致，黑底消失，正文 13px；
- 合成鼠标拖动实测：面板标题栏拖动 → 窗口 rect delta +140/+90；键盘本体拖动 → −200/−140；
- WebAudio 冒烟：`ctx=running synth=OK`；
- `node _t_run_8872.js` → 87 PASS 0 FAIL。

部署：release 已构建（`desktop/src-tauri/target/release/keyboard-cat.exe`）。
旧实例（PID 25408，提权）占着 `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat.exe` 无法覆盖，
新版本先放在同目录 `keyboard-cat-new.exe`；托盘→退出 关掉旧的之后，改名替换即可。

踩坑：给 shell.js 临时加测试注入时，把代码追加在 IIFE 外面会拿不到闭包里的 `T`，
`invoke('set_passthrough')` 静默失败 → 窗口仍是穿透态 → 拖动测试全 0 delta。写注入要用 `window.__TAURI__`。


## 十六、2026-11-06 23:15 面板与键盘拆成两个窗口 + 拖动白框修掉

### 1. 为什么要拆
面板和键盘原本在同一个 1160×860 的窗口里，拖任何一个都移动整窗。用户要求「菜单和键盘移动分开」。
在同一个窗口里用 CSS 反向补偿（拖动时把另一半反向位移）会让部件跑出窗口被裁，所以改成两个独立窗口。

- 窗口 main（label=main）= 挂件窗，index.html，shell 里 #panel{display:none!important}。
- 窗口 panel（label=panel）= 面板窗，tauri.conf.json 第二项：transparent / decorations:false / shadow:false / alwaysOnTop / skipTaskbar / resizable:false / center:true / visible:false。
- capabilities/default.json 的 windows 改成 ["main","panel"]。
- 前端靠 __TAURI__.window.getCurrentWindow().label === 'panel' 判断自己是哪个窗（不用 query 参数）。
- Rust：open_panel(app) 首次居中摆放 → set_ignore_cursor_events(false) → show → topmost → focus → emit("kc-open-panel")；close_panel(app) 只 hide。命令 open_panel_cmd / close_panel_cmd。

### 2. 两个窗口都要贴着自己的内容（用户报的"白框跟内容不匹配"）
透明窗口在拖动时系统画的是窗口矩形，窗口比内容大，就会看到一圈和内容对不上的白框。
所以新增命令 fit_window(label, w, h, anchor)：w/h 传物理像素（前端已乘 devicePixelRatio），anchor="br" 贴工作区右下（挂件用），"keep" 保持中心不动（面板用）。

- fitWidget()：#widget 560×262 CSS → 窗口 (560+16)×(262+16)=576×278 CSS → 乘 dpr 1.15 = 663×320 物理。
- fitPanel()：#panelBox 1120×780、scale(2/3) → 746.7×520 CSS → 窗口 763×536 CSS → 878×617 物理。面板 hidden 时 rect 全 0，所以 open 后 80ms 才量。
- Tauri 的 LogicalSize 和 WebView2 的 devicePixelRatio 不是一回事（本机 OS scale=1.0，WebView2 dpr=1.15），用 LogicalSize 传会小一圈、内容被裁；必须自己乘 dpr 再按 PhysicalSize 设置。
- #panelBox 在 flex 容器里会被压缩，必须 flex:0 0 auto!important（否则 1120 被压成 890，缩放后只剩 593 宽）。

验证：_enum.ps1 枚举本进程所有顶层窗口 → 面板 878×617、挂件 663×320，都等于内容+16px；静止截图内容正好填满窗口；拖动中截图窗口矩形与内容边缘重合，白框不再错位。node _t_run_8872.js 仍 87 PASS 0 FAIL。

部署：release 已构建，放在 C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe（23:14，13,430,784 字节）。旧的 keyboard-cat.exe 还在跑，托盘→退出 之后改名替换。


## 十七、2026-11-06 23:28 设置项「失效」的根因 + 自动演示下线

用户报「显示猫咪 / 猫咪尺寸 / OBS 隐私 现在失效了」，并要求把「自动演示」隐藏掉。

根因：面板窗口改设置只写 localStorage + emit `kc-store`；挂件窗的 `reloadStore()` 只覆盖了 `store` 对象，
再调 equipCats/renderCount。但 `hidden`(demo.html:332)、`privacyOn`(demo.html:724)、`CATW/CATH`(231)
都是**运行时变量**，不跟着 store 变，所以开关点了、存了、画面没反应。

修复：
- demo.html 新增 `window.kcApplyStore()`：hidden=!!store.noHide；privacyOn=!!store.privacy；autoOn=false；
  store.auto=false；setCatSize(store.catSize||44)；loadSprites()；equipCats()；renderCount()。
- `_sync_web.js` 的 `reloadStore()` 优先调它；`kc-store` 监听里 60ms 后补一次 `fitWidget('keep')`（尺寸变了要跟）。
- `fitWidget(anchor)` 加 size key 去重：尺寸没变就不动窗口，避免每次同步都把人拖好的位置拽回右下角。
- 删除设置页「自动演示」整行（swRow/bindSw 都删），存档里的 auto 不再生效。
- 自测：89 PASS 0 FAIL（新增「自动演示已从设置页移除」「kcApplyStore 重新套用设置」「kcApplyStore 复原」）。

实机验证（合成点击）：设置页已无自动演示；关「显示猫咪」→ 猫消失，开 → 猫回来；选 48px → 猫明显变大。

OBS 隐私的正确理解：它只影响**按下时的灯效** —— `const lit=k.press&&(!priv||isCat||k.flash>0)`
（demo.html:550），静止时没有任何视觉差别。所以「点了没反应」不是 bug，要敲键才看得出来。

部署：release 13,431,296 字节（23:27）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`。
坑：`target/release/keyboard-cat.exe` 被一个提权残留进程（keyboard_cat.exe PID 13516）锁住，
cargo 报 os error 5；改用 `CARGO_TARGET_DIR=src-tauri/target_r` 另编一份才成功。

## 十八、2026-11-08 16:30 挂件窗口贴合键盘本体 + 去掉左上角 ✕

用户 m02902：「这个界面边界仍然跟我的实际键盘不符。坐上角这个X按钮取消」（截图里键盘外一圈白框 + 左上红 ✕）。

1. **真凶不是窗口 PAD，是画布自带的透明留白**。#widget 里的 canvas 尺寸是
   `W=PAD*2+15*U=560`、`H=TOP+5*U+PAD=262`（demo.html:128/138，U=36,PAD=10,TOP=72），
   但画布只有下半部分有内容，上方 TOP 区和左下右 PAD 是透明的。
   用 `getImageData` 扫 alpha>10 的包围盒实测 = `640x226 off=2,71`（设备像素，DPR 1.15），
   即键盘上方有约 62 CSS px 透明带 —— 这才是「白框比键盘大一圈、上面多出一截」的来源。
2. **修法**（`desktop/_sync_web.js`）：新增 `hugCanvas(anchor)` 取代 `fitWidget`：
   扫 alpha 包围盒 → `sc=W/rect.width` → canvas 改 `position:fixed;left=-bbox.x/sc;top=-bbox.y/sc`
   → `invoke('fit_window',{label:'main',w:bbox.w,h:bbox.h})`。
   调用于 load 后 200ms、`kc-store` 后 120ms、以及 `ResizeObserver(#cv)` 后 120ms。
   同时挂件分支 CSS 改为 `#widget{position:fixed!important;left:0!important;top:0!important;filter:none!important}`，
   `html,body{margin:0;padding:0;overflow:hidden}`，去掉原来的 PAD=8 边距与外发光。
3. **删掉左上角红 ✕**（`#kcQuit` 整块移除，用户要求）；Esc 仍在交互态下恢复鼠标穿透。
   关闭程序的入口只剩托盘右键 → 退出。
4. 实测：窗口 640×226 物理，键盘正好填满窗口（截图加 16px 外扩后，键盘四边刚好压在窗口边界上）。
   自测 `node _t_run_8872.js` → 89 PASS 0 FAIL。
5. **踩坑**：PowerShell 截图必须脚本开头 `SetProcessDPIAware()`，否则 DPI 虚拟化让裁剪位置全错；
   `EnumWindows` 回调里 `Write-Output` 会被吞（结果要收集到 `$script:` 变量再打印）；
   `PrintWindow(h,dc,2)` 才能拍到透明窗口真实内容。
6. 部署：release 13,431,808 字节（16:32）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`。
   注意：桌面上残留的旧提权实例（keyboard_cat.exe）占着 single-instance 互斥体，
   直接双击新 exe 只会把旧窗口 show 出来 —— 必须先从托盘退出旧实例。

## 十九、2026-11-08 17:0x　CPU 拉满修复（异地机器反馈）

用户 m03164：「我发给别人了。他们那里cpu占用拉满」。本机空闲 CPU 实测 0ms/5s（16 核），
说明问题出在弱 GPU / 软件渲染 / 高刷屏的机器上，于是从代码层找不必要的工作量。

**根因 1 —— 无条件 60fps 全量重画**（`demo.html` 旧 `loop()`）
每帧 `draw()` 都会 `drawBoard()+drawKeys()`（60 颗键帽逐颗 drawImage / 渐变 / 文字）+ 调色 veil + 猫 + 粒子，
内容静止时也照画不误。软件渲染下这就是一台永不停的 CPU。

**修法（`demo.html`）**
1. 新增离屏静态层 `baseCv`：`drawBoard()+drawKeys()` 只在「内容真的变了」时重画进离屏画布，
   每帧只 `drawImage(baseCv,...)` 一次；动态层（猫 / 粒子 / 气泡 / flatten veil）照旧每帧画。
   - 变更判据 `baseSignature()`：theme|catSize|hidden|每颗键的 press + ceil(flash*4) + ceil(bounce*4)。
   - `rebuildBase()` 通过临时替换 `ctx` 实现：`const real=ctx; ctx=baseCtx; ...; ctx=real`，
     因此 `drawBoard/drawKeys` 一行没改（它们直接引用模块级 `ctx`）。
2. 帧率自适应：`dynSignature()`（猫的 pid/state/x/y/sq/punch/alpha、粒子、shake）变化 → `activeUntil=t+700`。
   有动静 `gap=33ms`（30fps），静置 `gap=110ms`（≈9fps）。`update(dt)` 仍每帧跑，物理不受影响。
3. **踩坑：纹理迟到**。首帧重建静态层时 `asf/cap_*.png / sk_*.png / sk_board.png` 可能还没 decode 完，
   缓存就会永远停在「无键帽」状态（headless 截图已经拍到了：只剩底板上一排字母）。
   修法：给 CAPTEX 30 张、SKIMG 60 张、SK_BOARD 都挂 `onload=()=>baseDirty=true`，
   并加 `document.fonts.ready.then(()=>baseDirty=true)`。

**根因 2 —— Rust 消息循环可能忙转**（`hook.rs`、`tray.rs`）
原写法 `while GetMessageW(&mut msg, None, 0, 0).as_bool()`：GetMessageW 出错返回 −1，
而 `BOOL(-1).as_bool()` 仍是 true ⇒ 一旦出错直接吃满一个核。
改成 `let r = GetMessageW(...); if r.0 == -1 || r.0 == 0 { break; }`。

**验证**
- `node _t_run_8872.js` → total 89 PASS 89 FAIL 0。
- headless Chrome 截图（wood 主题）→ 键帽齐全；CDP 注入 `store.theme='sakura'` + KeyG keydown → 樱夜主题正常。
- asInvoker 冒烟构建（临时改 manifest + 注释 single-instance）跑真机：进程存活、托盘 NIM_ADD ok=true、
  `WEB: HUG 640x226`、截图里键帽/猫/樱夜皮肤都正常；空闲 CPU 0ms/5s。
- 冒烟后已还原 manifest `requireAdministrator` 与 single-instance 插件。

**部署**：release（17:01，13,432,320 字节）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`；
旧的 16:32 构建留作 `keyboard-cat-prev.exe` 备份。**必须先托盘退出旧实例再双击**（互斥体）。


### 十九·补：用户否掉了降帧（17:3x），改为「恒定 60fps + 只保留静态层缓存」
用户原话：「你不应该改我的fps，这样整体都显得很卡很卡」。同时反馈朋友机器即便在降帧版下仍有 20-40% CPU。

**改动**：`demo.html` 里删掉 `activeUntil / dynSig / lastDrawT / dynSignature()` 与 gap 节流，
`loop()` 回到每帧 `update(dt); draw(); requestAnimationFrame(loop)`。静态层缓存（baseCv + baseSignature + baseDirty）全部保留。

**客观测量**（Chrome headless + CDP `Performance.getMetrics`，强制软件光栅，5 秒主线程耗时）：
- rAF 回调 = **61 次/秒**（确认满帧）
- 缓存路径（现状）= **395 ms / 5s**
- 用 `setInterval(()=>baseDirty=true,16)` 模拟旧版逐帧全量重画 = **3186 ms / 5s**
⇒ 同为 60fps，画布层 CPU 降到约 **1/8**。结论：省 CPU 靠缓存，不靠降帧。


## 二十、2026-11-08 18:20　OBS 捕获改为「本地 HTTP 浏览器源」

用户 m03148「我直播姬捕捉不到」、m03397 OBS 窗口采集也是黑块。查 OBS 日志
（`C:\Users\Administrator\AppData\Roaming\obs-studio\logs\2026-11-08 17-41-33.txt`、`17-49-50.txt`）：
提权前后都是 `CreateForWindow (0x80070057)` 后回落 BitBlt ⇒ **与权限无关**。
挂件窗口 `ex=0xC0138`（WS_EX_LAYERED 0x80000 + APPWINDOW + TOPMOST + TRANSPARENT）
⇒ WGC 拒绝为分层窗口建会话，BitBlt 也读不到分层窗口像素。两条老路被同一原因堵死。
（另：OBS 32 已经没有「允许透明」选项，那是 27 时代的。）
用户 m03434 拍板：「嗯 http源吧」。

**实现**（新文件 `desktop/src-tauri/src/obs.rs`，约 330 行，零依赖手写 HTTP/1.1）
- `start(app)` 绑 `127.0.0.1:17861`（占用则试到 17866），每连接一线程。
- 路由：`/obs` → 内嵌 `obs.html`；`/obs.js`、`/ui/*`、`/asf/*` → `app.asset_resolver().get(path)`
  （直接读编进 exe 的 frontendDist，不用另发文件夹）；`/events` → SSE；`/api/state` → 当前面板状态；
  `/api/key?code=KeyD&down=1` → 自测用，绕过系统钩子直接推一条按键。
  带扩展名的才当静态资源，否则 404（asset_resolver 会把未知路径回落成 index.html）。
- 事件源：`hub`（`Mutex<Hub{next,items:VecDeque,state}>` + `Condvar`），
  `hook.rs` 的发射线程在 `app.emit("kc-key")` 前调 `crate::obs::push_key(...)`，
  SSE 每 15s 无事件发 `: ping`。
- 页面：`desktop/_sync_web.js` 新增 `obs.js`/`obs.html` 生成（复用 demo.html，注入 obs.js 不注入 shell.js）。
  obs.js 隐藏 #panel/#bar/.plane/.taskbar、body 透明、canvas 按 alpha bbox 归位，
  并把宽高写进 `document.title`（"键盘猫 OBS 源 556 x 196"）。
  SSE 收到 `{t:'k'}` → `dispatchEvent(new KeyboardEvent(...))`；`{t:'s'}` → 换主题 / 改猫尺寸。
- 面板改动同步：`shell.js` 的 `pushObs()` 在 load 后 600ms 与每次 `kc-store` 时
  `invoke('obs_sync',{json})`（新增命令），所以面板里换主题，OBS 画面立刻跟着变。
- 托盘菜单新增「OBS 浏览器源地址」（`cmd /c start http://127.0.0.1:17861/obs`）。

**实测**
- curl：`/obs` 200 text/html 94757；`/obs.js` 200 3234；`/ui/tab_off.png` 200 image/png；`/asf/sk_board.png` 200 282069；
  `/api/state` = `{"t":"s","theme":"sakura","catSize":48}`（面板状态确实同步过来了）。
- SSE：`curl -sN /events` 收到 `retry: 1500` + state，随后真实按键 `{"t":"k","code":"KeyA"/"KeyS"/"KeyD"}`。
- headless Chrome + CDP（`desktop/_obs_cdp*.mjs`，端口 9336）：
  `INFO {"size":{"w":556,"h":196},"title":"键盘猫 OBS 源 556 x 196","theme":"sakura","cat":48,"panel":"none","bar":"none","ss":true}`；
  截图 `obs_clean.png / obs_page.png / obs_live.png` 显示整块樱夜键盘 + 猫、透明底、无面板。
  用 `/api/key` 驱动：页面计数 `{"c":2,"codes":["KeyD:down","KeyG:down"],"held":["KeyD","KeyG"]}` ⇒ SSE→按键派发链路通。

**两个注入坑（重要）**
1. headless Chrome 一起，`_press_hold.ps1` 注入的按键一条都到不了钩子；杀掉 Chrome 恢复。
2. 未提权的测试 exe + 前台是提权窗口时，UIPI 会把 SendInput 全丢掉，表现是「HOOK: installed 但零按键」。
   所以留了 `/api/key` 自测入口；正式版（requireAdministrator）不存在这个问题。

**构建与部署**
- `cd desktop && node _sync_web.js` 会同时生成 `web/index.html`、`web/obs.html`、`web/obs.js`。
- release：`CARGO_TARGET_DIR=<绝对路径>/desktop/target_r cmd.exe //c _build.bat --release`（约 2 分钟，后台跑）。
- 产物 13,501,440 字节（18:15）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`；
  上一版（17:37，60fps 缓存版）留成 `keyboard-cat-prev.exe`。
- `使用说明.txt` 的 OBS 段已整段重写为「浏览器源」流程（URL、556×196、为什么窗口采集不行、直播姬的替代做法）。
- 自测：`node _t_run_8872.js` → total 89 PASS 89 FAIL 0。

**给用户的用法**
OBS → 来源 → 「浏览器」→ URL `http://127.0.0.1:17861/obs`，宽 556 高 196，勾「关闭源时关闭页面」，
可等比放大；背景天然透明，也不额外吃 CPU。

## 二十·补、2026-11-08 18:35　OBS 源改成「像素级镜像」

用户反馈：「obs页面的状态跟我实际不一样……我捕捉以后只有一只猫，位置也跟我桌面上的不一样」。
原因：第一版 OBS 页面是**自己跑一份 demo.html**，只同步了 theme 和 catSize，
猫的编队、站位、灯效、浮空/粒子全靠页面自己模拟，所以猫的数量和位置都对不上。

改法：不做模拟，直接推画面。
- 挂件侧（`desktop/_sync_web.js` 的 SHELL 模板，挂件分支）：新增 `obsTick()`，每 70ms 比一次
  `obsSig()`（`baseSignature()` + `cats` 的 pid/state/x/y/sq/punch/alpha + 粒子数 + shake）；
  变了就 `cv.toDataURL('image/webp',0.92)`（失败退 PNG），
  带 `lastBBox`（hugCanvas 扫出来的 alpha 包围盒）调 `invoke('obs_frame',{...})`。
  静止时只每 1.5s 补一帧心跳（保证 OBS 页面刚打开就有画面）。
  **没有浏览器源连着就完全不编码**：每秒 `invoke('obs_clients')` 取当前 SSE 连接数，为 0 直接 return。
- Rust（`obs.rs`）：`static CLIENTS: AtomicUsize`，`sse()` 进函数 +1、用 `struct Decr` 的 Drop 在断开时 −1；
  `push_frame(data,bx,by,bw,bh)` 拼 `{"t":"f","bx":..,"by":..,"bw":..,"bh":..,"d":"data:image/webp;base64,..."}` 广播；
  新增命令 `obs_frame`、`obs_clients`。
- OBS 页面（`web/obs.html` 现在是独立空壳，不再复制 demo.html）：一个 `<img id="k">` + `obs.js`；
  `obs.js` 只做三件事：收 SSE 的 `t:'f'` → 换 `img.src`；按 `bx/by` 设负偏移裁掉透明留白；
  按 `bw/bh` 设 html/body 尺寸并写进 `document.title`。
  于是「桌面上几只猫、站哪、什么主题、灯效、浮空、粒子」在 OBS 里是同一帧画面，不再有模拟差异。
- 尺寸变了：源尺寸从 556×196（CSS）变成 **640×227（设备像素，1:1 不缩放，最清晰）**。
  `使用说明.txt` 已同步改。

**实测**（asInvoker 测试构建，端口 17862 因为用户自己的实例占着 17861）
- headless Chrome 打开 `/obs`：`document.title = "键盘猫 OBS 源 640 x 227"`，`window.__KCOBS={w:640,h:227}`，
  `img.naturalWidth=644`（整张画布）、`src` 长度约 59KB（webp）。
- 截图 `desktop/obs_mirror.png`：画面与桌面挂件完全一致 —— 三只猫（三花在 D/E 之间、两只橘猫在 H/I 与 L 附近）、
  樱夜主题，位置和数量都对上了（第一版页面只显示一只猫）。
- 空闲 CPU（页面连着、猫不动）：`cpu_delta_5s=0.016s`（约 0.3%），因为签名没变就只走 1.5s 心跳。
- 自测 `node _t_run_8872.js` → 89 PASS 0 FAIL。
- 部署：release 13,476,352 字节（18:33）→ `keyboard-cat-new.exe`；18:15 那版留成 `keyboard-cat-prev.exe`。

**注意**：用户当时正在跑的实例（18:15 版）占着 17861，且 OBS 里已经连着两条 SSE；
必须先从托盘退出旧实例再启动新版，OBS 的浏览器源会自动重连（EventSource 自带重试）。

## 二十一、2026-11-08 19:00　OBS 源换成「状态同步 + 页面自己渲染」（60fps）

用户 m03802：「因为现在每一帧都推，我要做到60fps，占用就会很大，现在网页端看起来掉帧严重，按你刚才说的只有9帧。
这不应该。2 弹跳和压边，网页自己模拟可以吗？」→ 答可以，用户 m03805「没问题。你开始改吧」。

推帧为什么到不了 60fps：每帧要编码 webp，实测 15–17ms/帧（软件渲染），60fps 需要 900ms/s，
物理上不可能；之前压在 70ms（14fps）就是这个原因。

改法：不推像素，推「猫在哪 / 什么状态 / 哪些键按着」，页面自己按 60fps 渲染。
- 挂件侧（`desktop/_sync_web.js` 的 SHELL 模板）：删掉 obsSig/obsTick/toDataURL 整套；
  新增 `obsSnap()` 生成
  `{"t":"st","th":主题,"cs":猫尺寸,"h":隐藏,"sh":抖动,"c":[[pid,星,x,y,状态码,sq,landT,alpha,punch,face,键码],...],"k":[[键码,press,flash,bounce],...]}`
  （状态码 0 guard / 1 walk / 2 fly / 3 dizzy），每 33ms 比一次字符串，变了才 `invoke('obs_state',{json})`；
  画面不变时每 1s 补一次心跳；没有浏览器源连着（`obs_clients` 为 0）时一条都不发。
- Rust：`obs.rs` 的 `push_frame` 换成 `push_state(json)`；`main.rs` 命令 `obs_frame` → `obs_state`。
  另加调试路由 `/api/push?j=<urlencoded JSON>`（直接往 SSE 里塞任意事件，排查页面用），
  配 `url_decode()`（obs.rs）。
- OBS 页面（`web/obs.html` 改回「demo.html + obs.js」，不再复制 shell.js）：`obs.js` 做四件事：
  1) 注入 CSS：透明背景、隐藏 #panel/#bar/#plane/#taskbar/#msg、#widget 固定 0,0、去 filter；
  2) `hug()` 扫 alpha 包围盒，把画布平移到 0,0 并把 html/body 尺寸设成包围盒（源尺寸即内容尺寸）；
  3) `applySnap(m)`：主题变了置 `baseDirty`、尺寸变了调 `setCatSize`、编队变了用 `makeCat` 重建 cats、
     逐只写 tx/ty/state/sq/landT/alpha/punch/face/key，键表写 press/flash/bounce（flash 跳变时本地补一簇粒子）；
  4) **改写 `window.update(dt)`**：不跑物理，只做 flash/bounce 衰减、粒子/气泡推进、猫的位置向 tx/ty 插值
     （fly 用 dt*30，其余 dt*18），sq/landT/punch 本地衰减 —— 弹跳、压扁、粒子都在页面本地播放，所以不受帧率限制。
  `window.__KCOBSSNAP=applySnap` 暴露出来方便调试。
- 调试期间留下的接口：`/api/push`、`/api/key`、`/api/state`、`window.__KCOBSSNAP`，都是本机自测用，无害。

**实测**（asInvoker 测试构建，端口 17862，因为用户实例占 17861）
- 页面尺寸 `window.__KCOBS={w:556,h:196}`、`document.title="键盘猫 OBS 源 556 x 196"`、body 556×196；
  面板 display:none、鱼干条 display:none。
- rAF 实测 **60fps**（2 秒计 120 帧）；页面主线程 5 秒 327ms ≈ **6.5% 单核**（软件渲染，最坏情况）。
- 快照应用（直接调 `__KCOBSSNAP`）：cats 从 3 只重建为 2 只、状态 guard/dizzy、st 星数、
  tx 200/320、KeyG press+flash=1、KeyH bounce=1 全部生效；0.4s 后猫插值到位（x=200/320）。
- 截图 `desktop/obs_state2.png`：2 只猫分别站在 G / H，樱夜主题，与推来的坐标一致。
- 真实链路：页面自己连上 `/events` 后拿到的是挂件实时状态（3 只猫，pid orange/calico/old），
  说明 SSE + 33ms 推送在跑。
- 自测 `node _t_run_8872.js` → 89 PASS 0 FAIL。

**尺寸变化**：源尺寸从上一版的 640×227（设备像素）改回 **556×196（CSS 像素）**，
因为 OBS 浏览器源的 devicePixelRatio 是 1，用 CSS 尺寸才是 1:1 不缩放。
`使用说明.txt` 的 OBS 段已重写（556×196 + 必须把「自定义帧率」勾上填 60）。

**部署**：release 13,507,584 字节（18:59）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`；
18:33 的镜像版留成 `keyboard-cat-prev.exe`，11-06 的 `keyboard-cat.exe` 还在。
用户当时在跑的实例（PID 18532，18:33 版）没有退出，**必须托盘退出再启动新版**，否则还是旧的镜像版。

**构建/清理约定**：临时测试要 app.manifest 改 asInvoker + 注释 single-instance 插件，
release 前必须还原（本轮已还原并复查过）；测试 exe 放 C: 盘（H: 盘 Shell_NotifyIcon 报 err=5）。
坑：`tools.edit` 改完 `_sync_web.js` 之后要重跑 `node _sync_web.js`，否则 web/ 里还是旧产物。



## 二十二、2026-11-08 20:35 在场 8 只 + 窗口留出跳跃空间

用户（m03945）："在场猫咪数开放到8只。红框位置需要拓展一些窗口高度，猫咪跳跃时会越过实际窗口边界"。

### 在场 8 只
- demo.html 新增 `const MAXC=8` 与初始站位表 `const CATSTART=['KeyJ','KeyF','KeyH','KeyG','KeyK','KeyD','KeyL','KeyS']`；
  `makeCat(pid,slot)` 原来只有 J/F/H 三个起始键，第 3 只以后全挤在 KeyH 上，现在按表错开。
- `equipOne` 的上限、`panelDex` 的编队格子数、页面提示文案全部改用 MAXC。
- `.slot` 加 `flex-wrap:wrap`：8 个格子 161px 宽会换行（实测 5+3，slot 内容宽 986 < 面板 1030，无裁切）。
- 自测从 89 条增加到 92 条：编队上限/空位数/8 只场上猫数/8 只站位不重复。

### 窗口留出跳跃空间
- 根因：挂件窗口是按画布 alpha 包围盒裁的，静止时只包住键盘；猫被打飞的抛物线顶点能到画布最上沿
  （`startFly` 里 `h = min(60+dist*.18+big*30, min(sy,ey)-CATH-4)` 就是按"顶到画布顶部"封顶的），
  飞到高处就被窗口边缘切掉。
- `desktop/_sync_web.js` 的 `hugCanvas()`：包围盒纵向不再裁剪，改成 `bbox={x:bbox.x,y:0,w:bbox.w,h:bbox.y+bbox.h}`，
  窗口高度 = 整条画布高度（CSS 560×262，以前是 560×200）。
- `fit_window` 新增 anchor `keepbr`（右/下边不动、只往上长）；挂件 hug 用 keepbr，避免窗口变高时把键盘往下推（以前 keep 是保中心）。
- OBS 页面 `obs.js` 的 `hug()` 同步改成不裁纵向、canvas top=0，OBS 源尺寸从 556×196 变 **560×262**。

### 验证
- 自测：`node _t_run_8872.js` → total 92 PASS 92 FAIL 0。
- 几何（headless + 本地 http，DPR1）：8 只在场、canvas 560×262、旧窗口 560×200、新窗口 560×262。
- 真机冒烟（asInvoker 临时构建，C: 盘）：`WEB: HUG 640x298 off=2,0`，窗口 rect 2800,1086 → 3440,1384
  （right/bottom 与旧版 2800,1157→3440,1384 完全一致，说明键盘没被推走，只往上长了 71px）；
  截图可见键盘上方那条透明的跳跃空间与桌面背景混在一起，落地猫站在最上排键上也完整。
- 冒烟后已还原 app.manifest（requireAdministrator）与 single-instance 插件。

### 发布
- release 13,508,608 字节（20:33）→ `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe`；
  18:59 那版挪成 `keyboard-cat-prev.exe`。
- `使用说明.txt`：OBS 源尺寸改 560×262 并注明"上面多的一截是跳跃空间，别裁"；编队改成最多 8 只。
- 清理：删掉 `src-tauri/target`(4.7G)、`desktop/target_t`(2.2G)、冒烟目录 kcsmoke2 与临时脚本。


## 二十三、2026-11-08 21:05 猫池门槛 300 / 删保底 / 上阵栏 2×4

### 用户需求（原话）
「猫池改为每天一次，计费攒够300次给一次，每天最多2次。保底机制现在好像没作用，菜单中的删除。上阵猫咪栏改为2行4列，缩小一下现在的卡片尺寸」

### 改动（`demo.html`）
- `const CATTHRESH=300;`（原 1500）：当天鱼干满 300 解锁当天第 2 次抽；结构仍是「免费 1 次 / 每天最多 2 次」。
- 保底整套删除：`pityNow()`/`setPity()`、`rollCat` 里的计数与「≥30 抽只出新猫」、`doGachaCat` 的 `setPity(0)`、`defaults()` 的 `pity:0`、面板那行「距保底还有 N 抽」。
  原因：抽到新猫就清零，11 个花色前 11 抽永远到不了 30，机制是死码。
- 上阵栏 `.slot`：`display:grid;grid-template-columns:repeat(4,161px);justify-content:start;gap:10px`；卡片 `.sl` 161×128（原 161×150），图 72×56（原 96×74），字号 12/10.5，加号 26px。
  试过 `minmax(0,1fr)`：卡片被拉成 203×132、slot 圆角图横向拉伸 1.44 倍，故改回固定 161。
- 自测：`node _t_run_8872.js` → `total 92 PASS 92 FAIL 0`。

### 验证
- `desktop/_dex42.cjs`（headless Chrome + 本地 http :9712 + CDP :9415，注入 `zoom:1.5` 与 `width:897` 模拟真实 `#panelBody`）：
  `cells:8, rows:2, perRow:4, cardLayout 161×128, slotLayoutW 841, bodyLayoutW 897, overflowX:false`；截图 `desktop/dex_4x2.png` 目视 8 格 4×2 对齐、收藏卡未被挤出。
- release 构建 1m20s → `desktop/target_r/release/keyboard-cat.exe` 13,511,680 字节（21:02）。

### 部署状态（未完成）
- `cp` 到 `C:\Users\Administrator\AppData\Local\KeyboardCat\keyboard-cat-new.exe` 报 **Device or resource busy** —— 用户实例仍在运行占用文件。
- 需要用户：托盘右键 → 退出，然后重新双击 `keyboard-cat-new.exe`（会弹 UAC）。或者由我做：杀进程 → 备份当前为 `keyboard-cat-prev.exe` → 覆盖 → 重启。
- `使用说明.txt` 已加两条：抽猫规则（每天 1 次 / 鱼干 300 再 1 次 / 无保底）、上阵最多 8 只且是 2 行 4 列、窗口上方留了跳跃空间。

## 二十四、2026-11-08 21:25 可执行文件集中到 keyboard-cat/dicts/

用户原话：「你把这些exe都塞到项目目录下一个dicts文件夹里，没有就新建，我好难找啊」

`keyboard-cat/dicts/`（新建，约 53MB）：
- `keyboard-cat-latest.exe` 13,511,680（21:02 release：猫池 300 / 删保底 / 上阵 4×2）
- `keyboard-cat-bak-2026-11-08-2036.exe` 13,508,608（上一正式版）
- `keyboard-cat-bak-2026-11-08-1859.exe` 13,507,584（OBS 状态同步首版）
- `keyboard-cat-bak-2026-11-06.exe` 13,362,688（初版桌面挂件）
- `使用说明.txt`

**坑**：这四个 exe 是复制过来的，运行中的 `keyboard-cat-new.exe` 仍锁着 `C:\Users\Administrator\AppData\Local\KeyboardCat\` 里的原文件，而任务栏/托盘图标锚定在原路径上。所以：任务栏固定的那个图标继续指向 AppData 目录；`dicts/` 里的 exe **不要直接双击覆盖**——等旧实例退出后用它替换 AppData 下的 `keyboard-cat-new.exe` 即可。
