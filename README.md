# 键盘猫 Keyboard Cat

一只趴在键盘上的猫。它会自己站到某个键上，你按下那个键，猫就被击飞出去、落到另一颗键上。

- **桌面挂件**：透明置顶的小窗口贴在屏幕右下角，可以拖动、可以鼠标穿透（Ctrl+Alt+K 切换）、托盘图标管理
- **OBS 直播源**：本地 HTTP 页面 `http://127.0.0.1:17861/obs`，在 OBS 里加「浏览器」源即可，透明背景、60fps

![demo](https://img.shields.io/badge/自测-92%20PASS-success)

## 玩法

- 猫被击飞落定时 +1 鱼干；连击第 3 下起每下 +1；空中接力（落地 0.2 秒内再打中）+2；同一颗键连飞 10 次收益减半
- 鱼干用来抽猫：每天第 1 次免费，当天攒够 300 鱼干解锁第 2 次，每天最多 2 次
- 同屏最多 8 只猫，同花色不能重复上阵；星级只影响外观

## 目录

```
demo.html            单文件 demo（canvas 零依赖，全部玩法/面板/主题都在这里）
demo-v1.html         第一版留档
make_sprite.js       生成猫位图 asf/sp_*.png
recolor.js           键帽三切贴图重上色
asf/                 猫位图、键帽贴图（cap_{theme}_{l,m,r}.png）、樱夜主题逐键贴图（sk_*.png）
ui/                  面板切图与标签页贴图
desktop/             Tauri v2 桌面外壳
  src-tauri/src/main.rs    窗口 / 鼠标穿透 / 贴合 / 托盘命令
  src-tauri/src/hook.rs    全局键盘钩子（只监听不拦截）
  src-tauri/src/tray.rs    自写 Win32 托盘（Shell_NotifyIconW + TrackPopupMenu）
  src-tauri/src/obs.rs     本地 HTTP + SSE，给 OBS 浏览器源
  _sync_web.js             把 demo.html + shell.js 注入生成 web/，并生成 obs.html
HANDOFF.md           开发交接文档（每一轮改了什么、踩过什么坑）
```

## 在浏览器里玩

直接双击 `demo.html`。或者起个静态服务器（`getImageData` 相关脚本需要 http）：

```bash
npx serve .
```

调试入口：`demo.html#test` 跑自测，`#geo` 布局探针，`#theme=wood` 指定主题。

## 构建桌面版

需要 Node、Rust、Visual Studio Build Tools（C++ 桌面开发）、WebView2 运行时。

```bash
cd desktop
node _sync_web.js
cd src-tauri
cargo build --release
```

产物在 `src-tauri/target/release/keyboard-cat.exe`。

注意：

- 改了 `demo.html` 之后必须重跑 `node _sync_web.js`，否则打进 exe 的仍是旧的 web 资源
- `app.manifest` 要求管理员权限（全局键盘钩子需要），运行时会弹 UAC
- **exe 要放在 C: 盘运行**。放在其它盘时托盘图标注册会失败（`Shell_NotifyIcon` 返回 ERROR_ACCESS_DENIED）

## OBS 用法

1. OBS 里「来源 → + → 浏览器」
2. URL 填 `http://127.0.0.1:17861/obs`（端口被占用时会自动顺延到 17862…，托盘菜单「OBS 浏览器源地址」里能直接看到实际地址）
3. 宽 560、高 262，勾选「自定义帧率」填 60
4. 背景自动透明（浏览器源默认带 alpha）

为什么不能用「窗口采集」：键盘猫的挂件是分层窗口（WS_EX_LAYERED），没有 DWM 重定向表面，Windows Graphics Capture 会直接拒绝（`CreateForWindow 0x80070057`），BitBlt 也读不到像素。所以走浏览器源。

## 主题

内置 8 套主题。第 8 套「樱夜」是用参考图逐颗扣键帽拼出来的。

## 许可

仅个人使用，字体与部分贴图素材来自第三方，请勿商用。
