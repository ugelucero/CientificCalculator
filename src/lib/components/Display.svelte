<script>
  /**
   * Display.svelte - Pantalla de la calculadora.
   * Muestra expresión actual, resultado y estado (modo, ángulo, memoria).
   */
  const {
    expression = '',
    result = '',
    error = null,
    angleMode = 'Deg',
    memory = null,
    mode = 'Standard',
  } = $props();

  /** Abreviaturas para modos angulares */
  const angleLabel = $derived(
    angleMode === 'Deg' ? 'DEG' : angleMode === 'Rad' ? 'RAD' : 'GRAD'
  );

  /** Etiqueta del modo actual */
  const modeLabel = $derived(
    mode === 'Scientific' ? 'SCI' : mode === 'Programmer' ? 'PROG' : ''
  );
</script>

<div class="display">
  <!-- Barra de estado: indicadores -->
  <div class="status-bar">
    <span class="status-left">
      {#if modeLabel}
        <span class="badge mode-badge">{modeLabel}</span>
      {/if}
    </span>
    <span class="status-right">
      {#if memory !== null}
        <span class="badge memory-badge">M</span>
      {/if}
      <span class="badge angle-badge">{angleLabel}</span>
    </span>
  </div>

  <!-- Expresión actual -->
  <div class="expression" class:error={error !== null}>
    {expression || '\u00A0'}
  </div>

  <!-- Resultado o mensaje de error -->
  <div class="result" class:error={error !== null}>
    {#if error}
      <span class="error-message">{error}</span>
    {:else}
      {result || '\u00A0'}
    {/if}
  </div>
</div>

<style>
  .display {
    background: #1a1d23;
    border-radius: 12px;
    padding: 8px 16px 16px;
    margin: 8px;
    min-height: 96px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    overflow: hidden;
  }

  /* ── Barra de estado ─────────────────────────────── */
  .status-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
    min-height: 20px;
  }

  .status-left,
  .status-right {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .badge {
    font-size: 11px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 3px;
    letter-spacing: 0.5px;
    user-select: none;
  }

  .mode-badge {
    background: #7c3aed;
    color: #e8d5ff;
  }

  .angle-badge {
    background: #374151;
    color: #9ca3af;
  }

  .memory-badge {
    background: #d97706;
    color: #fef3c7;
  }

  /* ── Expresión ──────────────────────────────────── */
  .expression {
    color: #6b7280;
    font-size: 16px;
    text-align: right;
    word-break: break-all;
    min-height: 24px;
    line-height: 1.4;
    overflow-x: auto;
    white-space: nowrap;
    direction: rtl;
    text-align: left;
  }

  /* ── Resultado ──────────────────────────────────── */
  .result {
    color: #f3f4f6;
    font-size: 32px;
    font-weight: 600;
    text-align: right;
    word-break: break-all;
    min-height: 40px;
    line-height: 1.2;
    transition: color 0.2s;
    overflow-x: auto;
    white-space: nowrap;
  }

  .result.error,
  .expression.error {
    color: #ef4444;
  }

  .error-message {
    font-size: 18px;
    font-weight: 500;
    white-space: normal;
    word-break: break-word;
  }
</style>
