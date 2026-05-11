<script>
  /**
   * Keypad.svelte - Teclado numérico y operadores básicos.
   * Emite eventos al presionar cada tecla.
   */
  const { onKeyPress = () => {} } = $props();

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
</script>

<div class="keypad">
  {#each buttons as row, i (i)}
    <div class="keypad-row">
      {#each row as btn, j (j)}
        <button
          class="key-btn {btn.variant}"
          onclick={() => onKeyPress(btn.value)}
        >
          {btn.label}
        </button>
      {/each}
    </div>
  {/each}
</div>

<style>
  .keypad {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px;
  }

  .keypad-row {
    display: flex;
    gap: 6px;
  }

  .key-btn {
    flex: 1;
    aspect-ratio: 1.2;
    border: none;
    border-radius: 10px;
    font-size: 20px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    user-select: none;
  }

  .key-btn:active {
    transform: scale(0.95);
    opacity: 0.85;
  }

  .key-btn.digit {
    background: #374151;
    color: #f3f4f6;
  }

  .key-btn.digit:hover {
    background: #4b5563;
  }

  .key-btn.operator {
    background: #2563eb;
    color: #ffffff;
  }

  .key-btn.operator:hover {
    background: #3b82f6;
  }

  .key-btn.action {
    background: #4b5563;
    color: #d1d5db;
  }

  .key-btn.action:hover {
    background: #6b7280;
  }

  .key-btn.equals {
    background: #059669;
    color: #ffffff;
    font-weight: 700;
  }

  .key-btn.equals:hover {
    background: #10b981;
  }
</style>
