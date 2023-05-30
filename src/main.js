const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let inputR1;
let inputR2;
let inputR3;

/*
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  //greetMsgEl.textContent = await invoke("resistor", { r1: "Blue", r2: "Green", r3: "Black" });
}
*/
async function resistor() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //greetMsgEl.textContent = await invoke("greet", { name: "greetInputEl.value" });
  greetMsgEl.textContent = await invoke("resistor", { input1: inputR1.value, input2: inputR2.value, input3: inputR3.value });
}

window.addEventListener("DOMContentLoaded", () => {
  inputR1 = document.querySelector("#ring-1");
  inputR2 = document.querySelector("#ring-2");
  inputR3 = document.querySelector("#ring-3");
  greetMsgEl = document.querySelector("#resistor-value-msg");
  document.querySelector("#calc-form").addEventListener("submit", (e) => {
    document.getElementById("box-ring-1").style.backgroundColor = document.getElementById("ring-1").value;
    document.getElementById("box-ring-2").style.backgroundColor = document.getElementById("ring-2").value;
    document.getElementById("box-ring-3").style.backgroundColor = document.getElementById("ring-3").value;
    e.preventDefault();
    resistor();
  });
});
greetInputEl.value