//! 自写 Windows 托盘图标：tray-icon crate 在本机上 NIM_ADD 失败且被静默吞掉
//! （tray-icon-0.25.1 src/platform_impl/windows/mod.rs:159），这里直接调 Shell_NotifyIconW，
//! 失败会写进 kc_boot.log，便于以后定位。

use std::sync::OnceLock;

use tauri::AppHandle;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconGetRect, Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD,
    NIM_DELETE, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER, NOTIFY_ICON_DATA_FLAGS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreateIconFromResourceEx, CreatePopupMenu, CreateWindowExW, DefWindowProcW,
    DestroyMenu, DispatchMessageW, GetCursorPos, GetMessageW, PostMessageW, PostQuitMessage,
    RegisterClassW, SetForegroundWindow, TrackPopupMenu, TranslateMessage, MF_SEPARATOR,
    MF_STRING, MSG, TPM_RETURNCMD, TPM_RIGHTBUTTON, WM_APP, WM_CONTEXTMENU, WM_DESTROY,
    WM_LBUTTONUP, WM_NULL, WM_RBUTTONUP, WNDCLASSW, WS_EX_TOOLWINDOW, WS_OVERLAPPED,
    LR_DEFAULTCOLOR,
};

const WM_TRAY: u32 = WM_APP + 1;

const ID_MENU: u32 = 1;
const ID_TOGGLE: u32 = 2;
const ID_CLICK: u32 = 3;
const ID_OBS: u32 = 5;
const ID_QUIT: u32 = 4;

static APP: OnceLock<AppHandle> = OnceLock::new();

/// 诊断：在主线程（Tauri 消息循环所在线程）上建 STATIC 窗口并登记图标
pub fn probe_main_thread() {
    unsafe {
        let hinst = match GetModuleHandleW(None) {
            Ok(h) => h,
            Err(e) => {
                log(&format!("probe GetModuleHandle failed: {e}"));
                return;
            }
        };
        match CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            windows::core::w!("STATIC"),
            windows::core::w!("kc-main-probe"),
            windows::Win32::UI::WindowsAndMessaging::WS_POPUP,
            0,
            0,
            10,
            10,
            None,
            None,
            Some(&HINSTANCE(hinst.0)),
            None,
        ) {
            Ok(h) => {
                log("probe main-thread window created");
                try_add(h, make_icon(), true, "mainthread");
            }
            Err(e) => log(&format!("probe main-thread window failed: {e}")),
        }
    }
}

pub fn start(app: AppHandle) {
    let _ = APP.set(app);
    std::thread::spawn(move || unsafe { run() });
}

fn log(msg: &str) {
    crate::dbg_pub(msg);
}

const ICO: &[u8] = include_bytes!("../icons/icon.ico");

/// 从 ICO 二进制里挑最大的一张，交给 CreateIconFromResourceEx 生成 HICON。
unsafe fn make_icon() -> Option<windows::Win32::UI::WindowsAndMessaging::HICON> {
    if ICO.len() < 6 {
        return None;
    }
    let count = u16::from_le_bytes([ICO[4], ICO[5]]) as usize;
    let mut best: Option<(u32, usize, usize)> = None;
    for i in 0..count {
        let p = 6 + i * 16;
        if p + 16 > ICO.len() {
            break;
        }
        let w = ICO[p] as u32;
        let bytes = u32::from_le_bytes([ICO[p + 8], ICO[p + 9], ICO[p + 10], ICO[p + 11]]) as usize;
        let off = u32::from_le_bytes([ICO[p + 12], ICO[p + 13], ICO[p + 14], ICO[p + 15]]) as usize;
        if best.map_or(true, |(bw, _, _)| w >= bw) {
            best = Some((w, off, bytes));
        }
    }
    let (_, off, len) = best?;
    if off + len > ICO.len() || len == 0 {
        return None;
    }
    match CreateIconFromResourceEx(&ICO[off..off + len], true, 0x0003_0000, 0, 0, LR_DEFAULTCOLOR) {
        Ok(h) if !h.is_invalid() => Some(h),
        _ => None,
    }
}

unsafe fn try_add(hwnd: HWND, icon: Option<windows::Win32::UI::WindowsAndMessaging::HICON>, with_icon: bool, tag: &str) -> bool {
    let mut nid = NOTIFYICONDATAW::default();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_TIP | if with_icon { NIF_ICON } else { NOTIFY_ICON_DATA_FLAGS(0) };
    nid.uCallbackMessage = WM_TRAY;
    if let Some(h) = icon {
        nid.hIcon = h;
    }
    let tip: Vec<u16> = "键盘猫\0".encode_utf16().collect();
    let n = tip.len().min(nid.szTip.len());
    nid.szTip[..n].copy_from_slice(&tip[..n]);
    windows::Win32::Foundation::SetLastError(windows::Win32::Foundation::WIN32_ERROR(0));
    let ok = Shell_NotifyIconW(NIM_ADD, &nid).as_bool();
    let err = std::io::Error::last_os_error().raw_os_error();
    log(&format!("tray NIM_ADD[{tag}] ok={ok} err={err:?} sizeof={}", nid.cbSize));
    if ok {
        let mut ident = NOTIFYICONIDENTIFIER::default();
        ident.cbSize = std::mem::size_of::<NOTIFYICONIDENTIFIER>() as u32;
        ident.hWnd = hwnd;
        ident.uID = 1;
        match Shell_NotifyIconGetRect(&ident) {
            Ok(r) => log(&format!(
                "tray [{tag}] rect={},{},{},{}",
                r.left, r.top, r.right, r.bottom
            )),
            Err(e) => log(&format!("tray [{tag}] GetRect failed: {e:?}")),
        }
    }
    ok
}

unsafe fn add_icon(hwnd: HWND) -> bool {
    let custom = make_icon();
    if try_add(hwnd, custom, true, "custom") {
        return true;
    }
    let sys = windows::Win32::UI::WindowsAndMessaging::LoadIconW(
        None,
        windows::Win32::UI::WindowsAndMessaging::IDI_APPLICATION,
    )
    .ok();
    if try_add(hwnd, sys, true, "system") {
        return true;
    }
    try_add(hwnd, None, false, "noicon")
}

unsafe fn show_menu(hwnd: HWND) {
    let Some(app) = APP.get() else { return };
    let Ok(menu) = CreatePopupMenu() else { return };
    let _ = AppendMenuW(menu, MF_STRING, ID_MENU as usize, windows::core::w!("菜单"));
    let _ = AppendMenuW(menu, MF_STRING, ID_TOGGLE as usize, windows::core::w!("显示 / 隐藏"));
    let _ = AppendMenuW(menu, MF_STRING, ID_CLICK as usize, windows::core::w!("切换鼠标穿透"));
    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
    let _ = AppendMenuW(menu, MF_STRING, ID_OBS as usize, windows::core::w!("OBS 浏览器源地址"));
    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
    let _ = AppendMenuW(menu, MF_STRING, ID_QUIT as usize, windows::core::w!("退出"));

    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);
    let cmd = TrackPopupMenu(
        menu,
        TPM_RETURNCMD | TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        0,
        hwnd,
        None,
    );
    let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
    let _ = DestroyMenu(menu);

    match cmd.0 as u32 {
        ID_MENU => crate::open_menu(app),
        ID_TOGGLE => crate::toggle_window(app),
        ID_CLICK => crate::toggle_interactive(app),
        // 打开默认浏览器看 OBS 页面（顺便能看到该用多大的源尺寸）
        ID_OBS => {
            let url = crate::obs::url();
            let _ = std::process::Command::new("cmd")
                .args(["/c", "start", "", &url])
                .spawn();
        }
        ID_QUIT => app.exit(0),
        _ => {}
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_TRAY => {
            let mouse = (l.0 & 0xffff) as u32;
            if let Some(app) = APP.get() {
                match mouse {
                    WM_LBUTTONUP => crate::toggle_window(app),
                    WM_RBUTTONUP | WM_CONTEXTMENU => show_menu(hwnd),
                    _ => {}
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, w, l),
    }
}

unsafe fn run() {
    let hinst = match GetModuleHandleW(None) {
        Ok(h) => h,
        Err(e) => {
            log(&format!("tray GetModuleHandle failed: {e}"));
            return;
        }
    };
    let class = windows::core::w!("KC_TrayWindow");
    let wc = WNDCLASSW {
        lpfnWndProc: Some(wndproc),
        hInstance: hinst.into(),
        lpszClassName: class,
        ..Default::default()
    };
    if RegisterClassW(&wc) == 0 {
        log("tray RegisterClass failed");
    }
    let hwnd = match CreateWindowExW(
        WS_EX_TOOLWINDOW,
        class,
        windows::core::w!("kc-tray"),
        WS_OVERLAPPED,
        0,
        0,
        0,
        0,
        None,
        None,
        Some(&HINSTANCE(hinst.0)),
        None,
    ) {
        Ok(h) => h,
        Err(e) => {
            log(&format!("tray CreateWindowEx failed: {e}"));
            return;
        }
    };
    if !add_icon(hwnd) {
        log("tray add_icon failed on all variants");
        // 探针：用系统 STATIC 类窗口 + WS_POPUP（与 PowerShell 成功用例完全一致）再试一次
        if let Ok(h2) = CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            windows::core::w!("STATIC"),
            windows::core::w!("kc-tray-probe"),
            windows::Win32::UI::WindowsAndMessaging::WS_POPUP,
            0,
            0,
            10,
            10,
            None,
            None,
            Some(&HINSTANCE(hinst.0)),
            None,
        ) {
            log("tray probe window created");
            if try_add(h2, make_icon(), true, "static-probe") {
                log("tray probe SUCCESS with STATIC window");
            }
        } else {
            log("tray probe window create failed");
        }
    }
    let mut msg = MSG::default();
    // GetMessageW 出错返回 -1，而 BOOL(-1).as_bool() 仍为 true —— 原写法会变成吃满一个核的忙循环
    loop {
        let r = GetMessageW(&mut msg, None, 0, 0);
        if r.0 == -1 || r.0 == 0 {
            break;
        }
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    let mut nid = NOTIFYICONDATAW::default();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
}