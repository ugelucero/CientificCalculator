<script>
  /**
   * App.svelte - Componente raíz de la calculadora científica.
   * Organiza: ModeSelector → Display → ScientificKeys → Keypad → History
   * Maneja eventos de teclado globales.
   */
  import { onMount } from 'svelte';
  import { calculator } from './lib/stores/calculator.js';
  import Display from './lib/components/Display.svelte';
  import Keypad from './lib/components/Keypad.svelte';
  import ScientificKeys from './lib/components/ScientificKeys.svelte';
  import History from './lib/components/History.svelte';
  import ModeSelector from './lib/components/ModeSelector.svelte';

  /** Estado reactivo derivado de la store */
  const state = $derived($calculator);

  /**
   * Mapa de teclas del teclado a acciones del store.
   * Cada entrada mapea una tecla física (event.key) a una función.
   */
  const keyMap = {
    // ── Dígitos ─────────────────────────────────────────────
    '0': () => calculator.append('0'),
    '1': () => calculator.append('1'),
    '2': () => calculator.append('2'),
    '3': () => calculator.append('3'),
    '4': () => calculator.append('4'),
    '5': () => calculator.append('5'),
    '6': () => calculator.append('6'),
    '7': () => calculator.append('7'),
    '8': () => calculator.append('8'),
    '9': () => calculator.append('9'),

    // ── Operadores ──────────────────────────────────────────
    '+': () => calculator.append('+'),
    '-': () => calculator.append('-'),
    '*': () => calculator.append('*'),
    '/': () => calculator.append('/'),
    '%': () => calculator.append('%'),
    '^': () => calculator.append('^'),

    // ── Paréntesis ──────────────────────────────────────────
    '(': () => calculator.append('('),
    ')': () => calculator.append(')'),

    // ── Punto decimal ───────────────────────────────────────
    '.': () => calculator.append('.'),

    // ── Evaluar ─────────────────────────────────────────────
    'Enter': () => calculator.calculate(),
    '=': () => calculator.calculate(),

    // ── Borrar ──────────────────────────────────────────────
    'Backspace': () => calculator.backspace(),
    'Delete': () => calculator.clear(),
    'Escape': () => calculator.clear(),
  };

  /** Maneja eventos de teclado globales */
  function handleKeyDown(event) {
    // Ignorar si el foco está en un input/textarea
    const tag = document.activeElement?.tagName?.toLowerCase();
    if (tag === 'input' || tag === 'textarea' || tag === 'select') return;

    // Ignorar combinaciones con Ctrl/Meta (atajos del navegador)
    if (event.ctrlKey || event.metaKey || event.altKey) return;

    const action = keyMap[event.key];
    if (action) {
      event.preventDefault();
      action();
    }
  }

  /** El listener de teclado se registra automáticamente via <svelte:window onkeydown> */

  // ── Handlers para componentes ────────────────────────────

  /** Delegado: Keypad y ScientificKeys llaman a este handler */
  function handleKeyPress(value) {
    switch (value) {
      case 'equals':
        calculator.calculate();
        break;
      case 'clear':
        calculator.clear();
        break;
      case 'backspace':
        calculator.backspace();
        break;
      case 'negate':
        calculator.append('negate');
        break;
      default:
        calculator.append(value);
    }
  }

  /** Cambio de modo (Standard, Scientific, etc.) */
  function handleModeChange(mode) {
    calculator.setMode(mode);
  }

  /** Cambio de modo angular (Deg, Rad, Grad) */
  function handleAngleModeChange(angleMode) {
    calculator.setAngleMode(angleMode);
  }

  /** Selección desde el historial: insertar expresión */
  function handleHistorySelect(entry) {
    // Reemplazar la expresión actual con la del historial
    calculator.clear();
    calculator.append(entry.expression);
  }

  /** Limpiar historial */
  function handleClearHistory() {
    calculator.setHistory([]);
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="calculator-app">
  <!-- Selector de modo en la cabecera -->
  <ModeSelector
    currentMode={state.mode}
    currentAngleMode={state.angleMode}
    onModeChange={handleModeChange}
    onAngleModeChange={handleAngleModeChange}
  />

  <!-- Display: expresión + resultado + indicadores -->
  <Display
    expression={state.expression}
    result={state.result}
    error={state.error}
    angleMode={state.angleMode}
    memory={state.memory}
    mode={state.mode}
  />

  <!-- Panel de teclas científicas expandible -->
  <ScientificKeys
    expanded={state.scientificExpanded}
    onKeyPress={handleKeyPress}
    onToggleExpand={() => calculator.toggleScientific()}
  />

  <!-- Teclado numérico principal -->
  <Keypad onKeyPress={handleKeyPress} />

  <!-- Panel de historial -->
  <History
    entries={state.history}
    onSelect={handleHistorySelect}
    onClear={handleClearHistory}
  />
</div>

<style>
  .calculator-app {
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
    min-height: 100vh;
    background: #111827;
    display: flex;
    flex-direction: column;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
</style>
