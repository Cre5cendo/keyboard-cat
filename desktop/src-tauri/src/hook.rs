use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::OnceLock;

use tauri::{AppHandle, Emitter};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

static APP: OnceLock<AppHandle> = OnceLock::new();
static TX: OnceLock<Sender<KeyPayload>> = OnceLock::new();
static LAST_VK: AtomicU32 = AtomicU32::new(0);
static LAST_TIME: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, serde::Serialize)]
struct KeyPayload {
    code: String,
    vk: u32,
    down: bool,
    repeat: bool,
}

pub fn start(app: AppHandle) {
    let _ = APP.set(app.clone());
    // 钩子回调里只做一次无阻塞 send：在回调里直接 emit 会超时，系统会把低级钩子悄悄摘掉
    let (tx, rx) = mpsc::channel::<KeyPayload>();
    let _ = TX.set(tx);
    std::thread::spawn(move || {
        while let Ok(p) = rx.recv() {
            // OBS 浏览器源要同一份按键流（直接推 SSE，不经过窗口）
            crate::obs::push_key(&p.code, p.down, p.repeat);
            let _ = app.emit("kc-key", p);
        }
    });
    std::thread::spawn(|| unsafe {
        let hook = match SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), None, 0) {
            Ok(h) => {
                crate::dbg_pub("HOOK: installed");
                h
            }
            Err(e) => {
                crate::dbg_pub(&format!("HOOK: install failed {e:?}"));
                return;
            }
        };
        let mut msg = MSG::default();
        // GetMessageW 出错返回 -1，而 BOOL(-1).as_bool() 仍为 true —— 原写法会变成吃满一个核的忙循环
        loop {
            let r = GetMessageW(&mut msg, None, 0, 0);
            if r.0 == -1 || r.0 == 0 {
                crate::dbg_pub(&format!("HOOK: message loop exit r={}", r.0));
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let _ = UnhookWindowsHookEx(hook);
    });
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 && lparam.0 != 0 {
        let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        let msg = wparam.0 as u32;
        let down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
        let up = msg == WM_KEYUP || msg == WM_SYSKEYUP;
        if down || up {
            let extended = (kb.flags.0 & LLKHF_EXTENDED.0) != 0;
            let code_str = map_scancode(kb.scanCode as u16, extended);
            let repeat = if down {
                let prev_vk = LAST_VK.swap(kb.vkCode, Ordering::Relaxed);
                let prev_t = LAST_TIME.swap(kb.time, Ordering::Relaxed);
                prev_vk == kb.vkCode && kb.time.wrapping_sub(prev_t) < 400
            } else {
                false
            };
            if let Some(tx) = TX.get() {
                let _ = tx.send(KeyPayload {
                    code: code_str.to_string(),
                    vk: kb.vkCode,
                    down,
                    repeat,
                });
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

/// Windows set-1 scancode -> DOM KeyboardEvent.code
fn map_scancode(sc: u16, extended: bool) -> &'static str {
    if extended {
        return match sc {
            0x1C => "NumpadEnter",
            0x1D => "ControlRight",
            0x35 => "NumpadDivide",
            0x38 => "AltRight",
            0x47 => "Home",
            0x48 => "ArrowUp",
            0x49 => "PageUp",
            0x4B => "ArrowLeft",
            0x4D => "ArrowRight",
            0x4F => "End",
            0x50 => "ArrowDown",
            0x51 => "PageDown",
            0x52 => "Insert",
            0x53 => "Delete",
            0x5B => "MetaLeft",
            0x5C => "MetaRight",
            0x5D => "ContextMenu",
            _ => "Unknown",
        };
    }
    match sc {
        0x01 => "Escape",
        0x02 => "Digit1",
        0x03 => "Digit2",
        0x04 => "Digit3",
        0x05 => "Digit4",
        0x06 => "Digit5",
        0x07 => "Digit6",
        0x08 => "Digit7",
        0x09 => "Digit8",
        0x0A => "Digit9",
        0x0B => "Digit0",
        0x0C => "Minus",
        0x0D => "Equal",
        0x0E => "Backspace",
        0x0F => "Tab",
        0x10 => "KeyQ",
        0x11 => "KeyW",
        0x12 => "KeyE",
        0x13 => "KeyR",
        0x14 => "KeyT",
        0x15 => "KeyY",
        0x16 => "KeyU",
        0x17 => "KeyI",
        0x18 => "KeyO",
        0x19 => "KeyP",
        0x1A => "BracketLeft",
        0x1B => "BracketRight",
        0x1C => "Enter",
        0x1D => "ControlLeft",
        0x1E => "KeyA",
        0x1F => "KeyS",
        0x20 => "KeyD",
        0x21 => "KeyF",
        0x22 => "KeyG",
        0x23 => "KeyH",
        0x24 => "KeyJ",
        0x25 => "KeyK",
        0x26 => "KeyL",
        0x27 => "Semicolon",
        0x28 => "Quote",
        0x29 => "Backquote",
        0x2A => "ShiftLeft",
        0x2B => "Backslash",
        0x2C => "KeyZ",
        0x2D => "KeyX",
        0x2E => "KeyC",
        0x2F => "KeyV",
        0x30 => "KeyB",
        0x31 => "KeyN",
        0x32 => "KeyM",
        0x33 => "Comma",
        0x34 => "Period",
        0x35 => "Slash",
        0x36 => "ShiftRight",
        0x37 => "NumpadMultiply",
        0x38 => "AltLeft",
        0x39 => "Space",
        0x3A => "CapsLock",
        0x3B => "F1",
        0x3C => "F2",
        0x3D => "F3",
        0x3E => "F4",
        0x3F => "F5",
        0x40 => "F6",
        0x41 => "F7",
        0x42 => "F8",
        0x43 => "F9",
        0x44 => "F10",
        0x45 => "NumLock",
        0x46 => "ScrollLock",
        0x47 => "Numpad7",
        0x48 => "Numpad8",
        0x49 => "Numpad9",
        0x4A => "NumpadSubtract",
        0x4B => "Numpad4",
        0x4C => "Numpad5",
        0x4D => "Numpad6",
        0x4E => "NumpadAdd",
        0x4F => "Numpad1",
        0x50 => "Numpad2",
        0x51 => "Numpad3",
        0x52 => "Numpad0",
        0x53 => "NumpadDecimal",
        0x56 => "IntlBackslash",
        0x57 => "F11",
        0x58 => "F12",
        _ => "Unknown",
    }
}