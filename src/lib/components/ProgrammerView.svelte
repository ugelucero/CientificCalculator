<script>
  /**
   * ProgrammerView.svelte — Panel de modo programador.
   *
   * Muestra:
   * - Selector de base: HEX | DEC | OCT | BIN
   * - Display con resultado actual en las 4 bases
   * - Indicador visual de 16 bits con LEDs
   */
  import { calculator } from '../stores/calculator.js';

  const {
    expression = '',
    result = '',
    error = null,
    base = 'DEC',
  } = $props();

  const bases = ['HEX', 'DEC', 'OCT', 'BIN'];

  /** Convierte un valor numérico a las 4 representaciones */
  function toBaseDisplays(value) {
    const num = parseInt(value, 10);
    if (value === '' || value === undefined || value === null || Number.isNaN(num)) {
      return { HEX: '0', DEC: '0', OCT: '0', BIN: '0' };
    }
    const abs = Math.abs(num);
    return {
      HEX: '0x' + abs.toString(16).toUpperCase(),
      DEC: String(abs),
      OCT: '0o' + abs.toString(8),
      BIN: '0b' + abs.toString(2),
    };
  }

  /** Valor a mostrar (prioriza result, luego expression) */
  const displayNum = $derived(result || expression || '0');

  /** Representaciones en las 4 bases */
  const baseDisplays = $derived(toBaseDisplays(displayNum));

  /** Array de 16 LEDs (true = encendido, false = apagado) para el valor actual */
  const bits = $derived(() => {
    const num = parseInt(displayNum, 10);
    if (Number.isNaN(num)) return Array(16).fill(false);
    const abs = Math.abs(num);
    const arr = [];
    for (let i = 15; i >= 0; i--) {
      arr.push(((abs >> i) & 1) === 1);
    }
    return arr;
  });

  /** Color para el LED según estado */
  function ledClass(on) {
    return on
      ? 'bg-green-400 dark:bg-green-500 shadow-[0_0_4px_rgba(74,222,128,0.6)]'
      : 'bg-gray-400 dark:bg-gray-600';
  }
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-4 pb-3 pt-2 mx-2 my-1">
  <!-- Selector de base -->
  <div class="flex gap-1 mb-2">
    {#each bases as b, i (i)}
      <button
        class="flex-1 py-1.5 text-xs font-semibold border border-gray-500 dark:border-gray-500 rounded-lg transition-all duration-150 cursor-pointer"
        class:bg-teal-600!={base === b}
        class:text-white!={base === b}
        class:bg-transparent={base !== b}
        class:text-gray-400={base !== b}
        class:hover:bg-gray-700={base !== b}
        class:hover:text-gray-300={base !== b}
        onclick={() => calculator.setBase(b)}
      >
        {b}
      </button>
    {/each}
  </div>

  <!-- Display en las 4 bases -->
  <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-3 py-2 mb-2 font-mono text-sm space-y-0.5">
    {#each bases as b, i (i)}
      <div class="flex items-center gap-2">
        <span class="w-8 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase">{b}:</span>
        <span class="text-gray-800 dark:text-gray-200 tabular-nums">
          {baseDisplays()[b]}
        </span>
      </div>
    {/each}
    {#if error}
      <div class="text-red-500 text-xs mt-1">{error}</div>
    {/if}
  </div>

  <!-- Indicador de bits (16 LEDs) -->
  <div class="flex items-center gap-0.5 justify-center">
    <span class="text-[10px] font-mono text-gray-500 dark:text-gray-400 mr-1">MSB</span>
    {#each bits() as on, i (i)}
      <div
        class="w-3.5 h-3.5 rounded-sm transition-all duration-150 {ledClass(on)}"
        title="Bit {15 - i}: {on ? 1 : 0}"
      ></div>
    {/each}
    <span class="text-[10px] font-mono text-gray-500 dark:text-gray-400 ml-1">LSB</span>
  </div>
</div>
