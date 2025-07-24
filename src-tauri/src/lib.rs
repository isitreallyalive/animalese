use rdev::{EventType, Key};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{AppHandle, DeviceEventFilter, Emitter, Manager};

/// Emit a key event to the frontend
fn emit(
    handle: &AppHandle,
    pressed: &mut HashSet<Key>,
    press: bool,
    key: Key,
) -> tauri::Result<()> {
    if press ^ pressed.contains(&key) {
        let _ = handle.emit_to(
            "main",
            if press { "press" } else { "release" },
            format!("{:?}", key),
        )?;
        if press {
            pressed.insert(key);
        } else {
            pressed.remove(&key);
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // plugins
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // if the app is launched again, just focus on the already open instance
            let _ = app
                .get_webview_window("main")
                .map(|w| w.set_focus().ok())
                .flatten();
        }))
        // don't let tauri consume the events when focused
        .device_event_filter(DeviceEventFilter::Always)
        .setup(|app| {
            // handle keypresses
            let handle = app.handle();
            let pressed = Arc::new(Mutex::new(HashSet::new()));
            std::thread::spawn({
                let handle = handle.clone();
                let pressed = pressed.clone();
                move || {
                    loop {
                        let handle = handle.clone();
                        let pressed = pressed.clone();
                        if let Err(e) = rdev::listen(move |event| {
                            // todo: distinguish between lctrl and altgr
                            match event.event_type {
                                EventType::KeyPress(key) => {
                                    let mut pressed = pressed.lock().unwrap();
                                    let _ = emit(&handle, &mut pressed, true, key);
                                }
                                EventType::KeyRelease(key) => {
                                    let mut pressed = pressed.lock().unwrap();
                                    let _ = emit(&handle, &mut pressed, false, key);
                                }
                                _ => {}
                            }
                        }) {
                            eprintln!("Error in rdev::listen: {:?}. Restarting listener...", e);
                            std::thread::sleep(Duration::from_millis(100));
                        } else {
                            break;
                        }
                    }
                }
            });

            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
