#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hook;
mod obs;
mod tray;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, PhysicalPosition};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// true = 鼠标穿透（默认），false = 可交互
static PASSTHROUGH: AtomicBool = AtomicBool::new(true);
/// 面板窗口是否已经摆过位置（只在第一次打开时居中）
static PANEL_PLACED: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
fn work_area() -> (i32, i32, i32, i32) {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };
    unsafe {
        let mut r = RECT::default();
        let ok = SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut r as *mut _ as *mut core::ffi::c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        if ok.is_ok() {
            (r.left, r.top, r.right, r.bottom)
        } else {
            (0, 0, 1920, 1080)
        }
    }
}

#[cfg(not(windows))]
fn work_area() -> (i32, i32, i32, i32) {
    (0, 0, 1920, 1080)
}

fn place_above_taskbar(win: &tauri::WebviewWindow) {
    let size = win.outer_size().unwrap_or(tauri::PhysicalSize::new(760, 460));
    let (left, _top, right, bottom) = work_area();
    let margin = 16;
    let x = right - size.width as i32 - margin;
    let y = bottom - size.height as i32;
    let _ = win.set_position(PhysicalPosition::new(x.max(left), y.max(0)));
}

pub(crate) fn set_passthrough_state(app: &tauri::AppHandle, on: bool) {
    PASSTHROUGH.store(on, Ordering::Relaxed);
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_ignore_cursor_events(on);
        let _ = win.set_always_on_top(true);
        if !on {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
    let _ = tauri::Emitter::emit(app, "kc-interactive", serde_json::json!({ "on": !on }));
}

pub(crate) fn toggle_interactive(app: &tauri::AppHandle) {
    let next = !PASSTHROUGH.load(Ordering::Relaxed);
    set_passthrough_state(app, next);
}

pub(crate) fn toggle_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        match w.is_visible() {
            Ok(true) => {
                let _ = w.hide();
            }
            _ => {
                let _ = w.show();
                let _ = w.set_always_on_top(true);
            }
        }
    }
}

/// 面板是独立窗口：和键盘挂件互不影响，各拖各的。
pub(crate) fn open_panel(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("panel") {
        if !PANEL_PLACED.swap(true, Ordering::Relaxed) {
            let size = w.outer_size().unwrap_or(tauri::PhysicalSize::new(760, 530));
            let (left, top, right, bottom) = work_area();
            let x = left + ((right - left) - size.width as i32) / 2;
            let y = top + ((bottom - top) - size.height as i32) / 2;
            let _ = w.set_position(PhysicalPosition::new(x.max(0), y.max(0)));
        }
        let _ = w.set_ignore_cursor_events(false);
        let _ = w.show();
        let _ = w.set_always_on_top(true);
        let _ = w.set_focus();
    }
    let _ = tauri::Emitter::emit(app, "kc-open-panel", serde_json::json!({}));
}

pub(crate) fn close_panel(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("panel") {
        let _ = w.hide();
    }
}

/// 托盘「菜单」：叫出面板窗口
pub(crate) fn open_menu(app: &tauri::AppHandle) {
    open_panel(app);
}

#[tauri::command]
fn set_passthrough(app: tauri::AppHandle, on: bool) {
    set_passthrough_state(&app, on);
}

/// 把窗口缩到内容的真实大小，消掉透明窗口里那圈多余的空白
/// （拖动时系统画的是窗口矩形，窗口比内容大就会看到一圈白框）
/// w/h 传物理像素（前端已乘 devicePixelRatio）
#[tauri::command]
fn fit_window(app: tauri::AppHandle, label: String, w: f64, h: f64, anchor: String) {
    if let Some(win) = app.get_webview_window(&label) {
        let old_pos = win.outer_position().ok();
        let old_size = win.outer_size().ok();
        let _ = win.set_size(tauri::PhysicalSize::new(
            w.max(32.0) as u32,
            h.max(32.0) as u32,
        ));
        let new_size = win
            .outer_size()
            .unwrap_or(tauri::PhysicalSize::new(w as u32, h as u32));
        let pos = if anchor == "br" {
            let (left, _top, right, bottom) = work_area();
            PhysicalPosition::new(
                (right - new_size.width as i32).max(left),
                bottom - new_size.height as i32,
            )
        } else if let (Some(p), Some(s)) = (old_pos, old_size) {
            if anchor == "keepbr" {
                // 挂件用：右／下边不动，窗口只往上长（留给猫跳跃的透明空间）
                PhysicalPosition::new(
                    p.x + (s.width as i32 - new_size.width as i32),
                    p.y + (s.height as i32 - new_size.height as i32),
                )
            } else {
                // 保持中心不动
                PhysicalPosition::new(
                    p.x + (s.width as i32 - new_size.width as i32) / 2,
                    p.y + (s.height as i32 - new_size.height as i32) / 2,
                )
            }
        } else {
            PhysicalPosition::new(0, 0)
        };
        let _ = win.set_position(pos);
    }
}

#[tauri::command]
fn open_panel_cmd(app: tauri::AppHandle) {
    open_panel(&app);
}

#[tauri::command]
fn close_panel_cmd(app: tauri::AppHandle) {
    close_panel(&app);
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

/// 面板里的主题 / 猫尺寸变化 -> 推给 OBS 浏览器源
#[tauri::command]
fn obs_sync(json: String) {
    obs::set_state(json);
}

#[tauri::command]
fn obs_state(json: String) {
    obs::push_state(json);
}

#[tauri::command]
fn obs_clients() -> usize {
    obs::clients()
}

#[tauri::command]
fn obs_url() -> String {
    obs::url()
}

#[tauri::command]
fn diag(msg: String) {
    dbg(&format!("WEB: {msg}"));
}

pub(crate) fn dbg_pub(msg: &str) {
    dbg(msg);
}

fn dbg(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("H:\\Users\\Administrator\\Documents\\deepseek-harness\\default-workspace\\keyboard-cat\\desktop\\kc_boot.log")
    {
        let _ = writeln!(f, "{msg}");
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        dbg(&format!("PANIC: {info}"));
    }));
    dbg(&format!("main() entered pid={}", std::process::id()));
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 重复启动时只把已有窗口亮出来，不再开第二个实例
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_always_on_top(true);
            }
        }))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_interactive(app);
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
        set_passthrough,
        quit_app,
        diag,
        open_panel_cmd,
        close_panel_cmd,
        fit_window,
        obs_sync,
        obs_state,
        obs_clients,
        obs_url
    ])

        .setup(|app| {
            dbg("setup entered");
            let win = app.get_webview_window("main").expect("main window missing");
            place_above_taskbar(&win);
            let _ = win.set_ignore_cursor_events(true);
            let _ = win.set_always_on_top(true);
            let _ = win.show();

            // 自写托盘：tray-icon crate 在本机 NIM_ADD 静默失败，改用 Shell_NotifyIconW 直接登记。
            tray::start(app.handle().clone());
            // OBS 浏览器源：本地 HTTP + SSE（挂件窗口是分层窗口，直播软件窗口采集抓不到）
            obs::start(app.handle().clone());
    // TMP TEST
    if std::env::var("KC_TEST_INTERACTIVE").ok().as_deref() == Some("1") {
        set_passthrough_state(app.handle(), false);
    }
    if std::env::var("KC_TEST_PANEL").ok().as_deref() == Some("1") {
        let h = app.handle().clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(4));
            open_panel(&h);
        });
    }
            dbg("tray thread started");

            let hotkey = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyK);
            if let Err(e) = app.global_shortcut().register(hotkey) {
                eprintln!("[kc] global shortcut register failed: {e}");
            }

            hook::start(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running keyboard-cat");
}