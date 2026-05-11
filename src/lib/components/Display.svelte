<script>
  /**
   * Display.svelte - Pantalla de la calculadora.
   * Muestra la expresión actual y el resultado.
   */
  let { expression = '', result = '', error = null } = $props();
</script>

<div class="display">
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
    padding: 16px 20px;
    margin: 8px;
    min-height: 80px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    overflow: hidden;
  }

  .expression {
    color: #6b7280;
    font-size: 16px;
    text-align: right;
    word-break: break-all;
    min-height: 24px;
    line-height: 1.4;
  }

  .result {
    color: #f3f4f6;
    font-size: 32px;
    font-weight: 600;
    text-align: right;
    word-break: break-all;
    min-height: 40px;
    line-height: 1.2;
    transition: color 0.2s;
  }

  .result.error,
  .expression.error {
    color: #ef4444;
  }

  .error-message {
    font-size: 18px;
    font-weight: 500;
  }
</style>
