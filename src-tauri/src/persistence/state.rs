//! Persistencia del `AppState` a un archivo JSON en el directorio de datos
//! de la aplicación.
//!
//! Usa un único archivo `calculator_state.json`. Si el archivo no existe o
//! está corrupto, se devuelve `AppState::default()`.

use std::fs;
use std::path::PathBuf;

use tauri::Manager;

use crate::models::errors::{CalcError, ErrorKind};
use crate::models::state::AppState;

/// Nombre del archivo de persistencia.
const STATE_FILENAME: &str = "calculator_state.json";

/// Resuelve la ruta completa al archivo de estado dentro del directorio
/// de datos de la aplicación.
fn state_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, CalcError> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| CalcError::new(
            ErrorKind::InternalError,
            format!("No se pudo determinar el directorio de datos de la app: {}", e),
        ))?;

    // Asegurarse de que el directorio exista.
    fs::create_dir_all(&app_data_dir).map_err(|e| CalcError::new(
        ErrorKind::InternalError,
        format!("No se pudo crear el directorio de datos: {}", e),
    ))?;

    Ok(app_data_dir.join(STATE_FILENAME))
}

/// Guarda el estado actual de la aplicación a disco.
///
/// Serializa el `AppState` como JSON y lo escribe en `calculator_state.json`.
/// Si ocurre un error de I/O, se devuelve `CalcError::InternalError`.
pub fn save_state(
    state: &AppState,
    app_handle: &tauri::AppHandle,
) -> Result<(), CalcError> {
    let path = state_path(app_handle)?;

    let json = serde_json::to_string_pretty(state).map_err(|e| {
        CalcError::new(
            ErrorKind::InternalError,
            format!("Error al serializar el estado: {}", e),
        )
    })?;

    fs::write(&path, json).map_err(|e| {
        CalcError::new(
            ErrorKind::InternalError,
            format!("Error al escribir el archivo de estado: {}", e),
        )
    })?;

    Ok(())
}

/// Carga el estado de la aplicación desde disco.
///
/// Si el archivo no existe, o si hay un error de parseo o de I/O,
/// devuelve `AppState::default()` en lugar de propagar el error,
/// ya que es una condición esperable en el primer arranque.
pub fn load_state(app_handle: &tauri::AppHandle) -> AppState {
    let path = match state_path(app_handle) {
        Ok(p) => p,
        Err(_) => return AppState::default(),
    };

    let json = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return AppState::default(),
    };

    serde_json::from_str(&json).unwrap_or_else(|_| AppState::default())
}
