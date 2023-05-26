
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn resistor(input_1: &str, input_2: &str, input_3: &str) -> String{
    let mut value = res::resistor_value(input_1, input_2, input_3);
    let prefix: &str;
    if value >= 1000000000.0 {
        value /= 1000000000.0;
        prefix = "G";
    } else if value >= 1000000.0 {
        value /= 1000000.0;
        prefix = "M";
    } else if value >= 1000.0 {
        value /= 1000.0;
        prefix = "k";
    } else {
        prefix = "";
    }
    format!("The resistor value is {:.2} {}Ω", value, prefix)

}

mod res;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, resistor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
