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

struct KeyStates {
    real: HashSet<Key>,
    simulated: HashSet<Key>,
}

/// Emit a key to the frontend
fn emit(
    event_type: EventType,
    handle: &AppHandle,
    key_states: &mut KeyStates,
    simulated: bool,
) -> tauri::Result<()> {
    let (press, key) = match event_type {
        EventType::KeyPress(key) => (true, key),
        EventType::KeyRelease(key) => (false, key),
        _ => return Ok(()),
    };

    let (this_set, other_set) = if simulated {
        (&mut key_states.simulated, &mut key_states.real)
    } else {
        (&mut key_states.real, &mut key_states.simulated)
    };

    // if the other set contains the key, ignore this event
    if other_set.contains(&key) {
        return Ok(());
    }

    if press ^ this_set.contains(&key) {
        let _ = handle.emit_to(
            "main",
            if press { "press" } else { "release" },
            format!("{:?}", key),
        )?;
        if press {
            this_set.insert(key);
        } else {
            this_set.remove(&key);
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
            let key_states = Arc::new(Mutex::new(KeyStates {
                real: HashSet::new(),
                simulated: HashSet::new(),
            }));

            // keypress listener thread (real)
            {
                let handle = handle.clone();
                let key_states = key_states.clone();
                thread::spawn(move || loop {
                    if let Err(e) = rdev::listen({
                        let handle = handle.clone();
                        let key_states = key_states.clone();
                        move |event| {
                            let mut ks = key_states.lock().unwrap();
                            let _ = emit(event.event_type, &handle, &mut *ks, false);
                        }
                    }) {
                        eprintln!("Error in rdev::listen: {:?}. Restarting listener...", e);
                        thread::sleep(Duration::from_millis(100));
                    } else {
                        break;
                    }
                });
            }

            // simulated keypress thread
            {
                let handle = handle.clone();
                let key_states = key_states.clone();
                thread::spawn(move || {
                    while let Ok(event_type) = rx.recv() {
                        let mut ks = key_states.lock().unwrap();
                        let _ = emit(event_type, &handle, &mut *ks, true);
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
