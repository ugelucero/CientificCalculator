<script>
  /**
   * ModeSelector.svelte - Selector de modo de calculadora.
   * Permite elegir entre Standard, Scientific, Programmer, etc.
   */
  let { currentMode = 'Standard', onModeChange = () => {} } = $props();

  const modes = [
    { value: 'Standard',    label: 'Standard' },
    { value: 'Scientific',  label: 'Scientific' },
    { value: 'Programmer',  label: 'Programmer' },
    { value: 'Statistics',  label: 'Statistics' },
    { value: 'Complex',     label: 'Complex' },
    { value: 'Matrix',      label: 'Matrix' },
  ];
</script>

<div class="mode-selector">
  {#each modes as mode}
    <button
      class="mode-btn"
      class:active={currentMode === mode.value}
      onclick={() => onModeChange(mode.value)}
    >
      {mode.label}
    </button>
  {/each}
</div>

<style>
  .mode-selector {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    padding: 8px;
  }

  .mode-btn {
    flex: 1;
    min-width: 80px;
    padding: 6px 10px;
    border: 1px solid #374151;
    border-radius: 6px;
    background: #111827;
    color: #9ca3af;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .mode-btn:hover {
    background: #1f2937;
    color: #d1d5db;
  }

  .mode-btn.active {
    background: #2563eb;
    color: #ffffff;
    border-color: #2563eb;
  }
</style>
