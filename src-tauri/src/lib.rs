use rdev::{EventType, Key};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, DeviceEventFilter, Emitter};

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
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        } else {
                            break;
                        }
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
