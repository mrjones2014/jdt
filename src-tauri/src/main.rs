// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

#[tokio::main]
async fn main() {
    viewmodel_api::init_storage()
        .await
        .expect("Failed to initialize storage directories.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::invoke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
