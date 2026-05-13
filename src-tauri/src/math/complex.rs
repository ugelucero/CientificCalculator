//! Aritmética de números complejos.
//!
//! Proporciona el tipo `Complex` con operaciones aritméticas básicas,
//! funciones trascendentes con soporte para fórmula de Euler, y
//! utilidades de formato.

use std::fmt;
use std::f64::consts;

// ═══════════════════════════════════════════════════════════════════════════
// Complex
// ═══════════════════════════════════════════════════════════════════════════

/// Número complejo con partes real e imaginaria en `f64`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    /// Constante imaginaria pura: `i`.
    pub const I: Complex = Complex { real: 0.0, imag: 1.0 };

    /// Constante cero.
    pub const ZERO: Complex = Complex { real: 0.0, imag: 0.0 };

    /// Constante uno real.
    pub const ONE: Complex = Complex { real: 1.0, imag: 0.0 };

    // ── Constructores ──────────────────────────────────────────────────

    /// Crea un complejo desde partes real e imaginaria.
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// Crea un complejo desde coordenadas polares (radio, ángulo en radianes).
    pub fn from_polar(radius: f64, angle: f64) -> Self {
        Self {
            real: radius * angle.cos(),
            imag: radius * angle.sin(),
        }
    }

    // ── Propiedades ────────────────────────────────────────────────────

    /// Módulo (valor absoluto, magnitud).
    pub fn abs(self) -> f64 {
        self.real.hypot(self.imag)
    }

    /// Norma al cuadrado (|z|² = a² + b²).
    pub fn norm_sqr(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    /// Argumento (fase) en radianes, en el rango (−π, π].
    pub fn arg(self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Conjugado: `a − bi`.
    pub fn conj(self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// ¿Es un número real? (parte imaginaria ≈ 0)
    pub fn is_real(self) -> bool {
        self.imag.abs() < 1e-12
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Operadores aritméticos
// ═══════════════════════════════════════════════════════════════════════════

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
        }
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Conversiones desde f64
// ═══════════════════════════════════════════════════════════════════════════

impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Self { real, imag: 0.0 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Display / formateo
// ═══════════════════════════════════════════════════════════════════════════

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag == 0.0 {
            write!(f, "{}", format_f64(self.real))
        } else if self.real == 0.0 {
            if self.imag == 1.0 {
                write!(f, "i")
            } else if self.imag == -1.0 {
                write!(f, "-i")
            } else {
                write!(f, "{}i", format_f64(self.imag))
            }
        } else {
            if self.imag > 0.0 {
                if self.imag == 1.0 {
                    write!(f, "{} + i", format_f64(self.real))
                } else {
                    write!(f, "{} + {}i", format_f64(self.real), format_f64(self.imag))
                }
            } else {
                if self.imag == -1.0 {
                    write!(f, "{} - i", format_f64(self.real))
                } else {
                    write!(f, "{} - {}i", format_f64(self.real), format_f64(-self.imag))
                }
            }
        }
    }
}

/// Formatea un f64 limpiamente (sin -0, sin .0 sobrante).
fn format_f64(v: f64) -> String {
    let v = if v == 0.0 { 0.0 } else { v };
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "∞" } else { "-∞" }.to_string();
    }
    let s = format!("{:.15}", v);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    if s.is_empty() || s == "-" {
        "0".to_string()
    } else {
        s.to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Funciones complejas (fórmula de Euler)
// ═══════════════════════════════════════════════════════════════════════════

/// Exponencial compleja: `e^z = e^x (cos y + i sin y)`.
pub fn cexp(z: Complex) -> Complex {
    let exp_real = z.real.exp();
    Complex {
        real: exp_real * z.imag.cos(),
        imag: exp_real * z.imag.sin(),
    }
}

/// Seno complejo: `sin(z) = sin(a)cosh(b) + i cos(a)sinh(b)`.
pub fn csin(z: Complex) -> Complex {
    Complex {
        real: z.real.sin() * z.imag.cosh(),
        imag: z.real.cos() * z.imag.sinh(),
    }
}

/// Coseno complejo: `cos(z) = cos(a)cosh(b) − i sin(a)sinh(b)`.
pub fn ccos(z: Complex) -> Complex {
    Complex {
        real: z.real.cos() * z.imag.cosh(),
        imag: -z.real.sin() * z.imag.sinh(),
    }
}

/// Tangente compleja: `tan(z) = sin(z) / cos(z)`.
pub fn ctan(z: Complex) -> Complex {
    csin(z) / ccos(z)
}

/// Logaritmo natural complejo: `ln(z) = ln|z| + i·arg(z)`.
pub fn cln(z: Complex) -> Complex {
    Complex {
        real: z.abs().ln(),
        imag: z.arg(),
    }
}

/// Logaritmo base 10.
pub fn clog10(z: Complex) -> Complex {
    cln(z) / Complex::from(consts::LN_10)
}

/// Logaritmo base 2.
pub fn clog2(z: Complex) -> Complex {
    cln(z) / Complex::from(consts::LN_2)
}

/// Raíz cuadrada compleja (rama principal).
pub fn csqrt(z: Complex) -> Complex {
    if z.imag == 0.0 && z.real >= 0.0 {
        Complex::new(z.real.sqrt(), 0.0)
    } else {
        let r = z.abs();
        let real = ((r + z.real) / 2.0).sqrt();
        let imag = if z.imag >= 0.0 {
            ((r - z.real) / 2.0).sqrt()
        } else {
            -((r - z.real) / 2.0).sqrt()
        };
        Complex::new(real, imag)
    }
}

/// Potencia compleja: `a^b = exp(b · ln(a))`.
/// Para exponentes enteros pequeños usa multiplicación iterativa exacta.
pub fn cpow(a: Complex, b: Complex) -> Complex {
    if b.imag == 0.0 && b.real.fract() == 0.0 && b.real >= 0.0 && b.real <= 100.0 {
        let n = b.real as u64;
        let mut result = Complex::ONE;
        for _ in 0..n {
            result = result * a;
        }
        return result;
    }
    if b.imag == 0.0 && b.real.fract() == 0.0 && b.real < 0.0 && b.real >= -100.0 {
        let n = (-b.real) as u64;
        let mut result = Complex::ONE;
        for _ in 0..n {
            result = result * a;
        }
        return Complex::ONE / result;
    }
    cexp(b * cln(a))
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_complex_eq(a: Complex, b: Complex, tolerance: f64) {
        assert!(
            (a.real - b.real).abs() < tolerance && (a.imag - b.imag).abs() < tolerance,
            "Expected {:?}, got {:?}",
            b,
            a,
        );
    }

    #[test]
    fn test_add() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);
        assert_complex_eq(a + b, Complex::new(4.0, 6.0), 1e-10);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);
        assert_complex_eq(a - b, Complex::new(2.0, 2.0), 1e-10);
    }

    #[test]
    fn test_mul() {
        // (1+i)² = 2i
        let a = Complex::new(1.0, 1.0);
        assert_complex_eq(a * a, Complex::new(0.0, 2.0), 1e-10);
    }

    #[test]
    fn test_i_squared() {
        let i = Complex::I;
        assert_complex_eq(i * i, Complex::new(-1.0, 0.0), 1e-10);
    }

    #[test]
    fn test_div() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);
        // (3+4i)/(1+2i) = (11-2i)/5 = 2.2 - 0.4i
        assert_complex_eq(a / b, Complex::new(2.2, -0.4), 1e-10);
    }

    #[test]
    fn test_abs() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.abs() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_conj() {
        let z = Complex::new(3.0, 4.0);
        assert_complex_eq(z.conj(), Complex::new(3.0, -4.0), 1e-10);
    }

    #[test]
    fn test_arg() {
        let z = Complex::new(1.0, 1.0);
        assert!((z.arg() - consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_from_polar() {
        let z = Complex::from_polar(2.0, consts::PI / 3.0);
        assert_complex_eq(z, Complex::new(1.0, 3.0f64.sqrt()), 1e-10);
    }

    #[test]
    fn test_display_real() {
        assert_eq!(Complex::new(5.0, 0.0).to_string(), "5");
    }

    #[test]
    fn test_display_imaginary() {
        assert_eq!(Complex::new(0.0, 3.0).to_string(), "3i");
    }

    #[test]
    fn test_display_complex() {
        assert_eq!(Complex::new(3.0, 4.0).to_string(), "3 + 4i");
    }

    #[test]
    fn test_display_negative_imag() {
        assert_eq!(Complex::new(3.0, -4.0).to_string(), "3 - 4i");
    }

    #[test]
    fn test_display_unit_imag() {
        assert_eq!(Complex::new(5.0, 1.0).to_string(), "5 + i");
    }

    #[test]
    fn test_exp_zero() {
        assert_complex_eq(cexp(Complex::ZERO), Complex::ONE, 1e-10);
    }

    #[test]
    fn test_exp_ipi() {
        let z = cexp(Complex::new(0.0, consts::PI));
        assert_complex_eq(z + Complex::ONE, Complex::ZERO, 1e-10);
    }

    #[test]
    fn test_sin_zero() {
        assert_complex_eq(csin(Complex::ZERO), Complex::ZERO, 1e-10);
    }

    #[test]
    fn test_cos_zero() {
        assert_complex_eq(ccos(Complex::ZERO), Complex::ONE, 1e-10);
    }

    #[test]
    fn test_ln_e() {
        let e = Complex::new(consts::E, 0.0);
        assert_complex_eq(cln(e), Complex::ONE, 1e-10);
    }

    #[test]
    fn test_sqrt_minus_one() {
        assert_complex_eq(csqrt(Complex::new(-1.0, 0.0)), Complex::I, 1e-10);
    }

    #[test]
    fn test_sqrt_complex() {
        // sqrt(3+4i) = 2+i porque (2+i)² = 3+4i
        assert_complex_eq(csqrt(Complex::new(3.0, 4.0)), Complex::new(2.0, 1.0), 1e-10);
    }

    #[test]
    fn test_pow_real_exponent() {
        // (1+i)^2 = 2i
        assert_complex_eq(
            cpow(Complex::new(1.0, 1.0), Complex::new(2.0, 0.0)),
            Complex::new(0.0, 2.0),
            1e-10,
        );
    }
}
