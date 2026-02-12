#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod models;
mod seed_data;

use commands::{get_all_bundles_with_items, get_progress_stats, update_item_status, AppState};
use database::Database;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("bundle-valley.db");

            println!("Database path: {:?}", db_path);

            let db = Database::new(db_path).expect("Failed to initialize database");

            seed_data::seed_database(&db).expect("Failed to seed database");

            app.manage(AppState { db: Mutex::new(db) });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_all_bundles_with_items,
            update_item_status,
            get_progress_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
