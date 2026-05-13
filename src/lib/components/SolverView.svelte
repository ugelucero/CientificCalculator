<script>
  /**
   * SolverView.svelte — Panel de resolución de ecuaciones (Newton-Raphson).
   *
   * Permite resolver ecuaciones como `x^2 - 4 = 0` introduciendo la función,
   * la derivada (o aproximación numérica), y una estimación inicial.
   *
   * Usa el backend Rust si Tauri está disponible, o un solver JS como fallback.
   */
  import { api } from '../utils/tauri-bridge.js';

  // ─── Parámetros de entrada ──────────────────────────────────────────

  /** Expresión de la función f(x) */
  let funcText = $state('x^2 - 4');

  /** Expresión de la derivada f'(x) (vacío = numérica) */
  let derivText = $state('2*x');

  /** Valor inicial x₀ */
  let initialGuess = $state(1);

  /** Tolerancia de convergencia */
  let tolerance = $state(1e-10);

  /** Máximo de iteraciones */
  let maxIterations = $state(100);

  // ─── Estado de resultados ───────────────────────────────────────────

  /** Resultado de la resolución */
  let result = $state(null);

  /** Mensaje de error */
  let errorMessage = $state('');

  /** Indica si se está calculando */
  let loading = $state(false);

  /** Panel de iteraciones colapsado */
  let iterationsCollapsed = $state(true);

  // ─── Acción "Resolver" ──────────────────────────────────────────────

  async function handleSolve() {
    errorMessage = '';
    loading = true;
    result = null;

    const func = funcText.trim();
    if (!func) {
      errorMessage = 'Debes introducir la función f(x).';
      loading = false;
      return;
    }

    const guess = Number(initialGuess);
    if (!Number.isFinite(guess)) {
      errorMessage = 'El valor inicial debe ser un número válido.';
      loading = false;
      return;
    }

    const tol = Number(tolerance);
    if (!Number.isFinite(tol) || tol <= 0) {
      errorMessage = 'La tolerancia debe ser un número positivo.';
      loading = false;
      return;
    }

    const maxIter = Number(maxIterations);
    if (!Number.isInteger(maxIter) || maxIter < 1) {
      errorMessage = 'El máximo de iteraciones debe ser un entero positivo.';
      loading = false;
      return;
    }

    try {
      const res = await api.solveNewton({
        func,
        derivative: derivText.trim(),
        guess,
        tolerance: tol,
        maxIter,
      });
      result = res;
    } catch (err) {
      const message =
        err?.message || (typeof err === 'string' ? err : 'Error desconocido al resolver la ecuación.');
      errorMessage = message;
    } finally {
      loading = false;
    }
  }

  /** Alterna el colapso del panel de iteraciones */
  function toggleIterations() {
    iterationsCollapsed = !iterationsCollapsed;
  }

  // ─── Formateo de números ───────────────────────────────────────────

  function fmt(v) {
    if (v === null || v === undefined || (typeof v === 'number' && !Number.isFinite(v))) return '—';
    if (typeof v === 'number') {
      if (Number.isInteger(v) && Math.abs(v) < 1e15) return String(v);
      // Mostrar con suficiente precisión
      const abs = Math.abs(v);
      if (abs >= 1e10 || (abs < 1e-8 && abs > 0)) {
        return v.toExponential(6);
      }
      const s = v.toPrecision(10);
      return s.replace(/(\.\d*?)0+$/, '$1').replace(/\.$/, '');
    }
    return String(v);
  }
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-3 py-2 mx-2 my-1">
  <!-- Título -->
  <div class="text-[11px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-2 text-center">
    Solver Newton-Raphson
  </div>

  <!-- ─── Parámetros ─────────────────────────────────────────────── -->
  <div class="space-y-1.5 mb-3">
    <!-- f(x) -->
    <div>
      <label for="solver-func" class="block text-[10px] font-medium text-gray-600 dark:text-gray-400 mb-0.5">
        Función <span class="italic">f(x)</span>
      </label>
      <input id="solver-func"
        bind:value={funcText}
        type="text"
        class="w-full px-2 py-1 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
               bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
               focus:outline-none focus:ring-1 focus:ring-emerald-500 font-mono"
        placeholder="x^2 - 4"
      />
    </div>

    <!-- f'(x) -->
    <div>
      <label for="solver-deriv" class="block text-[10px] font-medium text-gray-600 dark:text-gray-400 mb-0.5">
        Derivada <span class="italic">f'(x)</span>
        {#if !derivText.trim()}
          <span class="text-[9px] text-gray-400 dark:text-gray-500 italic">(auto: numérica)</span>
        {/if}
      </label>
      <div class="flex gap-1">
        <input id="solver-deriv"
          bind:value={derivText}
          type="text"
          class="flex-1 px-2 py-1 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 font-mono"
          placeholder="2*x (dejar vacío = numérica)"
        />
        <button
          onclick={() => { derivText = ''; }}
          class="px-2 py-1 text-[11px] font-semibold rounded-lg transition-all duration-150
                 bg-amber-500 hover:bg-amber-400 dark:bg-amber-600 dark:hover:bg-amber-500
                 text-white cursor-pointer select-none active:scale-95 shrink-0"
          title="Usar aproximación numérica de la derivada"
        >
          Auto
        </button>
      </div>
    </div>

    <!-- Fila: x₀, Tolerancia, MaxIter -->
    <div class="grid grid-cols-3 gap-2">
      <div>
        <label for="solver-guess" class="block text-[10px] font-medium text-gray-600 dark:text-gray-400 mb-0.5">
          x₀
        </label>
        <input id="solver-guess"
          bind:value={initialGuess}
          type="number"
          step="any"
          class="w-full px-2 py-1 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 font-mono"
        />
      </div>
      <div>
        <label for="solver-tol" class="block text-[10px] font-medium text-gray-600 dark:text-gray-400 mb-0.5">
          Tolerancia
        </label>
        <input id="solver-tol"
          bind:value={tolerance}
          type="number"
          step="any"
          class="w-full px-2 py-1 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 font-mono"
        />
      </div>
      <div>
        <label for="solver-maxiter" class="block text-[10px] font-medium text-gray-600 dark:text-gray-400 mb-0.5">
          Máx. iter.
        </label>
        <input id="solver-maxiter"
          bind:value={maxIterations}
          type="number"
          min="1"
          step="1"
          class="w-full px-2 py-1 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 font-mono"
        />
      </div>
    </div>

    <!-- Botón Resolver -->
    <button
      onclick={handleSolve}
      disabled={loading}
      class="w-full px-4 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-emerald-500 hover:bg-emerald-400 dark:bg-emerald-600 dark:hover:bg-emerald-500
             text-white cursor-pointer select-none active:scale-95
             disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {loading ? 'Resolviendo…' : 'Resolver'}
    </button>
  </div>

  <!-- ─── Mensaje de error ───────────────────────────────────────── -->
  {#if errorMessage}
    <div class="text-red-500 text-xs mb-2 px-1 bg-red-50 dark:bg-red-900/30 rounded-lg p-2 border border-red-300 dark:border-red-700">
      {errorMessage}
    </div>
  {/if}

  <!-- ─── Resultados ─────────────────────────────────────────────── -->
  {#if result}
    <div class="mb-2">
      <div class="text-[10px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-1">
        Resultado
      </div>
      <div class="grid grid-cols-3 gap-1.5">
        <!-- Raíz -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center border border-gray-400 dark:border-gray-700">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Raíz</div>
          <div class="text-sm font-bold text-emerald-700 dark:text-emerald-400 font-mono">
            x = {fmt(result.root)}
          </div>
        </div>
        <!-- f(x) en la raíz -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center border border-gray-400 dark:border-gray-700">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">f(x)</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100 font-mono">
            {fmt(result.fRoot)}
          </div>
        </div>
        <!-- Iteraciones -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center border border-gray-400 dark:border-gray-700">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Iter.</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">
            {result.iterations}
          </div>
        </div>
      </div>

      <!-- Estado de convergencia -->
      <div class="mt-1.5 px-2 py-1 rounded-lg text-xs font-medium text-center border
                  {result.converged
                    ? 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300 border-emerald-400 dark:border-emerald-700'
                    : 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300 border-red-400 dark:border-red-700'}"
      >
        {#if result.converged}
          ✓ Convergió en {result.iterations} iteración{result.iterations !== 1 ? 'es' : ''}
        {:else}
          ✗ {result.error || 'No convergió'}
        {/if}
      </div>
    </div>

    <!-- ─── Tabla de iteraciones (colapsable) ─────────────────────── -->
    {#if result.steps && result.steps.length > 0}
      <div>
        <button
          onclick={toggleIterations}
          class="flex items-center gap-1 text-[10px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors w-full"
        >
          <span class="transition-transform duration-150"
            class:rotate-90={!iterationsCollapsed}
          >▶</span>
          Iteraciones ({result.steps.length})
        </button>

        {#if !iterationsCollapsed}
          <div class="mt-1 overflow-x-auto max-h-48 overflow-y-auto">
            <table class="w-full text-[11px] font-mono border-collapse">
              <thead>
                <tr class="bg-gray-300 dark:bg-gray-700 text-gray-600 dark:text-gray-300 text-[10px] uppercase tracking-wide">
                  <th class="px-2 py-1 text-left border border-gray-400 dark:border-gray-600">n</th>
                  <th class="px-2 py-1 text-right border border-gray-400 dark:border-gray-600">xₙ</th>
                  <th class="px-2 py-1 text-right border border-gray-400 dark:border-gray-600">f(xₙ)</th>
                </tr>
              </thead>
              <tbody>
                {#each result.steps as step, i (i)}
                  <tr class="{i % 2 === 0 ? 'bg-gray-100 dark:bg-gray-800' : 'bg-gray-200 dark:bg-gray-900'} hover:bg-gray-300 dark:hover:bg-gray-700 transition-colors">
                    <td class="px-2 py-0.5 border border-gray-400 dark:border-gray-600 text-gray-500 dark:text-gray-400">{step.n}</td>
                    <td class="px-2 py-0.5 border border-gray-400 dark:border-gray-600 text-right text-gray-800 dark:text-gray-100">{fmt(step.xN)}</td>
                    <td class="px-2 py-0.5 border border-gray-400 dark:border-gray-600 text-right text-gray-800 dark:text-gray-100">{fmt(step.fXN)}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>
