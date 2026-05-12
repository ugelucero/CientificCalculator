<script>
  /**
   * ModeSelector.svelte - Selector de modo de calculadora, modo angular y precisión.
   * Permite elegir entre Standard, Scientific, Programmer, etc.
   * También alterna entre Deg, Rad, Grad y ajusta precisión decimal.
   */
  const {
    currentMode = 'Standard',
    currentAngleMode = 'Deg',
    precision = 10,
    onModeChange = () => {},
    onAngleModeChange = () => {},
    onPrecisionChange = () => {},
  } = $props();

  const modes = [
    { value: 'Standard',    label: 'Std' },
    { value: 'Scientific',  label: 'Sci' },
    { value: 'Programmer',  label: 'Prog' },
    { value: 'Complex',     label: 'Cmplx' },
    { value: 'Matrix',      label: 'Mat' },
    { value: 'Statistics',  label: 'Stat' },
  ];

  const angleModes = [
    { value: 'Deg',  label: 'DEG' },
    { value: 'Rad',  label: 'RAD' },
    { value: 'Grad', label: 'GRAD' },
  ];
</script>

<div class="flex flex-col gap-1">
  <!-- Fila superior: modo y ángulo -->
  <div class="flex justify-between items-center gap-1.5">
    <div class="flex gap-0.5">
      {#each modes as mode, i (i)}
        <button
          class="px-2 py-1 text-[11px] font-semibold border border-gray-600 dark:border-gray-600 rounded transition-all duration-150"
          class:bg-blue-600!={currentMode === mode.value}
          class:text-white!={currentMode === mode.value}
          class:bg-transparent={currentMode !== mode.value}
          class:text-gray-400={currentMode !== mode.value}
          class:hover:bg-gray-700={currentMode !== mode.value}
          class:hover:text-gray-300={currentMode !== mode.value}
          onclick={() => onModeChange(mode.value)}
        >
          {mode.label}
        </button>
      {/each}
    </div>

    <div class="flex gap-0.5">
      {#each angleModes as am, i (i)}
        <button
          class="px-2 py-1 text-[11px] font-semibold border border-gray-600 dark:border-gray-600 rounded transition-all duration-150"
          class:bg-purple-700!={currentAngleMode === am.value}
          class:text-white!={currentAngleMode === am.value}
          class:bg-transparent={currentAngleMode !== am.value}
          class:text-gray-400={currentAngleMode !== am.value}
          class:hover:bg-gray-700={currentAngleMode !== am.value}
          class:hover:text-gray-300={currentAngleMode !== am.value}
          onclick={() => onAngleModeChange(am.value)}
        >
          {am.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Fila inferior: control de precisión -->
  <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
    <span>Precision:</span>
    <select
      value={precision}
      onchange={(e) => onPrecisionChange(Number(e.target.value))}
      class="bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border border-gray-400 dark:border-gray-600 rounded px-1.5 py-0.5 text-xs focus:outline-none focus:ring-1 focus:ring-blue-500"
    >
      {#each Array.from({ length: 16 }, (_, i) => i) as val}
        <option value={val}>{val}</option>
      {/each}
    </select>
  </div>
</div>
