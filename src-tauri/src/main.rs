// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::{list_serial_ports, start_serial_reading};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            start_serial_reading
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
