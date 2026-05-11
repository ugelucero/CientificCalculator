/**
 * calculator.js - Svelte store para el estado de la calculadora.
 * Gestiona expresión, resultado, historial y configuración.
 */
import { writable } from 'svelte/store';

/** Estado inicial de la calculadora */
function createCalculatorStore() {
  const { subscribe, set, update } = writable({
    expression: '',
    result: '',
    error: null,
    mode: 'Standard',
    history: [],
    memory: null,
    angleMode: 'Deg',
    precision: 10,
    scientificExpanded: false,
  });

  return {
    subscribe,

    /** Agrega un carácter/token a la expresión actual */
    append: (value) => update(state => {
      if (value === 'clear') {
        return { ...state, expression: '', result: '', error: null };
      }
      if (value === 'backspace') {
        return { ...state, expression: state.expression.slice(0, -1), error: null };
      }
      if (value === 'equals') {
        // Se maneja en el componente que invoca evaluate_expression
        return state;
      }
      if (value === 'negate') {
        // TODO: implementar negación de la expresión actual
        return state;
      }
      return { ...state, expression: state.expression + value, error: null };
    }),

    /** Establece el resultado de una evaluación */
    setResult: (result, display) => update(state => ({
      ...state,
      result,
      expression: display || state.expression,
      error: null,
    })),

    /** Establece un mensaje de error */
    setError: (errorMessage) => update(state => ({
      ...state,
      error: errorMessage,
      result: '',
    })),

    /** Cambia el modo de la calculadora */
    setMode: (mode) => update(state => ({ ...state, mode })),

    /** Agrega entradas al historial */
    setHistory: (history) => update(state => ({ ...state, history })),

    /** Toggle del panel científico */
    toggleScientific: () => update(state => ({
      ...state,
      scientificExpanded: !state.scientificExpanded,
    })),

    /** Establece el valor en memoria */
    setMemory: (memory) => update(state => ({ ...state, memory })),

    /** Establece el modo angular */
    setAngleMode: (angleMode) => update(state => ({ ...state, angleMode })),

    reset: () => set({
      expression: '',
      result: '',
      error: null,
      mode: 'Standard',
      history: [],
      memory: null,
      angleMode: 'Deg',
      precision: 10,
      scientificExpanded: false,
    }),
  };
}

export const calculator = createCalculatorStore();
