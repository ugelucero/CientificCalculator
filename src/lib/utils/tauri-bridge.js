/**
 * tauri-bridge.js - Wrapper para invocar comandos Tauri con tipado.
 * Se importa @tauri-apps/api/core dinámicamente para evitar
 * errores en entornos sin Tauri (navegador normal).
 */

/** @type {import('@tauri-apps/api/core').invoke|null} */
let invokeFn = null;

async function getInvoke() {
  if (!invokeFn) {
    try {
      const tauriCore = await import('@tauri-apps/api/core');
      invokeFn = tauriCore.invoke;
    } catch {
      // Tauri no disponible (entorno navegador)
      invokeFn = null;
    }
  }
  return invokeFn;
}

/**
 * API de la calculadora - comandos Tauri.
 * Cada método se mapea a un comando snake_case del backend Rust.
 */
export const api = {
  async evaluateExpression(expr, mode) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('evaluate_expression', { expr, mode });
  },

  async convertUnits(value, from, to) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('convert_units', { value, from, to });
  },

  async getHistory() {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('get_history');
  },

  async clearHistory() {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('clear_history');
  },

  async memoryStore(value) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('memory_store', { value });
  },

  async memoryRecall() {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('memory_recall');
  },

  async memoryClear() {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('memory_clear');
  },

  async memoryAdd(value) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('memory_add', { value });
  },

  async memorySubtract(value) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('memory_subtract', { value });
  },

  async setAngleMode(mode) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('set_angle_mode', { mode });
  },

  async setPrecision(digits) {
    const invoke = await getInvoke();
    if (!invoke) throw new Error('Tauri API not available');
    return invoke('set_precision', { digits });
  },
};
