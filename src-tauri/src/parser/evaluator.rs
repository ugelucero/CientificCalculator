//! Evaluador de expresiones en notación polaca inversa (RPN).
//!
//! Recorre una secuencia de `AstNode` en orden RPN y evalúa usando una pila
//! de valores `f64`. Soporta modo angular configurable para funciones
//! trigonométricas.

use std::f64::consts;

use crate::models::errors::{CalcError, ErrorKind};
use crate::models::types::AngleMode;
use crate::parser::ast::{AstNode, BinaryOperator, UnaryOperator};

/// Evalúa una secuencia de nodos RPN y devuelve el resultado numérico.
///
/// # Argumentos
///
/// * `nodes` - Secuencia de `AstNode` en orden RPN (salida del shunting yard).
/// * `angle_mode` - Modo angular para funciones trigonométricas.
/// * `last_answer` - Último resultado evaluado, usado para la constante `ans`.
///
/// # Errores
///
/// * `DivisionByZero` - División o módulo por cero.
/// * `DomainError` - Argumento fuera del dominio (ej: sqrt(-1), log(0)).
/// * `ParseError` - Secuencia RPN inválida (pila desbalanceada).
pub fn evaluate_ast(
    nodes: &[AstNode],
    angle_mode: AngleMode,
    last_answer: f64,
) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::with_capacity(nodes.len());

    for node in nodes {
        match node {
            AstNode::Number(n) => {
                stack.push(*n);
            }

            AstNode::Constant(name) => {
                let value = resolve_constant(name, last_answer)?;
                stack.push(value);
            }

            AstNode::BinaryOp { op, .. } => {
                let right = pop_stack(&mut stack, "operando derecho")?;
                let left = pop_stack(&mut stack, "operando izquierdo")?;
                let result = eval_binary_op(op, left, right)?;
                stack.push(result);
            }

            AstNode::UnaryOp { op, .. } => {
                let operand = pop_stack(&mut stack, "operando unario")?;
                let result = eval_unary_op(op, operand)?;
                stack.push(result);
            }

            AstNode::Function { name, args: _ } => {
                // Todas las funciones actuales son unarias (1 argumento).
                // Si en el futuro hay multi-argumento, se usará args.len() o
                // un marcador en RPN.
                let arg = pop_stack(&mut stack, &format!("argumento de {}", name))?;
                let result = eval_function(name, arg, angle_mode)?;
                stack.push(result);
            }
        }
    }

    // Al final debe quedar exactamente un valor en la pila.
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

/// Extrae un valor de la pila, o devuelve un error descriptivo si está vacía.
fn pop_stack(stack: &mut Vec<f64>, context: &str) -> Result<f64, CalcError> {
    stack.pop().ok_or_else(|| {
        CalcError::new(
            ErrorKind::ParseError,
            format!("Faltan operandos en la expresión ({})", context),
        )
    })
}

/// Resuelve una constante por nombre.
fn resolve_constant(name: &str, last_answer: f64) -> Result<f64, CalcError> {
    match name {
        "pi" => Ok(consts::PI),
        "e" => Ok(consts::E),
        "ans" => Ok(last_answer),
        "i" => Err(CalcError::new(
            ErrorKind::DomainError,
            "La unidad imaginaria 'i' solo está disponible en modo Complejo".to_string(),
        )),
        _ => Err(CalcError::new(
            ErrorKind::UndefinedVariable,
            format!("Constante desconocida: '{}'", name),
        )),
    }
}

/// Evalúa un operador binario.
fn eval_binary_op(op: &BinaryOperator, left: f64, right: f64) -> Result<f64, CalcError> {
    match op {
        BinaryOperator::Add => Ok(left + right),
        BinaryOperator::Sub => Ok(left - right),
        BinaryOperator::Mul => Ok(left * right),
        BinaryOperator::Div => {
            if right == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DivisionByZero,
                    "División por cero".to_string(),
                ))
            } else {
                Ok(left / right)
            }
        }
        BinaryOperator::Mod => {
            if right == 0.0 {
                Err(CalcError::new(
                    ErrorKind::DivisionByZero,
                    "Módulo por cero".to_string(),
                ))
            } else {
                Ok(left % right)
            }
        }
        BinaryOperator::Pow => Ok(left.powf(right)),
        BinaryOperator::BitAnd => Ok(((left as i64) & (right as i64)) as f64),
        BinaryOperator::BitOr => Ok(((left as i64) | (right as i64)) as f64),
        BinaryOperator::BitXor => Ok(((left as i64) ^ (right as i64)) as f64),
        BinaryOperator::Shl => {
            let shift = right as u32;
            Ok(((left as i64) << shift) as f64)
        }
        BinaryOperator::Shr => {
            let shift = right as u32;
            Ok(((left as i64) >> shift) as f64)
        }
    }
}

/// Evalúa un operador unario.
fn eval_unary_op(op: &UnaryOperator, operand: f64) -> Result<f64, CalcError> {
    match op {
        UnaryOperator::Negate => Ok(-operand),
        UnaryOperator::Factorial => factorial(operand),
        UnaryOperator::BitNot => Ok(!(operand as i64) as f64),
    }
}

/// Calcula el factorial de un número.
/// Para enteros no negativos usa el producto iterativo.
/// Para no enteros usa la función Gamma: Γ(n+1) = n!
fn factorial(n: f64) -> Result<f64, CalcError> {
    if n < 0.0 && n.fract() != 0.0 {
        // Número negativo no entero: gamma no está definida en polos negativos.
        return Err(CalcError::new(
            ErrorKind::DomainError,
            "Factorial no definido para números negativos no enteros".to_string(),
        ));
    }

    // Si es un entero no negativo razonablemente pequeño, usar producto iterativo.
    if n >= 0.0 && n.fract() == 0.0 && n <= 170.0 {
        let n_int = n as u64;
        let mut result = 1.0f64;
        for i in 2..=n_int {
            result *= i as f64;
            if result.is_infinite() {
                return Err(CalcError::new(
                    ErrorKind::Overflow,
                    "Factorial produce desbordamiento".to_string(),
                ));
            }
        }
        return Ok(result);
    }

    // Para valores grandes o no enteros, usar la aproximación de Stirling
    // o la función gamma (ln_gamma).
    if n > 0.0 {
        Ok(gamma_approx(n + 1.0))
    } else if n == 0.0 {
        Ok(1.0)
    } else {
        // n < 0 entero: factorial de negativo no está definido (polos).
        Err(CalcError::new(
            ErrorKind::DomainError,
            "Factorial no definido para enteros negativos".to_string(),
        ))
    }
}

/// Aproximación de la función Gamma usando la fórmula de Stirling.
/// Gamma(x) ≈ √(2π/x) * (x/e)^x
fn gamma_approx(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    // Stirling: ln Γ(x) ≈ (x - 0.5)*ln(x) - x + 0.5*ln(2π)
    // Para precisión razonable usamos la aproximación de Lanczos simplificada.
    // Usamos la serie de Stirling con algunos términos.
    let sqrt_2pi = 2.5066282746310002; // √(2π)
    let e = consts::E;

    // Stirling básica: Γ(x) ≈ √(2π/x) * (x/e)^x
    let stirling = (x / e).powf(x) * (sqrt_2pi / x).sqrt();

    // Corrección: Γ(x) = Stirling(x) * (1 + 1/(12x) + 1/(288x²) - ...)
    let correction = 1.0 + 1.0 / (12.0 * x) + 1.0 / (288.0 * x * x)
        - 139.0 / (51840.0 * x * x * x)
        - 571.0 / (2488320.0 * x * x * x * x);

    stirling * correction
}

// ─── Funciones ──────────────────────────────────────────────────────────────

/// Evalúa una función matemática por nombre.
fn eval_function(name: &str, arg: f64, angle_mode: AngleMode) -> Result<f64, CalcError> {
    match name {
        "sin" => Ok(sin(arg, angle_mode)),
        "cos" => Ok(cos(arg, angle_mode)),
        "tan" => tan(arg, angle_mode),
        "asin" => asin(arg, angle_mode),
        "acos" => acos(arg, angle_mode),
        "atan" => Ok(atan(arg, angle_mode)),
        "sinh" => Ok(arg.sinh()),
        "cosh" => Ok(arg.cosh()),
        "tanh" => Ok(arg.tanh()),
        "ln" => ln(arg),
        "log" => log10(arg), // log == log10 por convención
        "log10" => log10(arg),
        "log2" => log2(arg),
        "sqrt" => sqrt(arg),
        "cbrt" => Ok(arg.cbrt()),
        "abs" => Ok(arg.abs()),
        "exp" => Ok(arg.exp()),
        "real" => Ok(arg),
        "imag" => Ok(0.0),
        "conj" => Ok(arg),
        "arg" => {
            if arg >= 0.0 {
                Ok(0.0)
            } else {
                Ok(std::f64::consts::PI)
            }
        }
        _ => Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Función desconocida: '{}'", name),
        )),
    }
}

// ─── Conversión de ángulos ──────────────────────────────────────────────────

/// Convierte un ángulo en el modo dado a radianes (para funciones directas).
fn to_radians(angle: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Deg => angle * consts::PI / 180.0,
        AngleMode::Rad => angle,
        AngleMode::Grad => angle * consts::PI / 200.0,
    }
}

/// Convierte radianes al modo angular dado (para funciones inversas).
fn from_radians(radians: f64, mode: AngleMode) -> f64 {
    match mode {
        AngleMode::Deg => radians * 180.0 / consts::PI,
        AngleMode::Rad => radians,
        AngleMode::Grad => radians * 200.0 / consts::PI,
    }
}

// ─── Trigonométricas directas ───────────────────────────────────────────────

fn sin(arg: f64, mode: AngleMode) -> f64 {
    to_radians(arg, mode).sin()
}

fn cos(arg: f64, mode: AngleMode) -> f64 {
    to_radians(arg, mode).cos()
}

fn tan(arg: f64, mode: AngleMode) -> Result<f64, CalcError> {
    let rad = to_radians(arg, mode);
    let cos_val = rad.cos();
    // tan = sin/cos. Si cos ≈ 0, hay asíntota → error de dominio.
    if cos_val.abs() < 1e-15 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            "Tangente no definida (cos ≈ 0, asíntota)".to_string(),
        ))
    } else {
        Ok(rad.sin() / cos_val)
    }
}

// ─── Trigonométricas inversas ───────────────────────────────────────────────

fn asin(arg: f64, mode: AngleMode) -> Result<f64, CalcError> {
    if arg < -1.0 || arg > 1.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!(
                "arcoseno requiere argumento en [-1, 1], recibido: {}",
                arg
            ),
        ))
    } else {
        Ok(from_radians(arg.asin(), mode))
    }
}

fn acos(arg: f64, mode: AngleMode) -> Result<f64, CalcError> {
    if arg < -1.0 || arg > 1.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!(
                "arcocoseno requiere argumento en [-1, 1], recibido: {}",
                arg
            ),
        ))
    } else {
        Ok(from_radians(arg.acos(), mode))
    }
}

fn atan(arg: f64, mode: AngleMode) -> f64 {
    from_radians(arg.atan(), mode)
}

// ─── Logaritmos ─────────────────────────────────────────────────────────────

fn ln(arg: f64) -> Result<f64, CalcError> {
    if arg <= 0.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!("Logaritmo natural requiere argumento > 0, recibido: {}", arg),
        ))
    } else {
        Ok(arg.ln())
    }
}

fn log10(arg: f64) -> Result<f64, CalcError> {
    if arg <= 0.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!(
                "Logaritmo base 10 requiere argumento > 0, recibido: {}",
                arg
            ),
        ))
    } else {
        Ok(arg.log10())
    }
}

fn log2(arg: f64) -> Result<f64, CalcError> {
    if arg <= 0.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!(
                "Logaritmo base 2 requiere argumento > 0, recibido: {}",
                arg
            ),
        ))
    } else {
        Ok(arg.log2())
    }
}

// ─── Raíz cuadrada ──────────────────────────────────────────────────────────

fn sqrt(arg: f64) -> Result<f64, CalcError> {
    if arg < 0.0 {
        Err(CalcError::new(
            ErrorKind::DomainError,
            format!(
                "Raíz cuadrada requiere argumento ≥ 0, recibido: {}",
                arg
            ),
        ))
    } else {
        Ok(arg.sqrt())
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: evalúa una expresión completa (tokenize → shunting_yard → evaluate_ast).
    fn eval(expr: &str, angle_mode: AngleMode) -> Result<f64, CalcError> {
        use crate::parser::ast::shunting_yard;
        use crate::parser::tokenizer::tokenize;
        let tokens = tokenize(expr)?;
        let nodes = shunting_yard(&tokens)?;
        evaluate_ast(&nodes, angle_mode, 0.0)
    }

    // ─── Aritmética básica ──────────────────────────────────────────

    #[test]
    fn test_addition() {
        assert!((eval("2+3", AngleMode::Rad).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_subtraction() {
        assert!((eval("10-3", AngleMode::Rad).unwrap() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiplication() {
        assert!((eval("4*5", AngleMode::Rad).unwrap() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_division() {
        assert!((eval("15/3", AngleMode::Rad).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_modulo() {
        assert!((eval("10%3", AngleMode::Rad).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_precedence() {
        // 2+3*4 = 14
        assert!((eval("2+3*4", AngleMode::Rad).unwrap() - 14.0).abs() < 1e-10);
    }

    #[test]
    fn test_parentheses_precedence() {
        // (2+3)*4 = 20
        assert!((eval("(2+3)*4", AngleMode::Rad).unwrap() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_nested_parentheses() {
        // ((2+3)*(4+5)) = 45
        assert!((eval("((2+3)*(4+5))", AngleMode::Rad).unwrap() - 45.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_numbers() {
        assert!((eval("-5+3", AngleMode::Rad).unwrap() - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_scientific_notation() {
        assert!((eval("1e10", AngleMode::Rad).unwrap() - 1e10).abs() < 1e-10);
        assert!((eval("1.5e-3", AngleMode::Rad).unwrap() - 0.0015).abs() < 1e-10);
    }

    #[test]
    fn test_division_by_zero() {
        let result = eval("1/0", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DivisionByZero);
    }

    #[test]
    fn test_modulo_by_zero() {
        let result = eval("5%0", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DivisionByZero);
    }

    // ─── Potenciación ────────────────────────────────────────────────

    #[test]
    fn test_power() {
        assert!((eval("2^10", AngleMode::Rad).unwrap() - 1024.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_right_associative() {
        // 2^3^2 = 2^(3^2) = 2^9 = 512 (no (2^3)^2 = 64)
        let result = eval("2^3^2", AngleMode::Rad).unwrap();
        assert!((result - 512.0).abs() < 1e-10, "Expected 512, got {}", result);
    }

    #[test]
    fn test_power_fractional() {
        // 8^(1/3) = 2
        assert!((eval("8^(1/3)", AngleMode::Rad).unwrap() - 2.0).abs() < 1e-10);
    }

    // ─── Funciones trigonométricas (radianes) ────────────────────────

    #[test]
    fn test_sin_rad() {
        let result = eval("sin(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10, "sin(0) = 0, got {}", result);
    }

    #[test]
    fn test_cos_rad() {
        let result = eval("cos(0)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "cos(0) = 1, got {}", result);
    }

    #[test]
    fn test_tan_pi_over_4_rad() {
        // tan(π/4) = 1 en radianes
        let result = eval("tan(pi/4)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "tan(pi/4) = 1, got {}", result);
    }

    #[test]
    fn test_sin_pi_over_2_rad() {
        let result = eval("sin(pi/2)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "sin(pi/2) = 1, got {}", result);
    }

    // ─── Funciones trigonométricas (grados) ──────────────────────────

    #[test]
    fn test_sin_deg() {
        let result = eval("sin(0)", AngleMode::Deg).unwrap();
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_sin_90_deg() {
        let result = eval("sin(90)", AngleMode::Deg).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "sin(90°) = 1, got {}", result);
    }

    #[test]
    fn test_cos_180_deg() {
        let result = eval("cos(180)", AngleMode::Deg).unwrap();
        assert!((result + 1.0).abs() < 1e-10, "cos(180°) = -1, got {}", result);
    }

    #[test]
    fn test_tan_45_deg() {
        let result = eval("tan(45)", AngleMode::Deg).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "tan(45°) = 1, got {}", result);
    }

    // ─── Funciones trigonométricas inversas ──────────────────────────

    #[test]
    fn test_asin() {
        let result = eval("asin(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10, "asin(0) = 0, got {}", result);
    }

    #[test]
    fn test_asin_deg() {
        let result = eval("asin(1)", AngleMode::Deg).unwrap();
        assert!((result - 90.0).abs() < 1e-10, "asin(1) = 90°, got {}", result);
    }

    #[test]
    fn test_acos() {
        let result = eval("acos(1)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10, "acos(1) = 0, got {}", result);
    }

    #[test]
    fn test_atan() {
        let result = eval("atan(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_asin_domain_error() {
        let result = eval("asin(2)", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DomainError);
    }

    // ─── Funciones hiperbólicas ──────────────────────────────────────

    #[test]
    fn test_sinh() {
        let result = eval("sinh(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_cosh() {
        let result = eval("cosh(0)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tanh() {
        let result = eval("tanh(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10);
    }

    // ─── Logaritmos ──────────────────────────────────────────────────

    #[test]
    fn test_ln() {
        let result = eval("ln(1)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10, "ln(1) = 0, got {}", result);
    }

    #[test]
    fn test_ln_e() {
        let result = eval("ln(e)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10, "ln(e) = 1, got {}", result);
    }

    #[test]
    fn test_log10() {
        let result = eval("log10(100)", AngleMode::Rad).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_log10_via_log() {
        // 'log' sin calificador debe interpretarse como log10
        let result = eval("log(100)", AngleMode::Rad).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_log2() {
        let result = eval("log2(8)", AngleMode::Rad).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_domain_error() {
        let result = eval("ln(-1)", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DomainError);
    }

    #[test]
    fn test_ln_zero_error() {
        let result = eval("ln(0)", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DomainError);
    }

    // ─── Raíces ─────────────────────────────────────────────────────

    #[test]
    fn test_sqrt() {
        let result = eval("sqrt(16)", AngleMode::Rad).unwrap();
        assert!((result - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_domain_error() {
        let result = eval("sqrt(-1)", AngleMode::Rad);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DomainError);
    }

    #[test]
    fn test_cbrt() {
        let result = eval("cbrt(8)", AngleMode::Rad).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_cbrt_negative() {
        let result = eval("cbrt(-8)", AngleMode::Rad).unwrap();
        assert!((result + 2.0).abs() < 1e-10);
    }

    // ─── Abs, Exp ────────────────────────────────────────────────────

    #[test]
    fn test_abs_positive() {
        assert!((eval("abs(5)", AngleMode::Rad).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_abs_negative() {
        assert!((eval("abs(-5)", AngleMode::Rad).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp() {
        let result = eval("exp(0)", AngleMode::Rad).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp_1() {
        let result = eval("exp(1)", AngleMode::Rad).unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    // ─── Constantes ──────────────────────────────────────────────────

    #[test]
    fn test_pi_constant() {
        let result = eval("pi", AngleMode::Rad).unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_e_constant() {
        let result = eval("e", AngleMode::Rad).unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    // ─── ans constant ────────────────────────────────────────────────

    #[test]
    fn test_ans_uses_passed_value() {
        use crate::parser::ast::shunting_yard;
        use crate::parser::tokenizer::tokenize;
        let tokens = tokenize("ans+5").unwrap();
        let nodes = shunting_yard(&tokens).unwrap();
        let result = evaluate_ast(&nodes, AngleMode::Rad, 42.0).unwrap();
        assert!((result - 47.0).abs() < 1e-10, "42 + 5 = 47, got {}", result);
    }

    // ─── Factorial ───────────────────────────────────────────────────

    #[test]
    fn test_factorial_0() {
        assert!((eval("0!", AngleMode::Rad).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial_5() {
        assert!((eval("5!", AngleMode::Rad).unwrap() - 120.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial_10() {
        assert!((eval("10!", AngleMode::Rad).unwrap() - 3628800.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial_negative_error() {
        let result = eval("(-5)!", AngleMode::Rad);
        assert!(result.is_err());
    }

    // ─── Negación unaria ─────────────────────────────────────────────

    #[test]
    fn test_unary_negate() {
        assert!((eval("-(5)", AngleMode::Rad).unwrap() + 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_unary_negate_expression() {
        // -(2+3) = -5
        assert!((eval("-(2+3)", AngleMode::Rad).unwrap() + 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_unary_negate_sin() {
        // -sin(0) = -0 = 0
        let result = eval("-sin(0)", AngleMode::Rad).unwrap();
        assert!(result.abs() < 1e-10);
    }

    // ─── Operaciones bitwise ────────────────────────────────────────

    #[test]
    fn test_bitwise_and() {
        assert!((eval("5 & 3", AngleMode::Rad).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bitwise_or() {
        assert!((eval("5 | 3", AngleMode::Rad).unwrap() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_bitwise_xor() {
        assert!((eval("5 xor 3", AngleMode::Rad).unwrap() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_bitwise_not() {
        // ~0 = -1 (en complemento a 2)
        assert!((eval("~0", AngleMode::Rad).unwrap() + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_shift_left() {
        assert!((eval("1 << 4", AngleMode::Rad).unwrap() - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_shift_right() {
        assert!((eval("16 >> 2", AngleMode::Rad).unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_hex_literal_ff() {
        assert!((eval("0xFF", AngleMode::Rad).unwrap() - 255.0).abs() < 1e-10);
    }

    #[test]
    fn test_bin_or() {
        // 0b1010 | 0b0101 = 15
        assert!((eval("0b1010 | 0b0101", AngleMode::Rad).unwrap() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_oct_literal() {
        assert!((eval("0o77", AngleMode::Rad).unwrap() - 63.0).abs() < 1e-10);
    }

    #[test]
    fn test_bitwise_precedence() {
        // 1 << 2 + 3: shift tiene menor precedencia que +, así que es 1 << 5 = 32
        assert!((eval("1 << 2 + 3", AngleMode::Rad).unwrap() - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_bitwise_and_vs_or_precedence() {
        // 1 | 2 & 3 = 1 | (2 & 3) = 1 | 2 = 3  (& mayor precedencia que |)
        assert!((eval("1 | 2 & 3", AngleMode::Rad).unwrap() - 3.0).abs() < 1e-10);
    }

    // ─── Combinaciones complejas ─────────────────────────────────────

    #[test]
    fn test_combined_expression() {
        // sin(pi/2) + 3! * 2^3
        // sin(pi/2) = 1, 3! = 6, 2^3 = 8, 6*8 = 48, 1+48 = 49
        let result = eval("sin(pi/2)+3!*2^3", AngleMode::Rad).unwrap();
        assert!((result - 49.0).abs() < 1e-10, "Expected 49, got {}", result);
    }

    #[test]
    fn test_unbalanced_rpn_error() {
        // Evaluar una secuencia RPN inválida manualmente
        use crate::parser::ast::AstNode;
        let nodes = vec![
            AstNode::Number(1.0),
            AstNode::Number(2.0),
            // Falta un operador → quedarán 2 valores en la pila
        ];
        let result = evaluate_ast(&nodes, AngleMode::Rad, 0.0);
        assert!(result.is_err());
    }
}
