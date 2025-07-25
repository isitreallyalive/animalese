use crossbeam_channel::{unbounded, Receiver, Sender};
use rdev::{EventType, Key};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::{AppHandle, DeviceEventFilter, Emitter, Manager};

/// Simulate a keypress event
#[tauri::command]
fn simulate(
    tx: tauri::State<'_, Sender<EventType>>,
    key: String,
    press: bool,
) -> Result<(), &'static str> {
    let key = serde_json::from_str::<Key>(&format!("{:?}", key)).map_err(|_| "Invalid key")?;
    let event = if press {
        EventType::KeyPress(key)
    } else {
        EventType::KeyRelease(key)
    };
    tx.send(event).map_err(|_| "Failed to send event")?;
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct EmittedKey {
    key: Key,
    simulated: bool,
}

/// Emit a key to the frontend
fn emit(
    event_type: EventType,
    handle: &AppHandle,
    pressed: &mut HashSet<Key>,
) -> tauri::Result<()> {
    // todo: distinguish between lctrl and altgr
    let (press, key) = match event_type {
        EventType::KeyPress(key) => (true, key),
        EventType::KeyRelease(key) => (false, key),
        _ => return Ok(()),
    };

    if press ^ pressed.contains(&key) {
        println!("{:?}", key);
        let _ = handle.emit_to(
            "main",
            if press { "press" } else { "release" },
            EmittedKey {
                key,
                simulated: false,
            },
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
    let (tx, rx): (Sender<EventType>, Receiver<EventType>) = unbounded();
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
        .setup(move |app| {
            let handle = app.handle();
            let pressed = Arc::new(Mutex::new(HashSet::new()));

            // keypress listener thread
            {
                let handle = handle.clone();
                let pressed = pressed.clone();
                thread::spawn(move || loop {
                    let handle = handle.clone();
                    let pressed = pressed.clone();
                    if let Err(e) = rdev::listen(move |event| {
                        let mut pressed = pressed.lock().unwrap();
                        let _ = emit(event.event_type, &handle, &mut pressed);
                    }) {
                        eprintln!("Error in rdev::listen: {:?}. Restarting listener...", e);
                        std::thread::sleep(Duration::from_millis(100));
                    } else {
                        break;
                    }
                });
            }

            // simulated keypress thread
            {
                let handle = handle.clone();
                thread::spawn(move || {
                    while let Ok(event_type) = rx.recv() {
                        let mut pressed = pressed.lock().unwrap();
                        let _ = emit(event_type, &handle, &mut pressed);
                    }
                });
            }

            app.manage(tx);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![simulate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
