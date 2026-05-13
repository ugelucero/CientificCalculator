use std::sync::Mutex;

use crate::models::state::AppState;
use crate::models::types::HistoryEntry;
use crate::persistence;

/// Obtiene el historial de evaluaciones ordenado por timestamp descendente.
/// Máximo 100 entradas.
#[tauri::command]
pub fn get_history(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Vec<HistoryEntry> {
    state
        .lock()
        .ok()
        .map(|s| s.history.clone())
        .unwrap_or_default()
}

/// Elimina todas las entradas del historial.
#[tauri::command]
pub fn clear_history(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        s.history.clear();
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}
