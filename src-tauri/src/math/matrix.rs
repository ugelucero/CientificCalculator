//! Operaciones con matrices.
//!
//! Proporciona el tipo `Matrix` con álgebra lineal básica:
//! suma, resta, multiplicación, determinante, inversa,
//! transpuesta, identidad, y multiplicación escalar.
//!
//! ## Limitaciones
//! - Determinante: hasta 4×4.
//! - Inversa: hasta 3×3 (por simplicidad).

use serde::{Deserialize, Serialize};

use crate::models::errors::{CalcError, ErrorKind};

/// Matriz bidimensional para operaciones de álgebra lineal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    // ── Constructores ──────────────────────────────────────────────────────

    /// Crea una matriz de ceros con las dimensiones dadas.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    /// Crea una matriz a partir de un vector de filas.
    ///
    /// Valida que todas las filas tengan la misma longitud.
    pub fn from_vec(data: Vec<Vec<f64>>) -> Result<Self, CalcError> {
        if data.is_empty() {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                "La matriz no puede estar vacía",
            ));
        }
        let rows = data.len();
        let cols = data[0].len();
        if cols == 0 {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                "Las filas no pueden estar vacías",
            ));
        }
        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                return Err(CalcError::new(
                    ErrorKind::DimensionMismatch,
                    format!(
                        "Fila {} tiene {} columnas, esperadas {}",
                        i,
                        row.len(),
                        cols
                    ),
                ));
            }
        }
        Ok(Self { rows, cols, data })
    }

    /// Crea la matriz identidad de tamaño `n`.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::new(n, n);
        for i in 0..n {
            m.data[i][i] = 1.0;
        }
        m
    }

    // ── Operaciones ────────────────────────────────────────────────────────

    /// Suma de dos matrices.
    pub fn add(a: &Self, b: &Self) -> Result<Self, CalcError> {
        if a.rows != b.rows || a.cols != b.cols {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                format!(
                    "No se pueden sumar matrices de {}×{} y {}×{}",
                    a.rows, a.cols, b.rows, b.cols
                ),
            ));
        }
        let mut result = Self::new(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                result.data[i][j] = a.data[i][j] + b.data[i][j];
            }
        }
        Ok(result)
    }

    /// Resta de dos matrices.
    pub fn subtract(a: &Self, b: &Self) -> Result<Self, CalcError> {
        if a.rows != b.rows || a.cols != b.cols {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                format!(
                    "No se pueden restar matrices de {}×{} y {}×{}",
                    a.rows, a.cols, b.rows, b.cols
                ),
            ));
        }
        let mut result = Self::new(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                result.data[i][j] = a.data[i][j] - b.data[i][j];
            }
        }
        Ok(result)
    }

    /// Multiplicación de dos matrices.
    pub fn multiply(a: &Self, b: &Self) -> Result<Self, CalcError> {
        if a.cols != b.rows {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                format!(
                    "No se pueden multiplicar matrices de {}×{} y {}×{} (cols de A debe coincidir con rows de B)",
                    a.rows, a.cols, b.rows, b.cols
                ),
            ));
        }
        let mut result = Self::new(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    sum += a.data[i][k] * b.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        Ok(result)
    }

    /// Multiplicación por un escalar.
    pub fn scalar_multiply(a: &Self, scalar: f64) -> Self {
        let mut result = a.clone();
        for i in 0..a.rows {
            for j in 0..a.cols {
                result.data[i][j] *= scalar;
            }
        }
        result
    }

    /// Transpuesta de la matriz.
    pub fn transpose(m: &Self) -> Self {
        let mut result = Self::new(m.cols, m.rows);
        for i in 0..m.rows {
            for j in 0..m.cols {
                result.data[j][i] = m.data[i][j];
            }
        }
        result
    }

    // ── Determinante ───────────────────────────────────────────────────────

    /// Calcula el determinante de una matriz cuadrada.
    ///
    /// Soporta matrices de 2×2, 3×3 y 4×4.
    /// Para 1×1 devuelve el único elemento.
    pub fn determinant(m: &Self) -> Result<f64, CalcError> {
        if m.rows != m.cols {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                format!(
                    "El determinante solo está definido para matrices cuadradas (recibida {}×{})",
                    m.rows, m.cols
                ),
            ));
        }

        match m.rows {
            0 => Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                "Matriz vacía no tiene determinante",
            )),
            1 => Ok(m.data[0][0]),
            2 => Ok(m.data[0][0] * m.data[1][1] - m.data[0][1] * m.data[1][0]),
            3 => {
                let a = &m.data;
                Ok(a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
                    - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
                    + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]))
            }
            4 => {
                // Expansión por la primera fila con sub-determinantes 3×3.
                let a = &m.data;
                let mut det = 0.0;
                let signs = [1.0, -1.0, 1.0, -1.0];

                for col in 0..4 {
                    let sub = Self::submatrix_3x3(a, 0, col);
                    let sub_det = sub[0][0]
                        * (sub[1][1] * sub[2][2] - sub[1][2] * sub[2][1])
                        - sub[0][1] * (sub[1][0] * sub[2][2] - sub[1][2] * sub[2][0])
                        + sub[0][2] * (sub[1][0] * sub[2][1] - sub[1][1] * sub[2][0]);
                    det += signs[col] * a[0][col] * sub_det;
                }
                Ok(det)
            }
            _ => Err(CalcError::new(
                ErrorKind::DomainError,
                format!(
                    "Determinante solo soportado hasta 4×4 (recibida {}×{})",
                    m.rows, m.cols
                ),
            )),
        }
    }

    /// Extrae una submatriz 3×3 eliminando la fila `skip_row` y columna `skip_col`
    /// de una matriz 4×4.
    fn submatrix_3x3(a: &[Vec<f64>], skip_row: usize, skip_col: usize) -> [[f64; 3]; 3] {
        let mut sub = [[0.0; 3]; 3];
        let mut si = 0;
        for i in 0..4 {
            if i == skip_row {
                continue;
            }
            let mut sj = 0;
            for j in 0..4 {
                if j == skip_col {
                    continue;
                }
                sub[si][sj] = a[i][j];
                sj += 1;
            }
            si += 1;
        }
        sub
    }

    // ── Inversa ────────────────────────────────────────────────────────────

    /// Calcula la inversa de una matriz cuadrada.
    ///
    /// Soporta matrices de 2×2 y 3×3.
    pub fn inverse(m: &Self) -> Result<Self, CalcError> {
        if m.rows != m.cols {
            return Err(CalcError::new(
                ErrorKind::DimensionMismatch,
                format!(
                    "La inversa solo está definida para matrices cuadradas (recibida {}×{})",
                    m.rows, m.cols
                ),
            ));
        }

        let det = Self::determinant(m)?;
        if det.abs() < 1e-15 {
            return Err(CalcError::new(
                ErrorKind::DomainError,
                "La matriz es singular (determinante ≈ 0), no tiene inversa",
            ));
        }

        match m.rows {
            1 => {
                let mut inv = Self::new(1, 1);
                inv.data[0][0] = 1.0 / m.data[0][0];
                Ok(inv)
            }
            2 => {
                let a = &m.data;
                let mut inv = Self::new(2, 2);
                inv.data[0][0] = a[1][1] / det;
                inv.data[0][1] = -a[0][1] / det;
                inv.data[1][0] = -a[1][0] / det;
                inv.data[1][1] = a[0][0] / det;
                Ok(inv)
            }
            3 => {
                // Usando la fórmula de la matriz adjunta / determinante.
                let a = &m.data;
                let mut cof = Self::new(3, 3);

                // Cofactores (con signo incluido).
                cof.data[0][0] = (a[1][1] * a[2][2] - a[1][2] * a[2][1]);
                cof.data[0][1] = -(a[0][1] * a[2][2] - a[0][2] * a[2][1]);
                cof.data[0][2] = (a[0][1] * a[1][2] - a[0][2] * a[1][1]);

                cof.data[1][0] = -(a[1][0] * a[2][2] - a[1][2] * a[2][0]);
                cof.data[1][1] = (a[0][0] * a[2][2] - a[0][2] * a[2][0]);
                cof.data[1][2] = -(a[0][0] * a[1][2] - a[0][2] * a[1][0]);

                cof.data[2][0] = (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
                cof.data[2][1] = -(a[0][0] * a[2][1] - a[0][1] * a[2][0]);
                cof.data[2][2] = (a[0][0] * a[1][1] - a[0][1] * a[1][0]);

                // Transponer para obtener la adjunta y dividir por determinante.
                let mut inv = Self::new(3, 3);
                for i in 0..3 {
                    for j in 0..3 {
                        inv.data[i][j] = cof.data[j][i] / det;
                    }
                }
                Ok(inv)
            }
            _ => Err(CalcError::new(
                ErrorKind::DomainError,
                format!(
                    "Inversa solo soportada hasta 3×3 (recibida {}×{})",
                    m.rows, m.cols
                ),
            )),
        }
    }

    // ── Formato ────────────────────────────────────────────────────────────

    /// Formatea la matriz como string multilínea para display visual.
    ///
    /// Usa un formato de caja con bordes Unicode.
    pub fn format(m: &Self) -> String {
        if m.rows == 0 || m.cols == 0 {
            return "[]".to_string();
        }

        // Formatear cada celda como string.
        let mut cell_strings: Vec<Vec<String>> = Vec::with_capacity(m.rows);
        for i in 0..m.rows {
            let mut row_strs = Vec::with_capacity(m.cols);
            for j in 0..m.cols {
                row_strs.push(format_f64_cell(m.data[i][j]));
            }
            cell_strings.push(row_strs);
        }

        // Calcular ancho máximo por columna.
        let mut col_widths: Vec<usize> = vec![0; m.cols];
        for i in 0..m.rows {
            for j in 0..m.cols {
                col_widths[j] = col_widths[j].max(cell_strings[i][j].len());
            }
        }

        // Construir la salida.
        let mut result = String::new();
        for i in 0..m.rows {
            if i == 0 {
                result.push_str("┌ ");
            } else if i == m.rows - 1 {
                result.push_str("└ ");
            } else {
                result.push_str("│ ");
            }
            for j in 0..m.cols {
                let cell = &cell_strings[i][j];
                result.push_str(&format!(
                    "{:>width$}",
                    cell,
                    width = col_widths[j]
                ));
                if j < m.cols - 1 {
                    result.push_str("  ");
                }
            }
            if i == 0 {
                result.push_str(" ┐\n");
            } else if i == m.rows - 1 {
                result.push_str(" ┘");
            } else {
                result.push_str(" │\n");
            }
        }

        result
    }
}

/// Formatea un f64 para celda de matriz.
fn format_f64_cell(v: f64) -> String {
    let v = if v == 0.0 { 0.0 } else { v };
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "∞" } else { "-∞" }.to_string();
    }

    // Redondear a 10 decimales significativos para la celda.
    if v.fract() == 0.0 && v.abs() < 1e15 {
        return format!("{:.0}", v);
    }

    let s = format!("{:.10}", v);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    if s.is_empty() || s == "-" {
        "0".to_string()
    } else {
        s.to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix_eq(a: &Matrix, b: &Matrix, tolerance: f64) {
        assert_eq!(
            a.rows, b.rows,
            "Filas distintas: {} vs {}",
            a.rows, b.rows
        );
        assert_eq!(
            a.cols, b.cols,
            "Columnas distintas: {} vs {}",
            a.cols, b.cols
        );
        for i in 0..a.rows {
            for j in 0..a.cols {
                assert!(
                    (a.data[i][j] - b.data[i][j]).abs() < tolerance,
                    "Diferencia en [{},{}]: {} vs {}",
                    i, j, a.data[i][j], b.data[i][j]
                );
            }
        }
    }

    // ─── Constructores ─────────────────────────────────────────────────

    #[test]
    fn test_new_zero_matrix() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(m.data[i][j], 0.0);
            }
        }
    }

    #[test]
    fn test_from_vec_valid() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.data[0][0], 1.0);
        assert_eq!(m.data[0][1], 2.0);
        assert_eq!(m.data[1][0], 3.0);
        assert_eq!(m.data[1][1], 4.0);
    }

    #[test]
    fn test_from_vec_inconsistent_rows() {
        let result = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0]]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DimensionMismatch);
    }

    #[test]
    fn test_from_vec_empty() {
        let result = Matrix::from_vec(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_vec_empty_rows() {
        let result = Matrix::from_vec(vec![vec![]]);
        assert!(result.is_err());
    }

    #[test]
    fn test_identity() {
        let i3 = Matrix::identity(3);
        assert_eq!(i3.rows, 3);
        assert_eq!(i3.cols, 3);
        for r in 0..3 {
            for c in 0..3 {
                assert_eq!(i3.data[r][c], if r == c { 1.0 } else { 0.0 });
            }
        }
    }

    // ─── Suma ──────────────────────────────────────────────────────────

    #[test]
    fn test_add_2x2() {
        let a = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let c = Matrix::add(&a, &b).unwrap();
        let expected =
            Matrix::from_vec(vec![vec![6.0, 8.0], vec![10.0, 12.0]]).unwrap();
        assert_matrix_eq(&c, &expected, 1e-10);
    }

    // ─── Resta ─────────────────────────────────────────────────────────

    #[test]
    fn test_subtract_2x2() {
        let a = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let b = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let c = Matrix::subtract(&a, &b).unwrap();
        let expected =
            Matrix::from_vec(vec![vec![4.0, 4.0], vec![4.0, 4.0]]).unwrap();
        assert_matrix_eq(&c, &expected, 1e-10);
    }

    // ─── Multiplicación ───────────────────────────────────────────────

    #[test]
    fn test_multiply_2x2() {
        let a = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let c = Matrix::multiply(&a, &b).unwrap();
        // [1*5+2*7=19, 1*6+2*8=22; 3*5+4*7=43, 3*6+4*8=50]
        let expected =
            Matrix::from_vec(vec![vec![19.0, 22.0], vec![43.0, 50.0]]).unwrap();
        assert_matrix_eq(&c, &expected, 1e-10);
    }

    #[test]
    fn test_multiply_2x3_by_3x2() {
        let a = Matrix::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let b = Matrix::from_vec(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]).unwrap();
        let c = Matrix::multiply(&a, &b).unwrap();
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        // [1*7+2*9+3*11=58, 1*8+2*10+3*12=64; 4*7+5*9+6*11=139, 4*8+5*10+6*12=154]
        assert!((c.data[0][0] - 58.0).abs() < 1e-10);
        assert!((c.data[0][1] - 64.0).abs() < 1e-10);
        assert!((c.data[1][0] - 139.0).abs() < 1e-10);
        assert!((c.data[1][1] - 154.0).abs() < 1e-10);
    }

    // ─── Error de dimensión en suma ──────────────────────────────────

    #[test]
    fn test_add_dimension_mismatch() {
        let a = Matrix::new(2, 2);
        let b = Matrix::new(3, 3);
        let result = Matrix::add(&a, &b);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DimensionMismatch);
    }

    // ─── Determinante ─────────────────────────────────────────────────

    #[test]
    fn test_determinant_2x2() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let det = Matrix::determinant(&m).unwrap();
        assert!((det - (-2.0)).abs() < 1e-10, "det = -2, got {}", det);
    }

    #[test]
    fn test_determinant_3x3() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 0.0],
        ])
        .unwrap();
        let det = Matrix::determinant(&m).unwrap();
        assert!((det - 27.0).abs() < 1e-10, "det = 27, got {}", det);
    }

    #[test]
    fn test_determinant_identity() {
        let i3 = Matrix::identity(3);
        let det = Matrix::determinant(&i3).unwrap();
        assert!((det - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_determinant_singular() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ])
        .unwrap();
        let det = Matrix::determinant(&m).unwrap();
        assert!(det.abs() < 1e-10, "Matriz singular, det = 0, got {}", det);
    }

    #[test]
    fn test_determinant_not_square() {
        let m = Matrix::new(2, 3);
        let result = Matrix::determinant(&m);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::DimensionMismatch);
    }

    // ─── Identidad × Matriz = Matriz ────────────────────────────────

    #[test]
    fn test_identity_times_matrix() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ])
        .unwrap();
        let i3 = Matrix::identity(3);
        let result = Matrix::multiply(&i3, &m).unwrap();
        assert_matrix_eq(&result, &m, 1e-10);

        let result2 = Matrix::multiply(&m, &i3).unwrap();
        assert_matrix_eq(&result2, &m, 1e-10);
    }

    // ─── Transpuesta ─────────────────────────────────────────────────

    #[test]
    fn test_transpose_2x3_to_3x2() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let t = Matrix::transpose(&m);
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.data[0][0], 1.0);
        assert_eq!(t.data[0][1], 4.0);
        assert_eq!(t.data[1][0], 2.0);
        assert_eq!(t.data[1][1], 5.0);
        assert_eq!(t.data[2][0], 3.0);
        assert_eq!(t.data[2][1], 6.0);
    }

    #[test]
    fn test_transpose_square() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let t = Matrix::transpose(&m);
        let expected =
            Matrix::from_vec(vec![vec![1.0, 3.0], vec![2.0, 4.0]]).unwrap();
        assert_matrix_eq(&t, &expected, 1e-10);
    }

    // ─── Inversa ──────────────────────────────────────────────────────

    #[test]
    fn test_inverse_2x2() {
        let m = Matrix::from_vec(vec![vec![4.0, 7.0], vec![2.0, 6.0]]).unwrap();
        let inv = Matrix::inverse(&m).unwrap();
        // inv = [0.6, -0.7; -0.2, 0.4]
        assert!((inv.data[0][0] - 0.6).abs() < 1e-10);
        assert!((inv.data[0][1] - (-0.7)).abs() < 1e-10);
        assert!((inv.data[1][0] - (-0.2)).abs() < 1e-10);
        assert!((inv.data[1][1] - 0.4).abs() < 1e-10);

        // Verificar: M * M⁻¹ = I
        let identity = Matrix::multiply(&m, &inv).unwrap();
        let expected_i = Matrix::identity(2);
        assert_matrix_eq(&identity, &expected_i, 1e-10);
    }

    #[test]
    fn test_inverse_singular_error() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![2.0, 4.0]]).unwrap();
        let result = Matrix::inverse(&m);
        assert!(result.is_err());
    }

    // ─── Multiplicación escalar ──────────────────────────────────────

    #[test]
    fn test_scalar_multiply() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let result = Matrix::scalar_multiply(&m, 2.0);
        let expected =
            Matrix::from_vec(vec![vec![2.0, 4.0], vec![6.0, 8.0]]).unwrap();
        assert_matrix_eq(&result, &expected, 1e-10);
    }

    // ─── Formato ──────────────────────────────────────────────────────

    #[test]
    fn test_format_2x2() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let s = Matrix::format(&m);
        assert!(s.contains("1"));
        assert!(s.contains("2"));
        assert!(s.contains("3"));
        assert!(s.contains("4"));
    }
}
