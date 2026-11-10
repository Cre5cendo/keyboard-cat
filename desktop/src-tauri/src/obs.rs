//! OBS 浏览器源：本地 HTTP + SSE。
//!
//! 挂件窗口是 WS_EX_LAYERED，WGC 的 CreateForWindow 会直接拒绝、BitBlt 也读不到像素，
//! 所以直播软件没法“窗口采集”它。这里换个思路：让 OBS 用「浏览器源」打开本机的一个
//! 小 HTTP 页面，页面自己画键盘，按压事件通过 SSE 实时推过去。浏览器源自带透明。

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::Duration;


static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
static PORT: AtomicU16 = AtomicU16::new(0);

struct Hub {
    next: u64,
    items: VecDeque<(u64, String)>,
    state: String,
}

static HUB: Mutex<Hub> = Mutex::new(Hub {
    next: 1,
    items: VecDeque::new(),
    state: String::new(),
});
static CVAR: Condvar = Condvar::new();
/// 当前有多少个 OBS 浏览器源连着（0 时挂件就不必编码推帧）
static CLIENTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn port() -> u16 {
    PORT.load(Ordering::Relaxed)
}

pub fn url() -> String {
    let p = port();
    if p == 0 {
        "http://127.0.0.1:17861/obs".to_string()
    } else {
        format!("http://127.0.0.1:{p}/obs")
    }
}

fn push(json: String) {
    {
        let mut h = HUB.lock().unwrap();
        let seq = h.next;
        h.next += 1;
        h.items.push_back((seq, json));
        while h.items.len() > 400 {
            h.items.pop_front();
        }
    }
    CVAR.notify_all();
}

/// 全局钩子里的按键，转发给所有 OBS 页面
pub fn push_key(code: &str, down: bool, repeat: bool) {
    if code.is_empty() || code == "Unknown" {
        return;
    }
    push(format!(
        r#"{{"t":"k","code":"{code}","down":{down},"repeat":{repeat}}}"#
    ));
}

/// 当前连着的 OBS 浏览器源数量（0 时挂件不必生成状态）
pub fn clients() -> usize {
    CLIENTS.load(std::sync::atomic::Ordering::Relaxed)
}

/// OBS 状态同步：挂件把「猫在哪 / 什么状态 / 哪些键按着」打成一个小 JSON 推过来，
/// OBS 页面收到后自己渲染（60fps），不传像素。
pub fn push_state(json: String) {
    push(json);
}

pub fn set_state(json: String) {
    {
        let mut h = HUB.lock().unwrap();
        h.state = json.clone();
    }
    push(json);
}

pub fn start(app: tauri::AppHandle) {
    let _ = APP.set(app);
    std::thread::spawn(|| {
        let mut listener: Option<TcpListener> = None;
        for p in 17861..17867u16 {
            if let Ok(l) = TcpListener::bind(("127.0.0.1", p)) {
                PORT.store(p, Ordering::Relaxed);
                listener = Some(l);
                break;
            }
        }
        let Some(listener) = listener else {
            crate::dbg_pub("OBS: bind 127.0.0.1:17861.. failed");
            return;
        };
        crate::dbg_pub(&format!("OBS: listening {}", url()));
        for s in listener.incoming() {
            match s {
                Ok(s) => {
                    std::thread::spawn(move || handle(s));
                }
                Err(_) => break,
            }
        }
    });
}

fn url_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'%' if i + 2 < b.len() => {
                let hi = (b[i + 1] as char).to_digit(16);
                let lo = (b[i + 2] as char).to_digit(16);
                match (hi, lo) {
                    (Some(h), Some(l)) => {
                        out.push((h * 16 + l) as u8);
                        i += 3;
                    }
                    _ => {
                        out.push(b[i]);
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn handle(stream: TcpStream) {
    let _ = stream.set_nodelay(true);
    let mut out = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("/").to_string();
    // 把 header 读干净，否则 close 时对端可能看到 RST
    let mut hbuf = String::new();
    loop {
        hbuf.clear();
        match reader.read_line(&mut hbuf) {
            Ok(0) => break,
            Ok(_) => {
                if hbuf == "\r\n" || hbuf == "\n" {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    if method != "GET" {
        respond(&mut out, "405 Method Not Allowed", "text/plain", b"GET only");
        return;
    }

    let path = target
        .split('?')
        .next()
        .unwrap_or("/")
        .trim_end_matches('/');
    let path = if path.is_empty() { "/" } else { path };

    if path == "/events" {
        sse(out);
        return;
    }
    // 自测用：不经过键盘钩子，直接推一条按键（全局注入在提权窗口前面不可靠）
    if path == "/api/key" {
        let q = target.split('?').nth(1).unwrap_or("");
        let mut code = String::new();
        let mut down = true;
        for kv in q.split('&') {
            let mut it = kv.splitn(2, '=');
            match (it.next(), it.next()) {
                (Some("code"), Some(v)) => code = v.to_string(),
                (Some("down"), Some(v)) => down = v == "1" || v == "true",
                _ => {}
            }
        }
        if !code.is_empty() {
            push_key(&code, down, false);
        }
        respond(&mut out, "200 OK", "text/plain", b"ok");
        return;
    }
    // 自测用：直接推任意一条事件 JSON（调试 OBS 页面用，不经过挂件）
    if path == "/api/push" {
        let q = target.split('?').nth(1).unwrap_or("");
        let mut j = String::new();
        for kv in q.split('&') {
            let mut it = kv.splitn(2, '=');
            if let (Some("j"), Some(v)) = (it.next(), it.next()) {
                j = url_decode(v);
            }
        }
        if !j.is_empty() {
            push(j);
        }
        respond(&mut out, "200 OK", "text/plain", b"ok");
        return;
    }
    if path == "/api/size" || path == "/api/state" {
        let body = {
            let h = HUB.lock().unwrap();
            if h.state.is_empty() {
                "{}".to_string()
            } else {
                h.state.clone()
            }
        };
        respond(&mut out, "200 OK", "application/json; charset=utf-8", body.as_bytes());
        return;
    }

    let asset_path = match path {
        "/" | "/obs" | "/obs.html" => "obs.html".to_string(),
        other => other.trim_start_matches('/').to_string(),
    };
    // asset_resolver 对未知路径会回落 index.html；这里只放行带扩展名的资源
    let last_seg = asset_path.rsplit('/').next().unwrap_or("");
    if asset_path != "obs.html" && !last_seg.contains('.') {
        crate::dbg_pub(&format!("OBS: 404 (no ext) {path}"));
        respond(&mut out, "404 Not Found", "text/plain; charset=utf-8", b"404");
        return;
    }
    let Some(app) = APP.get() else {
        respond(&mut out, "503 Service Unavailable", "text/plain", b"app not ready");
        return;
    };
    match app.asset_resolver().get(asset_path.clone()) {
        Some(asset) => {
            let mime = asset.mime_type().to_string();
            let bytes = asset.bytes().to_vec();
            respond(&mut out, "200 OK", &mime, &bytes);
        }
        None => {
            crate::dbg_pub(&format!("OBS: 404 {path}"));
            respond(&mut out, "404 Not Found", "text/plain; charset=utf-8", b"404");
        }
    }
}

fn respond(stream: &mut TcpStream, code: &str, mime: &str, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 {code}\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

fn sse(mut out: TcpStream) {
    CLIENTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    struct Decr;
    impl Drop for Decr {
        fn drop(&mut self) {
            CLIENTS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    let _decr = Decr;
    let head = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream; charset=utf-8\r\nCache-Control: no-store\r\nConnection: keep-alive\r\nAccess-Control-Allow-Origin: *\r\nX-Accel-Buffering: no\r\n\r\n";
    if out.write_all(head.as_bytes()).is_err() {
        return;
    }
    let _ = out.write_all(b"retry: 1500\n\n");

    // 连上先补一份当前状态（主题 / 猫尺寸）
    let (mut last, state) = {
        let h = HUB.lock().unwrap();
        (h.next - 1, h.state.clone())
    };
    if !state.is_empty() {
        if out
            .write_all(format!("data: {state}\n\n").as_bytes())
            .is_err()
        {
            return;
        }
    }
    let _ = out.flush();

    loop {
        let pending: Vec<(u64, String)> = {
            let h = HUB.lock().unwrap();
            h.items
                .iter()
                .filter(|(s, _)| *s > last)
                .cloned()
                .collect()
        };
        if pending.is_empty() {
            let h = HUB.lock().unwrap();
            let (_h, res) = CVAR.wait_timeout(h, Duration::from_secs(15)).unwrap();
            if res.timed_out() {
                drop(_h);
                // 心跳，避免 OBS 的 CEF 把长连接当死链
                if out.write_all(b": ping\n\n").is_err() || out.flush().is_err() {
                    return;
                }
            }
            continue;
        }
        let mut buf = String::new();
        for (seq, json) in pending {
            if seq <= last {
                continue;
            }
            buf.push_str("data: ");
            buf.push_str(&json);
            buf.push_str("\n\n");
            last = seq;
        }
        if out.write_all(buf.as_bytes()).is_err() || out.flush().is_err() {
            return;
        }
    }
}