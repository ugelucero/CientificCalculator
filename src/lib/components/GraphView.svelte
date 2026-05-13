<script>
  /**
   * GraphView.svelte — Panel principal del modo Gráficos.
   *
   * Permite definir funciones, ajustar el viewport y visualizar
   * las curvas en GraphCanvas.
   */
  import GraphCanvas from './GraphCanvas.svelte';
  import { evaluateFunc } from '../utils/tauri-bridge.js';

  const COLOR_PRESETS = ['#3498db', '#e74c3c', '#2ecc71', '#f39c12', '#9b59b6', '#1abc9c'];

  /** Lista de funciones a graficar */
  let functions = $state([
    { expr: 'x^2', color: '#3498db', label: 'f(x) = x^2' }
  ]);

  /** Límites del viewport */
  let xMin = $state(-10);
  let xMax = $state(10);
  let yMin = $state(-10);
  let yMax = $state(10);

  /** Añade una nueva función vacía a la lista */
  function addFunction() {
    const idx = functions.length;
    const color = COLOR_PRESETS[idx % COLOR_PRESETS.length];
    functions.push({ expr: '', color, label: '' });
  }

  /** Elimina una función por índice (mínimo 1) */
  function removeFunction(index) {
    if (functions.length <= 1) return;
    functions.splice(index, 1);
  }

  /** Actualiza la expresión de una función y su label */
  function handleExprChange(index, value) {
    functions[index].expr = value;
    functions[index].label = value ? `f(x) = ${value}` : '';
  }

  /** Cambia el color de una función */
  function handleColorChange(index, color) {
    functions[index].color = color;
  }

  /** Restaura el viewport a [-10, 10] en ambos ejes */
  function resetView() {
    xMin = -10;
    xMax = 10;
    yMin = -10;
    yMax = 10;
  }

  /** Auto-escala el eje Y para mostrar todas las funciones visibles */
  function autoFit() {
    const samples = 400;
    const step = (xMax - xMin) / samples;
    let minY = Infinity;
    let maxY = -Infinity;
    let found = false;

    for (const fn of functions) {
      if (!fn.expr || !fn.expr.trim()) continue;
      for (let i = 0; i <= samples; i++) {
        const x = xMin + i * step;
        try {
          const y = evaluateFunc(fn.expr, x);
          if (Number.isFinite(y) && Math.abs(y) < 1e15) {
            if (y < minY) minY = y;
            if (y > maxY) maxY = y;
            found = true;
          }
        } catch { /* skip */ }
      }
    }

    if (found && Number.isFinite(minY) && Number.isFinite(maxY)) {
      const range = maxY - minY;
      const padding = range > 0 ? range * 0.1 : 1;
      yMin = minY - padding;
      yMax = maxY + padding;
    }
  }

  /** Sincroniza el viewport cuando el canvas cambia por zoom/pan */
  function handleViewportChange(vp) {
    xMin = vp.xMin;
    xMax = vp.xMax;
    yMin = vp.yMin;
    yMax = vp.yMax;
  }
</script>

<div class="flex flex-col gap-2 p-2 w-full">
  <!-- ─── Panel de funciones ────────────────────────────────────── -->
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-300 dark:border-gray-700
           p-2 space-y-1.5"
  >
    <div class="text-[11px] font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide mb-1">
      Funciones
    </div>

    {#each functions as fn, i (i)}
      <div class="flex items-center gap-1.5">
        <span class="text-[11px] font-mono text-gray-500 dark:text-gray-400 shrink-0">
          f<sub>{i + 1}</sub>(x) =
        </span>
        <input
          type="text"
          value={fn.expr}
          oninput={(e) => handleExprChange(i, e.target.value)}
          placeholder="x^2, sin(x), ..."
          class="flex-1 min-w-0 px-2 py-1 text-xs font-mono bg-gray-100 dark:bg-gray-700
                 border border-gray-300 dark:border-gray-600 rounded
                 text-gray-800 dark:text-gray-200
                 placeholder:text-gray-400 dark:placeholder:text-gray-500
                 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500
                 transition-colors"
        />
        <!-- Selector de color: 6 botones predefinidos -->
        <div class="flex gap-0.5 shrink-0">
          {#each COLOR_PRESETS as color}
            <button
              class="w-4 h-4 rounded-full border-2 transition-all duration-150
                     hover:scale-110 focus:outline-none focus:ring-1 focus:ring-blue-500
                     {color === fn.color
                       ? 'border-gray-800 dark:border-white scale-110'
                       : 'border-transparent'}"
              style="background-color: {color};"
              onclick={() => handleColorChange(i, color)}
              aria-label="Color {color}"
            ></button>
          {/each}
        </div>
        <!-- Botón eliminar -->
        <button
          class="shrink-0 w-5 h-5 flex items-center justify-center rounded-full
                 text-xs font-bold text-gray-400 hover:text-red-500
                 hover:bg-red-50 dark:hover:bg-red-900/30
                 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
          onclick={() => removeFunction(i)}
          disabled={functions.length <= 1}
          aria-label="Eliminar función"
        >
          ×
        </button>
      </div>
    {/each}

    <!-- Botón añadir función -->
    <button
      onclick={addFunction}
      class="w-full mt-1 px-3 py-1.5 text-xs font-semibold rounded-lg
             border border-dashed border-gray-400 dark:border-gray-600
             text-gray-600 dark:text-gray-400
             hover:bg-gray-100 dark:hover:bg-gray-700
             hover:text-blue-600 dark:hover:text-blue-400
             hover:border-blue-400 dark:hover:border-blue-500
             transition-colors"
    >
      + Añadir función
    </button>
  </div>

  <!-- ─── Panel de controles de vista ──────────────────────────── -->
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-300 dark:border-gray-700
           p-2"
  >
    <div class="text-[11px] font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide mb-1.5">
      Vista
    </div>

    <div class="grid grid-cols-2 gap-x-3 gap-y-1.5">
      <!-- X Min -->
      <div class="flex items-center gap-1.5">
        <span class="text-[11px] font-mono text-gray-500 dark:text-gray-400 w-8 shrink-0">
          xMin
        </span>
        <input
          type="number"
          value={xMin}
          oninput={(e) => { xMin = Number(e.target.value); }}
          step="any"
          aria-label="xMin"
          class="flex-1 min-w-0 px-1.5 py-0.5 text-xs font-mono bg-gray-100 dark:bg-gray-700
                 border border-gray-300 dark:border-gray-600 rounded
                 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-blue-500
                 transition-colors"
        />
      </div>
      <!-- X Max -->
      <div class="flex items-center gap-1.5">
        <span class="text-[11px] font-mono text-gray-500 dark:text-gray-400 w-8 shrink-0">
          xMax
        </span>
        <input
          type="number"
          value={xMax}
          oninput={(e) => { xMax = Number(e.target.value); }}
          step="any"
          aria-label="xMax"
          class="flex-1 min-w-0 px-1.5 py-0.5 text-xs font-mono bg-gray-100 dark:bg-gray-700
                 border border-gray-300 dark:border-gray-600 rounded
                 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-blue-500
                 transition-colors"
        />
      </div>
      <!-- Y Min -->
      <div class="flex items-center gap-1.5">
        <span class="text-[11px] font-mono text-gray-500 dark:text-gray-400 w-8 shrink-0">
          yMin
        </span>
        <input
          type="number"
          value={yMin}
          oninput={(e) => { yMin = Number(e.target.value); }}
          step="any"
          aria-label="yMin"
          class="flex-1 min-w-0 px-1.5 py-0.5 text-xs font-mono bg-gray-100 dark:bg-gray-700
                 border border-gray-300 dark:border-gray-600 rounded
                 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-blue-500
                 transition-colors"
        />
      </div>
      <!-- Y Max -->
      <div class="flex items-center gap-1.5">
        <span class="text-[11px] font-mono text-gray-500 dark:text-gray-400 w-8 shrink-0">
          yMax
        </span>
        <input
          type="number"
          value={yMax}
          oninput={(e) => { yMax = Number(e.target.value); }}
          step="any"
          aria-label="yMax"
          class="flex-1 min-w-0 px-1.5 py-0.5 text-xs font-mono bg-gray-100 dark:bg-gray-700
                 border border-gray-300 dark:border-gray-600 rounded
                 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-blue-500
                 transition-colors"
        />
      </div>
    </div>

    <!-- Botones de acción -->
    <div class="flex gap-2 mt-2">
      <button
        onclick={autoFit}
        class="flex-1 px-3 py-1 text-xs font-semibold rounded-lg
               bg-blue-600 hover:bg-blue-700 text-white
               transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        Ajustar
      </button>
      <button
        onclick={resetView}
        class="flex-1 px-3 py-1 text-xs font-semibold rounded-lg
               bg-gray-200 dark:bg-gray-700
               text-gray-700 dark:text-gray-300
               hover:bg-gray-300 dark:hover:bg-gray-600
               transition-colors focus:outline-none focus:ring-2 focus:ring-gray-400"
      >
        Reset vista
      </button>
    </div>
  </div>

  <!-- ─── Canvas de gráficos ───────────────────────────────────── -->
  <div
    class="flex-1 min-h-[320px] rounded-lg overflow-hidden
           border border-gray-300 dark:border-gray-700"
  >
    <GraphCanvas
      {functions}
      {xMin}
      {xMax}
      {yMin}
      {yMax}
      onViewportChange={handleViewportChange}
    />
  </div>
</div>
