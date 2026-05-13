<script>
  /**
   * UnitConverter.svelte — Panel de conversión de unidades.
   *
   * Permite seleccionar una categoría, unidad origen, valor numérico,
   * unidad destino y muestra el resultado de la conversión.
   *
   * Usa `api.convertUnits()` del tauri-bridge para invocar el backend
   * Rust o el fallback local.
   */
  import { api } from '../utils/tauri-bridge.js';

  // ─── Catálogo de unidades ───────────────────────────────────────────────

  const CATEGORIES = [
    { value: 'Length',       label: '[L] Longitud' },
    { value: 'Mass',         label: '[M] Masa' },
    { value: 'Temperature',  label: '[T] Temperatura' },
    { value: 'Volume',       label: '[V] Volumen' },
    { value: 'Area',         label: '[A] Área' },
    { value: 'Time',         label: '[t] Tiempo' },
    { value: 'Speed',        label: '[v] Velocidad' },
    { value: 'Pressure',     label: '[P] Presión' },
    { value: 'Energy',       label: '[E] Energía' },
    { value: 'Data',         label: '[D] Datos' },
  ];

  const UNITS_BY_CATEGORY = {
    Length: [
      { symbol: 'mm', name: 'Milímetro' },
      { symbol: 'cm', name: 'Centímetro' },
      { symbol: 'm',  name: 'Metro' },
      { symbol: 'km', name: 'Kilómetro' },
      { symbol: 'in', name: 'Pulgada' },
      { symbol: 'ft', name: 'Pie' },
      { symbol: 'yd', name: 'Yarda' },
      { symbol: 'mi', name: 'Milla' },
    ],
    Mass: [
      { symbol: 'mg',  name: 'Miligramo' },
      { symbol: 'g',   name: 'Gramo' },
      { symbol: 'kg',  name: 'Kilogramo' },
      { symbol: 'ton', name: 'Tonelada' },
      { symbol: 'lb',  name: 'Libra' },
      { symbol: 'oz',  name: 'Onza' },
    ],
    Temperature: [
      { symbol: 'C', name: 'Celsius' },
      { symbol: 'F', name: 'Fahrenheit' },
      { symbol: 'K', name: 'Kelvin' },
    ],
    Volume: [
      { symbol: 'mL',    name: 'Mililitro' },
      { symbol: 'L',     name: 'Litro' },
      { symbol: 'gal',   name: 'Galón (US)' },
      { symbol: 'fl_oz', name: 'Onza líquida' },
      { symbol: 'cup',   name: 'Taza' },
      { symbol: 'pt',    name: 'Pinta' },
      { symbol: 'qt',    name: 'Cuarto' },
    ],
    Area: [
      { symbol: 'mm2', name: 'mm²' },
      { symbol: 'cm2', name: 'cm²' },
      { symbol: 'm2',  name: 'm²' },
      { symbol: 'km2', name: 'km²' },
      { symbol: 'ha',  name: 'Hectárea' },
      { symbol: 'acre', name: 'Acre' },
      { symbol: 'ft2', name: 'ft²' },
      { symbol: 'in2', name: 'in²' },
    ],
    Time: [
      { symbol: 'ms',   name: 'Milisegundo' },
      { symbol: 's',    name: 'Segundo' },
      { symbol: 'min',  name: 'Minuto' },
      { symbol: 'h',    name: 'Hora' },
      { symbol: 'day',  name: 'Día' },
      { symbol: 'week', name: 'Semana' },
      { symbol: 'year', name: 'Año' },
    ],
    Speed: [
      { symbol: 'm/s',  name: 'm/s' },
      { symbol: 'km/h', name: 'km/h' },
      { symbol: 'mph',  name: 'mph' },
      { symbol: 'kn',   name: 'Nudo' },
    ],
    Pressure: [
      { symbol: 'Pa',   name: 'Pascal' },
      { symbol: 'kPa',  name: 'Kilopascal' },
      { symbol: 'MPa',  name: 'Megapascal' },
      { symbol: 'bar',  name: 'Bar' },
      { symbol: 'atm',  name: 'Atmósfera' },
      { symbol: 'psi',  name: 'PSI' },
      { symbol: 'mmHg', name: 'mmHg' },
    ],
    Energy: [
      { symbol: 'J',    name: 'Julio' },
      { symbol: 'kJ',   name: 'Kilojulio' },
      { symbol: 'cal',  name: 'Caloría' },
      { symbol: 'kcal', name: 'Kilocaloría' },
      { symbol: 'Wh',   name: 'Watt-hora' },
      { symbol: 'kWh',  name: 'kW-hora' },
      { symbol: 'eV',   name: 'Electronvoltio' },
    ],
    Data: [
      { symbol: 'bit', name: 'Bit' },
      { symbol: 'B',   name: 'Byte' },
      { symbol: 'KB',  name: 'Kilobyte' },
      { symbol: 'MB',  name: 'Megabyte' },
      { symbol: 'GB',  name: 'Gigabyte' },
      { symbol: 'TB',  name: 'Terabyte' },
    ],
  };

  // ─── Estado local ───────────────────────────────────────────────────────

  let category = $state('Length');
  let fromUnit = $state('m');
  let toUnit = $state('cm');
  let inputValue = $state('1');
  let result = $state(null);
  let error = $state(null);
  let loading = $state(false);

  // ─── Derivados ──────────────────────────────────────────────────────────

  const units = $derived(UNITS_BY_CATEGORY[category] || []);
  const fromUnitLabel = $derived(units.find(u => u.symbol === fromUnit)?.name || fromUnit);
  const toUnitLabel = $derived(units.find(u => u.symbol === toUnit)?.name || toUnit);

  // ─── Handlers ───────────────────────────────────────────────────────────

  /** Al cambiar de categoría, reajustar unidades si la actual no existe. */
  function handleCategoryChange(newCategory) {
    category = newCategory;
    const available = UNITS_BY_CATEGORY[newCategory] || [];
    if (!available.find(u => u.symbol === fromUnit)) {
      fromUnit = available[0]?.symbol || '';
    }
    if (!available.find(u => u.symbol === toUnit)) {
      // Elegir una distinta de fromUnit si es posible
      toUnit = (available.find(u => u.symbol !== fromUnit) || available[0])?.symbol || '';
    }
    result = null;
    error = null;
  }

  /** Intercambia unidad origen y destino. */
  function swapUnits() {
    const tmp = fromUnit;
    fromUnit = toUnit;
    toUnit = tmp;
    result = null;
    error = null;
  }

  /** Ejecuta la conversión. */
  async function handleConvert() {
    const value = parseFloat(inputValue);
    if (Number.isNaN(value)) {
      error = 'Ingresa un número válido';
      result = null;
      return;
    }

    loading = true;
    error = null;
    result = null;

    try {
      const res = await api.convertUnits({
        value,
        fromUnit,
        toUnit,
      });
      result = res.formatted;
    } catch (err) {
      // En Tauri los errores pueden ser string (JSON) u objeto
      let detail = 'Error desconocido';
      if (typeof err === 'string') {
        try {
          const parsed = JSON.parse(err);
          detail = parsed.message || parsed.kind || err;
        } catch {
          detail = err;
        }
      } else if (err?.message) {
        detail = err.message;
      } else {
        detail = JSON.stringify(err);
      }
      error = `Error: ${detail}`;
      console.error('[convertUnits]', err, typeof err);
    } finally {
      loading = false;
    }
  }

  /** Convierte al presionar Enter. */
  function handleKeyDown(e) {
    if (e.key === 'Enter') {
      handleConvert();
    }
  }
</script>

<div class="unit-converter rounded-xl bg-white dark:bg-gray-800 p-4 border border-gray-200 dark:border-gray-700 shadow-lg">
  <!-- Cabecera -->
  <div class="flex items-center justify-between mb-3">
    <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200">
      ⇄ Conversor de Unidades
    </h2>
  </div>

  <!-- Selector de categoría -->
  <div class="mb-3">
    <label for="category-select" class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">
      Categoría
    </label>
    <select
      id="category-select"
      class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700
             text-gray-800 dark:text-gray-200 px-3 py-2 text-sm
             focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none
             transition-colors"
      bind:value={category}
      onchange={(e) => handleCategoryChange(e.target.value)}
    >
      {#each CATEGORIES as cat}
        <option value={cat.value}>{cat.label}</option>
      {/each}
    </select>
  </div>

  <!-- Input numérico -->
  <div class="mb-3">
    <label for="value-input" class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">
      Valor
    </label>
    <input
      id="value-input"
      type="number"
      class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700
             text-gray-800 dark:text-gray-200 px-3 py-2 text-sm font-mono
             focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none
             transition-colors"
      bind:value={inputValue}
      onkeydown={handleKeyDown}
      placeholder="Ingresa un valor"
      step="any"
    />
  </div>

  <!-- From / Swap / To -->
  <div class="flex items-end gap-2 mb-3">
    <!-- From -->
    <div class="flex-1 min-w-0">
      <label for="from-select" class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">
        De:
      </label>
      <select
        id="from-select"
        class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700
               text-gray-800 dark:text-gray-200 px-2 py-2 text-sm
               focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none
               transition-colors"
        bind:value={fromUnit}
        onchange={() => { result = null; error = null; }}
      >
        {#each units as unit}
          <option value={unit.symbol}>{unit.symbol} — {unit.name}</option>
        {/each}
      </select>
    </div>

    <!-- Swap button -->
    <button
      onclick={swapUnits}
      class="flex-shrink-0 p-2 rounded-lg bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600
             text-gray-600 dark:text-gray-300 transition-colors mb-0.5"
      aria-label="Intercambiar unidades"
      title="Intercambiar origen y destino"
    >
      ⇄
    </button>

    <!-- To -->
    <div class="flex-1 min-w-0">
      <label for="to-select" class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">
        A:
      </label>
      <select
        id="to-select"
        class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700
               text-gray-800 dark:text-gray-200 px-2 py-2 text-sm
               focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none
               transition-colors"
        bind:value={toUnit}
        onchange={() => { result = null; error = null; }}
      >
        {#each units as unit}
          <option value={unit.symbol}>{unit.symbol} — {unit.name}</option>
        {/each}
      </select>
    </div>
  </div>

  <!-- Botón Convertir -->
  <button
    onclick={handleConvert}
    disabled={loading}
    class="w-full py-2.5 rounded-lg font-semibold text-sm text-white
           bg-blue-600 hover:bg-blue-700 active:bg-blue-800
           disabled:opacity-50 disabled:cursor-not-allowed
           transition-colors mb-3"
  >
    {#if loading}
      ⏳ Convirtiendo...
    {:else}
      🔄 Convertir
    {/if}
  </button>

  <!-- Resultado -->
  {#if result !== null}
    <div class="rounded-lg bg-green-50 dark:bg-green-900/30 border border-green-200 dark:border-green-800 p-3">
      <p class="text-xs text-green-600 dark:text-green-400 mb-1">Resultado:</p>
      <p class="text-xl font-mono font-bold text-green-800 dark:text-green-200">
        {result} <span class="text-sm font-normal">{toUnit}</span>
      </p>
    </div>
  {/if}

  <!-- Error -->
  {#if error}
    <div class="rounded-lg bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 p-3">
      <p class="text-sm text-red-700 dark:text-red-300">{error}</p>
    </div>
  {/if}
</div>
