<script>
  /**
   * ScientificKeys.svelte - Panel de funciones científicas expandible.
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
</script>

<div class="scientific-panel">
  <button class="toggle-btn" onclick={onToggleExpand}>
    {expanded ? '▲' : '▼'} Scientific
  </button>

  {#if expanded}
    <div class="scientific-keys">
      {#each scientificButtons as row, i (i)}
        <div class="sci-row">
          {#each row as btn, j (j)}
            <button
              class="sci-btn"
              onclick={() => onKeyPress(btn.value)}
            >
              {btn.label}
            </button>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .scientific-panel {
    padding: 4px 8px;
  }

  .toggle-btn {
    width: 100%;
    padding: 8px 12px;
    background: #1f2937;
    color: #9ca3af;
    border: 1px solid #374151;
    border-radius: 8px;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .toggle-btn:hover {
    background: #374151;
  }

  .scientific-keys {
    margin-top: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .sci-row {
    display: flex;
    gap: 4px;
  }

  .sci-btn {
    flex: 1;
    aspect-ratio: 1.6;
    border: none;
    border-radius: 8px;
    background: #374151;
    color: #d1d5db;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .sci-btn:hover {
    background: #4b5563;
  }

  .sci-btn:active {
    transform: scale(0.95);
  }
</style>
