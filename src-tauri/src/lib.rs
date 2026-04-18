use enigo::{Enigo, Key, Keyboard, Mouse, Settings};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::sleep;
use std::time::Duration;
use tauri::{Emitter, Manager};

#[cfg(windows)]
use windows::Win32::Foundation::POINT;
#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F8;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetCursorPos, GetMessageW, SetWindowsHookExW,
    UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WH_MOUSE_LL,
    WM_KEYDOWN, WM_LBUTTONDOWN, WM_MBUTTONDOWN, WM_RBUTTONDOWN,
};

static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);
static LAST_POS: Mutex<(i32, i32)> = Mutex::new((0, 0));
const MOVEMENT_THRESHOLD: i32 = 24;

// 0=None, 1=F8, 2=MiddleClick, 3=LeftClick, 4=RightClick
static HOTKEY_MODE: AtomicI32 = AtomicI32::new(1);
static HOTKEY_HOOK_RUNNING: AtomicBool = AtomicBool::new(false);

// Global app handle stored once for use in low-level hook callbacks
static GLOBAL_APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

fn get_cursor_pos_win() -> Option<(i32, i32)> {
    #[cfg(windows)]
    unsafe {
        let mut pt = POINT::default();
        if GetCursorPos(&mut pt).is_ok() {
            return Some((pt.x, pt.y));
        }
    }
    None
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    ((dx * dx + dy * dy) as f64).sqrt() as i32
}

#[derive(serde::Deserialize)]
pub enum ClickButton {
    Left,
    Right,
    Middle,
    WheelDown,
    Enter,
}

#[derive(Serialize, Clone)]
pub struct MouseMovedPayload {}

#[derive(Serialize, Clone)]
pub struct HotkeyPayload {}

#[tauri::command]
fn click_mouse(button: ClickButton) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    match button {
        ClickButton::Left => {
            enigo.button(enigo::Button::Left, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.button(enigo::Button::Left, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }
        ClickButton::Right => {
            enigo.button(enigo::Button::Right, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.button(enigo::Button::Right, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }
        ClickButton::Middle => {
            enigo.button(enigo::Button::Middle, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.button(enigo::Button::Middle, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }
        ClickButton::WheelDown => {
            enigo.scroll(3, enigo::Axis::Vertical).map_err(|e| e.to_string())?;
        }
        ClickButton::Enter => {
            enigo.key(Key::Return, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(Key::Return, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
fn start_mouse_monitor(app_handle: tauri::AppHandle) -> Result<(), String> {
    if MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    if let Some(pos) = get_cursor_pos_win() {
        *LAST_POS.lock().unwrap() = pos;
    }

    std::thread::spawn(move || loop {
        sleep(Duration::from_millis(100));
        if let Some(pos) = get_cursor_pos_win() {
            let last = *LAST_POS.lock().unwrap();
            if distance(pos, last) >= MOVEMENT_THRESHOLD {
                let _ = app_handle.emit("mouse-moved", &MouseMovedPayload {});
                *LAST_POS.lock().unwrap() = pos;
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn set_hotkey_mode(mode: i32) {
    HOTKEY_MODE.store(mode, Ordering::SeqCst);
}

#[tauri::command]
fn start_hotkey_monitor(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Store handle globally (only once)
    let _ = GLOBAL_APP_HANDLE.set(app_handle);

    if HOTKEY_HOOK_RUNNING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    std::thread::spawn(|| {
        #[cfg(windows)]
        unsafe {
            unsafe extern "system" fn keyboard_hook(
                code: i32,
                wparam: windows::Win32::Foundation::WPARAM,
                lparam: windows::Win32::Foundation::LPARAM,
            ) -> windows::Win32::Foundation::LRESULT {
                if code >= 0 && wparam.0 as u32 == WM_KEYDOWN {
                    let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
                    if HOTKEY_MODE.load(Ordering::SeqCst) == 1
                        && kb.vkCode == VK_F8.0 as u32
                    {
                        if let Some(h) = GLOBAL_APP_HANDLE.get() {
                            let _ = h.emit("hotkey-triggered", &HotkeyPayload {});
                        }
                    }
                }
                CallNextHookEx(HHOOK::default(), code, wparam, lparam)
            }

            unsafe extern "system" fn mouse_hook(
                code: i32,
                wparam: windows::Win32::Foundation::WPARAM,
                lparam: windows::Win32::Foundation::LPARAM,
            ) -> windows::Win32::Foundation::LRESULT {
                if code >= 0 {
                    let mode = HOTKEY_MODE.load(Ordering::SeqCst);
                    let msg = wparam.0 as u32;
                    let triggered = matches!(
                        (mode, msg),
                        (2, WM_MBUTTONDOWN) | (3, WM_LBUTTONDOWN) | (4, WM_RBUTTONDOWN)
                    );
                    if triggered {
                        if let Some(h) = GLOBAL_APP_HANDLE.get() {
                            let _ = h.emit("hotkey-triggered", &HotkeyPayload {});
                        }
                    }
                }
                CallNextHookEx(HHOOK::default(), code, wparam, lparam)
            }

            let kb_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)
                .expect("keyboard hook failed");
            let ms_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), None, 0)
                .expect("mouse hook failed");

            let mut msg = MSG::default();
            loop {
                if GetMessageW(&mut msg, None, 0, 0).0 <= 0 {
                    break;
                }
                DispatchMessageW(&msg);
            }

            UnhookWindowsHookEx(kb_hook).ok();
            UnhookWindowsHookEx(ms_hook).ok();
        }
    });

    Ok(())
}

// ── Window position persistence ──────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Copy)]
struct WindowPos {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn window_state_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("no app data dir")
        .join("window_state.json")
}

fn load_window_pos(app: &tauri::AppHandle) -> Option<WindowPos> {
    let path = window_state_path(app);
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn save_window_pos(app: &tauri::AppHandle, pos: WindowPos) {
    let path = window_state_path(app);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(data) = serde_json::to_string(&pos) {
        let _ = std::fs::write(path, data);
    }
}

/// Returns true if the given window rect overlaps at least one available monitor.
fn is_on_any_monitor(monitors: &[tauri::Monitor], x: i32, y: i32, w: u32, h: u32) -> bool {
    for m in monitors {
        let mp = m.position();
        let ms = m.size();
        let mx0 = mp.x;
        let my0 = mp.y;
        let mx1 = mx0 + ms.width as i32;
        let my1 = my0 + ms.height as i32;

        // Window rect
        let wx0 = x;
        let wy0 = y;
        let wx1 = x + w as i32;
        let wy1 = y + h as i32;

        // Overlap check
        if wx0 < mx1 && wx1 > mx0 && wy0 < my1 && wy1 > my0 {
            return true;
        }
    }
    false
}

fn restore_window_position(app: &tauri::AppHandle) {
    let window = match app.get_webview_window("main") {
        Some(w) => w,
        None => return,
    };

    let saved = match load_window_pos(app) {
        Some(p) => p,
        None => {
            // First launch — center manually
            let _ = window.center();
            return;
        }
    };

    let monitors: Vec<tauri::Monitor> = window.available_monitors().unwrap_or_default();

    if is_on_any_monitor(&monitors, saved.x, saved.y, saved.width, saved.height) {
        let _ = window.set_position(tauri::PhysicalPosition::new(saved.x, saved.y));
        let _ = window.set_size(tauri::PhysicalSize::new(saved.width, saved.height));
    } else {
        // Monitor gone — center on current primary monitor
        let _ = window.center();
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            restore_window_position(&app.handle().clone());

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            // Save position on window move / resize
            let handle = app.handle().clone();
            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::Moved(pos) => {
                            // grab current size too
                            if let Some(win) = handle.get_webview_window("main") {
                                if let Ok(size) = win.outer_size() {
                                    save_window_pos(
                                        &handle,
                                        WindowPos {
                                            x: pos.x,
                                            y: pos.y,
                                            width: size.width,
                                            height: size.height,
                                        },
                                    );
                                }
                            }
                        }
                        tauri::WindowEvent::Resized(size) => {
                            if let Some(win) = handle.get_webview_window("main") {
                                if let Ok(pos) = win.outer_position() {
                                    save_window_pos(
                                        &handle,
                                        WindowPos {
                                            x: pos.x,
                                            y: pos.y,
                                            width: size.width,
                                            height: size.height,
                                        },
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![
            click_mouse,
            start_mouse_monitor,
            set_hotkey_mode,
            start_hotkey_monitor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
