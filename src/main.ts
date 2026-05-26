import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function start_timer() {
  if (greetMsgEl && greetInputEl) {
    try{
      greetMsgEl.textContent = await invoke("start_timer_cmd", {
      file: String(greetInputEl.value,)
    });
  }catch (e){console.log(e)}
  }
}

async function stop_timer() {
  if (greetMsgEl && greetInputEl) {
    try{
      greetMsgEl.textContent = await invoke("stop_timer_cmd", {
      file: String(greetInputEl.value,)
    });
  }catch (e){console.log(e)}
  }
}
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  
  document.querySelector("#start")?.addEventListener("click", (e) => {
    console.log("start")
      e.preventDefault();
      start_timer()
    });
  document.querySelector("#stop")?.addEventListener("click", (e) => {
    console.log("stop")
      e.preventDefault();
      stop_timer()
    });

});
