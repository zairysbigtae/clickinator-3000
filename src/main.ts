import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

let isRunning = false;

async function startListener(keybind: string) {
  try {
    await invoke("handle_pause_resume_state", { keybind: keybind });
    isRunning = await invoke("get_autoclicker_state");
    console.log(`Autoclicker is now ${isRunning ? "running" : "stopped"}`);
  } catch (err) {
    console.error("Failed to toggle autoclicker: ", err);
  }
}

async function startAutoclicker() {
  try {
    await invoke("start_autoclicker");
    console.log("autoclicker started");
  } catch (err) {
    console.error(err);
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

function returnDropdownSelectedChoice() {
  let selectedKeybind = "";
  setInterval(() => {
    // TODO: take a look at the form and then implement the keybind changes based on the dropdown choice
    const choice = document.getElementById("alphabets")?.getElementsByTagName("option");

    if (choice) {
      for (let i = 0; i < choice.length; i++) {
        const option = choice[i];

        if (option.getAttribute("selected") === "selected") {
           selectedKeybind = option.value;
        }
      }
    }
  }, 1000);
  return selectedKeybind
}

changeCurrentKeybindMessage();
let choice = returnDropdownSelectedChoice();
if (choice) {
  startListener(choice);
}

async function getCurrentKeybind() {
  try {
    let keybind = await invoke("get_current_keybind", { keybind: choice });
    return keybind;
  } catch (err) {
    console.error("Failed to get the current keybind: ", err);
  }
}

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("stop")?.addEventListener("click", stopAutoclicker);
  document.getElementById("start")?.addEventListener("click", startAutoclicker);

  document.getElementById("alphabets")?.getElementsByTagName("option")[4].setAttribute("selected", "selected");
});