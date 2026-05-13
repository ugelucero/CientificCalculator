<script>
  /**
   * StatisticsView.svelte — Panel de estadísticas descriptivas y regresión lineal.
   *
   * Proporciona:
   * - Área de entrada de datos (textarea, números separados por coma)
   * - Botón "Calcular" que construye la expresión y llama al backend
   * - Tarjetas con resultados: N, Sum, Mean, Median, Mode, StdDev, Variance,
   *   Min, Max, Range, Quartiles (Q1, Q2, Q3)
   * - Sección de regresión lineal con inputs para x e y
   */
  import { calculator } from '../stores/calculator.js';

  // ─── Datos de entrada ──────────────────────────────────────────────

  /** Textarea con datos separados por coma */
  let dataText = $state('2,4,4,4,5,5,7,9');

  /** Datos numéricos parseados */
  let data = $derived(parseData(dataText));

  /** Textarea para valores X de regresión */
  let xText = $state('1,2,3,4,5');

  /** Textarea para valores Y de regresión */
  let yText = $state('2,4,6,8,10');

  /** Datos X parseados */
  let xData = $derived(parseData(xText));

  /** Datos Y parseados */
  let yData = $derived(parseData(yText));

  /** Resultados de estadísticas */
  let stats = $state(null);

  /** Resultados de regresión lineal */
  let regression = $state(null);

  /** Mensaje de error */
  let errorMessage = $state('');

  /** Indica si se está calculando */
  let loading = $state(false);

  // ─── Parseo ────────────────────────────────────────────────────────

  /** Parsea un string separado por comas a array de números */
  function parseData(text) {
    return text
      .split(',')
      .map(s => s.trim())
      .filter(s => s.length > 0)
      .map(s => {
        const n = parseFloat(s);
        return isNaN(n) ? null : n;
      })
      .filter(v => v !== null);
  }

  // ─── Funciones estadísticas ────────────────────────────────────────

  function computeSum(arr) {
    return arr.reduce((a, b) => a + b, 0);
  }

  function computeMean(arr) {
    if (arr.length === 0) return 0;
    return computeSum(arr) / arr.length;
  }

  function computeMedian(arr) {
    if (arr.length === 0) return 0;
    const sorted = [...arr].sort((a, b) => a - b);
    const n = sorted.length;
    if (n % 2 === 1) return sorted[Math.floor(n / 2)];
    return (sorted[n / 2 - 1] + sorted[n / 2]) / 2;
  }

  function computeMode(arr) {
    if (arr.length === 0) return [];
    const freq = new Map();
    for (const v of arr) {
      freq.set(v, (freq.get(v) || 0) + 1);
    }
    let maxCount = 0;
    for (const count of freq.values()) {
      if (count > maxCount) maxCount = count;
    }
    if (maxCount <= 1) return [];
    const modes = [];
    for (const [val, count] of freq) {
      if (count === maxCount) modes.push(val);
    }
    return modes.sort((a, b) => a - b);
  }

  function computeVariance(arr, population = true) {
    if (arr.length === 0) return 0;
    const mean = computeMean(arr);
    const sumSq = arr.reduce((acc, v) => acc + (v - mean) ** 2, 0);
    const divisor = population ? arr.length : arr.length - 1;
    if (divisor <= 0) return 0;
    return sumSq / divisor;
  }

  function computeStdDev(arr, population = true) {
    return Math.sqrt(computeVariance(arr, population));
  }

  function computeMin(arr) {
    if (arr.length === 0) return NaN;
    return Math.min(...arr);
  }

  function computeMax(arr) {
    if (arr.length === 0) return NaN;
    return Math.max(...arr);
  }

  function computeRange(arr) {
    if (arr.length === 0) return 0;
    return computeMax(arr) - computeMin(arr);
  }

  function computeQuartiles(arr) {
    if (arr.length === 0) return [0, 0, 0];
    const sorted = [...arr].sort((a, b) => a - b);
    const n = sorted.length;
    const q2 = computeMedian(sorted);
    const mid = Math.floor(n / 2);
    const lower = sorted.slice(0, mid);
    const upperStart = n % 2 === 0 ? mid : mid + 1;
    const upper = sorted.slice(upperStart);
    const q1 = lower.length > 0 ? computeMedian(lower) : 0;
    const q3 = upper.length > 0 ? computeMedian(upper) : 0;
    return [q1, q2, q3];
  }

  function computeLinearRegression(xArr, yArr) {
    if (xArr.length !== yArr.length || xArr.length < 2) {
      return null;
    }
    const n = xArr.length;
    const xMean = computeMean(xArr);
    const yMean = computeMean(yArr);

    let ssX = 0, ssY = 0, ssXY = 0;
    for (let i = 0; i < n; i++) {
      const dx = xArr[i] - xMean;
      const dy = yArr[i] - yMean;
      ssX += dx * dx;
      ssY += dy * dy;
      ssXY += dx * dy;
    }

    if (Math.abs(ssX) < 1e-15) return null;

    const slope = ssXY / ssX;
    const intercept = yMean - slope * xMean;

    let r2;
    if (Math.abs(ssY) < 1e-15) {
      r2 = 1;
    } else {
      let ssRes = 0;
      for (let i = 0; i < n; i++) {
        const yPred = slope * xArr[i] + intercept;
        ssRes += (yArr[i] - yPred) ** 2;
      }
      r2 = 1 - ssRes / ssY;
    }

    return { slope, intercept, r2 };
  }

  // ─── Formateo de números ───────────────────────────────────────────

  function fmt(v) {
    if (v === null || v === undefined || (typeof v === 'number' && isNaN(v))) return '—';
    if (typeof v === 'number') {
      if (Number.isInteger(v) && Math.abs(v) < 1e15) return String(v);
      const s = v.toPrecision(8);
      return s.replace(/(\.\d*?)0+$/, '$1').replace(/\.$/, '');
    }
    return String(v);
  }

  // ─── Acción "Calcular" ─────────────────────────────────────────────

  async function handleCalculate() {
    errorMessage = '';
    loading = true;

    const arr = data;
    if (arr.length === 0) {
      errorMessage = 'No hay datos válidos. Ingresa números separados por coma.';
      loading = false;
      return;
    }

    // Calcular estadísticas localmente
    const sum = computeSum(arr);
    const mean = computeMean(arr);
    const median = computeMedian(arr);
    const modes = computeMode(arr);
    const variance = computeVariance(arr, true);
    const stdDev = computeStdDev(arr, true);
    const min = computeMin(arr);
    const max = computeMax(arr);
    const range = computeRange(arr);
    const [q1, q2, q3] = computeQuartiles(arr);

    stats = {
      n: arr.length,
      sum,
      mean,
      median,
      mode: modes,
      stdDev,
      variance,
      min,
      max,
      range,
      q1,
      q2,
      q3,
    };

    // También llamar al backend con la expresión para historial
    try {
      const expr = '[' + arr.join(',') + ']';
      calculator.clear();
      calculator.append(expr);
      await calculator.calculate();
    } catch (e) {
      // El error de backend no interfiere con la UI local
    }

    loading = false;
  }

  /** Ejecuta la regresión lineal */
  async function handleRegression() {
    if (xData.length === 0 || yData.length === 0) {
      regression = null;
      return;
    }
    if (xData.length !== yData.length) {
      regression = null;
      errorMessage = 'X e Y deben tener la misma cantidad de valores.';
      return;
    }

    const result = computeLinearRegression(xData, yData);
    if (!result) {
      regression = null;
      errorMessage = 'No se puede calcular la regresión (datos insuficientes o varianza cero).';
      return;
    }

    regression = result;

    // Llamar al backend para historial
    try {
      const expr = 'linreg([' + xData.join(',') + '],[' + yData.join(',') + '])';
      calculator.clear();
      calculator.append(expr);
      await calculator.calculate();
    } catch (e) {
      // No crítico
    }
  }

  /** Valor de etiqueta para modo */
  function modeLabel(modes) {
    if (modes.length === 0) return '—';
    return modes.map(fmt).join(', ');
  }
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-3 py-2 mx-2 my-1">
  <!-- Entrada de datos -->
  <div class="mb-2">
    <label for="stats-data-input" class="block text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide mb-1">
      Datos (separados por coma)
    </label>
    <textarea id="stats-data-input"
      bind:value={dataText}
      rows="2"
      class="w-full px-2 py-1.5 text-sm border border-gray-400 dark:border-gray-600 rounded-lg
             bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
             focus:outline-none focus:ring-1 focus:ring-emerald-500 resize-none font-mono"
      placeholder="2, 4, 4, 4, 5, 5, 7, 9"
    ></textarea>
    <div class="flex justify-between items-center mt-1">
      <span class="text-[10px] text-gray-500 dark:text-gray-400">
        {data.length} valor{data.length !== 1 ? 'es' : ''} detectado{data.length !== 1 ? 's' : ''}
      </span>
      <button
        onclick={handleCalculate}
        disabled={loading}
        class="px-4 py-1 text-sm font-semibold rounded-lg transition-all duration-150
               bg-emerald-500 hover:bg-emerald-400 dark:bg-emerald-600 dark:hover:bg-emerald-500
               text-white cursor-pointer select-none active:scale-95
               disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? 'Calculando…' : 'Calcular'}
      </button>
    </div>
  </div>

  <!-- Mensaje de error -->
  {#if errorMessage}
    <div class="text-red-500 text-xs mb-2 px-1">{errorMessage}</div>
  {/if}

  <!-- Resultados de estadísticas -->
  {#if stats}
    <div class="mb-2">
      <div class="text-[10px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-1">
        Estadísticas Descriptivas
      </div>
      <div class="grid grid-cols-3 gap-1.5">
        <!-- N -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">N</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{stats.n}</div>
        </div>
        <!-- Sum -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Sum</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.sum)}</div>
        </div>
        <!-- Mean -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Mean</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.mean)}</div>
        </div>
        <!-- Median -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Med</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.median)}</div>
        </div>
        <!-- Mode -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Mode</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{modeLabel(stats.mode)}</div>
        </div>
        <!-- StdDev -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">StdDev</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.stdDev)}</div>
        </div>
        <!-- Variance -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Var</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.variance)}</div>
        </div>
        <!-- Min -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Min</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.min)}</div>
        </div>
        <!-- Max -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Max</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.max)}</div>
        </div>
        <!-- Range -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Range</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.range)}</div>
        </div>
        <!-- Q1 -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Q1</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.q1)}</div>
        </div>
        <!-- Q2 -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Q2</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.q2)}</div>
        </div>
        <!-- Q3 -->
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Q3</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(stats.q3)}</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Regresión Lineal -->
  <div>
    <div class="text-[10px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-1">
      Regresión Lineal
    </div>
    <div class="flex gap-2 mb-1">
      <div class="flex-1">
        <label for="stats-x-input" class="block text-[10px] font-medium text-gray-500 dark:text-gray-400 mb-0.5">X (separados por coma)</label>
        <textarea id="stats-x-input"
          bind:value={xText}
          rows="1"
          class="w-full px-2 py-1 text-xs border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 resize-none font-mono"
          placeholder="1, 2, 3, 4, 5"
        ></textarea>
      </div>
      <div class="flex-1">
        <label for="stats-y-input" class="block text-[10px] font-medium text-gray-500 dark:text-gray-400 mb-0.5">Y (separados por coma)</label>
        <textarea id="stats-y-input"
          bind:value={yText}
          rows="1"
          class="w-full px-2 py-1 text-xs border border-gray-400 dark:border-gray-600 rounded-lg
                 bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                 focus:outline-none focus:ring-1 focus:ring-emerald-500 resize-none font-mono"
          placeholder="2, 4, 6, 8, 10"
        ></textarea>
      </div>
    </div>
    <button
      onclick={handleRegression}
      class="px-3 py-1 text-xs font-semibold rounded-lg transition-all duration-150
             bg-emerald-500 hover:bg-emerald-400 dark:bg-emerald-600 dark:hover:bg-emerald-500
             text-white cursor-pointer select-none active:scale-95 mb-1"
    >
      Calcular Regresión
    </button>

    {#if regression}
      <div class="grid grid-cols-3 gap-1.5">
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Slope</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(regression.slope)}</div>
        </div>
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">Intercept</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(regression.intercept)}</div>
        </div>
        <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-2 py-1.5 text-center">
          <div class="text-[10px] font-medium text-gray-500 dark:text-gray-400 uppercase">r²</div>
          <div class="text-sm font-semibold text-gray-800 dark:text-gray-100">{fmt(regression.r2)}</div>
        </div>
      </div>
      <div class="text-[11px] text-gray-600 dark:text-gray-400 mt-1 font-mono text-center">
        y = {fmt(regression.slope)}x {regression.intercept >= 0 ? '+ ' : '- '}{fmt(Math.abs(regression.intercept))}
      </div>
    {/if}
  </div>
</div>
