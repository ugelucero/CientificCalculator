<script>
  /**
   * App.svelte - Componente raíz de la calculadora científica.
   * Organiza: ThemeToggle → ModeSelector → Display → ScientificKeys → Keypad → History
   * Maneja eventos de teclado globales.
   */
  import { calculator } from './lib/stores/calculator.js';
  import Display from './lib/components/Display.svelte';
  import Keypad from './lib/components/Keypad.svelte';
  import ScientificKeys from './lib/components/ScientificKeys.svelte';
  import History from './lib/components/History.svelte';
  import ModeSelector from './lib/components/ModeSelector.svelte';

  /** Estado reactivo derivado de la store */
  const state = $derived($calculator);

  /** Sincroniza la clase `dark` en <html> con el tema actual */
  $effect(() => {
    const root = document.documentElement;
    if (state.theme === 'dark') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
  });

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

  /** Cambio de precisión decimal */
  function handlePrecisionChange(precision) {
    calculator.setPrecision(precision);
  }

  /** Selección desde el historial: insertar expresión */
  function handleHistorySelect(entry) {
    calculator.clear();
    calculator.append(entry.expression);
  }

  /** Limpiar historial */
  function handleClearHistory() {
    calculator.setHistory([]);
  }

  /** Toggle de tema claro/oscuro */
  function toggleTheme() {
    calculator.toggleTheme();
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col max-w-[420px] mx-auto w-full">
  <!-- Cabecera: selector de modo + toggle de tema -->
  <div class="flex items-start px-2 pt-2">
    <div class="flex-1 min-w-0">
      <ModeSelector
        currentMode={state.mode}
        currentAngleMode={state.angleMode}
        precision={state.precision}
        onModeChange={handleModeChange}
        onAngleModeChange={handleAngleModeChange}
        onPrecisionChange={handlePrecisionChange}
      />
    </div>
    <button
      onclick={toggleTheme}
      class="flex-shrink-0 p-2 text-xl leading-none rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
      aria-label="Toggle theme"
    >
      {state.theme === 'dark' ? '☀️' : '🌙'}
    </button>
  </div>

  <!-- Display: expresión + resultado + indicadores -->
  <Display
    expression={state.expression}
    result={state.result}
    error={state.error}
    angleMode={state.angleMode}
    memory={state.memory}
    mode={state.mode}
    precision={state.precision}
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
