<script>
  /**
   * ScientificKeys.svelte - Panel de funciones científicas y constantes expandible.
   */
  const { expanded = false, onKeyPress = () => {}, onToggleExpand = () => {} } = $props();

  const scientificButtons = [
    [
      { label: 'sin',  value: 'sin(' },
      { label: 'cos',  value: 'cos(' },
      { label: 'tan',  value: 'tan(' },
      { label: 'π',    value: 'pi' },
    ],
    [
      { label: 'log',  value: 'log(' },
      { label: 'ln',   value: 'ln(' },
      { label: '√',    value: 'sqrt(' },
      { label: 'e',    value: 'e' },
    ],
    [
      { label: 'x²',   value: '^2' },
      { label: 'xʸ',   value: '^' },
      { label: '(',    value: '(' },
      { label: ')',    value: ')' },
    ],
    [
      { label: 'EXP',  value: 'exp(' },
      { label: '|x|',  value: 'abs(' },
      { label: 'n!',   value: '!' },
      { label: '1/x',  value: '^(-1)' },
    ],
  ];

  /** Constantes predefinidas */
  const constants = [
    { label: 'π', value: 'pi',   tooltip: '3.14159...' },
    { label: 'e', value: 'e',    tooltip: '2.71828...' },
    { label: 'φ', value: 'phi',  tooltip: '1.61803...' },
    { label: 'c', value: 'c',    tooltip: '299 792 458' },
  ];
</script>

<div class="px-2">
  <button
    class="w-full py-2 px-3 bg-gray-300 dark:bg-gray-800 text-gray-500 dark:text-gray-400 border border-gray-400 dark:border-gray-700 rounded-lg text-sm cursor-pointer transition-colors duration-150 hover:bg-gray-400 dark:hover:bg-gray-700"
    onclick={onToggleExpand}
  >
    {expanded ? '▲' : '▼'} Scientific
  </button>

  {#if expanded}
    <!-- Botones científicos -->
    <div class="mt-1.5 flex flex-col gap-1">
      {#each scientificButtons as row, i (i)}
        <div class="flex gap-1">
          {#each row as btn, j (j)}
            <button
              class="flex-1 aspect-[1.6] border-none rounded-lg bg-gray-300 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm font-medium cursor-pointer transition-all duration-150 hover:bg-gray-400 dark:hover:bg-gray-600 active:scale-95"
              onclick={() => onKeyPress(btn.value)}
            >
              {btn.label}
            </button>
          {/each}
        </div>
      {/each}
    </div>

    <!-- Sección de constantes -->
    <div class="mt-2 border border-gray-400 dark:border-gray-700 rounded-lg p-2">
      <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1.5 uppercase tracking-wide">Constantes</div>
      <div class="flex gap-1">
        {#each constants as c, i (i)}
          <button
            class="flex-1 aspect-[1.6] border-none rounded-lg bg-indigo-200 dark:bg-indigo-900 text-indigo-700 dark:text-indigo-300 text-sm font-medium cursor-pointer transition-all duration-150 hover:bg-indigo-300 dark:hover:bg-indigo-800 active:scale-95"
            title={c.tooltip}
            onclick={() => onKeyPress(c.value)}
          >
            {c.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
