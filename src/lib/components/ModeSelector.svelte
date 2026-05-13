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
    { value: 'Complex',     label: 'Cmplx' },
    { value: 'Matrix',      label: 'Mat' },
    { value: 'Statistics',  label: 'Stat' },
    { value: 'Solver',      label: 'Solv' },
    { value: 'Graph',       label: 'Graph' },
  ];

  const angleModes = [
    { value: 'Deg',  label: 'DEG' },
    { value: 'Rad',  label: 'RAD' },
    { value: 'Grad', label: 'GRAD' },
  ];
</script>

<div class="flex justify-between items-center gap-1.5">
    <div class="flex gap-0.5 flex-wrap">
      {#each modes as mode, i (i)}
        <button
          class="px-1.5 py-1 text-[10px] font-semibold border border-gray-600 dark:border-gray-600 rounded transition-all duration-150"
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

    <div class="flex gap-0.5 flex-shrink-0">
      {#each angleModes as am, i (i)}
        <button
          class="px-1.5 py-1 text-[10px] font-semibold border border-gray-600 dark:border-gray-600 rounded transition-all duration-150"
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
