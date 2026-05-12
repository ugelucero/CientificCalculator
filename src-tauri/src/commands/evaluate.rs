use std::sync::Mutex;

use crate::models::errors::CalcError;
use crate::models::state::AppState;
use crate::models::types::{CalcMode, ExpressionResult, FormatType, HistoryEntry};
use crate::parser;
use crate::persistence;

/// Evalúa una expresión matemática y retorna el resultado formateado.
/// Es el comando núcleo de la calculadora.
///
/// Efectos secundarios:
/// - Actualiza `last_answer` en el estado.
/// - Agrega una entrada al historial.
/// - Persiste el estado a disco.
#[tauri::command]
pub fn evaluate_expression(
    expr: String,
    mode: CalcMode,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<ExpressionResult, CalcError> {
    // Obtener angle_mode y last_answer del estado.
    let (angle_mode, last_answer) = {
        let s = state
            .lock()
            .map_err(|e| CalcError::new(
                crate::models::errors::ErrorKind::InternalError,
                format!("Error al acceder al estado: {}", e),
            ))?;
        (s.angle_mode, s.last_answer)
    };

    // Evaluar la expresión pasando el last_answer actual para `ans`.
    let result = parser::evaluate(&expr, angle_mode, last_answer)?;

    let formatted = format_result_for_mode(result, &mode);

    // Generar ID único y timestamp para la entrada de historial.
    let history_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let entry = HistoryEntry {
        id: history_id,
        expression: expr.clone(),
        result: formatted.clone(),
        timestamp,
        mode: mode.clone(),
    };

    // Actualizar el estado: last_answer, historial, y persistir.
    {
        let mut s = state
            .lock()
            .map_err(|e| CalcError::new(
                crate::models::errors::ErrorKind::InternalError,
                format!("Error al acceder al estado: {}", e),
            ))?;

        s.last_answer = result;
        s.history.insert(0, entry);

        // Limitar el historial a 100 entradas.
        s.history.truncate(100);

        // Persistir el estado a disco (ignorar error de persistencia, no es crítico).
        let _ = persistence::state::save_state(&s, &app_handle);
    }

    Ok(ExpressionResult {
        result: formatted,
        display: expr,
        format: FormatType::Decimal,
    })
}

/// Formatea el resultado según el modo de la calculadora.
fn format_result_for_mode(value: f64, mode: &CalcMode) -> String {
    match mode {
        CalcMode::Programmer => format_programmer_result(value),
        _ => format_result(value),
    }
}

/// Formatea el resultado para modo Programador: muestra el valor en las 4 bases.
fn format_programmer_result(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };
    let int_val = value as i64;
    let u32_val = int_val as u32;
    let dec_str = format!("{}", int_val);
    let hex_str = format!("0x{:X}", u32_val);
    let oct_str = format!("0o{:o}", u32_val);
    let bin_str = format!("0b{:b}", u32_val);
    format!("{}  |  {}  |  {}  |  {}", hex_str, dec_str, oct_str, bin_str)
}

/// Formatea el resultado numérico como string.
///
/// - Si es entero sin parte decimal, se muestra sin `.0`.
/// - Si tiene parte decimal, se muestra con precisión completa evitando
///   notación científica para números cotidianos.
fn format_result(value: f64) -> String {
    // Evitar -0.0
    let value = if value == 0.0 { 0.0 } else { value };

    // Si es NaN o infinito, devolver representación directa
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "∞" } else { "-∞" }.to_string();
    }

    // Para números muy grandes o muy pequeños usar notación científica
    let abs = value.abs();
    if abs >= 1e15 || (abs < 1e-10 && abs > 0.0) {
        return format!("{:e}", value);
    }

    // Formatear con hasta 15 dígitos significativos
    let formatted = format!("{:.15}", value);

    // Eliminar ceros finales después del punto decimal
    let formatted = formatted.trim_end_matches('0');

    // Si el último carácter es '.', eliminarlo (número entero)
    let formatted = formatted.trim_end_matches('.');

    formatted.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_integer() {
        assert_eq!(format_result(42.0), "42");
    }

    #[test]
    fn test_format_decimal() {
        let result = format_result(3.14159);
        assert!(result.starts_with("3.14159"));
    }

    #[test]
    fn test_format_negative_zero() {
        assert_eq!(format_result(-0.0), "0");
    }

    #[test]
    fn test_format_scientific_large() {
        let result = format_result(1e20);
        assert!(result.contains('e'));
    }
}
