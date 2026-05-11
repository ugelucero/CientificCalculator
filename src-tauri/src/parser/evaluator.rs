//! Evaluador del AST con contexto de estado (modo angular, precisión, variables).
//! Implementación pendiente para Fase 1.

use crate::models::types::AngleMode;
use crate::parser::ast::AstNode;

/// Contexto de evaluación compartido.
pub struct EvalContext {
    pub angle_mode: AngleMode,
    pub precision: u8,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            angle_mode: AngleMode::Deg,
            precision: 10,
        }
    }
}

/// Evalúa un AST y devuelve el resultado numérico.
pub fn evaluate(_ast: &AstNode, _ctx: &EvalContext) -> Result<f64, crate::models::errors::CalcError> {
    // TODO: Implementar evaluador completo en Fase 1
    Ok(0.0)
}
