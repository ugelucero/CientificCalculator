use crate::models::types::HistoryEntry;

/// Obtiene el historial de evaluaciones ordenado por timestamp descendente.
#[tauri::command]
pub fn get_history() -> Vec<HistoryEntry> {
    // TODO: Implementar persistencia en Fase 2
    vec![]
}

/// Elimina todas las entradas del historial.
#[tauri::command]
pub fn clear_history() {
    // TODO: Implementar persistencia en Fase 2
}
