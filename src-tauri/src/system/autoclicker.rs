use autopilot::mouse::Button;
use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::State;
use std::{sync::{Arc, Mutex}, time::Duration};

#[derive(Clone)]
pub struct AutoclickerState {
    pub running: Arc<Mutex<bool>>,
    pub paused: Arc<Mutex<bool>>,
}

#[tauri::command]
pub fn start_autoclicker(state: State<Arc<AutoclickerState>>) {
    let state_clone = Arc::clone(&state);
    
    std::thread::spawn(move || {
        loop {
            {
                let running = *state_clone.running.lock().unwrap();
                let paused = *state_clone.paused.lock().unwrap();

                if !running {
                    break;
                }

                if !paused {
                    autopilot::mouse::click(Button::Left, Some(200));
                }
            }

            std::thread::sleep(Duration::from_millis(50));
        }

        println!("Autoclicker thread exited cleanly");
        std::process::exit(0);
    });
}

#[tauri::command]
pub fn stop_autoclicker(state: State<Arc<AutoclickerState>>) {
    let mut running = state.running.lock().unwrap();
    *running = false;

    println!("autoclicker stopped");
}

fn toggle_autoclicker(state: Arc<AutoclickerState>) {
    let mut paused = state.paused.lock().unwrap();
    println!("Before toggle - Paused: {}, State Memory Address: {:p}", *paused, Arc::as_ptr(&state));

    *paused = !*paused;

    println!("After toggle - Paused: {}, State Memory Address: {:p}", *paused, Arc::as_ptr(&state));
}

#[tauri::command]
pub fn handle_pause_resume_state(state: State<Arc<AutoclickerState>>, keybind: String) {
    let state_clone = Arc::clone(&state.inner());

    println!("Spawning thread - State Memory Address: {:p}", Arc::as_ptr(&state_clone));
    std::thread::spawn(move || {
        println!("Initializing DeviceState...");
        let device_state = DeviceState::new();
        println!("DeviceState initialized successfully!");

        let keybind = match keybind.parse::<Keycode>() {
            Ok(k) => k,
            Err(_) => {
                println!("Failed to parse keybind: {}", keybind);
                return;
            },
        };
        println!("Keybind parsed successfully!");
        println!("Parsed keybind: {}", keybind);

        println!("Thread started! Entering keybind loop...");
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if keys.contains(&Keycode::LControl) && keys.contains(&keybind) {
                println!("Keybind pressed");
                toggle_autoclicker(state_clone.clone());
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    });
}

#[tauri::command]
pub fn get_autoclicker_state(state: State<Arc<AutoclickerState>>) -> bool {
    *state.paused.lock().unwrap()
}

#[tauri::command]
pub fn get_current_keybind(_state: State<Arc<AutoclickerState>>, keybind: String) -> String {
    let keybind = keybind.parse::<Keycode>().unwrap();
    keybind.to_string()
}