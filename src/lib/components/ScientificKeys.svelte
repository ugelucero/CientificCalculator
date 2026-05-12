<script>
  /**
   * ScientificKeys.svelte - Botonera científica horizontal compacta (ribbon).
   * Se muestra como una barra desplazable con botones pequeños.
   */
  const { expanded = false, onKeyPress = () => {}, onToggleExpand = () => {} } = $props();

  /** Botones científicos en orden de uso frecuente */
  const sciButtons = [
    { label: 'sin',  value: 'sin(' },
    { label: 'cos',  value: 'cos(' },
    { label: 'tan',  value: 'tan(' },
    { label: 'asin', value: 'asin(' },
    { label: 'acos', value: 'acos(' },
    { label: 'atan', value: 'atan(' },
    { label: 'sinh', value: 'sinh(' },
    { label: 'cosh', value: 'cosh(' },
    { label: 'tanh', value: 'tanh(' },
    { label: 'log',  value: 'log(' },
    { label: 'ln',   value: 'ln(' },
    { label: 'log₂', value: 'log2(' },
    { label: '√',    value: 'sqrt(' },
    { label: '∛',    value: 'cbrt(' },
    { label: 'x²',   value: '^2' },
    { label: 'xʸ',   value: '^' },
    { label: 'exp',  value: 'exp(' },
    { label: '|x|',  value: 'abs(' },
    { label: 'n!',   value: '!' },
    { label: '1/x',  value: '^(-1)' },
    { label: 'π',    value: 'pi' },
    { label: 'e',    value: 'e' },
    { label: 'φ',    value: 'phi' },
    { label: 'c',    value: 'c' },
  ];
</script>

<div class="px-2">
  <button
    class="w-full py-1.5 px-3 bg-gray-300 dark:bg-gray-800 text-gray-500 dark:text-gray-400 border border-gray-400 dark:border-gray-700 rounded-lg text-xs cursor-pointer transition-colors duration-150 hover:bg-gray-400 dark:hover:bg-gray-700"
    onclick={onToggleExpand}
  >
    {expanded ? '▲' : '▼'} Científicas
  </button>

  {#if expanded}
    <!-- Ribbon horizontal con scroll -->
    <div class="mt-1 overflow-x-auto scrollbar-thin">
      <div class="flex gap-1 min-w-max px-0.5 py-1">
        {#each sciButtons as btn, i (i)}
          <button
            class="flex-none px-2 py-1 border-none rounded-md bg-gray-300 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs font-medium cursor-pointer transition-all duration-100 hover:bg-gray-400 dark:hover:bg-gray-600 active:scale-95 whitespace-nowrap"
            onclick={() => onKeyPress(btn.value)}
            title={btn.value}
          >
            {btn.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
