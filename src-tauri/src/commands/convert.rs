use crate::math;
use crate::models::errors::CalcError;
use crate::models::types::{ConversionRequest, ConversionResult};

/// Convierte un valor entre dos unidades compatibles.
///
/// Recibe un `ConversionRequest` con el valor numérico, la unidad de
/// origen y la unidad de destino (por símbolo o nombre, case-insensitive).
/// Devuelve un `ConversionResult` con el valor convertido y formateado.
///
/// # Errores
///
/// * `InvalidConversion` si alguna unidad no existe.
/// * `InvalidConversion` si las unidades son de distinta categoría.
#[tauri::command]
pub fn convert_units(request: ConversionRequest) -> Result<ConversionResult, CalcError> {
    math::convert::convert(&request)
}
