// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, Runtime};

mod client;
mod commands;
mod roblox_api;
mod tray;
mod types;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(roblox_api::init())
        // tray
        .system_tray(tray::utils::get_system_tray(vec![]))
        .on_system_tray_event(move |app, event| tray::utils::on_system_tray_event(app, event))
        .plugin(tray::init())
        // main init
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
