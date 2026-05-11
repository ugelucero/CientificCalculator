//! Tokenizador de expresiones matemáticas.
//! Implementación pendiente para Fase 1.

/// Tokens que componen una expresión matemática.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    // Funciones científicas
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Sqrt,
    // Constantes
    Pi,
    E,
    // Variables y otros
    Identifier(String),
    Comma,
}

/// Convierte una cadena de expresión en un vector de tokens.
pub fn tokenize(_expr: &str) -> Result<Vec<Token>, crate::models::errors::CalcError> {
    // TODO: Implementar tokenizador completo en Fase 1
    Ok(vec![])
}
