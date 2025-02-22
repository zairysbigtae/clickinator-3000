import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

let isRunning = false;

async function startListener(keybind: string) {
  try {
    await invoke("handle_pause_resume_state", { keybind });
    isRunning = await invoke("get_autoclicker_state");
    console.log(`Autoclicker is now ${isRunning ? "running" : "stopped"}`);
  } catch (err) {
    console.error("Failed to toggle autoclicker: ", err);
  }
}

async function startAutoclicker() {
  try {
    await invoke("start_autoclicker");
    console.log("Autoclicker started");
  } catch (err) {
    console.error("Error starting autoclicker: ", err);
  }
}

async function stopAutoclicker() {
  try {
    await invoke("stop_autoclicker");
    console.log("Autoclicker stopped");
  } catch (err) {
    console.error("Error stopping autoclicker: ", err);
  }
}

async function getCurrentKeybind(choice: string): Promise<string> {
  try {
    return await invoke("get_current_keybind", { keybind: choice });
  } catch (err) {
    console.error("Failed to get the current keybind: ", err);
    return "";
  }
}

async function updateKeybindMessage(choice: string) {
  const current_keybind_msg = document.getElementById("current_keybind");
  if (current_keybind_msg) {
    const keybind = await getCurrentKeybind(choice);
    current_keybind_msg.innerHTML = `Current keybind: Ctrl + ${keybind}`;
  } else {
    console.error("Couldn't edit the message");
  }
}

function listenForKeybindChange(callback: (choice: string) => void) {
  const dropdown = document.getElementById("alphabets") as HTMLSelectElement;
  if (!dropdown) {
    console.error("Dropdown element not found!");
    return;
  }

  dropdown.addEventListener("change", () => {
    const choice = dropdown.value;
    console.log("Selected keybind:", choice);
    callback(choice);
  });

  callback(dropdown.value);
}

async function setMpc() {
  const cps_input = document.getElementById("mpc") as HTMLInputElement;
  cps_input.addEventListener("input", function() {
    invoke("set_mpc", { mpc: +this.value });
    console.log("detected input");
  });
}
setMpc();

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("stop")?.addEventListener("click", stopAutoclicker);
  document.getElementById("start")?.addEventListener("click", () => {
    startAutoclicker();
    console.log("Started autoclicker, frontend logged");
  });

  const dropdown = document.getElementById("alphabets") as HTMLSelectElement;
  if (dropdown) dropdown.options[4].selected = true;

  listenForKeybindChange((choice) => {
    updateKeybindMessage(choice);
    startListener(choice);
  });
});

