#![deny(clippy::all, clippy::pedantic, rust_2018_idioms, clippy::unwrap_used)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

#[tokio::main]
async fn main() {
    viewmodel_api::init_storage()
        .await
        .expect("Failed to initialize storage directories.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::get_repositories_view_model,
            api::add_repository,
            api::delete_resource,
            api::update_repo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
