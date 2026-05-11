use crate::models::errors::CalcError;
use crate::models::types::Unit;

/// Convierte un valor entre dos unidades compatibles.
#[tauri::command]
pub fn convert_units(
    value: f64,
    from: Unit,
    to: Unit,
) -> Result<f64, CalcError> {
    // TODO: Implementar en Fase 2
    // Verificar que las categorías coincidan
    if from.category != to.category {
        return Err(CalcError::new(
            crate::models::errors::ErrorKind::DomainError,
            format!(
                "Cannot convert from {:?} to {:?}: incompatible categories",
                from.category, to.category
            ),
        ));
    }

    // Fórmula de conversión lineal: valor * factor_from / factor_to
    let result = value * from.to_base_factor / to.to_base_factor;
    Ok(result)
}
