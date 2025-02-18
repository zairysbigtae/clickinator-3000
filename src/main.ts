import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

let isRunning = false;



async function startListener() {
  try {
    await invoke("handle_pause_resume_state", { keybind: "E" });
    isRunning = await invoke("get_autoclicker_state");
    console.log(`Autoclicker is now ${isRunning ? "running" : "stopped"}`);
  } catch (err) {
    console.error("Failed to toggle autoclicker: ", err);
  }
}
startListener();

async function startAutoclicker() {
  try {
    await invoke("start_autoclicker");
    console.log("autoclicker started");
  } catch (err) {
    console.error(err);
  }
}

async function getCurrentKeybind() {
  try {
    let keybind = await invoke("get_current_keybind", { keybind: "A" });
    return keybind;
  } catch (err) {
    console.error("Failed to get the current keybind: ", err);
  }
}

async function stopAutoclicker() {
  try {
    await invoke("stop_autoclicker");
    console.log("autoclicker stopped");
  } catch (err) {
    console.error(err);
  }
}

function changeCurrentKeybindMessage() {
  setInterval(async () => {
    const current_keybind_msg = document.getElementById("current_keybind");

    if (current_keybind_msg) {
      current_keybind_msg.innerHTML = `Current keybind: Ctrl + ${await getCurrentKeybind()}`;
    } else {
      console.error("Couldnt edit the msg");
    }
  }, 1000);
}

changeCurrentKeybindMessage();

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("stop")?.addEventListener("click", stopAutoclicker);
  document.getElementById("start")?.addEventListener("click", startAutoclicker);

  document.getElementById("cars")?.getElementsByTagName("option")[4].setAttribute("selected", "selected");
});