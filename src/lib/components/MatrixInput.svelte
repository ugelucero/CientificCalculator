<script>
  /**
   * MatrixInput.svelte — Editor y operaciones con matrices.
   *
   * Proporciona:
   * - Selector de tamaño (filas × columnas, 1-5)
   * - Grid de entrada para matriz A (y matriz B para operaciones binarias)
   * - Botones de operación: +, -, ×, det, inv, transp, identidad
   * - Display del resultado con formato de caja
   */
  import { calculator } from '../stores/calculator.js';

  let rows = $state(2);
  let cols = $state(2);

  /** Matriz A: matriz principal de entrada */
  let matrixA = $state([]);

  /** Matriz B: segunda matriz para operaciones binarias */
  let matrixB = $state([]);

  /** Crea una matriz vacía (ceros) de r × c */
  function createEmptyMatrix(r, c) {
    return Array.from({ length: r }, () => Array(c).fill(0));
  }

  /** Operación binaria activa (muestra matriz B) */
  let showB = $state(false);

  /** Resultado formateado devuelto por el backend */
  let resultDisplay = $state('');

  /** Mensaje de error */
  let errorMessage = $state('');

  /** Indica si hubo un resultado */
  let hasResult = $state(false);

  /** Inicializar al montar (solo la primera vez) */
  let initialized = false;
  $effect(() => {
    if (!initialized && rows > 0 && cols > 0) {
      initialized = true;
      matrixA = createEmptyMatrix(rows, cols);
      matrixB = createEmptyMatrix(rows, cols);
    }
  });

  /** Redimensiona cuando cambian rows/cols, sin bucle infinito */
  function onSizeChange() {
    const r = rows;
    const c = cols;
    matrixA = createEmptyMatrix(r, c);
    matrixB = createEmptyMatrix(r, c);
    resultDisplay = '';
    errorMessage = '';
    hasResult = false;
  }

  /** Serializa una matriz al formato [[a,b],[c,d]] */
  function serializeMatrix(mat) {
    return '[' + mat.map(row => '[' + row.join(',') + ']').join(',') + ']';
  }

  /** Parsea valores vacíos a 0 para una matriz */
  function sanitizeMatrix(mat) {
    return mat.map(row => row.map(v => {
      const n = parseFloat(v);
      return isNaN(n) ? 0 : n;
    }));
  }

  /** Ejecuta una operación llamando al backend */
  async function executeOperation(op) {
    const sanitizedA = sanitizeMatrix(matrixA);
    errorMessage = '';
    hasResult = false;

    let expr = '';

    switch (op) {
      case 'add':
        expr = serializeMatrix(sanitizedA) + ' + ' + serializeMatrix(sanitizeMatrix(matrixB));
        break;
      case 'subtract':
        expr = serializeMatrix(sanitizedA) + ' - ' + serializeMatrix(sanitizeMatrix(matrixB));
        break;
      case 'multiply':
        expr = serializeMatrix(sanitizedA) + ' * ' + serializeMatrix(sanitizeMatrix(matrixB));
        break;
      case 'det':
        expr = 'det(' + serializeMatrix(sanitizedA) + ')';
        break;
      case 'inv':
        expr = 'inv(' + serializeMatrix(sanitizedA) + ')';
        break;
      case 'transpose':
        expr = 'trans(' + serializeMatrix(sanitizedA) + ')';
        break;
      case 'identity':
        // Generar identidad localmente
        const ident = createIdentity(rows);
        resultDisplay = formatMatrixDisplay(ident);
        hasResult = true;
        return;
      default:
        return;
    }

    // Para operaciones binarias, aseguramos que showB esté activo
    if (op === 'add' || op === 'subtract' || op === 'multiply') {
      showB = true;
    }

    try {
      calculator.clear();
      calculator.append(expr);
      await calculator.calculate();

      // Leer el resultado del store
      const unsub = calculator.subscribe((state) => {
        if (state.result) {
          resultDisplay = state.result;
          hasResult = true;
        }
        if (state.error) {
          errorMessage = state.error;
          hasResult = false;
        }
      });
      unsub();
    } catch (err) {
      errorMessage = err?.message || 'Error al ejecutar la operación';
    }
  }

  /** Crea una matriz identidad de tamaño n */
  function createIdentity(n) {
    return Array.from({ length: n }, (_, i) =>
      Array.from({ length: n }, (_, j) => (i === j ? 1 : 0))
    );
  }

  /** Formatea una matriz 2D como string para mostrar (estilo tabla simple) */
  function formatMatrixDisplay(mat) {
    if (!mat.length || !mat[0].length) return '[]';
    const cols = mat[0].length;
    // Formatear celdas
    const formatted = mat.map(row =>
      row.map(v => {
        if (Number.isInteger(v) && Math.abs(v) < 1e15) return String(v);
        const s = v.toFixed(10).replace(/\.?0+$/, '');
        return s;
      })
    );
    // Calcular anchos de columna
    const colWidths = Array(cols).fill(0);
    for (const row of formatted) {
      for (let j = 0; j < cols; j++) {
        colWidths[j] = Math.max(colWidths[j], row[j].length);
      }
    }
    // Construir display
    const lines = [];
    for (let i = 0; i < mat.length; i++) {
      const cells = formatted[i].map((c, j) => c.padStart(colWidths[j]));
      if (i === 0) {
        lines.push('┌ ' + cells.join('  ') + ' ┐');
      } else if (i === mat.length - 1) {
        lines.push('└ ' + cells.join('  ') + ' ┘');
      } else {
        lines.push('│ ' + cells.join('  ') + ' │');
      }
    }
    return lines.join('\n');
  }

  /** Tipo de operación: 'binary' (necesita B) o 'unary' (solo A) */
  function isBinaryOp(op) {
    return op === 'add' || op === 'subtract' || op === 'multiply';
  }

  /** Alterna mostrar/esconder matriz B */
  function toggleShowB() {
    showB = !showB;
  }

  /** Actualiza un valor en matrixA */
  function setMatrixAValue(row, col, value) {
    const newMatrix = matrixA.map(r => [...r]);
    newMatrix[row][col] = value === '' ? 0 : parseFloat(value) || 0;
    matrixA = newMatrix;
  }

  /** Actualiza un valor en matrixB */
  function setMatrixBValue(row, col, value) {
    const newMatrix = matrixB.map(r => [...r]);
    newMatrix[row][col] = value === '' ? 0 : parseFloat(value) || 0;
    matrixB = newMatrix;
  }
</script>

<div class="bg-gray-200 dark:bg-gray-800 rounded-xl px-3 py-2 mx-2 my-1">
  <!-- Selector de tamaño -->
  <div class="flex items-center gap-2 mb-2 text-sm text-gray-700 dark:text-gray-300">
    <span class="font-medium">Tamaño:</span>
    <label class="flex items-center gap-1">
      <span class="text-xs">Filas</span>
      <input
        type="number"
        min="1"
        max="5"
        bind:value={rows}
        onchange={onSizeChange}
        class="w-14 px-1.5 py-0.5 rounded border border-gray-400 dark:border-gray-600
               bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
               text-center text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500"
      />
    </label>
    <span class="text-gray-400">×</span>
    <label class="flex items-center gap-1">
      <span class="text-xs">Columnas</span>
      <input
        type="number"
        min="1"
        max="5"
        bind:value={cols}
        onchange={onSizeChange}
        class="w-14 px-1.5 py-0.5 rounded border border-gray-400 dark:border-gray-600
               bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
               text-center text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500"
      />
    </label>
  </div>

  <!-- Matriz A -->
  <div class="mb-2">
    <div class="flex items-center gap-2 mb-1">
      <span class="text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide">Matriz A</span>
      {#if !showB}
        <button
          onclick={toggleShowB}
          class="text-[10px] px-1.5 py-0.5 rounded bg-gray-400 dark:bg-gray-600
                 text-gray-700 dark:text-gray-300 hover:bg-gray-500 dark:hover:bg-gray-500
                 transition-colors cursor-pointer"
          title="Mostrar segunda matriz para operaciones binarias"
        >
          + B
        </button>
      {/if}
    </div>
    <div class="flex flex-col gap-0.5">
      {#each matrixA as row, i (i)}
        <div class="flex gap-0.5 justify-center">
          {#each row as cell, j (j)}
            <input
              type="number"
              value={cell}
              oninput={(e) => setMatrixAValue(i, j, e.target.value)}
              step="any"
              class="w-12 h-8 text-center text-sm border border-gray-400 dark:border-gray-600 rounded
                     bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                     focus:outline-none focus:ring-1 focus:ring-indigo-500
                     [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
          {/each}
        </div>
      {/each}
    </div>
  </div>

  <!-- Matriz B (solo para operaciones binarias) -->
  {#if showB}
    <div class="mb-2">
      <div class="flex items-center gap-2 mb-1">
        <span class="text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide">Matriz B</span>
        <button
          onclick={toggleShowB}
          class="text-[10px] px-1.5 py-0.5 rounded bg-gray-400 dark:bg-gray-600
                 text-gray-700 dark:text-gray-300 hover:bg-gray-500 dark:hover:bg-gray-500
                 transition-colors cursor-pointer"
          title="Ocultar segunda matriz"
        >
          Ocultar
        </button>
      </div>
      <div class="flex flex-col gap-0.5">
        {#each matrixB as row, i (i)}
          <div class="flex gap-0.5 justify-center">
            {#each row as cell, j (j)}
              <input
                type="number"
                value={cell}
                oninput={(e) => setMatrixBValue(i, j, e.target.value)}
                step="any"
                class="w-12 h-8 text-center text-sm border border-gray-400 dark:border-gray-600 rounded
                       bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-gray-200
                       focus:outline-none focus:ring-1 focus:ring-indigo-500
                       [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
              />
            {/each}
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Botones de operación -->
  <div class="flex flex-wrap gap-1.5 mb-2">
    <button
      onclick={() => { showB = true; executeOperation('add'); }}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Sumar matrices"
    >+</button>
    <button
      onclick={() => { showB = true; executeOperation('subtract'); }}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Restar matrices"
    >−</button>
    <button
      onclick={() => { showB = true; executeOperation('multiply'); }}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Multiplicar matrices"
    >×</button>
    <button
      onclick={() => executeOperation('det')}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Determinante"
    >det</button>
    <button
      onclick={() => executeOperation('inv')}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Inversa"
    >inv</button>
    <button
      onclick={() => executeOperation('transpose')}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Transpuesta"
    >transp</button>
    <button
      onclick={() => executeOperation('identity')}
      class="px-3 py-1.5 text-sm font-semibold rounded-lg transition-all duration-150
             bg-indigo-500 hover:bg-indigo-400 dark:bg-indigo-600 dark:hover:bg-indigo-500
             text-white cursor-pointer select-none active:scale-95"
      title="Matriz identidad"
    >identidad</button>
  </div>

  <!-- Resultado -->
  {#if hasResult || errorMessage}
    <div class="bg-gray-300 dark:bg-gray-900 rounded-lg px-3 py-2 min-h-[48px]">
      <div class="text-[10px] font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-400 mb-1">
        Resultado
      </div>
      {#if errorMessage}
        <div class="text-red-500 text-sm font-medium">{errorMessage}</div>
      {:else if resultDisplay}
        <pre class="text-sm font-mono text-gray-800 dark:text-gray-200 leading-relaxed whitespace-pre-wrap">{resultDisplay}</pre>
      {/if}
    </div>
  {/if}
</div>
