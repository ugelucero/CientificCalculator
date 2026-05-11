<script>
  import { calculator } from './lib/stores/calculator.js';
  import Display from './lib/components/Display.svelte';
  import Keypad from './lib/components/Keypad.svelte';
  import ScientificKeys from './lib/components/ScientificKeys.svelte';
  import History from './lib/components/History.svelte';
  import ModeSelector from './lib/components/ModeSelector.svelte';

  let state = $derived($calculator);

  function handleKeyPress(value) {
    if (value === 'equals') {
      // TODO: invoke evaluate_expression from Tauri API
      calculator.setResult('TODO: connect Rust backend', state.expression);
      return;
    }
    calculator.append(value);
  }

  function handleModeChange(mode) {
    calculator.setMode(mode);
  }

  function handleHistorySelect(entry) {
    calculator.append(entry.expression);
  }

  function handleClearHistory() {
    calculator.setHistory([]);
  }
</script>

<div class="calculator-app">
  <ModeSelector
    currentMode={state.mode}
    onModeChange={handleModeChange}
  />

  <Display
    expression={state.expression}
    result={state.result}
    error={state.error}
  />

  <ScientificKeys
    expanded={state.scientificExpanded}
    onKeyPress={handleKeyPress}
    onToggleExpand={() => calculator.toggleScientific()}
  />

  <Keypad onKeyPress={handleKeyPress} />

  <History
    entries={state.history}
    onSelect={handleHistorySelect}
    onClear={handleClearHistory}
  />
</div>

<style>
  .calculator-app {
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
    min-height: 100vh;
    background: #111827;
    display: flex;
    flex-direction: column;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
</style>
