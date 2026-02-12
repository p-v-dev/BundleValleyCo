use crate::database::Database;
use crate::models::{Bundle, ProgressStats};
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub db: Mutex<Database>,
}

#[tauri::command]
pub fn get_all_bundles_with_items(state: State<AppState>) -> Result<Vec<Bundle>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_all_bundles_with_items().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_item_status(
    state: State<AppState>,
    item_id: String,
    status: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_item_status(&item_id, &status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_progress_stats(state: State<AppState>) -> Result<ProgressStats, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_progress_stats().map_err(|e| e.to_string())
}
