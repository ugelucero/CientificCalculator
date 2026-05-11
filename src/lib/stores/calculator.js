/**
 * calculator.js - Svelte store para el estado de la calculadora.
 * Gestiona expresión, resultado, historial, memoria y configuración.
 *
 * Acciones principales:
 *   append(value)   — inserta un token en la expresión
 *   clear()         — borra expresión, resultado y error
 *   backspace()     — elimina el último token/carácter de la expresión
 *   calculate()     — evalúa la expresión vía Tauri (o fallback local)
 *   setAngleMode()  — cambia entre Deg, Rad, Grad
 *   memoryStore()   — guarda el resultado actual en memoria
 *   memoryRecall()  — inserta el valor de memoria en la expresión
 *   memoryClear()   — borra la memoria
 */

import { writable } from 'svelte/store';
import { api } from '../utils/tauri-bridge.js';

// ─── Helpers ───────────────────────────────────────────────────────────────

/**
 * Determina si un carácter es un dígito numérico.
 */
function isDigit(ch) {
  return ch >= '0' && ch <= '9';
}

/**
 * Determina si un carácter es una letra (para nombres de funciones/constantes).
 */
function isAlpha(ch) {
  return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
}

/**
 * Encuentra el token completo hacia atrás desde el final de la expresión.
 * Esto permite que backspace elimine tokens completos como 'sin(' o 'log10('
 * en lugar de carácter por carácter.
 *
 * Retorna la expresión sin el último token.
 */
function removeLastToken(expr) {
  if (!expr) return expr;

  let end = expr.length;

  // Eliminar espacios en blanco al final
  while (end > 0 && expr[end - 1] === ' ') {
    end--;
  }
  if (end === 0) return '';

  const lastChar = expr[end - 1];

  // ── Token: número (incluye notación científica y punto decimal) ──
  if (isDigit(lastChar) || lastChar === '.') {
    let start = end - 1;
    // Retroceder mientras sean dígitos, punto, o parte de notación científica
    while (start > 0) {
      const ch = expr[start - 1];
      if (isDigit(ch) || ch === '.' || ch === 'e' || ch === 'E' || ch === '+' || ch === '-') {
        // '+' y '-' solo dentro de notación científica (ej: 1e-5)
        if ((ch === '+' || ch === '-') && start > 1) {
          const prev = expr[start - 2];
          if (prev === 'e' || prev === 'E') {
            start--;
            continue;
          }
          break;
        }
        start--;
      } else {
        break;
      }
    }
    return expr.slice(0, start);
  }

  // ── Token: paréntesis y operadores simples ──
  if ('()+-*/^%!,.'.includes(lastChar)) {
    return expr.slice(0, end - 1);
  }

  // ── Token: identificador (función o constante) ──
  if (isAlpha(lastChar) || lastChar === '_') {
    let start = end - 1;
    while (start > 0) {
      const ch = expr[start - 1];
      if (isAlpha(ch) || isDigit(ch) || ch === '_') {
        start--;
      } else {
        break;
      }
    }
    return expr.slice(0, start);
  }

  // Fallback: eliminar un carácter
  return expr.slice(0, end - 1);
}

// ─── Store ─────────────────────────────────────────────────────────────────

/** Estado inicial */
const initialState = {
  expression: '',
  result: '',
  error: null,
  mode: 'Standard',
  history: [],
  memory: null,
  angleMode: 'Deg',
  precision: 10,
  scientificExpanded: false,
};

function createCalculatorStore() {
  const { subscribe, set, update } = writable({ ...initialState });

  /**
   * Agrega un token/valor a la expresión actual.
   *
   * @param {string} value - Token a agregar: dígito, operador, función con '(', etc.
   */
  function append(value) {
    update((state) => {
      const expr = state.expression;

      // ── Operadores binarios: agregar espacios alrededor ──
      if (['+', '-', '*', '/', '^', '%'].includes(value)) {
        // Si la expresión termina con operador, reemplazarlo
        const trimmedEnd = expr.trimEnd();
        if (
          trimmedEnd.endsWith('+') ||
          trimmedEnd.endsWith('-') ||
          trimmedEnd.endsWith('*') ||
          trimmedEnd.endsWith('/') ||
          trimmedEnd.endsWith('^') ||
          trimmedEnd.endsWith('%')
        ) {
          return {
            ...state,
            expression: trimmedEnd.slice(0, -1).trimEnd() + ` ${value} `,
            error: null,
          };
        }
        return {
          ...state,
          expression: expr + ` ${value} `,
          error: null,
        };
      }

      // ── Paréntesis ──
      if (value === '(' || value === ')') {
        return { ...state, expression: expr + value, error: null };
      }

      // ── Punto decimal ──
      if (value === '.') {
        // Verificar que el último número no tenga ya un punto
        const lastNumMatch = expr.match(/([\d.]+)$/);
        if (lastNumMatch && lastNumMatch[1].includes('.')) {
          return state; // Ya tiene punto decimal, ignorar
        }
        // Si no hay número previo, anteponer 0
        if (!lastNumMatch || lastNumMatch.index === undefined) {
          return { ...state, expression: expr + '0.', error: null };
        }
        return { ...state, expression: expr + '.', error: null };
      }

      // ── Signo negativo (negate): agregar (- si no hay, o extraer si ya está ──
      if (value === 'negate') {
        // Buscar el último número real (token Number) para negarlo
        // Estrategia simple: si la expresión termina con un número, negarlo
        const numRegex = /(-?[\d.]+(?:[eE][+-]?\d+)?)$/;
        const match = expr.match(numRegex);
        if (match) {
          const num = match[1];
          const idx = match.index;
          if (num.startsWith('-')) {
            // Remover el signo negativo
            return {
              ...state,
              expression: expr.slice(0, idx) + expr.slice(idx + 1),
              error: null,
            };
          } else {
            // Agregar signo negativo
            return {
              ...state,
              expression: expr.slice(0, idx) + '-' + expr.slice(idx),
              error: null,
            };
          }
        }
        // Si no hay número, agregar '-'
        return { ...state, expression: expr + '-', error: null };
      }

      // ── Default: concatenar el valor ──
      return { ...state, expression: expr + value, error: null };
    });
  }

  /** Borra expresión, resultado y error. */
  function clear() {
    update((state) => ({
      ...state,
      expression: '',
      result: '',
      error: null,
    }));
  }

  /** Elimina el último token de la expresión. */
  function backspace() {
    update((state) => ({
      ...state,
      expression: removeLastToken(state.expression),
      error: null,
    }));
  }

  /**
   * Evalúa la expresión actual.
   * - Si Tauri está disponible: invoca el comando Rust `evaluate_expression`.
   * - Si no: usa el evaluador local como fallback.
   *
   * Almacena el resultado en `state.result` y agrega al historial.
   * En caso de error, almacena el mensaje en `state.error`.
   */
  async function calculate() {
    let expr, mode;
    // Leer la expresión actual del store de forma síncrona
    const unsub = subscribe((state) => {
      expr = state.expression;
      mode = state.mode;
    });
    unsub();

    if (!expr || !expr.trim()) {
      update((state) => ({
        ...state,
        error: 'Expresión vacía',
        result: '',
      }));
      return;
    }

    try {
      const result = await api.evaluateExpression(expr, mode);

      // Agregar al historial
      const historyEntry = {
        expression: expr,
        result: result.result,
      };

      update((state) => {
        const newHistory = [historyEntry, ...state.history].slice(0, 100); // límite 100
        return {
          ...state,
          result: result.result,
          expression: result.display || expr,
          error: null,
          history: newHistory,
        };
      });
    } catch (err) {
      // Extraer mensaje del error (Rust CalcError o Error JS)
      const message =
        err?.message || (typeof err === 'string' ? err : 'Error desconocido');

      update((state) => ({
        ...state,
        error: message,
        result: '',
      }));
    }
  }

  /** Cambia el modo de la calculadora (Standard, Scientific, etc.). */
  function setMode(mode) {
    update((state) => ({ ...state, mode }));
  }

  /** Cambia el modo angular (Deg, Rad, Grad). */
  function setAngleMode(angleMode) {
    update((state) => ({ ...state, angleMode }));
    // Sincronizar con backend si está disponible
    api.setAngleMode(angleMode).catch(() => {});
  }

  /** Toggle del panel de teclas científicas. */
  function toggleScientific() {
    update((state) => ({
      ...state,
      scientificExpanded: !state.scientificExpanded,
    }));
  }

  // ─── Memoria ──────────────────────────────────────────────────────

  /** Guarda el resultado actual en memoria. */
  function memoryStore() {
    update((state) => {
      const value = state.result ? parseFloat(state.result) : null;
      if (value === null || Number.isNaN(value)) return state;
      // Sincronizar con backend
      api.memoryStore(value).catch(() => {});
      return { ...state, memory: value };
    });
  }

  /** Inserta el valor de memoria en la expresión. */
  function memoryRecall() {
    update((state) => {
      if (state.memory === null) return state;
      return {
        ...state,
        expression: state.expression + String(state.memory),
        error: null,
      };
    });
    // También sincronizar con backend (aunque el backend actualmente es stub)
    api.memoryRecall().catch(() => {});
  }

  /** Borra el valor almacenado en memoria. */
  function memoryClear() {
    update((state) => ({ ...state, memory: null }));
    api.memoryClear().catch(() => {});
  }

  // ─── Utilidades ───────────────────────────────────────────────────

  /** Reemplaza el historial completo (ej: al cargar desde backend). */
  function setHistory(history) {
    update((state) => ({ ...state, history }));
  }

  /** Establece el resultado manualmente (para casos especiales). */
  function setResult(result, display) {
    update((state) => ({
      ...state,
      result,
      expression: display || state.expression,
      error: null,
    }));
  }

  /** Establece un mensaje de error manualmente. */
  function setError(errorMessage) {
    update((state) => ({
      ...state,
      error: errorMessage,
      result: '',
    }));
  }

  /** Resetea todo al estado inicial. */
  function reset() {
    set({ ...initialState });
  }

  return {
    subscribe,
    append,
    clear,
    backspace,
    calculate,
    setMode,
    setAngleMode,
    toggleScientific,
    memoryStore,
    memoryRecall,
    memoryClear,
    setHistory,
    setResult,
    setError,
    reset,
  };
}

export const calculator = createCalculatorStore();
