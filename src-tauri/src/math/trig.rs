//! Funciones trigonométricas con soporte para distintos modos angulares.
//! Implementación pendiente para Fase 1-2.

use crate::models::types::AngleMode;

/// Convierte un ángulo al modo especificado a radianes para cálculos internos.
pub fn to_radians(angle: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Deg => angle * std::f64::consts::PI / 180.0,
        AngleMode::Rad => angle,
        AngleMode::Grad => angle * std::f64::consts::PI / 200.0,
    }
}

/// Convierte de radianes al modo angular especificado.
pub fn from_radians(radians: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Deg => radians * 180.0 / std::f64::consts::PI,
        AngleMode::Rad => radians,
        AngleMode::Grad => radians * 200.0 / std::f64::consts::PI,
    }
}
