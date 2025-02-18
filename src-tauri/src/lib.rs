// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod system;
use system::autoclicker::{handle_pause_resume_state, start_autoclicker, stop_autoclicker, get_autoclicker_state, get_current_keybind, AutoclickerState};
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_autoclicker,
            stop_autoclicker,
            handle_pause_resume_state, 
            get_autoclicker_state,
            get_current_keybind,
        ])
        .manage(Arc::new(AutoclickerState {
            running: Arc::new(Mutex::new(true)),
            paused: Arc::new(Mutex::new(false)),
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
