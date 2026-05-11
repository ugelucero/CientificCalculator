use serde::{Deserialize, Serialize};

/// Clasificación del tipo de error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ErrorKind {
    DivisionByZero,
    DomainError,
    ParseError,
    Overflow,
    InvalidMode,
    DimensionMismatch,
    UndefinedVariable,
    InternalError,
}

/// Error estructurado de la calculadora.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalcError {
    pub kind: ErrorKind,
    pub message: String,
    pub position: Option<usize>,
}

impl CalcError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            position: None,
        }
    }

    pub fn with_position(mut self, position: usize) -> Self {
        self.position = Some(position);
        self
    }
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CalcError {}
