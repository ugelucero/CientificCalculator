use std::sync::Mutex;

use crate::math::matrix;
use crate::math::stats;
use crate::models::errors::{CalcError, ErrorKind};
use crate::models::state::AppState;
use crate::models::types::{CalcMode, ExpressionResult, FormatType, HistoryEntry};
use crate::parser;
use crate::persistence;

/// Evalúa una expresión matemática y retorna el resultado formateado.
/// Es el comando núcleo de la calculadora.
///
/// Efectos secundarios:
/// - Actualiza `last_answer` en el estado.
/// - Agrega una entrada al historial.
/// - Persiste el estado a disco.
#[tauri::command]
pub fn evaluate_expression(
    expr: String,
    mode: CalcMode,
    state: tauri::State<'_, Mutex<AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<ExpressionResult, CalcError> {
    // Obtener angle_mode y last_answer del estado.
    let (angle_mode, last_answer) = {
        let s = state
            .lock()
            .map_err(|e| CalcError::new(
                crate::models::errors::ErrorKind::InternalError,
                format!("Error al acceder al estado: {}", e),
            ))?;
        (s.angle_mode, s.last_answer)
    };

    // Evaluar la expresión pasando el last_answer actual para `ans`.
    let (formatted, result_value) = match &mode {
        CalcMode::Complex => {
            let complex_result = parser::evaluate_complex(&expr, angle_mode, last_answer)?;
            let ans_value = if complex_result.is_real() {
                complex_result.real
            } else {
                complex_result.abs()
            };
            (complex_result.to_string(), ans_value)
        }
        CalcMode::Matrix => {
            let (display, value) = evaluate_matrix_expression(&expr)?;
            (display, value)
        }
        CalcMode::Statistics => {
            let (display, value) = evaluate_statistics_expression(&expr)?;
            (display, value)
        }
        _ => {
            let result = parser::evaluate(&expr, angle_mode, last_answer)?;
            (format_result_for_mode(result, &mode), result)
        }
    };

    // Generar ID único y timestamp para la entrada de historial.
    let history_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let entry = HistoryEntry {
        id: history_id,
        expression: expr.clone(),
        result: formatted.clone(),
        timestamp,
        mode: mode.clone(),
    };

    // Actualizar el estado: last_answer, historial, y persistir.
    {
        let mut s = state
            .lock()
            .map_err(|e| CalcError::new(
                crate::models::errors::ErrorKind::InternalError,
                format!("Error al acceder al estado: {}", e),
            ))?;

        s.last_answer = result_value;
        s.history.insert(0, entry);

        // Limitar el historial a 100 entradas.
        s.history.truncate(100);

        // Persistir el estado a disco (ignorar error de persistencia, no es crítico).
        let _ = persistence::state::save_state(&s, &app_handle);
    }

    Ok(ExpressionResult {
        result: formatted,
        display: expr,
        format: FormatType::Decimal,
    })
}

/// Formatea el resultado según el modo de la calculadora.
fn format_result_for_mode(value: f64, mode: &CalcMode) -> String {
    match mode {
        CalcMode::Programmer => format_programmer_result(value),
        _ => format_result(value),
    }
}

/// Formatea el resultado para modo Programador: muestra el valor en las 4 bases.
fn format_programmer_result(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };
    let int_val = value as i64;
    let u32_val = int_val as u32;
    let dec_str = format!("{}", int_val);
    let hex_str = format!("0x{:X}", u32_val);
    let oct_str = format!("0o{:o}", u32_val);
    let bin_str = format!("0b{:b}", u32_val);
    format!("{}  |  {}  |  {}  |  {}", hex_str, dec_str, oct_str, bin_str)
}

/// Formatea el resultado numérico como string.
///
/// - Si es entero sin parte decimal, se muestra sin `.0`.
/// - Si tiene parte decimal, se muestra con precisión completa evitando
///   notación científica para números cotidianos.
fn format_result(value: f64) -> String {
    // Evitar -0.0
    let value = if value == 0.0 { 0.0 } else { value };

    // Si es NaN o infinito, devolver representación directa
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "∞" } else { "-∞" }.to_string();
    }

    // Para números muy grandes o muy pequeños usar notación científica
    let abs = value.abs();
    if abs >= 1e15 || (abs < 1e-10 && abs > 0.0) {
        return format!("{:e}", value);
    }

    // Formatear con hasta 15 dígitos significativos
    let formatted = format!("{:.15}", value);

    // Eliminar ceros finales después del punto decimal
    let formatted = formatted.trim_end_matches('0');

    // Si el último carácter es '.', eliminarlo (número entero)
    let formatted = formatted.trim_end_matches('.');

    formatted.to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// Evaluación de expresiones matriciales
// ═══════════════════════════════════════════════════════════════════════════

/// Evalúa una expresión en modo Matriz.
///
/// Soporta:
/// - Literal de matriz: `[[1,2],[3,4]]`
/// - Suma, resta: `[[1,2],[3,4]] + [[5,6],[7,8]]`
/// - Multiplicación: `M * N` o `escalar * M`
/// - Funciones: `det(M)`, `inv(M)`, `trans(M)`, `id(n)`
fn evaluate_matrix_expression(expr: &str) -> Result<(String, f64), CalcError> {
    let expr = expr.trim();

    // Intentar parsear como llamada a función: det(...), inv(...), trans(...), id(...)
    if let Some(result) = try_matrix_function(expr)? {
        return Ok(result);
    }

    // Intentar parsear operaciones binarias: M + N, M - N, M * N, k * M, M * k
    // Buscar operador de menor precedencia fuera de corchetes.
    if let Some((left_str, op, right_str)) = split_at_matrix_operator(expr) {
        let left_is_matrix = left_str.trim().starts_with('[');
        let right_is_matrix = right_str.trim().starts_with('[');

        // Escalar * Matriz o Matriz * Escalar
        if !left_is_matrix && right_is_matrix {
            let scalar = parser::evaluate(left_str, crate::models::types::AngleMode::Rad, 0.0)?;
            let right = parse_matrix_literal(right_str)?;
            let result = matrix::Matrix::scalar_multiply(&right, scalar);
            let display = matrix::Matrix::format(&result);
            return Ok((display, 0.0));
        }
        if !right_is_matrix && left_is_matrix {
            let scalar = parser::evaluate(right_str, crate::models::types::AngleMode::Rad, 0.0)?;
            let left = parse_matrix_literal(left_str)?;
            let result = matrix::Matrix::scalar_multiply(&left, scalar);
            let display = matrix::Matrix::format(&result);
            return Ok((display, 0.0));
        }

        // Ambos lados deben ser matrices para operaciones matriciales
        let left = parse_matrix_literal(left_str)?;
        let right = parse_matrix_literal(right_str)?;

        let result = match op {
            '+' => matrix::Matrix::add(&left, &right),
            '-' => matrix::Matrix::subtract(&left, &right),
            '*' => matrix::Matrix::multiply(&left, &right),
            _ => {
                return Err(CalcError::new(
                    ErrorKind::ParseError,
                    format!("Operador '{}' no soportado para matrices", op),
                ))
            }
        }?;

        let display = matrix::Matrix::format(&result);
        return Ok((display, 0.0));
    }

    // Intentar parsear como literal de matriz simple.
    if expr.starts_with('[') {
        let m = parse_matrix_literal(expr)?;
        let display = matrix::Matrix::format(&m);
        return Ok((display, 0.0));
    }

    Err(CalcError::new(
        ErrorKind::ParseError,
        format!("Expresión matricial no reconocida: '{}'", expr),
    ))
}

/// Intenta evaluar una función matricial: det(...), inv(...), trans(...), id(...)
fn try_matrix_function(expr: &str) -> Result<Option<(String, f64)>, CalcError> {
    let expr = expr.trim();

    for name in &["det", "inv", "trans", "id"] {
        let prefix = format!("{} (", name);

        // Intentar con espacio antes del paréntesis: "det ("
        // o sin espacio: "det("
        if let Some(rest) = expr.strip_prefix(&prefix).or_else(|| {
            let lower = expr.to_lowercase();
            let lower_prefix = format!("{} (", name);
            if lower.starts_with(&lower_prefix) {
                Some(&expr[lower_prefix.len()..])
            } else {
                None
            }
        }) {
            // Find the matching closing paren.
            if let Some((args_str, _)) = find_matching_paren(rest, 0) {
                let args_str = args_str.trim();
                match *name {
                    "det" => {
                        let m = parse_matrix_literal(args_str)?;
                        let det = matrix::Matrix::determinant(&m)?;
                        return Ok(Some((format_result(det), det)));
                    }
                    "inv" => {
                        let m = parse_matrix_literal(args_str)?;
                        let inv = matrix::Matrix::inverse(&m)?;
                        let display = matrix::Matrix::format(&inv);
                        return Ok(Some((display, 0.0)));
                    }
                    "trans" => {
                        let m = parse_matrix_literal(args_str)?;
                        let t = matrix::Matrix::transpose(&m);
                        let display = matrix::Matrix::format(&t);
                        return Ok(Some((display, 0.0)));
                    }
                    "id" => {
                        let n = parser::evaluate(
                            args_str,
                            crate::models::types::AngleMode::Rad,
                            0.0,
                        )?;
                        if n < 1.0 || n > 100.0 || n.fract() != 0.0 {
                            return Err(CalcError::new(
                                ErrorKind::DomainError,
                                format!("id() requiere un entero positivo, recibido: {}", n),
                            ));
                        }
                        let m = matrix::Matrix::identity(n as usize);
                        let display = matrix::Matrix::format(&m);
                        return Ok(Some((display, 0.0)));
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(None)
}

/// Parsea un literal de matriz en formato `[[a,b,...],[c,d,...],...]`.
fn parse_matrix_literal(s: &str) -> Result<matrix::Matrix, CalcError> {
    let s = s.trim();
    if !s.starts_with('[') {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Se esperaba una matriz ([[...]]), recibido: '{}'", s),
        ));
    }

    // Encontrar la matriz completa (contar corchetes).
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;
    let mut end = 0;

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if depth != 0 || end == 0 {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            "Matriz mal formada: corchetes desbalanceados",
        ));
    }

    let inner = &chars[1..end - 1]; // Eliminar corchetes exteriores
    let inner_str: String = inner.iter().collect();

    // Parsear filas separadas por '],[' o solo '[' al inicio y ']' al final de cada fila.
    let mut data: Vec<Vec<f64>> = Vec::new();
    let row_chars: Vec<char> = inner_str.chars().collect();
    let mut i = 0;
    let len = row_chars.len();

    while i < len {
        // Cada fila empieza con '['
        while i < len && row_chars[i] != '[' {
            i += 1;
        }
        if i >= len {
            break;
        }
        i += 1; // saltar '['

        let mut row: Vec<f64> = Vec::new();
        let mut num_start = i;

        while i < len && row_chars[i] != ']' {
            if row_chars[i] == ',' {
                // Parsear el número entre num_start e i.
                let num_str: String = row_chars[num_start..i].iter().collect();
                let num_str = num_str.trim();
                if !num_str.is_empty() {
                    let val = num_str
                        .parse::<f64>()
                        .map_err(|_| CalcError::new(
                            ErrorKind::ParseError,
                            format!("Número inválido en matriz: '{}'", num_str),
                        ))?;
                    row.push(val);
                }
                i += 1; // saltar ','
                num_start = i;
            } else {
                i += 1;
            }
        }

        // Último número antes de ']'
        if num_start < i {
            let num_str: String = row_chars[num_start..i].iter().collect();
            let num_str = num_str.trim();
            if !num_str.is_empty() {
                let val = num_str
                    .parse::<f64>()
                    .map_err(|_| CalcError::new(
                        ErrorKind::ParseError,
                        format!("Número inválido en matriz: '{}'", num_str),
                    ))?;
                row.push(val);
            }
        }

        if !row.is_empty() {
            data.push(row);
        }

        i += 1; // saltar ']'

        // Saltar coma entre filas: ],[ o solo ,
        while i < len && row_chars[i] == ',' {
            i += 1;
        }
    }

    matrix::Matrix::from_vec(data)
}

/// Divide una expresión en (izquierda, operador, derecha) para operaciones
/// matriciales, respetando los corchetes.
fn split_at_matrix_operator(expr: &str) -> Option<(&str, char, &str)> {
    let chars: Vec<char> = expr.chars().collect();
    let mut depth = 0;
    let mut i = 0;

    // Saltar primer operando (puede ser matriz con corchetes).
    while i < chars.len() {
        match chars[i] {
            '[' => depth += 1,
            ']' => depth -= 1,
            '+' | '-' | '*' if depth == 0 => {
                // Verificar que no sea parte de un número (ej: 1e-5).
                if chars[i] == '-' && i > 0 && chars[i - 1].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                if chars[i] == '-' && i > 0 && chars[i - 1] == 'e' {
                    i += 1;
                    continue;
                }
                let op = chars[i];
                return Some((
                    expr[..i].trim(),
                    op,
                    expr[i + 1..].trim(),
                ));
            }
            _ => {}
        }
        i += 1;
    }

    None
}

/// Encuentra el paréntesis de cierre correspondiente.
/// `start` debe apuntar justo después del '(' de apertura.
fn find_matching_paren(s: &str, start: usize) -> Option<(&str, usize)> {
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;
    let mut i = start;

    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return Some((&s[start..i], i + 1));
                }
                depth -= 1;
            }
            _ => {}
        }
        i += 1;
    }

    None
}

// ═══════════════════════════════════════════════════════════════════════════
// Evaluación de expresiones estadísticas
// ═══════════════════════════════════════════════════════════════════════════

/// Evalúa una expresión en modo Estadístico.
///
/// Soporta:
/// - Lista simple: `[2,4,4,4,5,5,7,9]` → muestra estadísticas básicas.
/// - Función con lista: `sum([...])`, `mean([...])`, `median([...])`,
///   `mode([...])`, `stddev([...])`, `var([...])`, `min([...])`, `max([...])`,
///   `range([...])`, `quartiles([...])`.
/// - Regresión: `linreg([x1,x2,...],[y1,y2,...])`
fn evaluate_statistics_expression(expr: &str) -> Result<(String, f64), CalcError> {
    let expr = expr.trim();

    // Intentar parsear como llamada a función estadística.
    if let Some(result) = try_stats_function(expr)? {
        return Ok(result);
    }

    // Intentar parsear como lista simple: [a,b,c,...]
    if expr.starts_with('[') {
        let data = parse_list_literal(expr)?;
        let display = format_stats_summary(&data);
        return Ok((display, 0.0));
    }

    Err(CalcError::new(
        ErrorKind::ParseError,
        format!("Expresión estadística no reconocida: '{}'", expr),
    ))
}

/// Intenta evaluar una función estadística: sum(...), mean(...), etc.
fn try_stats_function(expr: &str) -> Result<Option<(String, f64)>, CalcError> {
    let expr = expr.trim();
    let lower = expr.to_lowercase();

    // Funciones de un argumento (lista).
    let single_arg_funcs = [
        "sum", "mean", "median", "mode", "stddev", "var", "min", "max",
        "range", "quartiles",
    ];

    for name in &single_arg_funcs {
        let prefix = format!("{} (", name);
        if lower.starts_with(&prefix) {
            let rest = &expr[prefix.len()..];
            if let Some((args_str, _)) = find_matching_paren(rest, 0) {
                let data = parse_list_literal(args_str)?;
                let (display, value) = eval_stats_single_arg(name, &data)?;
                return Ok(Some((display, value)));
            }
        }
    }

    // Función linreg: dos argumentos.
    let prefix = "linreg (";
    if lower.starts_with(prefix) {
        let rest = &expr[prefix.len()..];
        if let Some((args_str, _)) = find_matching_paren(rest, 0) {
            // args_str debería contener dos listas separadas por coma.
            let (x_data, y_data) = parse_two_lists(args_str)?;
            let (slope, intercept, r2) = stats::linear_regression(&x_data, &y_data)?;
            let display = format!(
                "y = {:.6}x + {:.6}\nr² = {:.6}",
                slope, intercept, r2
            );
            return Ok(Some((display, r2)));
        }
    }

    Ok(None)
}

/// Evalúa una función estadística de un solo argumento.
fn eval_stats_single_arg(name: &str, data: &[f64]) -> Result<(String, f64), CalcError> {
    match name {
        "sum" => {
            let val = stats::sum(data);
            Ok((format_result(val), val))
        }
        "mean" => {
            let val = stats::mean(data);
            Ok((format_result(val), val))
        }
        "median" => {
            let val = stats::median(data);
            Ok((format_result(val), val))
        }
        "mode" => {
            let modes = stats::mode(data);
            if modes.is_empty() {
                Ok(("Sin moda".to_string(), 0.0))
            } else {
                let display = modes
                    .iter()
                    .map(|m| format_result(*m))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok((display, modes[0]))
            }
        }
        "stddev" => {
            let val = stats::std_dev(data, true);
            Ok((format_result(val), val))
        }
        "var" => {
            let val = stats::variance(data, true);
            Ok((format_result(val), val))
        }
        "min" => {
            let val = stats::min(data);
            Ok((format_result(val), val))
        }
        "max" => {
            let val = stats::max(data);
            Ok((format_result(val), val))
        }
        "range" => {
            let val = stats::range(data);
            Ok((format_result(val), val))
        }
        "quartiles" => {
            let (q1, q2, q3) = stats::quartiles(data);
            let display = format!(
                "Q1 = {}\nQ2 = {}\nQ3 = {}",
                format_result(q1),
                format_result(q2),
                format_result(q3)
            );
            Ok((display, q2))
        }
        _ => Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Función estadística desconocida: '{}'", name),
        )),
    }
}

/// Parsea un literal de lista de números: `[a, b, c, ...]`
fn parse_list_literal(s: &str) -> Result<Vec<f64>, CalcError> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            format!("Se esperaba una lista ([...]), recibido: '{}'", s),
        ));
    }

    let inner = &s[1..s.len() - 1];
    if inner.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut data = Vec::new();
    for part in inner.split(',') {
        let part = part.trim();
        if !part.is_empty() {
            let val = part.parse::<f64>().map_err(|_| {
                CalcError::new(
                    ErrorKind::ParseError,
                    format!("Número inválido en lista: '{}'", part),
                )
            })?;
            data.push(val);
        }
    }

    Ok(data)
}

/// Parsea dos listas separadas por coma: `[x1,x2,...],[y1,y2,...]`
fn parse_two_lists(s: &str) -> Result<(Vec<f64>, Vec<f64>), CalcError> {
    let s = s.trim();

    // Encontrar la primera lista.
    if !s.starts_with('[') {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            "Se esperaban dos listas separadas por coma",
        ));
    }

    let mut depth = 0;
    let mut first_end = 0;
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    first_end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    let first_str = &s[..first_end];
    let rest = s[first_end..].trim();

    // Saltar coma separadora.
    let rest = if rest.starts_with(',') {
        rest[1..].trim()
    } else {
        return Err(CalcError::new(
            ErrorKind::ParseError,
            "Se esperaba una coma separando las dos listas",
        ));
    };

    let x = parse_list_literal(first_str)?;
    let y = parse_list_literal(rest)?;

    Ok((x, y))
}

/// Genera un resumen textual de estadísticas básicas para una lista de datos.
fn format_stats_summary(data: &[f64]) -> String {
    if data.is_empty() {
        return "Lista vacía".to_string();
    }

    let n = data.len();
    let s = stats::sum(data);
    let m = stats::mean(data);
    let med = stats::median(data);
    let modes = stats::mode(data);
    let sd = stats::std_dev(data, true);
    let min_val = stats::min(data);
    let max_val = stats::max(data);
    let (q1, q2, q3) = stats::quartiles(data);

    let mode_str = if modes.is_empty() {
        "—".to_string()
    } else {
        modes
            .iter()
            .map(|x| format_result(*x))
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!(
        "n = {}\nSuma = {}\nMedia = {}\nMediana = {}\nModa = {}\nDesv.Est. = {}\nMín = {}\nMáx = {}\nQ1 = {}, Q2 = {}, Q3 = {}",
        n,
        format_result(s),
        format_result(m),
        format_result(med),
        mode_str,
        format_result(sd),
        format_result(min_val),
        format_result(max_val),
        format_result(q1),
        format_result(q2),
        format_result(q3),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_integer() {
        assert_eq!(format_result(42.0), "42");
    }

    #[test]
    fn test_format_decimal() {
        let result = format_result(3.14159);
        assert!(result.starts_with("3.14159"));
    }

    #[test]
    fn test_format_negative_zero() {
        assert_eq!(format_result(-0.0), "0");
    }

    #[test]
    fn test_format_scientific_large() {
        let result = format_result(1e20);
        assert!(result.contains('e'));
    }
}
