//! Comando Tauri para el método de Newton-Raphson.
//!
//! Expone `solve_newton` que orquesta la resolución de ecuaciones
//! llamando al módulo `math::solver`.

use crate::math::solver;
use crate::models::types::NewtonResult;

/// Resuelve una ecuación usando el método de Newton-Raphson.
///
/// # Argumentos
///
/// * `func` — Expresión de la función f(x), ej: `"x^2 - 4"`.
/// * `derivative` — Expresión de la derivada f'(x), ej: `"2*x"`.
/// * `initial_guess` — Valor inicial de x.
/// * `tolerance` — Tolerancia de convergencia (ej: 1e-10).
/// * `max_iterations` — Máximo de iteraciones (ej: 100).
///
/// # Retorna
///
/// Un `NewtonResult` con la raíz, el valor de f(x), iteraciones, estado de
/// convergencia, y la tabla completa de iteraciones.
#[tauri::command]
pub fn solve_newton(
    func: String,
    derivative: String,
    initial_guess: f64,
    tolerance: f64,
    max_iterations: u32,
) -> NewtonResult {
    let deriv = if derivative.trim().is_empty() {
        // Si no se proporciona derivada, usamos una aproximación numérica
        // construyendo una expresión que el parser puede evaluar.
        // Nota: en el frontend JS ya se maneja la aproximación numérica,
        // pero aquí en Rust preferimos que el usuario la proporcione.
        // Si está vacía, devolvemos un error controlado.
        return NewtonResult {
            root: 0.0,
            f_root: f64::NAN,
            iterations: 0,
            converged: false,
            error: Some(
                "Debes proporcionar la derivada f'(x) o dejarla vacía para usar la aproximación numérica (fallback web).".to_string(),
            ),
            steps: Vec::new(),
        };
    };

    solver::newton_raphson_detailed(&func, &deriv, initial_guess, tolerance, max_iterations)
}
