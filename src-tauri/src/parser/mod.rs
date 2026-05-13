//! Motor de parsing de expresiones matemáticas.
//!
//! Este módulo implementa el pipeline completo de evaluación:
//! 1. **Tokenización** (`tokenizer`): string → `Vec<Token>`
//! 2. **Shunting Yard** (`ast`): tokens infijos → RPN como `Vec<AstNode>`
//! 3. **Evaluación** (`evaluator`): nodos RPN → `f64`
//! 4. **Evaluación compleja** (`complex_eval`): nodos RPN → `Complex`
//!
//! La función pública principal es [`evaluate`].

pub mod tokenizer;
pub mod ast;
pub mod evaluator;
pub mod complex_eval;

use crate::math::complex::Complex;
use crate::models::errors::CalcError;
use crate::models::types::{AngleMode, CalcMode};

/// Evalúa una expresión matemática en notación infija (modo real).
///
/// Encadena el pipeline completo: tokenize → shunting yard → evaluate RPN.
pub fn evaluate(input: &str, angle_mode: AngleMode, last_answer: f64) -> Result<f64, CalcError> {
    let tokens = tokenizer::tokenize(input)?;
    let rpn = ast::shunting_yard(&tokens)?;
    evaluator::evaluate_ast(&rpn, angle_mode, last_answer)
}

/// Evalúa una expresión matemática en notación infija (modo complejo).
///
/// Retorna un `Complex`. Si el resultado no tiene parte imaginaria,
/// se comporta como real.
pub fn evaluate_complex(
    input: &str,
    angle_mode: AngleMode,
    last_answer: f64,
) -> Result<Complex, CalcError> {
    let tokens = tokenizer::tokenize(input)?;
    let rpn = ast::shunting_yard(&tokens)?;
    complex_eval::evaluate_ast_complex(&rpn, angle_mode, last_answer)
}

/// Evalúa una expresión según el modo de la calculadora.
///
/// Devuelve el resultado formateado como string según el modo.
pub fn evaluate_with_mode(
    input: &str,
    angle_mode: AngleMode,
    last_answer: f64,
    calc_mode: &CalcMode,
) -> Result<String, CalcError> {
    match calc_mode {
        CalcMode::Complex => {
            let result = evaluate_complex(input, angle_mode, last_answer)?;
            Ok(result.to_string())
        }
        _ => {
            let result = evaluate(input, angle_mode, last_answer)?;
            Ok(result.to_string())
        }
    }
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
