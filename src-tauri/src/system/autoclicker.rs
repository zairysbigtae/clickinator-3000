use autopilot::mouse::Button;
use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::State;
use std::{fs, sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc}, time::Duration};

#[derive(Clone)]
pub struct AutoclickerState {
    pub running: Arc<AtomicBool>,
    pub paused: Arc<AtomicBool>,
    pub mpc: Arc<AtomicU64>,
}
#[derive(serde::Serialize)]
struct KeybindData {
    key: String,
}

#[tauri::command]
pub fn start_autoclicker(state: State<Arc<AutoclickerState>>) {
    let state_clone = Arc::clone(&state);
    
    std::thread::spawn(move || {
        loop {
            {

                if !state_clone.running.load(Ordering::SeqCst) {
                    break;
                }

                if !state_clone.paused.load(Ordering::SeqCst) {
                    autopilot::mouse::click(Button::Left, None);
                    std::thread::sleep(Duration::from_millis(state_clone.mpc.load(Ordering::SeqCst)));
                }
            }
        }

        println!("Autoclicker thread exited cleanly");
        std::process::exit(0);
    });
}

#[tauri::command]
pub fn stop_autoclicker(state: State<Arc<AutoclickerState>>) {
    state.running.store(false, Ordering::SeqCst);

    println!("autoclicker stopped");
}

fn toggle_autoclicker(state: Arc<AutoclickerState>) {
    println!("Before toggle - Paused: {}, State Memory Address: {:p}", state.paused.load(Ordering::SeqCst), Arc::as_ptr(&state));

    state.paused.store(!state.paused.load(Ordering::SeqCst), Ordering::SeqCst);

    println!("After toggle - Paused: {}, State Memory Address: {:p}", state.paused.load(Ordering::SeqCst), Arc::as_ptr(&state));
}

#[tauri::command]
pub fn handle_pause_resume_state(state: State<Arc<AutoclickerState>>, Keybind: String) {
    let state_clone = Arc::clone(&state.inner());

    println!("Spawning thread - State Memory Address: {:p}", Arc::as_ptr(&state_clone));
    std::thread::spawn(move || {
        println!("Initializing DeviceState...");
        let device_state = DeviceState::new();
        println!("DeviceState initialized successfully!");

        let Keybind = match Keybind.parse::<Keycode>() {
            Ok(k) => k,
            Err(_) => {
                println!("Failed to parse keybind: {}", Keybind);
                return;
            },
        };
        println!("keybind parsed successfully!");
        println!("Parsed keybind: {}", Keybind);

        println!("Thread started! Entering keybind loop...");
        loop {
            let mut keys: Vec<Keycode> = device_state.get_keys();
            if keys.contains(&Keycode::LControl) && keys.contains(&Keybind) {
                keys.clear();
                println!("Keybind pressed");
                toggle_autoclicker(state_clone.clone());
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    });
}

#[tauri::command]
pub fn get_autoclicker_state(state: State<Arc<AutoclickerState>>) -> bool {
    state.paused.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn get_current_keybind(_state: State<Arc<AutoclickerState>>, Keybind: String) -> String {
    println!("Executed, Received keybind: {}", Keybind);
    Keybind
}

#[tauri::command]
pub fn save_current_keybind(_state: State<Arc<AutoclickerState>>, Keybind: String) -> std::io::Result<()> {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("clickinator_3000");
    fs::create_dir_all(&config_path)?;

    let file_path = config_path.join("config.yaml");

    let config = KeybindData {
        key: Keybind
    };

    let yaml_string = serde_yaml::to_string(&config).unwrap();
    fs::write(&file_path, yaml_string)?;

    println!("Config saved to {:?}", file_path);
    Ok(())
}

#[tauri::command]
pub fn set_mpc(state: State<Arc<AutoclickerState>>, mpc: u64) {
    state.mpc.store(mpc, Ordering::SeqCst);
    println!("MPC set to {}", mpc);
}