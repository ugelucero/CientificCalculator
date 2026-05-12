//! Motor de parsing de expresiones matemáticas.
//!
//! Este módulo implementa el pipeline completo de evaluación:
//! 1. **Tokenización** (`tokenizer`): string → `Vec<Token>`
//! 2. **Shunting Yard** (`ast`): tokens infijos → RPN como `Vec<AstNode>`
//! 3. **Evaluación** (`evaluator`): nodos RPN → `f64`
//!
//! La función pública principal es [`evaluate`].

pub mod tokenizer;
pub mod ast;
pub mod evaluator;

use crate::models::errors::CalcError;
use crate::models::types::AngleMode;

/// Evalúa una expresión matemática en notación infija.
///
/// Encadena el pipeline completo: tokenize → shunting yard → evaluate RPN.
///
/// # Argumentos
///
/// * `input` - Expresión matemática en notación infija.
/// * `angle_mode` - Modo angular para funciones trigonométricas.
/// * `last_answer` - Último resultado evaluado (para la constante `ans`).
///
/// # Ejemplos
///
/// ```
/// // Desde dentro del crate (no doc-test funcional, solo ilustrativo)
/// // let result = evaluate("2+3*4", AngleMode::Rad, 0.0)?;
/// // assert_eq!(result, 14.0);
/// ```
pub fn evaluate(input: &str, angle_mode: AngleMode, last_answer: f64) -> Result<f64, CalcError> {
    let tokens = tokenizer::tokenize(input)?;
    let rpn = ast::shunting_yard(&tokens)?;
    evaluator::evaluate_ast(&rpn, angle_mode, last_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_simple() {
        let result = evaluate("2+3", AngleMode::Rad, 0.0).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_with_whitespace() {
        let result = evaluate("  2  +  3 * 4 ", AngleMode::Rad, 0.0).unwrap();
        assert!((result - 14.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_power_associativity() {
        // 2^3^2 = 512
        let result = evaluate("2^3^2", AngleMode::Rad, 0.0).unwrap();
        assert!((result - 512.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_parse_error() {
        let result = evaluate("2+", AngleMode::Rad, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_domain_error() {
        let result = evaluate("sqrt(-1)", AngleMode::Rad, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let result = evaluate("1/0", AngleMode::Rad, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_with_ans() {
        let result = evaluate("ans+5", AngleMode::Rad, 42.0).unwrap();
        assert!((result - 47.0).abs() < 1e-10);
    }
}
