<script>
  /**
   * ComplexInput.svelte — Indicador de modo números complejos.
   *
   * Muestra un display con indicación de formato complejo.
   * Botón para alternar entre forma rectangular (a + bi) y polar (r∠θ).
   */
  const {
    expression = '',
    result = '',
    error = null,
  } = $props();

  /** Formato actual: 'rectangular' | 'polar' */
  let complexFormat = $state('rectangular');

  function toggleFormat() {
    complexFormat = complexFormat === 'rectangular' ? 'polar' : 'rectangular';
  }

  const formatLabel = $derived(
    complexFormat === 'rectangular' ? 'a + bi' : 'r∠θ'
  );
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-4 pb-3 pt-2 mx-2 my-1">
  <!-- Cabecera con toggle de formato -->
  <div class="flex justify-between items-center mb-1">
    <div class="flex items-center gap-1.5">
      <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-purple-700 text-purple-200">CMPLX</span>
      <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-violet-700 text-violet-200">{formatLabel}</span>
    </div>
    <button
      onclick={toggleFormat}
      class="text-[11px] px-2 py-0.5 rounded-sm font-semibold bg-gray-400 dark:bg-gray-600
             text-gray-700 dark:text-gray-300 hover:bg-gray-500 dark:hover:bg-gray-500
             transition-colors cursor-pointer select-none"
      title="Alternar formato rectangular/polar"
    >
      ↔ {complexFormat === 'rectangular' ? 'r∠θ' : 'a + bi'}
    </button>
  </div>

  <!-- Display de expresión -->
  <div class="text-right text-base min-h-[24px] leading-relaxed overflow-x-auto whitespace-nowrap text-gray-500 dark:text-gray-400">
    {expression || '\u00A0'}
  </div>

  <!-- Display de resultado -->
  <div class="text-right min-h-[40px] leading-tight overflow-x-auto whitespace-nowrap transition-colors duration-200"
    class:text-red-500={error !== null}
  >
    {#if error}
      <span class="text-lg font-medium break-words whitespace-normal">{error}</span>
    {:else}
      <span class="text-3xl font-semibold text-gray-900 dark:text-gray-100">
        {result || '\u00A0'}
      </span>
    {/if}
  </div>
</div>
