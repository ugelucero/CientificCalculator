<script>
  /**
   * Display.svelte - Pantalla de la calculadora.
   * Muestra expresión actual, resultado y estado (modo, ángulo, memoria, precisión).
   */
  const {
    expression = '',
    result = '',
    error = null,
    angleMode = 'Deg',
    memory = null,
    mode = 'Standard',
    precision = 10,
  } = $props();

  /** Abreviaturas para modos angulares */
  const angleLabel = $derived(
    angleMode === 'Deg' ? 'DEG' : angleMode === 'Rad' ? 'RAD' : 'GRAD'
  );

  /** Etiqueta del modo actual */
  const modeLabel = $derived(
    mode === 'Scientific' ? 'SCI' : mode === 'Programmer' ? 'PROG' : ''
  );

  /** Clases dinámicas para la expresión */
  const expressionClasses = $derived(
    'text-right text-base min-h-[24px] leading-relaxed overflow-x-auto whitespace-nowrap' +
    (error !== null ? ' text-red-500' : ' text-gray-500 dark:text-gray-400')
  );
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-4 pb-4 pt-2 mx-2 my-1 min-h-[96px] flex flex-col justify-end overflow-hidden">
  <!-- Barra de estado: indicadores -->
  <div class="flex justify-between items-center mb-1 min-h-[20px]">
    <div class="flex gap-1.5 items-center">
      {#if modeLabel}
        <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-purple-700 text-purple-200">{modeLabel}</span>
      {/if}
      <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-gray-600 dark:bg-gray-700 text-gray-400 dark:text-gray-400">P{precision}</span>
    </div>
    <div class="flex gap-1.5 items-center">
      {#if memory !== null}
        <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-amber-600 text-amber-100">M</span>
      {/if}
      <span class="text-[11px] font-semibold px-1.5 py-0.5 rounded-sm select-none bg-gray-600 dark:bg-gray-700 text-gray-400 dark:text-gray-400">{angleLabel}</span>
    </div>
  </div>

  <!-- Expresión actual -->
  <div class={expressionClasses}>
    {expression || '\u00A0'}
  </div>

  <!-- Resultado o mensaje de error -->
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
