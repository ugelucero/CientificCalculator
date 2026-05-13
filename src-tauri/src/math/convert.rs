//! Motor de conversión de unidades.
//!
//! Proporciona un registro de unidades (`UnitRegistry`) y una función
//! pública [`convert`] que recibe un `ConversionRequest` y devuelve
//! un `ConversionResult` con el valor convertido y formateado.
//!
//! Soporta 10 categorías: Length, Mass, Temperature, Volume, Area,
//! Time, Speed, Pressure, Energy, Data.
//!
//! La temperatura se maneja con fórmulas no lineales (C ↔ F ↔ K).
//! El resto de categorías usan conversión lineal vía factor SI.

use std::collections::HashMap;

use crate::models::errors::{CalcError, ErrorKind};
use crate::models::types::{ConversionRequest, ConversionResult, UnitCategory, UnitInfo};

/// Registro global de unidades accesible por símbolo (case-insensitive).
pub struct UnitRegistry {
    units: HashMap<String, UnitInfo>,
}

impl UnitRegistry {
    /// Construye el registro con todas las unidades comunes.
    pub fn new() -> Self {
        let mut registry = Self {
            units: HashMap::new(),
        };

        // ── Length (base: metre) ────────────────────────────────────
        registry.add(UnitInfo {
            name: "Millimeter".into(),
            symbol: "mm".into(),
            category: UnitCategory::Length,
            to_si: 0.001,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Centimeter".into(),
            symbol: "cm".into(),
            category: UnitCategory::Length,
            to_si: 0.01,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Meter".into(),
            symbol: "m".into(),
            category: UnitCategory::Length,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilometer".into(),
            symbol: "km".into(),
            category: UnitCategory::Length,
            to_si: 1000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Inch".into(),
            symbol: "in".into(),
            category: UnitCategory::Length,
            to_si: 0.0254,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Foot".into(),
            symbol: "ft".into(),
            category: UnitCategory::Length,
            to_si: 0.3048,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Yard".into(),
            symbol: "yd".into(),
            category: UnitCategory::Length,
            to_si: 0.9144,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Mile".into(),
            symbol: "mi".into(),
            category: UnitCategory::Length,
            to_si: 1609.344,
            offset: 0.0,
            is_linear: true,
        });

        // ── Mass (base: kilogram) ───────────────────────────────────
        registry.add(UnitInfo {
            name: "Milligram".into(),
            symbol: "mg".into(),
            category: UnitCategory::Mass,
            to_si: 1e-6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Gram".into(),
            symbol: "g".into(),
            category: UnitCategory::Mass,
            to_si: 0.001,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilogram".into(),
            symbol: "kg".into(),
            category: UnitCategory::Mass,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Tonne".into(),
            symbol: "ton".into(),
            category: UnitCategory::Mass,
            to_si: 1000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Pound".into(),
            symbol: "lb".into(),
            category: UnitCategory::Mass,
            to_si: 0.45359237,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Ounce".into(),
            symbol: "oz".into(),
            category: UnitCategory::Mass,
            to_si: 0.028349523125,
            offset: 0.0,
            is_linear: true,
        });

        // ── Temperature (base: Kelvin) ──────────────────────────────
        registry.add(UnitInfo {
            name: "Celsius".into(),
            symbol: "C".into(),
            category: UnitCategory::Temperature,
            to_si: 1.0,
            offset: 273.15,
            is_linear: false,
        });
        registry.add(UnitInfo {
            name: "Fahrenheit".into(),
            symbol: "F".into(),
            category: UnitCategory::Temperature,
            to_si: 5.0 / 9.0,
            offset: 459.67,
            is_linear: false,
        });
        registry.add(UnitInfo {
            name: "Kelvin".into(),
            symbol: "K".into(),
            category: UnitCategory::Temperature,
            to_si: 1.0,
            offset: 0.0,
            is_linear: false,
        });

        // ── Volume (base: litre) ────────────────────────────────────
        registry.add(UnitInfo {
            name: "Millilitre".into(),
            symbol: "mL".into(),
            category: UnitCategory::Volume,
            to_si: 0.001,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Litre".into(),
            symbol: "L".into(),
            category: UnitCategory::Volume,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Gallon".into(),
            symbol: "gal".into(),
            category: UnitCategory::Volume,
            to_si: 3.78541,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Fluid Ounce".into(),
            symbol: "fl_oz".into(),
            category: UnitCategory::Volume,
            to_si: 0.0295735,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Cup".into(),
            symbol: "cup".into(),
            category: UnitCategory::Volume,
            to_si: 0.236588,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Pint".into(),
            symbol: "pt".into(),
            category: UnitCategory::Volume,
            to_si: 0.473176,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Quart".into(),
            symbol: "qt".into(),
            category: UnitCategory::Volume,
            to_si: 0.946353,
            offset: 0.0,
            is_linear: true,
        });

        // ── Area (base: square metre) ───────────────────────────────
        registry.add(UnitInfo {
            name: "Square Millimetre".into(),
            symbol: "mm2".into(),
            category: UnitCategory::Area,
            to_si: 1e-6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Square Centimetre".into(),
            symbol: "cm2".into(),
            category: UnitCategory::Area,
            to_si: 0.0001,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Square Metre".into(),
            symbol: "m2".into(),
            category: UnitCategory::Area,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Square Kilometre".into(),
            symbol: "km2".into(),
            category: UnitCategory::Area,
            to_si: 1e6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Hectare".into(),
            symbol: "ha".into(),
            category: UnitCategory::Area,
            to_si: 10000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Acre".into(),
            symbol: "acre".into(),
            category: UnitCategory::Area,
            to_si: 4046.8564224,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Square Foot".into(),
            symbol: "ft2".into(),
            category: UnitCategory::Area,
            to_si: 0.09290304,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Square Inch".into(),
            symbol: "in2".into(),
            category: UnitCategory::Area,
            to_si: 0.00064516,
            offset: 0.0,
            is_linear: true,
        });

        // ── Time (base: second) ─────────────────────────────────────
        registry.add(UnitInfo {
            name: "Millisecond".into(),
            symbol: "ms".into(),
            category: UnitCategory::Time,
            to_si: 0.001,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Second".into(),
            symbol: "s".into(),
            category: UnitCategory::Time,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Minute".into(),
            symbol: "min".into(),
            category: UnitCategory::Time,
            to_si: 60.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Hour".into(),
            symbol: "h".into(),
            category: UnitCategory::Time,
            to_si: 3600.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Day".into(),
            symbol: "day".into(),
            category: UnitCategory::Time,
            to_si: 86400.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Week".into(),
            symbol: "week".into(),
            category: UnitCategory::Time,
            to_si: 604800.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Year".into(),
            symbol: "year".into(),
            category: UnitCategory::Time,
            to_si: 31557600.0,
            offset: 0.0,
            is_linear: true,
        });

        // ── Speed (base: m/s) ───────────────────────────────────────
        registry.add(UnitInfo {
            name: "Meters per Second".into(),
            symbol: "m/s".into(),
            category: UnitCategory::Speed,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilometers per Hour".into(),
            symbol: "km/h".into(),
            category: UnitCategory::Speed,
            to_si: 1.0 / 3.6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Miles per Hour".into(),
            symbol: "mph".into(),
            category: UnitCategory::Speed,
            to_si: 0.44704,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Knot".into(),
            symbol: "kn".into(),
            category: UnitCategory::Speed,
            to_si: 0.514444444,
            offset: 0.0,
            is_linear: true,
        });

        // ── Pressure (base: pascal) ─────────────────────────────────
        registry.add(UnitInfo {
            name: "Pascal".into(),
            symbol: "Pa".into(),
            category: UnitCategory::Pressure,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilopascal".into(),
            symbol: "kPa".into(),
            category: UnitCategory::Pressure,
            to_si: 1000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Megapascal".into(),
            symbol: "MPa".into(),
            category: UnitCategory::Pressure,
            to_si: 1e6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Bar".into(),
            symbol: "bar".into(),
            category: UnitCategory::Pressure,
            to_si: 100000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Atmosphere".into(),
            symbol: "atm".into(),
            category: UnitCategory::Pressure,
            to_si: 101325.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Pounds per Square Inch".into(),
            symbol: "psi".into(),
            category: UnitCategory::Pressure,
            to_si: 6894.75729,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Millimetre of Mercury".into(),
            symbol: "mmHg".into(),
            category: UnitCategory::Pressure,
            to_si: 133.322368,
            offset: 0.0,
            is_linear: true,
        });

        // ── Energy (base: joule) ────────────────────────────────────
        registry.add(UnitInfo {
            name: "Joule".into(),
            symbol: "J".into(),
            category: UnitCategory::Energy,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilojoule".into(),
            symbol: "kJ".into(),
            category: UnitCategory::Energy,
            to_si: 1000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Calorie".into(),
            symbol: "cal".into(),
            category: UnitCategory::Energy,
            to_si: 4.184,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilocalorie".into(),
            symbol: "kcal".into(),
            category: UnitCategory::Energy,
            to_si: 4184.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Watt-hour".into(),
            symbol: "Wh".into(),
            category: UnitCategory::Energy,
            to_si: 3600.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilowatt-hour".into(),
            symbol: "kWh".into(),
            category: UnitCategory::Energy,
            to_si: 3.6e6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Electronvolt".into(),
            symbol: "eV".into(),
            category: UnitCategory::Energy,
            to_si: 1.602176634e-19,
            offset: 0.0,
            is_linear: true,
        });

        // ── Data (base: byte, prefijos decimales) ───────────────────
        registry.add(UnitInfo {
            name: "Bit".into(),
            symbol: "bit".into(),
            category: UnitCategory::Data,
            to_si: 0.125,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Byte".into(),
            symbol: "B".into(),
            category: UnitCategory::Data,
            to_si: 1.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Kilobyte".into(),
            symbol: "KB".into(),
            category: UnitCategory::Data,
            to_si: 1000.0,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Megabyte".into(),
            symbol: "MB".into(),
            category: UnitCategory::Data,
            to_si: 1e6,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Gigabyte".into(),
            symbol: "GB".into(),
            category: UnitCategory::Data,
            to_si: 1e9,
            offset: 0.0,
            is_linear: true,
        });
        registry.add(UnitInfo {
            name: "Terabyte".into(),
            symbol: "TB".into(),
            category: UnitCategory::Data,
            to_si: 1e12,
            offset: 0.0,
            is_linear: true,
        });

        registry
    }

    fn add(&mut self, info: UnitInfo) {
        // Insertar por símbolo (minúsculas) y por nombre (minúsculas)
        self.units
            .insert(info.symbol.to_lowercase(), info.clone());
        self.units
            .insert(info.name.to_lowercase(), info);
    }

    /// Busca una unidad por símbolo o nombre (case-insensitive).
    pub fn lookup(&self, key: &str) -> Option<&UnitInfo> {
        self.units.get(&key.to_lowercase())
    }

    /// Devuelve todas las unidades de una categoría dada.
    pub fn by_category(&self, category: &UnitCategory) -> Vec<&UnitInfo> {
        let mut result: Vec<&UnitInfo> = self
            .units
            .values()
            .filter(|u| u.category == *category)
            .collect();
        // Dedesduplicar (misma unidad insertada por nombre y símbolo)
        result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        result.dedup_by(|a, b| a.symbol == b.symbol);
        result
    }
}

/// Convierte una temperatura entre Celsius, Fahrenheit y Kelvin.
///
/// Se usan fórmulas directas porque la conversión no es lineal.
fn convert_temperature(value: f64, from_sym: &str, to_sym: &str) -> f64 {
    match (from_sym, to_sym) {
        ("C", "F") => value * 9.0 / 5.0 + 32.0,
        ("C", "K") => value + 273.15,
        ("F", "C") => (value - 32.0) * 5.0 / 9.0,
        ("F", "K") => (value + 459.67) * 5.0 / 9.0,
        ("K", "C") => value - 273.15,
        ("K", "F") => value * 9.0 / 5.0 - 459.67,
        _ => value, // Misma unidad
    }
}

/// Formatea un número con precisión adecuada para visualización.
fn format_value(value: f64) -> String {
    if value.is_nan() {
        return "NaN".into();
    }
    if !value.is_finite() {
        return if value.is_sign_positive() { "∞" } else { "-∞" }.into();
    }

    let abs = value.abs();

    // Para valores muy grandes o muy pequeños, usar notación científica
    if abs >= 1e12 || (abs < 1e-10 && abs > 0.0) {
        return format!("{:.6e}", value);
    }

    // Para valores con parte decimal significativa
    if abs < 1e6 && (abs - abs.round()).abs() > 1e-10 {
        let formatted = format!("{:.10}", value);
        // Eliminar ceros finales y punto decimal sobrante
        let trimmed = formatted
            .trim_end_matches('0')
            .trim_end_matches('.');
        return trimmed.to_string();
    }

    // Para valores enteros o grandes sin notación científica
    if value == value.round() && abs < 1e12 {
        return format!("{:.0}", value);
    }

    format!("{:.6}", value)
}

/// Realiza la conversión de unidades.
///
/// # Argumentos
///
/// * `request` - `ConversionRequest` con valor, unidad origen y unidad destino.
///
/// # Errores
///
/// * `InvalidConversion` si alguna unidad no se encuentra.
/// * `InvalidConversion` si las unidades son de distinta categoría.
pub fn convert(request: &ConversionRequest) -> Result<ConversionResult, CalcError> {
    let registry = UnitRegistry::new();

    let from_info = registry.lookup(&request.from_unit).ok_or_else(|| {
        CalcError::new(
            ErrorKind::InvalidConversion,
            format!("Unknown unit: '{}'", request.from_unit),
        )
    })?;

    let to_info = registry.lookup(&request.to_unit).ok_or_else(|| {
        CalcError::new(
            ErrorKind::InvalidConversion,
            format!("Unknown unit: '{}'", request.to_unit),
        )
    })?;

    if from_info.category != to_info.category {
        return Err(CalcError::new(
            ErrorKind::InvalidConversion,
            format!(
                "Cannot convert '{}' ({:?}) to '{}' ({:?}): incompatible categories",
                from_info.symbol,
                from_info.category,
                to_info.symbol,
                to_info.category
            ),
        ));
    }

    let result = if from_info.category == UnitCategory::Temperature {
        convert_temperature(request.value, &from_info.symbol, &to_info.symbol)
    } else {
        // Fórmula lineal: (value * from.to_si + from.offset) / to.to_si - to.offset
        (request.value * from_info.to_si + from_info.offset) / to_info.to_si
            - to_info.offset
    };

    Ok(ConversionResult {
        value: result,
        formatted: format_value(result),
        from: from_info.symbol.clone(),
        to: to_info.symbol.clone(),
    })
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn conv(value: f64, from: &str, to: &str) -> f64 {
        let req = ConversionRequest {
            value,
            from_unit: from.into(),
            to_unit: to.into(),
        };
        convert(&req).unwrap().value
    }

    fn assert_approx(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() < tolerance,
            "assertion failed: {} ≈ {} (diff: {})",
            actual,
            expected,
            (actual - expected).abs()
        );
    }

    #[test]
    fn test_length_m_to_cm() {
        assert_approx(conv(1.0, "m", "cm"), 100.0, 1e-10);
    }

    #[test]
    fn test_length_m_to_km() {
        assert_approx(conv(1000.0, "m", "km"), 1.0, 1e-10);
    }

    #[test]
    fn test_length_inch_to_cm() {
        assert_approx(conv(1.0, "in", "cm"), 2.54, 1e-10);
    }

    #[test]
    fn test_length_mile_to_km() {
        assert_approx(conv(1.0, "mi", "km"), 1.609344, 1e-6);
    }

    #[test]
    fn test_mass_kg_to_lb() {
        assert_approx(conv(1.0, "kg", "lb"), 2.20462, 1e-4);
    }

    #[test]
    fn test_mass_kg_to_g() {
        assert_approx(conv(1.0, "kg", "g"), 1000.0, 1e-10);
    }

    #[test]
    fn test_mass_lb_to_kg() {
        assert_approx(conv(1.0, "lb", "kg"), 0.453592, 1e-5);
    }

    #[test]
    fn test_mass_oz_to_g() {
        assert_approx(conv(1.0, "oz", "g"), 28.349523125, 1e-6);
    }

    #[test]
    fn test_temperature_c_to_f() {
        assert_approx(conv(0.0, "C", "F"), 32.0, 1e-10);
    }

    #[test]
    fn test_temperature_f_to_c() {
        assert_approx(conv(100.0, "F", "C"), 37.7777777778, 1e-4);
    }

    #[test]
    fn test_temperature_c_to_k() {
        assert_approx(conv(0.0, "C", "K"), 273.15, 1e-10);
    }

    #[test]
    fn test_temperature_k_to_c() {
        assert_approx(conv(273.15, "K", "C"), 0.0, 1e-10);
    }

    #[test]
    fn test_temperature_f_to_k() {
        assert_approx(conv(32.0, "F", "K"), 273.15, 1e-10);
    }

    #[test]
    fn test_temperature_k_to_f() {
        assert_approx(conv(273.15, "K", "F"), 32.0, 1e-4);
    }

    #[test]
    fn test_temperature_same_unit() {
        assert_approx(conv(100.0, "C", "C"), 100.0, 1e-10);
        assert_approx(conv(50.0, "F", "F"), 50.0, 1e-10);
    }

    #[test]
    fn test_volume_gal_to_l() {
        assert_approx(conv(1.0, "gal", "L"), 3.78541, 1e-5);
    }

    #[test]
    fn test_volume_l_to_gal() {
        assert_approx(conv(3.78541, "L", "gal"), 1.0, 1e-4);
    }

    #[test]
    fn test_area_acre_to_m2() {
        assert_approx(conv(1.0, "acre", "m2"), 4046.8564224, 1e-6);
    }

    #[test]
    fn test_time_h_to_s() {
        assert_approx(conv(1.0, "h", "s"), 3600.0, 1e-10);
    }

    #[test]
    fn test_time_day_to_h() {
        assert_approx(conv(1.0, "day", "h"), 24.0, 1e-10);
    }

    #[test]
    fn test_speed_kmh_to_ms() {
        assert_approx(conv(36.0, "km/h", "m/s"), 10.0, 1e-10);
    }

    #[test]
    fn test_speed_mph_to_kmh() {
        assert_approx(conv(60.0, "mph", "km/h"), 96.56064, 1e-1);
    }

    #[test]
    fn test_pressure_atm_to_pa() {
        assert_approx(conv(1.0, "atm", "Pa"), 101325.0, 1e-10);
    }

    #[test]
    fn test_pressure_bar_to_psi() {
        assert_approx(conv(1.0, "bar", "psi"), 14.5038, 1e-3);
    }

    #[test]
    fn test_energy_kcal_to_j() {
        assert_approx(conv(1.0, "kcal", "J"), 4184.0, 1e-10);
    }

    #[test]
    fn test_energy_kwh_to_j() {
        assert_approx(conv(1.0, "kWh", "J"), 3600000.0, 1e-10);
    }

    #[test]
    fn test_data_mb_to_kb() {
        assert_approx(conv(1.0, "MB", "KB"), 1000.0, 1e-10);
    }

    #[test]
    fn test_data_gb_to_mb() {
        assert_approx(conv(1.0, "GB", "MB"), 1000.0, 1e-10);
    }

    #[test]
    fn test_data_bit_to_byte() {
        assert_approx(conv(8.0, "bit", "B"), 1.0, 1e-10);
    }

    #[test]
    fn test_error_incompatible_categories() {
        let req = ConversionRequest {
            value: 1.0,
            from_unit: "m".into(),
            to_unit: "kg".into(),
        };
        let result = convert(&req);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidConversion);
    }

    #[test]
    fn test_error_unknown_unit() {
        let req = ConversionRequest {
            value: 1.0,
            from_unit: "xyzzy".into(),
            to_unit: "m".into(),
        };
        let result = convert(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_case_insensitive_lookup() {
        assert_approx(conv(1.0, "KM", "m"), 1000.0, 1e-10);
        assert_approx(conv(1.0, "Kg", "LB"), 2.20462, 1e-4);
    }

    #[test]
    fn test_formatted_result() {
        let req = ConversionRequest {
            value: 100.0,
            from_unit: "m".into(),
            to_unit: "cm".into(),
        };
        let result = convert(&req).unwrap();
        assert_eq!(result.formatted, "10000");
        assert_eq!(result.from, "m");
        assert_eq!(result.to, "cm");
    }

    #[test]
    fn test_by_category() {
        let registry = UnitRegistry::new();
        let lengths = registry.by_category(&UnitCategory::Length);
        assert_eq!(lengths.len(), 8); // mm, cm, m, km, in, ft, yd, mi
    }
}
