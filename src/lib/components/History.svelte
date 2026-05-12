<script>
  /**
   * History.svelte - Panel de historial de evaluaciones.
   * Muestra la lista de expresiones previas y sus resultados.
   */
  const { entries = [], onSelect = () => {}, onClear = () => {} } = $props();
</script>

<div class="px-2 pb-2">
  <div class="flex justify-between items-center mb-2">
    <h3 class="m-0 text-sm font-semibold text-gray-600 dark:text-gray-400">History</h3>
    {#if entries.length > 0}
      <button
        class="bg-transparent border border-gray-400 dark:border-gray-600 text-gray-500 dark:text-gray-400 px-2 py-1 rounded text-xs cursor-pointer hover:bg-gray-300 dark:hover:bg-gray-700 hover:text-red-500 dark:hover:text-red-400"
        onclick={onClear}
      >Clear</button>
    {/if}
  </div>

  <div class="max-h-[200px] overflow-y-auto flex flex-col gap-1">
    {#if entries.length === 0}
      <p class="text-gray-400 dark:text-gray-600 text-xs text-center py-4">No history yet</p>
    {:else}
      {#each entries as entry, i (i)}
        <button
          class="flex flex-col bg-gray-200 dark:bg-gray-800 border-none rounded-lg px-3 py-2 cursor-pointer text-left transition-colors duration-150 hover:bg-gray-300 dark:hover:bg-gray-700"
          onclick={() => onSelect(entry)}
        >
          <span class="text-sm text-gray-500 dark:text-gray-400">{entry.expression}</span>
          <span class="text-base font-semibold text-gray-800 dark:text-gray-100">{entry.result}</span>
        </button>
      {/each}
    {/if}
  </div>
</div>
