# 键盘猫 桌面外壳（Tauri v2）

## 现在能干什么
- 无边框透明窗口、置顶、不进任务栏，贴在**主屏工作区右下角**（工作区 = 去掉任务栏后的区域）
- **鼠标穿透**：默认整窗穿透，点得到下面的窗口
- **全局键盘钩子**：`WH_KEYBOARD_LL`，只监听不拦截（回调必 `CallNextHookEx`），任何程序里的按键都会喂给页面
- 托盘：显示/隐藏、切换鼠标穿透、退出
- `Ctrl+Alt+K`：切换「穿透 ↔ 可交互」（要开设置面板时按它；面板里按 Esc 恢复穿透）

## 目录
```
desktop/
  _sync_web.js      把 ../demo.html + ui/ + asf/ 同步到 web/，并注入 shell.js
  _mkicon.js        用 asf/sp_orange_idle_44.png 生成 32/128/ico
  web/              同步产物（index.html、shell.js、ui/、asf/）
  src-tauri/
    src/main.rs     窗口/托盘/位置/穿透/快捷键
    src/hook.rs     全局键盘钩子 + scancode→DOM code 映射
    app.manifest    requireAdministrator（见下）
```

## 构建
```bash
node desktop/_sync_web.js          # 改了 demo.html 之后必须重跑
cd desktop/src-tauri && cargo run --release
```

## 两个必须知道的坑
1. **提权**：`app.manifest` 里写了 `requireAdministrator`。低权限进程的钩子收不到管理员程序里的按键（UIPI），要「全局生效」就只能提权。代价是每次启动要 UAC 确认（当前会话已是管理员，开发时不弹）。
2. **独占全屏游戏收不到按键**：真·独占全屏（DirectX exclusive）下低层键盘钩子会被绕过，只有窗口化/无边框全屏稳。这个不是代码能解决的，属于系统行为。

## 还没做
- 打包安装器（NSIS）：`cargo install tauri-cli` 后 `cargo tauri build`
- 开机自启（插件已装未接）
- 多显示器：现在固定主屏右下
