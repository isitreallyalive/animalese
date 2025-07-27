use crossbeam_channel::unbounded;
use tauri::{DeviceEventFilter, Manager};

mod keyboard;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = unbounded();

    tauri::Builder::default()
        // plugins
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // if the app is launched again, just focus on the already open instance
            let _ = app
                .get_webview_window("main")
                .and_then(|w| w.set_focus().ok());
        }))
        // don't let tauri consume the events when focused
        .device_event_filter(DeviceEventFilter::Always)
        .setup(move |app| {
            let handle = app.handle();
            keyboard::setup(handle.clone(), rx);
            app.manage(tx);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![keyboard::simulate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
