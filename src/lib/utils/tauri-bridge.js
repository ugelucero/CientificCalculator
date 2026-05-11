/**
 * tauri-bridge.js - Wrapper para invocar comandos Tauri con tipado.
 * Se importa @tauri-apps/api/core dinámicamente para evitar
 * errores en entornos sin Tauri (navegador normal).
 *
 * Incluye un evaluador local como fallback para desarrollo en navegador.
 */

/** @type {import('@tauri-apps/api/core').invoke|null} */
let invokeFn = null;

async function getInvoke() {
  if (!invokeFn) {
    try {
      const tauriCore = await import('@tauri-apps/api/core');
      invokeFn = tauriCore.invoke;
    } catch {
      // Tauri no disponible (entorno navegador)
      invokeFn = null;
    }
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
 * Evalúa una expresión localmente para desarrollo en navegador.
 * Soporta: +, -, *, /, ^, %, !, (), sin, cos, tan, asin, acos, atan,
 * sinh, cosh, tanh, log (base 10), ln, log2, log10, sqrt, cbrt, abs,
 * exp, pi, e, ans.
 *
 * La precisión es limitada comparada con el backend Rust.
 *
 * @param {string} expression - Expresión matemática en notación infija
 * @returns {{ result: string, display: string, format: string }}
 */
function evaluateLocal(expression) {
  const trimmed = expression.trim();

  if (!trimmed) {
    return { result: '0', display: expression, format: 'Decimal' };
  }

  // Validar solo caracteres permitidos (previene inyección de código)
  if (!/^[\d+\-*/().%\s^!a-zA-Z]+$/.test(trimmed)) {
    throw new Error('Caracteres no permitidos en la expresión');
  }

  let expr = trimmed;

  // ── Reemplazar funciones (orden: más largas primero) ──
  // Logaritmos (log10/log2 antes que log genérico)
  expr = expr.replace(/\blog10\(/g, 'Math.log10(');
  expr = expr.replace(/\blog2\(/g,  'Math.log2(');
  expr = expr.replace(/\blog\(/g,   'Math.log10(');  // log = log10
  expr = expr.replace(/\bln\(/g,    'Math.log(');     // ln  = natural

  // Trigonométricas inversas e hiperbólicas (antes que las directas)
  expr = expr.replace(/\basin\(/g,  'Math.asin(');
  expr = expr.replace(/\bacos\(/g,  'Math.acos(');
  expr = expr.replace(/\batan\(/g,  'Math.atan(');
  expr = expr.replace(/\bsinh\(/g,  'Math.sinh(');
  expr = expr.replace(/\bcosh\(/g,  'Math.cosh(');
  expr = expr.replace(/\btanh\(/g,  'Math.tanh(');

  // Trigonométricas directas
  expr = expr.replace(/\bsin\(/g,   'Math.sin(');
  expr = expr.replace(/\bcos\(/g,   'Math.cos(');
  expr = expr.replace(/\btan\(/g,   'Math.tan(');

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

  // ── Operadores ──
  // Potencia: ^ → **
  expr = expr.replace(/\^/g, '**');

  // Factorial (postfijo): n! → localFactorial(n), (expr)! → localFactorial(expr)
  expr = expr.replace(/([\d.]+|\))\s*!/g, 'localFactorial($1)');

  // ── Evaluación segura ──
  let result;
  try {
    // new Function() es más seguro que eval(): no tiene acceso al scope local
    const fn = new Function('Math', 'localFactorial', `"use strict"; return (${expr});`);
    result = fn(Math, localFactorial);
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
  async evaluateExpression(expr, mode) {
    const invoke = await getInvoke();
    if (invoke) {
      return invoke('evaluate_expression', { expr, mode });
    }
    // Fallback: evaluación local para desarrollo en navegador
    return evaluateLocal(expr);
  },

  async convertUnits(value, from, to) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('convert_units', { value, from, to });
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
};
