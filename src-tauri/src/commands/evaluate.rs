use crate::models::errors::CalcError;
use crate::models::types::{AngleMode, CalcMode, ExpressionResult, FormatType};
use crate::parser;

/// Evalúa una expresión matemática y retorna el resultado formateado.
/// Es el comando núcleo de la calculadora.
#[tauri::command]
pub fn evaluate_expression(
    expr: String,
    mode: CalcMode,
) -> Result<ExpressionResult, CalcError> {
    // Por ahora usamos Rad como modo angular por defecto.
    // En Fase 2 se usará el AppState global para obtener el angle_mode real.
    let angle_mode = match mode {
        CalcMode::Standard | CalcMode::Scientific | CalcMode::Statistics
        | CalcMode::Complex | CalcMode::Matrix => AngleMode::Rad,
        CalcMode::Programmer => AngleMode::Rad, // En modo programador no aplica trigonometría
    };

    let result = parser::evaluate(&expr, angle_mode)?;

    Ok(ExpressionResult {
        result: format_result(result),
        display: expr,
        format: FormatType::Decimal,
    })
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
