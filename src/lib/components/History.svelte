<script>
  /**
   * History.svelte - Panel de historial de evaluaciones.
   * Muestra la lista de expresiones previas y sus resultados.
   */
  const { entries = [], onSelect = () => {}, onClear = () => {} } = $props();
</script>

<div class="history-panel">
  <div class="history-header">
    <h3>History</h3>
    {#if entries.length > 0}
      <button class="clear-btn" onclick={onClear}>Clear</button>
    {/if}
  </div>

  <div class="history-list">
    {#if entries.length === 0}
      <p class="empty">No history yet</p>
    {:else}
      {#each entries as entry, i (i)}
        <button class="history-entry" onclick={() => onSelect(entry)}>
          <span class="entry-expr">{entry.expression}</span>
          <span class="entry-result">{entry.result}</span>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .history-panel {
    padding: 8px;
  }

  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .history-header h3 {
    margin: 0;
    color: #d1d5db;
    font-size: 14px;
    font-weight: 600;
  }

  .clear-btn {
    background: transparent;
    border: 1px solid #374151;
    color: #9ca3af;
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
  }

  .clear-btn:hover {
    background: #374151;
    color: #ef4444;
  }

  .history-list {
    max-height: 200px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .empty {
    color: #6b7280;
    font-size: 13px;
    text-align: center;
    padding: 16px;
  }

  .history-entry {
    display: flex;
    flex-direction: column;
    background: #1f2937;
    border: none;
    border-radius: 8px;
    padding: 8px 12px;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s;
  }

  .history-entry:hover {
    background: #374151;
  }

  .entry-expr {
    color: #9ca3af;
    font-size: 13px;
  }

  .entry-result {
    color: #f3f4f6;
    font-size: 16px;
    font-weight: 600;
  }
</style>
