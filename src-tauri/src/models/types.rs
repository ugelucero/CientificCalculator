use serde::{Deserialize, Serialize};

/// Modo de la calculadora: determina qué funciones y operadores están disponibles.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CalcMode {
    Standard,
    Scientific,
    Programmer,
    Statistics,
    Complex,
    Matrix,
}

/// Modo angular para funciones trigonométricas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AngleMode {
    Deg,
    Rad,
    Grad,
}

/// Formato de presentación del resultado.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FormatType {
    Decimal,
    Scientific,
    Fraction,
    Hex,
    Oct,
    Bin,
    Complex,
}

/// Resultado de la evaluación de una expresión.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionResult {
    pub result: String,
    pub display: String,
    pub format: FormatType,
}

/// Entrada del historial de evaluaciones.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub expression: String,
    pub result: String,
    pub timestamp: String,
    pub mode: CalcMode,
}

/// Categoría de unidad para conversiones.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UnitCategory {
    Length,
    Mass,
    Time,
    Temperature,
    Speed,
    Area,
    Volume,
    Energy,
    Pressure,
    Data,
    Angle,
}

/// Unidad de medida para conversiones (legado, mantenido por compatibilidad).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub category: UnitCategory,
    pub name: String,
    pub symbol: String,
    pub to_base_factor: f64,
}

/// Información completa de una unidad para el registro de conversiones.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnitInfo {
    pub name: String,
    pub symbol: String,
    pub category: UnitCategory,
    /// Factor multiplicativo hacia la unidad SI base.
    pub to_si: f64,
    /// Desplazamiento (ej: Celsius → Kelvin: +273.15).
    pub offset: f64,
    /// false para temperatura (no lineal), true para el resto.
    pub is_linear: bool,
}

/// Petición de conversión de unidades.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRequest {
    pub value: f64,
    pub from_unit: String,
    pub to_unit: String,
}

/// Resultado de una conversión de unidades.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionResult {
    pub value: f64,
    pub formatted: String,
    pub from: String,
    pub to: String,
}

/// Matriz para operaciones de álgebra lineal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

/// Número complejo.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

/// Paso individual del método de Newton-Raphson.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewtonStep {
    pub n: u32,
    pub x_n: f64,
    pub f_x_n: f64,
}

/// Resultado completo del método de Newton-Raphson.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewtonResult {
    pub root: f64,
    pub f_root: f64,
    pub iterations: u32,
    pub converged: bool,
    pub error: Option<String>,
    pub steps: Vec<NewtonStep>,
}
