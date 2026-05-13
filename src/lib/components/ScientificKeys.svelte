<script>
  /**
   * ScientificKeys.svelte - Panel de funciones científicas.
   * 3 filas compactas con todas las funciones visibles sin scroll.
   * Las constantes tienen color distinto para distinguirse.
   */
  const { expanded = false, onKeyPress = () => {}, onToggleExpand = () => {} } = $props();

  /** Filas de botones: cada sub-array es una fila */
  const rows = [
    // Fila 1 — Trigonométricas + paréntesis
    [
      { label: 'sin',  value: 'sin(',  type: 'func' },
      { label: 'cos',  value: 'cos(',  type: 'func' },
      { label: 'tan',  value: 'tan(',  type: 'func' },
      { label: 'asin', value: 'asin(', type: 'func' },
      { label: 'acos', value: 'acos(', type: 'func' },
      { label: 'atan', value: 'atan(', type: 'func' },
      { label: '(',    value: '(',     type: 'paren' },
      { label: ')',    value: ')',     type: 'paren' },
    ],
    // Fila 2 — Logaritmos, raíces, potencias y otras funciones
    [
      { label: 'log',  value: 'log(',  type: 'func' },
      { label: 'ln',   value: 'ln(',   type: 'func' },
      { label: 'log₂', value: 'log2(', type: 'func' },
      { label: '√',    value: 'sqrt(', type: 'func' },
      { label: '∛',    value: 'cbrt(', type: 'func' },
      { label: 'x²',   value: '^2',    type: 'func' },
      { label: 'xʸ',   value: '^',     type: 'op' },
      { label: 'exp',  value: 'exp(',  type: 'func' },
      { label: '|x|',  value: 'abs(',  type: 'func' },
      { label: 'n!',   value: '!',     type: 'func' },
      { label: '1/x',  value: '^(-1)', type: 'func' },
    ],
    // Fila 3 — Constantes (color distinto)
    [
      { label: 'π', value: 'pi',   type: 'const', tooltip: '3.14159...' },
      { label: 'e', value: 'e',    type: 'const', tooltip: '2.71828...' },
      { label: 'φ', value: 'phi',  type: 'const', tooltip: '1.61803...' },
      { label: 'c', value: 'c',    type: 'const', tooltip: '299 792 458' },
    ],
  ];
</script>

<div class="px-2">
  <button
    class="w-full py-1.5 px-3 bg-gray-200 dark:bg-gray-800 text-gray-500 dark:text-gray-400 border border-gray-300 dark:border-gray-700 rounded-lg text-xs cursor-pointer transition-colors duration-150 hover:bg-gray-300 dark:hover:bg-gray-700"
    onclick={onToggleExpand}
  >
    {expanded ? '▲' : '▼'} Científicas
  </button>

  {#if expanded}
    <div class="mt-1.5 flex flex-col gap-1">
      {#each rows as row, ri (ri)}
        <div class="flex gap-1 flex-wrap">
          {#each row as btn, bi (bi)}
            <button
              class="flex-1 min-w-0 py-1.5 px-1.5 border-none rounded-md text-xs font-medium cursor-pointer transition-all duration-100 active:scale-95 whitespace-nowrap
                {btn.type === 'const'
                  ? 'bg-violet-200 dark:bg-violet-900/60 text-violet-700 dark:text-violet-300 hover:bg-violet-300 dark:hover:bg-violet-800/80'
                  : btn.type === 'paren'
                    ? 'bg-amber-200 dark:bg-amber-900/50 text-amber-700 dark:text-amber-300 hover:bg-amber-300 dark:hover:bg-amber-800/70'
                    : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
                }"
              onclick={() => onKeyPress(btn.value)}
              title={btn.tooltip || btn.value}
            >
              {btn.label}
            </button>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>
