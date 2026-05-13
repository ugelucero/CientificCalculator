//! Evaluador de expresiones en modo complejo.
//!
//! Opera sobre una pila de `Complex` en lugar de `f64`.
//! Las constantes reales se promueven automáticamente a complejo.

use crate::math::complex::{self, Complex};
use crate::models::errors::{CalcError, ErrorKind};
use crate::models::types::AngleMode;
use crate::parser::ast::{AstNode, BinaryOperator, UnaryOperator};

/// Evalúa una secuencia de nodos RPN en modo complejo.
pub fn evaluate_ast_complex(
    nodes: &[AstNode],
    angle_mode: AngleMode,
    last_answer: f64,
) -> Result<Complex, CalcError> {
    let mut stack: Vec<Complex> = Vec::with_capacity(nodes.len());

    for node in nodes {
        match node {
            AstNode::Number(n) => {
                stack.push(Complex::new(*n, 0.0));
            }

            AstNode::Constant(name) => {
                let value = resolve_constant_complex(name, last_answer)?;
                stack.push(value);
            }

            AstNode::BinaryOp { op, .. } => {
                let right = pop_stack(&mut stack, "operando derecho")?;
                let left = pop_stack(&mut stack, "operando izquierdo")?;
                let result = eval_binary_op_complex(op, left, right)?;
                stack.push(result);
            }

            AstNode::UnaryOp { op, .. } => {
                let operand = pop_stack(&mut stack, "operando unario")?;
                let result = eval_unary_op_complex(op, operand)?;
                stack.push(result);
            }

            AstNode::Function { name, args: _ } => {
                let arg = pop_stack(&mut stack, &format!("argumento de {}", name))?;
                let result = eval_function_complex(name, arg, angle_mode)?;
                stack.push(result);
            }
        }
    }

    if stack.len() != 1 {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            format!(
                "Expresión inválida: la pila de evaluación contiene {} valores (esperaba 1)",
                stack.len()
            ),
        ));
    }

    Ok(stack[0])
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn pop_stack(stack: &mut Vec<Complex>, context: &str) -> Result<Complex, CalcError> {
    stack.pop().ok_or_else(|| {
        CalcError::new(
            ErrorKind::ParseError,
            format!("Faltan operandos en la expresión ({})", context),
        )
    })
}

fn resolve_constant_complex(name: &str, last_answer: f64) -> Result<Complex, CalcError> {
    match name {
        "pi" => Ok(Complex::new(std::f64::consts::PI, 0.0)),
        "e" => Ok(Complex::new(std::f64::consts::E, 0.0)),
        "ans" => Ok(Complex::new(last_answer, 0.0)),
        "i" => Ok(Complex::I),
        _ => Err(CalcError::new(
            ErrorKind::UndefinedVariable,
            format!("Constante desconocida: '{}'", name),
        )),
    }
}

fn eval_binary_op_complex(
    op: &BinaryOperator,
    left: Complex,
    right: Complex,
) -> Result<Complex, CalcError> {
    match op {
        BinaryOperator::Add => Ok(left + right),
        BinaryOperator::Sub => Ok(left - right),
        BinaryOperator::Mul => Ok(left * right),
        BinaryOperator::Div => {
            if right.real == 0.0 && right.imag == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DivisionByZero,
                    "División por cero".to_string(),
                ))
            } else {
                Ok(left / right)
            }
        }
        BinaryOperator::Pow => Ok(complex::cpow(left, right)),
        BinaryOperator::Mod => {
            Err(CalcError::new(
                ErrorKind::DomainError,
                "El operador módulo (%) no está definido para números complejos".to_string(),
            ))
        }
        BinaryOperator::BitAnd
        | BinaryOperator::BitOr
        | BinaryOperator::BitXor
        | BinaryOperator::Shl
        | BinaryOperator::Shr => {
            Err(CalcError::new(
                ErrorKind::DomainError,
                "Los operadores bitwise no están disponibles en modo Complejo".to_string(),
            ))
        }
    }
}

fn eval_unary_op_complex(op: &UnaryOperator, operand: Complex) -> Result<Complex, CalcError> {
    match op {
        UnaryOperator::Negate => Ok(-operand),
        UnaryOperator::Factorial => Err(CalcError::new(
            ErrorKind::DomainError,
            "Factorial no implementado en modo Complejo".to_string(),
        )),
        UnaryOperator::BitNot => Err(CalcError::new(
            ErrorKind::DomainError,
            "El operador NOT bitwise no está disponible en modo Complejo".to_string(),
        )),
    }
}

fn eval_function_complex(
    name: &str,
    arg: Complex,
    angle_mode: AngleMode,
) -> Result<Complex, CalcError> {
    match name {
        "sin" => {
            let z = to_radians_complex(arg, angle_mode);
            Ok(complex::csin(z))
        }
        "cos" => {
            let z = to_radians_complex(arg, angle_mode);
            Ok(complex::ccos(z))
        }
        "tan" => {
            let z = to_radians_complex(arg, angle_mode);
            let cosz = complex::ccos(z);
            if cosz.real.abs() < 1e-15 && cosz.imag.abs() < 1e-15 {
                Err(CalcError::new(
                    ErrorKind::DomainError,
                    "Tangente no definida (cos ≈ 0, asíntota)".to_string(),
                ))
            } else {
                Ok(complex::csin(z) / cosz)
            }
        }
        "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" => {
            Err(CalcError::new(
                ErrorKind::DomainError,
                format!("La función '{}' no está disponible en modo Complejo", name),
            ))
        }
        "ln" => {
            if arg.real == 0.0 && arg.imag == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DomainError,
                    "Logaritmo de cero no está definido".to_string(),
                ))
            } else {
                Ok(complex::cln(arg))
            }
        }
        "log" | "log10" => {
            if arg.real == 0.0 && arg.imag == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DomainError,
                    "Logaritmo de cero no está definido".to_string(),
                ))
            } else {
                Ok(complex::clog10(arg))
            }
        }
        "log2" => {
            if arg.real == 0.0 && arg.imag == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DomainError,
                    "Logaritmo de cero no está definido".to_string(),
                ))
            } else {
                Ok(complex::clog2(arg))
            }
        }
        "sqrt" => Ok(complex::csqrt(arg)),
        "cbrt" => Ok(complex::cpow(arg, Complex::new(1.0 / 3.0, 0.0))),
        "abs" => Ok(Complex::new(arg.abs(), 0.0)),
        "exp" => Ok(complex::cexp(arg)),
        "real" => Ok(Complex::new(arg.real, 0.0)),
        "imag" => Ok(Complex::new(arg.imag, 0.0)),
        "conj" => Ok(arg.conj()),
        "arg" => Ok(Complex::new(arg.arg(), 0.0)),
        _ => Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Función desconocida: '{}'", name),
        )),
    }
}

fn to_radians_complex(angle: Complex, mode: AngleMode) -> Complex {
    match mode {
        AngleMode::Rad => angle,
        AngleMode::Deg => Complex::new(
            angle.real * std::f64::consts::PI / 180.0,
            angle.imag * std::f64::consts::PI / 180.0,
        ),
        AngleMode::Grad => Complex::new(
            angle.real * std::f64::consts::PI / 200.0,
            angle.imag * std::f64::consts::PI / 200.0,
        ),
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::shunting_yard;
    use crate::parser::tokenizer::tokenize;

    fn eval_complex(expr: &str, angle_mode: AngleMode) -> Result<Complex, CalcError> {
        let tokens = tokenize(expr)?;
        let nodes = shunting_yard(&tokens)?;
        evaluate_ast_complex(&nodes, angle_mode, 0.0)
    }

    fn assert_complex_eq(a: Complex, b: Complex, tolerance: f64) {
        assert!(
            (a.real - b.real).abs() < tolerance && (a.imag - b.imag).abs() < tolerance,
            "Expected {:?}, got {:?}",
            b,
            a,
        );
    }

    #[test]
    fn test_complex_addition() {
        // (3+4i) + (1+2i) = 4+6i
        let z = eval_complex("(3+4i)+(1+2i)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(4.0, 6.0), 1e-10);
    }

    #[test]
    fn test_complex_multiplication() {
        // (1+i)² = 2i
        let z = eval_complex("(1+i)*(1+i)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(0.0, 2.0), 1e-10);
    }

    #[test]
    fn test_i_squared_eval() {
        let z = eval_complex("i*i", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(-1.0, 0.0), 1e-10);
    }

    #[test]
    fn test_real_addition_promotes() {
        let z = eval_complex("2+3", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(5.0, 0.0), 1e-10);
    }

    #[test]
    fn test_sin_complex() {
        let z = eval_complex("sin(0)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::ZERO, 1e-10);
    }

    #[test]
    fn test_cos_complex() {
        let z = eval_complex("cos(0)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::ONE, 1e-10);
    }

    #[test]
    fn test_exp_complex() {
        let z = eval_complex("exp(0)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::ONE, 1e-10);
    }

    #[test]
    fn test_ln_complex() {
        let z = eval_complex("ln(e)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::ONE, 1e-10);
    }

    #[test]
    fn test_sqrt_negative() {
        let z = eval_complex("sqrt(-1)", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::I, 1e-10);
    }

    #[test]
    fn test_implicit_multiplication() {
        let z = eval_complex("4i", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(0.0, 4.0), 1e-10);
    }

    #[test]
    fn test_complex_division() {
        let z = eval_complex("(2+2i)/2", AngleMode::Rad).unwrap();
        assert_complex_eq(z, Complex::new(1.0, 1.0), 1e-10);
    }

    #[test]
    fn test_conj_function() {
        let z = Complex::new(3.0, 4.0).conj();
        assert_complex_eq(z, Complex::new(3.0, -4.0), 1e-10);
    }

    #[test]
    fn test_abs_function() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.abs() - 5.0).abs() < 1e-10);
    }
}
