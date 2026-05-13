use serde::{Deserialize, Serialize};

use crate::models::types::{AngleMode, HistoryEntry};

/// Estado completo de la aplicación que persiste entre sesiones.
///
/// Se guarda como un único archivo JSON en el directorio de datos de la app.
/// Se accede desde los comandos Tauri mediante `tauri::State<Mutex<AppState>>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    /// Valor almacenado en memoria (teclas M+, M-, MR, MC).
    /// `None` si la memoria está vacía.
    pub memory: Option<f64>,

    /// Historial de evaluaciones (máximo 100 entradas).
    pub history: Vec<HistoryEntry>,

    /// Modo angular para funciones trigonométricas.
    pub angle_mode: AngleMode,

    /// Precisión decimal para display de resultados (0–15).
    pub precision: u8,

    /// Último resultado evaluado (para la constante `ans`).
    pub last_answer: f64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            memory: None,
            history: Vec::new(),
            angle_mode: AngleMode::Rad,
            precision: 10,
            last_answer: 0.0,
        }
    }
}
