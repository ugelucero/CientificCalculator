//! Resolución de ecuaciones no lineales — Método de Newton-Raphson.
//!
//! Implementa el algoritmo iterativo de Newton-Raphson para encontrar
//! raíces de funciones reales de una variable `x`.
//!
//! El método evalúa `f(x)` y `f'(x)` usando el parser existente,
//! inyectando el valor de `x` como variable mediante sustitución textual.

use crate::models::errors::{CalcError, ErrorKind};
use crate::models::types::{AngleMode, NewtonResult, NewtonStep};
use crate::parser;

/// Versión detallada de `newton_raphson` que también retorna el historial de iteraciones.
///
/// Similar a `newton_raphson`, pero devuelve un `NewtonResult` con todos
/// los pasos intermedios, ideal para mostrar la tabla de iteraciones en la UI.
pub fn newton_raphson_detailed(
    func: &str,
    derivative: &str,
    initial_guess: f64,
    tolerance: f64,
    max_iterations: u32,
) -> NewtonResult {
    let mut x = initial_guess;
    let mut steps: Vec<NewtonStep> = Vec::new();

    for iteration in 0..max_iterations {
        let fx = match evaluate_with_x(func, x) {
            Ok(v) => v,
            Err(e) => {
                return NewtonResult {
                    root: x,
                    f_root: f64::NAN,
                    iterations: iteration,
                    converged: false,
                    error: Some(format!("Error al evaluar f(x): {}", e.message)),
                    steps,
                };
            }
        };
        let dfx = match evaluate_with_x(derivative, x) {
            Ok(v) => v,
            Err(e) => {
                return NewtonResult {
                    root: x,
                    f_root: fx,
                    iterations: iteration,
                    converged: false,
                    error: Some(format!("Error al evaluar f'(x): {}", e.message)),
                    steps,
                };
            }
        };

        steps.push(NewtonStep {
            n: iteration + 1,
            x_n: x,
            f_x_n: fx,
        });

        // Verificar convergencia.
        if fx.abs() < tolerance {
            return NewtonResult {
                root: x,
                f_root: fx,
                iterations: iteration + 1,
                converged: true,
                error: None,
                steps,
            };
        }

        // Verificar derivada cero.
        if dfx.abs() < 1e-15 {
            return NewtonResult {
                root: x,
                f_root: fx,
                iterations: iteration + 1,
                converged: false,
                error: Some(format!(
                    "Derivada cercana a cero (≈ {}) en x ≈ {}; el método no puede continuar",
                    dfx, x
                )),
                steps,
            };
        }

        // Iteración de Newton.
        let x_new = x - fx / dfx;

        // Verificar estancamiento.
        if (x_new - x).abs() < tolerance * 1e-3 && fx.abs() >= tolerance {
            return NewtonResult {
                root: x,
                f_root: fx,
                iterations: iteration + 1,
                converged: false,
                error: Some(format!(
                    "El método se estancó en x ≈ {} (f(x) ≈ {}). No se encuentra raíz con la precisión deseada.",
                    x, fx
                )),
                steps,
            };
        }

        x = x_new;
    }

    // No convergió en max_iterations.
    let final_fx = evaluate_with_x(func, x).unwrap_or(f64::NAN);
    NewtonResult {
        root: x,
        f_root: final_fx,
        iterations: max_iterations,
        converged: false,
        error: Some(format!(
            "No convergió en {} iteraciones. Último valor: x ≈ {} (f(x) ≈ {})",
            max_iterations, x, final_fx
        )),
        steps,
    }
}

/// Encuentra una raíz de `func` usando el método de Newton-Raphson.
///
/// # Argumentos
///
/// * `func` — Expresión de la función f(x), ej: `"x^2 - 4"`.
/// * `derivative` — Expresión de la derivada f'(x), ej: `"2*x"`.
/// * `initial_guess` — Valor inicial de x para comenzar la iteración.
/// * `tolerance` — Criterio de convergencia: |f(x_n)| < tolerance.
/// * `max_iterations` — Número máximo de iteraciones antes de fallar.
///
/// # Retorna
///
/// * `Ok(root)` — La raíz aproximada encontrada.
/// * `Err(...)` — Si no converge, derivada cero, o error de parseo.
///
/// # Algoritmo
///
/// Itera: `x_{n+1} = x_n - f(x_n) / f'(x_n)`
///
/// Converge cuando `|f(x_n)| < tolerance`.
pub fn newton_raphson(
    func: &str,
    derivative: &str,
    initial_guess: f64,
    tolerance: f64,
    max_iterations: u32,
) -> Result<f64, CalcError> {
    let mut x = initial_guess;

    for _iteration in 0..max_iterations {
        let fx = evaluate_with_x(func, x)?;
        let dfx = evaluate_with_x(derivative, x)?;

        // Verificar convergencia.
        if fx.abs() < tolerance {
            return Ok(x);
        }

        // Verificar derivada cero (evitar división por cero).
        if dfx.abs() < 1e-15 {
            return Err(CalcError::new(
                ErrorKind::DomainError,
                format!(
                    "Derivada cercana a cero (≈ {}) en x ≈ {}; el método de Newton-Raphson no puede continuar",
                    dfx, x
                ),
            ));
        }

        // Iteración de Newton: x_{n+1} = x_n - f(x_n)/f'(x_n)
        let x_new = x - fx / dfx;

        // Verificar si hay progreso (evitar oscilaciones estacionarias).
        if (x_new - x).abs() < tolerance * 1e-3 && fx.abs() >= tolerance {
            // El método se estancó sin converger.
            return Err(CalcError::new(
                ErrorKind::DomainError,
                format!(
                    "El método se estancó en x ≈ {} (f(x) ≈ {}). No se encuentra raíz con la precisión deseada.",
                    x, fx
                ),
            ));
        }

        x = x_new;
    }

    // No convergió en max_iterations.
    Err(CalcError::new(
        ErrorKind::DomainError,
        format!(
            "No convergió en {} iteraciones. Último valor: x ≈ {} (f(x) ≈ {})",
            max_iterations,
            x,
            evaluate_with_x(func, x).unwrap_or(f64::NAN)
        ),
    ))
}

/// Evalúa una expresión matemática sustituyendo `x` por `value`.
///
/// La sustitución es inteligente: solo reemplaza `x` cuando aparece
/// como identificador independiente (no dentro de `exp`, `xor`, `0x`, etc.).
fn evaluate_with_x(expr: &str, x_value: f64) -> Result<f64, CalcError> {
    let substituted = substitute_variable(expr, "x", x_value);
    parser::evaluate(&substituted, AngleMode::Rad, 0.0)
}

/// Reemplaza todas las ocurrencias de `var` como identificador independiente
/// por la representación numérica de `value`.
///
/// Un identificador independiente es aquel que no está precedido ni seguido
/// por un carácter alfanumérico.
fn substitute_variable(expr: &str, var: &str, value: f64) -> String {
    let chars: Vec<char> = expr.chars().collect();
    let var_chars: Vec<char> = var.chars().collect();
    let val_str = format_f64(value);
    let mut result = String::with_capacity(expr.len());
    let mut i = 0;

    while i < chars.len() {
        // Verificar si encontramos `var` en la posición actual.
        if i + var_chars.len() <= chars.len() {
            let slice = &chars[i..i + var_chars.len()];
            if slice == var_chars.as_slice() {
                // Verificar fronteras: no debe ser parte de un identificador más largo.
                let before_ok = i == 0 || !chars[i - 1].is_alphanumeric();
                let after_ok =
                    i + var_chars.len() >= chars.len() || !chars[i + var_chars.len()].is_alphanumeric();

                if before_ok && after_ok {
                    result.push_str(&val_str);
                    i += var_chars.len();
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Formatea un f64 para inyectarlo como string en una expresión.
///
/// Usa notación con punto decimal (nunca científica) para que el tokenizador
/// la reconozca correctamente.
fn format_f64(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };

    if value.is_nan() {
        return "0".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "1e308" } else { "-1e308" }.to_string();
    }

    // Para valores dentro de un rango razonable, usar formato decimal.
    let abs = value.abs();
    if abs >= 1e-10 && abs < 1e15 {
        let s = format!("{:.15}", value);
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        if s.is_empty() || s == "-" {
            "0".to_string()
        } else {
            s.to_string()
        }
    } else {
        // Para valores extremos, usar notación científica con precisión.
        format!("{:.15e}", value)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: llama a newton_raphson con parámetros por defecto.
    fn solve(func: &str, deriv: &str, guess: f64) -> Result<f64, CalcError> {
        newton_raphson(func, deriv, guess, 1e-10, 100)
    }

    // ─── x² - 4 = 0 ────────────────────────────────────────────────────────

    #[test]
    fn test_x_squared_minus_4() {
        let root = solve("x^2 - 4", "2*x", 1.0).unwrap();
        assert!(
            (root - 2.0).abs() < 1e-8,
            "Raíz esperada ≈ 2, obtenida {}",
            root
        );
    }

    #[test]
    fn test_x_squared_minus_4_negative_guess() {
        let root = solve("x^2 - 4", "2*x", -1.0).unwrap();
        assert!(
            (root + 2.0).abs() < 1e-8,
            "Raíz esperada ≈ -2, obtenida {}",
            root
        );
    }

    // ─── x² + 1 = 0 (no converge en reales) ────────────────────────────────

    #[test]
    fn test_x_squared_plus_1_no_real_root() {
        // f(x) = x² + 1 siempre es positivo, no tiene raíz real.
        // Newton-Raphson debería errar (no converger o estancarse).
        let result = solve("x^2 + 1", "2*x", 1.0);
        assert!(result.is_err(), "Esperaba error, no raíz real para x²+1=0");
    }

    // ─── x³ - 8 = 0 ────────────────────────────────────────────────────────

    #[test]
    fn test_x_cubed_minus_8() {
        let root = solve("x^3 - 8", "3*x^2", 1.0).unwrap();
        assert!(
            (root - 2.0).abs() < 1e-8,
            "Raíz esperada ≈ 2, obtenida {}",
            root
        );
    }

    // ─── Derivada cero ─────────────────────────────────────────────────────

    #[test]
    fn test_zero_derivative() {
        // f(x) = x, f'(x) = 0 (error: derivada constante 0).
        // En x=0, f(0)=0, pero con derivada 0 el método falla.
        let result = newton_raphson("x", "0", 0.0, 1e-10, 10);
        assert!(result.is_err());
    }

    // ─── Convergencia rápida ────────────────────────────────────────────────

    #[test]
    fn test_simple_linear() {
        // f(x) = x - 5, f'(x) = 1 → raíz exacta en 1 iteración.
        let root = solve("x - 5", "1", 0.0).unwrap();
        assert!((root - 5.0).abs() < 1e-8);
    }

    // ─── Sustitución de variable ────────────────────────────────────────────

    #[test]
    fn test_substitute_variable_simple() {
        let result = substitute_variable("x + 2", "x", 3.0);
        assert_eq!(result, "3 + 2");
    }

    #[test]
    fn test_substitute_variable_in_function() {
        let result = substitute_variable("sin(x)", "x", 1.0);
        assert_eq!(result, "sin(1)");
    }

    #[test]
    fn test_substitute_variable_does_not_touch_exp() {
        let result = substitute_variable("exp(x)", "x", 2.0);
        assert_eq!(result, "exp(2)");
    }

    #[test]
    fn test_substitute_variable_does_not_touch_xor() {
        let result = substitute_variable("5 xor 3", "x", 2.0);
        assert_eq!(result, "5 xor 3");
    }

    #[test]
    fn test_substitute_variable_does_not_affect_hex() {
        let result = substitute_variable("0xFF", "x", 2.0);
        assert_eq!(result, "0xFF");
    }

    #[test]
    fn test_substitute_variable_multiple_occurrences() {
        let result = substitute_variable("x^2 + x - 4", "x", 3.0);
        assert_eq!(result, "3^2 + 3 - 4");
    }

    // ─── No convergencia por iteraciones máximas ────────────────────────────

    #[test]
    fn test_max_iterations_exceeded() {
        // f(x) = x² + 1 no tiene raíz real. Con max_iterations bajo debe fallar.
        let result = newton_raphson("x^2 + 1", "2*x", 10.0, 1e-15, 5);
        assert!(result.is_err());
    }
}
