use std::sync::Mutex;

use crate::models::state::AppState;
use crate::models::types::AngleMode;
use crate::persistence;

/// Almacena un valor en la memoria (sobrescribe el anterior).
#[tauri::command]
pub fn memory_store(
    value: f64,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        s.memory = Some(value);
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}

/// Recupera el valor almacenado en memoria.
/// Devuelve `None` si la memoria está vacía.
#[tauri::command]
pub fn memory_recall(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Option<f64> {
    state.lock().ok()?.memory
}

/// Borra la memoria.
#[tauri::command]
pub fn memory_clear(
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        s.memory = None;
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}

/// Suma un valor al contenido actual de la memoria.
/// Si la memoria está vacía, usa 0 como base.
#[tauri::command]
pub fn memory_add(
    value: f64,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        let current = s.memory.unwrap_or(0.0);
        s.memory = Some(current + value);
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}

/// Resta un valor al contenido actual de la memoria.
/// Si la memoria está vacía, usa 0 como base.
#[tauri::command]
pub fn memory_subtract(
    value: f64,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        let current = s.memory.unwrap_or(0.0);
        s.memory = Some(current - value);
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}

/// Establece el modo angular para funciones trigonométricas.
#[tauri::command]
pub fn set_angle_mode(
    mode: AngleMode,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        s.angle_mode = mode;
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}

/// Configura el número de dígitos decimales para display (0–15).
#[tauri::command]
pub fn set_precision(
    digits: u8,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) {
    if let Ok(mut s) = state.lock() {
        let clamped = digits.clamp(0, 15);
        s.precision = clamped;
        let _ = persistence::state::save_state(&s, &app_handle);
    }
}
