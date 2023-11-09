// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod api;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args[0] == "--gen-types" {
        tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                api::get_repositories_view_model,
                api::add_repository
            ])
            .path("./src/api.ts") // RUN FROM REPO ROOT
            .export()
            .expect("Failed to generate types!");
        return;
    } else {
        println!("{:?}", args);
    }
    viewmodel_api::init_storage()
        .await
        .expect("Failed to initialize storage directories.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::get_repositories_view_model,
            api::add_repository
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
