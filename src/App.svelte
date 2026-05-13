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
  import UnitConverter from './lib/components/UnitConverter.svelte';
  import ProgrammerView from './lib/components/ProgrammerView.svelte';
  import ComplexInput from './lib/components/ComplexInput.svelte';
  import MatrixInput from './lib/components/MatrixInput.svelte';
  import StatisticsView from './lib/components/StatisticsView.svelte';
  import SolverView from './lib/components/SolverView.svelte';
  import GraphView from './lib/components/GraphView.svelte';

  /** Estado reactivo derivado de la store */
  const state = $derived($calculator);

  /** Mostrar/ocultar panel de conversión de unidades */
  let showUnitConverter = $state(false);

  /** Sincroniza la clase `dark` en <html> con el tema actual */
  $effect(() => {
    const root = document.documentElement;
    if (state.theme === 'dark') {
      root.classList.add('dark');
      root.style.colorScheme = 'dark';
    } else {
      root.classList.remove('dark');
      root.style.colorScheme = 'light';
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

    // ── Paréntesis y corchetes ──────────────────────────────
    '(': () => calculator.append('('),
    ')': () => calculator.append(')'),
    '[': () => calculator.append('['),
    ']': () => calculator.append(']'),

    // ── Punto decimal y separadores ─────────────────────────
    '.': () => calculator.append('.'),
    ',': () => calculator.append(','),
    ';': () => calculator.append(';'),

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

  /** Selección desde el historial: restaurar expresión y modo */
  function handleHistorySelect(entry) {
    // Restaurar el modo en que se creó la entrada
    if (entry.mode) {
      calculator.setMode(entry.mode);
    }
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

  /** Toggle del panel de conversión de unidades */
  function toggleUnitConverter() {
    showUnitConverter = !showUnitConverter;
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col max-w-[420px] mx-auto w-full">
  <!-- Cabecera: selector de modo + toggle de tema + conversor -->
  <div class="px-2 pt-2 flex flex-col gap-0.5">
    <!-- Fila 1: Modos + Ángulos -->
    <div class="flex items-center gap-1">
      <ModeSelector
        currentMode={state.mode}
        currentAngleMode={state.angleMode}
        precision={state.precision}
        onModeChange={handleModeChange}
        onAngleModeChange={handleAngleModeChange}
        onPrecisionChange={handlePrecisionChange}
      />
    </div>
    <!-- Fila 2: Precisión + ⇄ + Tema -->
    <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
      <span>Precision:</span>
      <select
        value={state.precision}
        onchange={(e) => handlePrecisionChange(Number(e.target.value))}
        class="bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border border-gray-400 dark:border-gray-600 rounded px-1.5 py-0.5 text-xs focus:outline-none focus:ring-1 focus:ring-blue-500"
      >
        {#each Array.from({ length: 16 }, (_, i) => i) as val}
          <option value={val}>{val}</option>
        {/each}
      </select>
      <button
        onclick={toggleUnitConverter}
        class="ml-auto px-2 py-1 text-[11px] font-semibold rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors border border-gray-400 dark:border-gray-600"
        class:bg-blue-100={showUnitConverter}
        class:dark:bg-blue-900={showUnitConverter}
        aria-label="Toggle unit converter"
        title="Conversor de unidades"
      >
        Conv
      </button>
      <button
        onclick={toggleTheme}
        class="px-2 py-1 text-[11px] font-semibold rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors border border-gray-400 dark:border-gray-600"
        aria-label="Toggle theme"
      >
        {state.theme === 'dark' ? 'Claro' : 'Oscuro'}
      </button>
    </div>
  </div>

  <!-- Display: expresión + resultado + indicadores -->
  {#if state.mode === 'Programmer'}
    <ProgrammerView
      expression={state.expression}
      result={state.result}
      error={state.error}
      base={state.base}
    />
  {:else if state.mode === 'Complex'}
    <ComplexInput
      expression={state.expression}
      result={state.result}
      error={state.error}
    />
  {:else}
    <Display
      expression={state.expression}
      result={state.result}
      error={state.error}
      angleMode={state.angleMode}
      memory={state.memory}
      mode={state.mode}
      precision={state.precision}
      base={state.base}
    />
  {/if}

  <!-- Panel de conversión de unidades (toggle) -->
  {#if showUnitConverter}
    <div class="px-2 pb-1">
      <UnitConverter />
    </div>
  {/if}

  <!-- Panel de teclas científicas expandible -->
  <ScientificKeys
    expanded={state.scientificExpanded}
    onKeyPress={handleKeyPress}
    onToggleExpand={() => calculator.toggleScientific()}
  />

  <!-- Teclado numérico principal (reemplazado en modos Matrix, Statistics y Solver) -->
  {#if state.mode === 'Matrix'}
    <MatrixInput />
  {:else if state.mode === 'Statistics'}
    <StatisticsView />
  {:else if state.mode === 'Solver'}
    <SolverView />
  {:else if state.mode === 'Graph'}
    <GraphView />
  {:else}
    <Keypad
      onKeyPress={handleKeyPress}
      mode={state.mode}
      base={state.base}
    />
  {/if}

  <!-- Panel de historial -->
  <History
    entries={state.history}
    onSelect={handleHistorySelect}
    onClear={handleClearHistory}
  />
</div>
