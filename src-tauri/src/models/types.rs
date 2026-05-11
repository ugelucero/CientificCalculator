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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Unidad de medida para conversiones.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub category: UnitCategory,
    pub name: String,
    pub symbol: String,
    pub to_base_factor: f64,
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
