/// Almacena un valor en la memoria (sobrescribe el anterior).
#[tauri::command]
pub fn memory_store(value: f64) {
    // TODO: Implementar con AppState en Fase 2
    let _ = value;
}

/// Recupera el valor almacenado en memoria.
#[tauri::command]
pub fn memory_recall() -> Option<f64> {
    // TODO: Implementar con AppState en Fase 2
    None
}

/// Borra la memoria.
#[tauri::command]
pub fn memory_clear() {
    // TODO: Implementar con AppState en Fase 2
}

/// Suma un valor al contenido actual de la memoria.
#[tauri::command]
pub fn memory_add(value: f64) {
    // TODO: Implementar con AppState en Fase 2
    let _ = value;
}

/// Resta un valor al contenido actual de la memoria.
#[tauri::command]
pub fn memory_subtract(value: f64) {
    // TODO: Implementar con AppState en Fase 2
    let _ = value;
}

/// Establece el modo angular para funciones trigonométricas.
#[tauri::command]
pub fn set_angle_mode(mode: crate::models::types::AngleMode) {
    // TODO: Implementar con AppState en Fase 2
    let _ = mode;
}

/// Configura el número de dígitos decimales para display.
#[tauri::command]
pub fn set_precision(digits: u8) {
    // TODO: Implementar con AppState en Fase 2
    let _ = digits;
}
