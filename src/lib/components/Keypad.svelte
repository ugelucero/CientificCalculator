<script>
  /**
   * Keypad.svelte - Teclado numérico y operadores básicos.
   * Emite eventos al presionar cada tecla.
   * Soporta teclas adicionales para modo Programmer (HEX A-F, bitwise ops).
   */
  const {
    onKeyPress = () => {},
    mode = 'Standard',
    base = 'DEC',
  } = $props();

  const buttons = [
    // Fila 1
    [
      { label: 'C',   value: 'clear',    variant: 'action' },
      { label: '⌫',  value: 'backspace', variant: 'action' },
      { label: '%',  value: '%',         variant: 'operator' },
      { label: '÷',  value: '/',         variant: 'operator' },
    ],
    // Fila 2
    [
      { label: '7',  value: '7',  variant: 'digit' },
      { label: '8',  value: '8',  variant: 'digit' },
      { label: '9',  value: '9',  variant: 'digit' },
      { label: '×',  value: '*',  variant: 'operator' },
    ],
    // Fila 3
    [
      { label: '4',  value: '4',  variant: 'digit' },
      { label: '5',  value: '5',  variant: 'digit' },
      { label: '6',  value: '6',  variant: 'digit' },
      { label: '−',  value: '-',  variant: 'operator' },
    ],
    // Fila 4
    [
      { label: '1',  value: '1',  variant: 'digit' },
      { label: '2',  value: '2',  variant: 'digit' },
      { label: '3',  value: '3',  variant: 'digit' },
      { label: '+',  value: '+',  variant: 'operator' },
    ],
    // Fila 5
    [
      { label: '0',  value: '0',  variant: 'digit' },
      { label: '.',  value: '.',  variant: 'digit' },
      { label: '±',  value: 'negate', variant: 'action' },
      { label: '=',  value: 'equals', variant: 'equals' },
    ],
  ];

  /** Botones HEX A-F (solo cuando mode=Programmer y base=HEX) */
  const hexDigits = [
    { label: 'A', value: 'A', variant: 'digit' },
    { label: 'B', value: 'B', variant: 'digit' },
    { label: 'C', value: 'C', variant: 'digit' },
    { label: 'D', value: 'D', variant: 'digit' },
    { label: 'E', value: 'E', variant: 'digit' },
    { label: 'F', value: 'F', variant: 'digit' },
  ];

  /** Botones de operaciones bitwise (solo cuando mode=Programmer) */
  const bitwiseOps = [
    { label: 'AND', value: '&',   variant: 'bitwise' },
    { label: 'OR',  value: '|',   variant: 'bitwise' },
    { label: 'XOR', value: '^',   variant: 'bitwise' },
    { label: 'NOT', value: '~',   variant: 'bitwise' },
    { label: 'SHL', value: '<<',  variant: 'bitwise' },
    { label: 'SHR', value: '>>',  variant: 'bitwise' },
  ];

  /** Clases Tailwind por variante de botón */
  const variantClasses = {
    digit:
      'bg-gray-300 dark:bg-gray-700 hover:bg-gray-400 dark:hover:bg-gray-600 active:scale-95 active:opacity-85 text-gray-800 dark:text-gray-100',
    operator:
      'bg-blue-500 dark:bg-blue-600 hover:bg-blue-400 dark:hover:bg-blue-500 active:scale-95 active:opacity-85 text-white',
    action:
      'bg-gray-400 dark:bg-gray-600 hover:bg-gray-500 dark:hover:bg-gray-500 active:scale-95 active:opacity-85 text-gray-800 dark:text-gray-300',
    equals:
      'bg-green-500 dark:bg-green-600 hover:bg-green-400 dark:hover:bg-green-500 active:scale-95 active:opacity-85 text-white font-bold',
    bitwise:
      'bg-teal-500 dark:bg-teal-600 hover:bg-teal-400 dark:hover:bg-teal-500 active:scale-95 active:opacity-85 text-white',
  };
</script>

<div class="flex flex-col gap-1.5 p-2">
  {#if mode === 'Programmer'}
    <!-- Fila de dígitos HEX (A-F) solo si base es HEX -->
    {#if base === 'HEX'}
      <div class="flex gap-1.5">
        {#each hexDigits as btn, i (i)}
          <button
            class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 {variantClasses[btn.variant]}"
            onclick={() => onKeyPress(btn.value)}
          >
            {btn.label}
          </button>
        {/each}
      </div>
    {/if}

    <!-- Fila de operaciones bitwise -->
    <div class="flex gap-1.5">
      {#each bitwiseOps as btn, i (i)}
        <button
          class="flex-1 aspect-[1.2] border-none rounded-xl text-sm font-bold cursor-pointer select-none transition-all duration-150 {variantClasses[btn.variant]}"
          onclick={() => onKeyPress(btn.value)}
        >
          {btn.label}
        </button>
      {/each}
    </div>
  {/if}

  {#if mode === 'Matrix'}
    <!-- Botones para modo Matriz: [, ], ,, ; -->
    <div class="flex gap-1.5">
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 bg-indigo-500 dark:bg-indigo-600 hover:bg-indigo-400 dark:hover:bg-indigo-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('[')}
      >
        [
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 bg-indigo-500 dark:bg-indigo-600 hover:bg-indigo-400 dark:hover:bg-indigo-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress(']')}
      >
        ]
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 bg-indigo-500 dark:bg-indigo-600 hover:bg-indigo-400 dark:hover:bg-indigo-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress(',')}
      >
        ,
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 bg-indigo-500 dark:bg-indigo-600 hover:bg-indigo-400 dark:hover:bg-indigo-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress(';')}
      >
        ;
      </button>
    </div>
  {/if}

  {#if mode === 'Complex'}
    <!-- Botones para modo Complejo: i, conj, arg, real, imag -->
    <div class="flex gap-1.5">
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-bold cursor-pointer select-none transition-all duration-150 bg-purple-500 dark:bg-purple-600 hover:bg-purple-400 dark:hover:bg-purple-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('i')}
        title="Unidad imaginaria i"
      >
        i
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-sm font-bold cursor-pointer select-none transition-all duration-150 bg-violet-500 dark:bg-violet-600 hover:bg-violet-400 dark:hover:bg-violet-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('conj(')}
        title="Conjugado"
      >
        conj
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-sm font-bold cursor-pointer select-none transition-all duration-150 bg-violet-500 dark:bg-violet-600 hover:bg-violet-400 dark:hover:bg-violet-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('arg(')}
        title="Argumento (fase)"
      >
        arg
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-sm font-bold cursor-pointer select-none transition-all duration-150 bg-violet-500 dark:bg-violet-600 hover:bg-violet-400 dark:hover:bg-violet-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('real(')}
        title="Parte real"
      >
        real
      </button>
      <button
        class="flex-1 aspect-[1.2] border-none rounded-xl text-sm font-bold cursor-pointer select-none transition-all duration-150 bg-violet-500 dark:bg-violet-600 hover:bg-violet-400 dark:hover:bg-violet-500 active:scale-95 active:opacity-85 text-white"
        onclick={() => onKeyPress('imag(')}
        title="Parte imaginaria"
      >
        imag
      </button>
    </div>
  {/if}

  <!-- Teclado numérico principal -->
  {#each buttons as row, i (i)}
    <div class="flex gap-1.5">
      {#each row as btn, j (j)}
        <button
          class="flex-1 aspect-[1.2] border-none rounded-xl text-xl font-medium cursor-pointer select-none transition-all duration-150 {variantClasses[btn.variant]}"
          onclick={() => onKeyPress(btn.value)}
        >
          {btn.label}
        </button>
      {/each}
    </div>
  {/each}
</div>
