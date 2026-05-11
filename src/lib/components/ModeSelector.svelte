<script>
  /**
   * ModeSelector.svelte - Selector de modo de calculadora y modo angular.
   * Permite elegir entre Standard, Scientific, Programmer, etc.
   * También alterna entre Deg, Rad, Grad.
   */
  const {
    currentMode = 'Standard',
    currentAngleMode = 'Deg',
    onModeChange = () => {},
    onAngleModeChange = () => {},
  } = $props();

  const modes = [
    { value: 'Standard',    label: 'Std' },
    { value: 'Scientific',  label: 'Sci' },
    { value: 'Programmer',  label: 'Prog' },
  ];

  const angleModes = [
    { value: 'Deg',  label: 'DEG' },
    { value: 'Rad',  label: 'RAD' },
    { value: 'Grad', label: 'GRAD' },
  ];
</script>

<div class="mode-selector">
  <div class="mode-group">
    {#each modes as mode, i (i)}
      <button
        class="mode-btn"
        class:active={currentMode === mode.value}
        onclick={() => onModeChange(mode.value)}
      >
        {mode.label}
      </button>
    {/each}
  </div>

  <div class="angle-group">
    {#each angleModes as am, i (i)}
      <button
        class="angle-btn"
        class:active={currentAngleMode === am.value}
        onclick={() => onAngleModeChange(am.value)}
      >
        {am.label}
      </button>
    {/each}
  </div>
</div>

<style>
  .mode-selector {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
  }

  .mode-group,
  .angle-group {
    display: flex;
    gap: 3px;
  }

  .mode-btn,
  .angle-btn {
    padding: 4px 8px;
    border: 1px solid #374151;
    border-radius: 5px;
    background: #111827;
    color: #9ca3af;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .mode-btn:hover,
  .angle-btn:hover {
    background: #1f2937;
    color: #d1d5db;
  }

  .mode-btn.active {
    background: #2563eb;
    color: #ffffff;
    border-color: #2563eb;
  }

  .angle-btn.active {
    background: #7c3aed;
    color: #ffffff;
    border-color: #7c3aed;
  }
</style>
