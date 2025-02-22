// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod system;
use system::autoclicker::{handle_pause_resume_state, start_autoclicker, stop_autoclicker, get_autoclicker_state, get_current_keybind,set_mpc, AutoclickerState};
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64}};

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
            set_mpc,
        ])
        .manage(Arc::new(AutoclickerState {
            running: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
            mpc: Arc::new(AtomicU64::new(2000)),
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
