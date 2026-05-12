/**
 * tauri-bridge.js - Wrapper para invocar comandos Tauri con tipado.
 * Se importa @tauri-apps/api/core dinámicamente para evitar
 * errores en entornos sin Tauri (navegador normal).
 *
 * Incluye un evaluador local como fallback para desarrollo en navegador.
 */

/** @type {import('@tauri-apps/api/core').invoke|null} */
let invokeFn = null;
let tauriAvailable = null; // null = no verificado aún

/** Determina si la app se ejecuta dentro de Tauri */
async function isTauriEnvironment() {
  if (tauriAvailable !== null) return tauriAvailable;
  try {
    const tauriCore = await import('@tauri-apps/api/core');
    tauriAvailable = typeof tauriCore.isTauri === 'function' && tauriCore.isTauri();
    if (tauriAvailable) {
      invokeFn = tauriCore.invoke;
    }
  } catch {
    tauriAvailable = false;
    invokeFn = null;
  }
  return tauriAvailable;
}

async function getInvoke() {
  if (invokeFn === null && tauriAvailable === null) {
    await isTauriEnvironment();
  }
  return invokeFn;
}

// ─── Evaluador local (fallback para desarrollo) ────────────────────────────

let lastAnswer = 0;

/**
 * Factorial para el evaluador local.
 * Soporta enteros positivos (producto iterativo) y 0! = 1.
 * Para valores negativos o no enteros retorna Infinity como fallback.
 */
function localFactorial(n) {
  if (n < 0) {
    throw new Error('Factorial no definido para números negativos');
  }
  if (n === 0 || n === 1) return 1;
  if (n > 170) return Infinity;
  if (!Number.isInteger(n)) {
    // Aproximación gamma simple para no enteros (solo dev)
    return Infinity;
  }
  let result = 1;
  for (let i = 2; i <= n; i++) {
    result *= i;
  }
  return result;
}

/**
 * Convierte un ángulo según el modo angular especificado.
 * @param {number} value - Ángulo en la unidad de origen
 * @param {string} fromMode - 'Deg' | 'Rad' | 'Grad'
 * @param {'toRad'|'fromRad'} direction - 'toRad' para convertir a radianes (sin/cos/tan),
 *                                        'fromRad' para convertir desde radianes (asin/acos/atan)
 * @returns {number}
 */
function convertAngle(value, fromMode, direction) {
  if (fromMode === 'Rad') return value;
  if (direction === 'toRad') {
    if (fromMode === 'Deg') return value * Math.PI / 180;
    if (fromMode === 'Grad') return value * Math.PI / 200;
  } else {
    // fromRad: convertir radianes a la unidad destino
    if (fromMode === 'Deg') return value * 180 / Math.PI;
    if (fromMode === 'Grad') return value * 200 / Math.PI;
  }
  return value;
}

/**
 * Evalúa una expresión localmente para desarrollo en navegador.
 * Soporta: +, -, *, /, ^, %, !, (), sin, cos, tan, asin, acos, atan,
 * sinh, cosh, tanh, log (base 10), ln, log2, log10, sqrt, cbrt, abs,
 * exp, pi, e, ans, phi, c.
 *
 * La precisión es limitada comparada con el backend Rust.
 *
 * @param {string} expression - Expresión matemática en notación infija
 * @param {string} [angleMode='Rad'] - 'Deg' | 'Rad' | 'Grad'
 * @returns {{ result: string, display: string, format: string }}
 */
function evaluateLocal(expression, angleMode = 'Rad') {
  const trimmed = expression.trim();

  if (!trimmed) {
    return { result: '0', display: expression, format: 'Decimal' };
  }

  // Validar solo caracteres permitidos (previene inyección de código)
  if (!/^[\d+\-*/().%\s^!a-zA-Z]+$/.test(trimmed)) {
    throw new Error('Caracteres no permitidos en la expresión');
  }

  // ── Wrappers trigonométricos con conversión angular ──
  const angleMode_ = angleMode || 'Rad';
  const __trig = {
    sin:  (x) => Math.sin(convertAngle(x, angleMode_, 'toRad')),
    cos:  (x) => Math.cos(convertAngle(x, angleMode_, 'toRad')),
    tan:  (x) => Math.tan(convertAngle(x, angleMode_, 'toRad')),
    asin: (x) => convertAngle(Math.asin(x), angleMode_, 'fromRad'),
    acos: (x) => convertAngle(Math.acos(x), angleMode_, 'fromRad'),
    atan: (x) => convertAngle(Math.atan(x), angleMode_, 'fromRad'),
  };

  let expr = trimmed;

  // ── Reemplazar funciones (orden: más largas primero) ──
  // Logaritmos (log10/log2 antes que log genérico)
  expr = expr.replace(/\blog10\(/g, 'Math.log10(');
  expr = expr.replace(/\blog2\(/g,  'Math.log2(');
  expr = expr.replace(/\blog\(/g,   'Math.log10(');  // log = log10
  expr = expr.replace(/\bln\(/g,    'Math.log(');     // ln  = natural

  // Trigonométricas inversas e hiperbólicas (antes que las directas)
  // Trigonométricas INVERSAS (resultado en modo angular)
  expr = expr.replace(/\basin\(/g,  '__trig.asin(');
  expr = expr.replace(/\bacos\(/g,  '__trig.acos(');
  expr = expr.replace(/\batan\(/g,  '__trig.atan(');

  // Trigonométricas HIPERBÓLICAS (no afectadas por modo angular)
  expr = expr.replace(/\bsinh\(/g,  'Math.sinh(');
  expr = expr.replace(/\bcosh\(/g,  'Math.cosh(');
  expr = expr.replace(/\btanh\(/g,  'Math.tanh(');

  // Trigonométricas DIRECTAS (argumento en modo angular → radianes)
  expr = expr.replace(/\bsin\(/g,   '__trig.sin(');
  expr = expr.replace(/\bcos\(/g,   '__trig.cos(');
  expr = expr.replace(/\btan\(/g,   '__trig.tan(');

  // Raíces, abs, exp
  expr = expr.replace(/\bsqrt\(/g,  'Math.sqrt(');
  expr = expr.replace(/\bcbrt\(/g,  'Math.cbrt(');
  expr = expr.replace(/\babs\(/g,   'Math.abs(');
  expr = expr.replace(/\bexp\(/g,   'Math.exp(');

  // ── Constantes (después de funciones para no romper nombres) ──
  expr = expr.replace(/\bpi\b/g,  'Math.PI');
  expr = expr.replace(/\bans\b/g, `(${lastAnswer})`);
  // 'e' como constante: cuidado de no capturar 'e' en notación científica.
  // \be\b no hace match en "1e5" porque no hay word boundary entre dígito y letra.
  expr = expr.replace(/\be\b/g,   'Math.E');

  // phi (φ) constante áurea
  expr = expr.replace(/\bphi\b/g, '(1.618033988749895)');

  // c = velocidad de la luz
  expr = expr.replace(/\bc\b/g,   '(299792458)');

  // ── Operadores ──
  // Potencia: ^ → **
  expr = expr.replace(/\^/g, '**');

  // Factorial (postfijo): n! → localFactorial(n), (expr)! → localFactorial(expr)
  expr = expr.replace(/([\d.]+|\))\s*!/g, 'localFactorial($1)');

  // ── Evaluación segura ──
  let result;
  try {
    // new Function() es más seguro que eval(): no tiene acceso al scope local
    const fn = new Function('Math', 'localFactorial', '__trig', `"use strict"; return (${expr});`);
    result = fn(Math, localFactorial, __trig);
  } catch (e) {
    throw new Error(`Error al evaluar: ${e.message}`, { cause: e });
  }

  if (typeof result !== 'number' || !Number.isFinite(result)) {
    if (Number.isNaN(result)) {
      throw new Error('Resultado no es un número (NaN)');
    }
    return {
      result: result > 0 ? '∞' : '-∞',
      display: expression,
      format: 'Decimal',
    };
  }

  // ── Formateo del resultado ──
  lastAnswer = result;

  let resultStr;
  const abs = Math.abs(result);

  if (abs >= 1e15 || (abs < 1e-10 && abs > 0)) {
    resultStr = result.toExponential(10);
  } else if (Number.isInteger(result) && abs < 1e15) {
    resultStr = String(result);
  } else {
    // Eliminar ceros finales y punto decimal sobrante
    resultStr = result.toPrecision(15);
    resultStr = resultStr.replace(/(\.\d*?)0+$/, '$1');
    resultStr = resultStr.replace(/\.$/, '');
    // Si notación científica colada por toPrecision, dejarla
    if (!resultStr.includes('e') && !resultStr.includes('E')) {
      // ok
    }
  }

  return {
    result: resultStr,
    display: expression,
    format: 'Decimal',
  };
}

/** Obtiene el último resultado evaluado localmente. */
export function getLastAnswer() {
  return lastAnswer;
}

/** Reinicia el último resultado (ej: al limpiar). */
export function resetLastAnswer() {
  lastAnswer = 0;
}

// ─── Conversor local (fallback para desarrollo) ────────────────────────────

/**
 * Registro de unidades para el fallback local.
 * Cada entrada: { category, toSi, offset, isLinear }
 */
const LOCAL_UNITS = {
  // Length (base: m)
  mm:    { category: 'Length', toSi: 0.001, offset: 0, isLinear: true },
  cm:    { category: 'Length', toSi: 0.01, offset: 0, isLinear: true },
  m:     { category: 'Length', toSi: 1.0, offset: 0, isLinear: true },
  km:    { category: 'Length', toSi: 1000.0, offset: 0, isLinear: true },
  in:   { category: 'Length', toSi: 0.0254, offset: 0, isLinear: true },
  ft:    { category: 'Length', toSi: 0.3048, offset: 0, isLinear: true },
  yd:    { category: 'Length', toSi: 0.9144, offset: 0, isLinear: true },
  mi:    { category: 'Length', toSi: 1609.344, offset: 0, isLinear: true },
  // Mass (base: kg)
  mg:    { category: 'Mass', toSi: 1e-6, offset: 0, isLinear: true },
  g:     { category: 'Mass', toSi: 0.001, offset: 0, isLinear: true },
  kg:    { category: 'Mass', toSi: 1.0, offset: 0, isLinear: true },
  ton:   { category: 'Mass', toSi: 1000.0, offset: 0, isLinear: true },
  lb:    { category: 'Mass', toSi: 0.45359237, offset: 0, isLinear: true },
  oz:    { category: 'Mass', toSi: 0.028349523125, offset: 0, isLinear: true },
  // Temperature (base: K, non-linear)
  C:     { category: 'Temperature', toSi: 1.0, offset: 273.15, isLinear: false },
  F:     { category: 'Temperature', toSi: 5 / 9, offset: 459.67, isLinear: false },
  K:     { category: 'Temperature', toSi: 1.0, offset: 0, isLinear: false },
  // Volume (base: L)
  mL:    { category: 'Volume', toSi: 0.001, offset: 0, isLinear: true },
  L:     { category: 'Volume', toSi: 1.0, offset: 0, isLinear: true },
  gal:   { category: 'Volume', toSi: 3.78541, offset: 0, isLinear: true },
  fl_oz: { category: 'Volume', toSi: 0.0295735, offset: 0, isLinear: true },
  cup:   { category: 'Volume', toSi: 0.236588, offset: 0, isLinear: true },
  pt:    { category: 'Volume', toSi: 0.473176, offset: 0, isLinear: true },
  qt:    { category: 'Volume', toSi: 0.946353, offset: 0, isLinear: true },
  // Area (base: m²)
  mm2:   { category: 'Area', toSi: 1e-6, offset: 0, isLinear: true },
  cm2:   { category: 'Area', toSi: 0.0001, offset: 0, isLinear: true },
  m2:    { category: 'Area', toSi: 1.0, offset: 0, isLinear: true },
  km2:   { category: 'Area', toSi: 1e6, offset: 0, isLinear: true },
  ha:    { category: 'Area', toSi: 10000.0, offset: 0, isLinear: true },
  acre:  { category: 'Area', toSi: 4046.8564224, offset: 0, isLinear: true },
  ft2:   { category: 'Area', toSi: 0.09290304, offset: 0, isLinear: true },
  in2:   { category: 'Area', toSi: 0.00064516, offset: 0, isLinear: true },
  // Time (base: s)
  ms:    { category: 'Time', toSi: 0.001, offset: 0, isLinear: true },
  s:     { category: 'Time', toSi: 1.0, offset: 0, isLinear: true },
  min:   { category: 'Time', toSi: 60.0, offset: 0, isLinear: true },
  h:     { category: 'Time', toSi: 3600.0, offset: 0, isLinear: true },
  day:   { category: 'Time', toSi: 86400.0, offset: 0, isLinear: true },
  week:  { category: 'Time', toSi: 604800.0, offset: 0, isLinear: true },
  year:  { category: 'Time', toSi: 31557600.0, offset: 0, isLinear: true },
  // Speed (base: m/s)
  'm/s': { category: 'Speed', toSi: 1.0, offset: 0, isLinear: true },
  'km/h':{ category: 'Speed', toSi: 1 / 3.6, offset: 0, isLinear: true },
  mph:   { category: 'Speed', toSi: 0.44704, offset: 0, isLinear: true },
  kn:    { category: 'Speed', toSi: 0.514444444, offset: 0, isLinear: true },
  // Pressure (base: Pa)
  Pa:    { category: 'Pressure', toSi: 1.0, offset: 0, isLinear: true },
  kPa:   { category: 'Pressure', toSi: 1000.0, offset: 0, isLinear: true },
  MPa:   { category: 'Pressure', toSi: 1e6, offset: 0, isLinear: true },
  bar:   { category: 'Pressure', toSi: 100000.0, offset: 0, isLinear: true },
  atm:   { category: 'Pressure', toSi: 101325.0, offset: 0, isLinear: true },
  psi:   { category: 'Pressure', toSi: 6894.75729, offset: 0, isLinear: true },
  mmHg:  { category: 'Pressure', toSi: 133.322368, offset: 0, isLinear: true },
  // Energy (base: J)
  J:     { category: 'Energy', toSi: 1.0, offset: 0, isLinear: true },
  kJ:    { category: 'Energy', toSi: 1000.0, offset: 0, isLinear: true },
  cal:   { category: 'Energy', toSi: 4.184, offset: 0, isLinear: true },
  kcal:  { category: 'Energy', toSi: 4184.0, offset: 0, isLinear: true },
  Wh:    { category: 'Energy', toSi: 3600.0, offset: 0, isLinear: true },
  kWh:   { category: 'Energy', toSi: 3.6e6, offset: 0, isLinear: true },
  eV:    { category: 'Energy', toSi: 1.602176634e-19, offset: 0, isLinear: true },
  // Data (base: byte)
  bit:   { category: 'Data', toSi: 0.125, offset: 0, isLinear: true },
  B:     { category: 'Data', toSi: 1.0, offset: 0, isLinear: true },
  KB:    { category: 'Data', toSi: 1000.0, offset: 0, isLinear: true },
  MB:    { category: 'Data', toSi: 1e6, offset: 0, isLinear: true },
  GB:    { category: 'Data', toSi: 1e9, offset: 0, isLinear: true },
  TB:    { category: 'Data', toSi: 1e12, offset: 0, isLinear: true },
};

/**
 * Convierte temperatura entre C, F, K con fórmulas directas.
 */
function convertTemperatureLocal(value, fromSym, toSym) {
  const formulas = {
    'C-F': (v) => v * 9 / 5 + 32,
    'C-K': (v) => v + 273.15,
    'F-C': (v) => (v - 32) * 5 / 9,
    'F-K': (v) => (v + 459.67) * 5 / 9,
    'K-C': (v) => v - 273.15,
    'K-F': (v) => v * 9 / 5 - 459.67,
  };
  const key = `${fromSym}-${toSym}`;
  if (formulas[key]) return formulas[key](value);
  return value; // same unit
}

/**
 * Formatea un valor numérico para display.
 */
function formatValueLocal(value) {
  if (!Number.isFinite(value)) return value > 0 ? '∞' : '-∞';
  if (Number.isNaN(value)) return 'NaN';

  const abs = Math.abs(value);
  if (abs >= 1e12 || (abs < 1e-10 && abs > 0)) {
    return value.toExponential(6);
  }
  if (Number.isInteger(value) && abs < 1e12) {
    return String(value);
  }
  // Eliminar ceros finales
  const s = value.toPrecision(12);
  return s.replace(/(\.\d*?)0+$/, '$1').replace(/\.$/, '');
}

/**
 * Conversor local de unidades (fallback para desarrollo).
 * Soporta las mismas 10 categorías que el backend Rust.
 *
 * @param {number} value
 * @param {string} fromUnit - símbolo de la unidad origen
 * @param {string} toUnit - símbolo de la unidad destino
 * @returns {{ value: number, formatted: string, from: string, to: string }}
 */
function convertUnitsLocal(value, fromUnit, toUnit) {
  const fromKey = fromUnit.toLowerCase();
  const toKey = toUnit.toLowerCase();

  // Buscar unidades (case-insensitive), conservando el símbolo canónico
  const fromMatch = LOCAL_UNITS[fromKey]
    ? [fromKey, LOCAL_UNITS[fromKey]]
    : Object.entries(LOCAL_UNITS).find(([k]) => k.toLowerCase() === fromKey);
  const toMatch = LOCAL_UNITS[toKey]
    ? [toKey, LOCAL_UNITS[toKey]]
    : Object.entries(LOCAL_UNITS).find(([k]) => k.toLowerCase() === toKey);

  if (!fromMatch) throw new Error(`Unidad desconocida: '${fromUnit}'`);
  if (!toMatch) throw new Error(`Unidad desconocida: '${toUnit}'`);

  const [fromSym, fromEntry] = fromMatch;
  const [toSym, toEntry] = toMatch;

  if (fromEntry.category !== toEntry.category) {
    throw new Error(
      `No se puede convertir '${fromUnit}' (${fromEntry.category}) a '${toUnit}' (${toEntry.category}): categorías incompatibles`
    );
  }

  let result;
  if (fromEntry.category === 'Temperature') {
    result = convertTemperatureLocal(value, fromSym, toSym);
  } else {
    // Fórmula lineal: (value * from.toSi + from.offset) / to.toSi - to.offset
    result = (value * fromEntry.toSi + fromEntry.offset) / toEntry.toSi - toEntry.offset;
  }

  return {
    value: result,
    formatted: formatValueLocal(result),
    from: fromUnit,
    to: toUnit,
  };
}

// ─── API de comandos Tauri ──────────────────────────────────────────────────

export const api = {
  /**
   * Evalúa una expresión matemática.
   * Si Tauri está disponible, usa el backend Rust.
   * Si no, usa el evaluador local como fallback de desarrollo.
   *
   * @param {string} expr - Expresión matemática
   * @param {string} mode - CalcMode (Standard, Scientific, etc.)
   * @returns {Promise<{result: string, display: string, format: string}>}
   */
  async evaluateExpression(expr, mode, angleMode) {
    const invoke = await getInvoke();
    if (invoke) {
      return invoke('evaluate_expression', { expr, mode, angleMode: angleMode || 'Rad' });
    }
    // Fallback: evaluación local para desarrollo en navegador
    return evaluateLocal(expr, angleMode);
  },

  /**
   * Convierte un valor entre dos unidades.
   *
   * @param {{ value: number, fromUnit: string, toUnit: string }} request
   * @returns {Promise<{ value: number, formatted: string, from: string, to: string }>}
   */
  async convertUnits({ value, fromUnit, toUnit }) {
    const invoke = await getInvoke();
    if (invoke) {
      return invoke('convert_units', {
        value,
        fromUnit,
        toUnit,
      });
    }
    // Fallback local para desarrollo en navegador
    return convertUnitsLocal(value, fromUnit, toUnit);
  },

  async getHistory() {
    const invoke = await getInvoke();
    if (!invoke) return [];
    return invoke('get_history');
  },

  async clearHistory() {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('clear_history');
  },

  async memoryStore(value) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('memory_store', { value });
  },

  async memoryRecall() {
    const invoke = await getInvoke();
    if (!invoke) return null;
    return invoke('memory_recall');
  },

  async memoryClear() {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('memory_clear');
  },

  async memoryAdd(value) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('memory_add', { value });
  },

  async memorySubtract(value) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('memory_subtract', { value });
  },

  async setAngleMode(mode) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('set_angle_mode', { mode });
  },

  async setPrecision(digits) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('set_precision', { digits });
  },

  async setBase(base) {
    const invoke = await getInvoke();
    if (!invoke) return;
    return invoke('set_base', { base });
  },
};
