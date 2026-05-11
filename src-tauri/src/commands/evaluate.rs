use crate::models::errors::CalcError;
use crate::models::types::{CalcMode, ExpressionResult, FormatType};

/// Evalúa una expresión matemática y retorna el resultado formateado.
/// Es el comando núcleo de la calculadora.
#[tauri::command]
pub fn evaluate_expression(
    expr: String,
    mode: CalcMode,
) -> Result<ExpressionResult, CalcError> {
    // TODO: Implementar en Fase 1 - delegar a parser
    Ok(ExpressionResult {
        result: expr.clone(),
        display: expr,
        format: FormatType::Decimal,
    })
}
