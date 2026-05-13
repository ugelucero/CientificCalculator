<script>
  /**
   * GraphCanvas.svelte — Motor de gráficos 2D con Canvas HTML5.
   *
   * Dibuja funciones f(x) con muestreo adaptativo, zoom centrado en el ratón,
   * paneo por arrastre y tooltip de coordenadas.
   *
   * Props:
   *   functions  — Array<{ expr: string, color: string, label: string }>
   *   xMin,xMax  — Dominio visible inicial
   *   yMin,yMax  — Rango visible inicial
   *   onViewportChange — Callback opcional al cambiar el viewport
   */
  import { evaluateFunc } from '../utils/tauri-bridge.js';

  /** @type {{
   *   functions?: Array<{ expr: string, color: string, label: string }>,
   *   xMin?: number, xMax?: number, yMin?: number, yMax?: number,
   *   onViewportChange?: (vp: {xMin:number,xMax:number,yMin:number,yMax:number}) => void
   * }} */
  let {
    functions = [],
    xMin = -10,
    xMax = 10,
    yMin = -10,
    yMax = 10,
    onViewportChange = null
  } = $props();

  // ─── Internal reactive state ───────────────────────────────────────
  let canvasEl = $state(null);
  let containerEl = $state(null);

  let viewXMin = $state(xMin);
  let viewXMax = $state(xMax);
  let viewYMin = $state(yMin);
  let viewYMax = $state(yMax);

  let canvasWidth = $state(600);
  let canvasHeight = $state(400);

  let isDark = $state(false);

  // Pan state
  let isPanning = $state(false);
  let panStartX = $state(0);
  let panStartY = $state(0);
  let panViewXMin = $state(0);
  let panViewXMax = $state(0);
  let panViewYMin = $state(0);
  let panViewYMax = $state(0);

  // Tooltip state
  let tooltip = $state(null);

  // ─── Sync initial props → internal viewport ────────────────────────
  $effect(() => {
    viewXMin = xMin;
    viewXMax = xMax;
    viewYMin = yMin;
    viewYMax = yMax;
  });

  // ─── Theme detection via <html class="dark"> ──────────────────────
  $effect(() => {
    const update = () => {
      isDark = document.documentElement.classList.contains('dark');
    };
    update();
    const mo = new MutationObserver(update);
    mo.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    });
    return () => mo.disconnect();
  });

  // ─── ResizeObserver: llena el contenedor padre ────────────────────
  $effect(() => {
    if (!containerEl) return;
    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        canvasWidth = Math.max(280, Math.floor(entry.contentRect.width));
        canvasHeight = Math.max(200, Math.floor(entry.contentRect.height));
      }
    });
    ro.observe(containerEl);
    return () => ro.disconnect();
  });

  // ─── Coordinate conversion helpers ─────────────────────────────────
  function toScreenX(mx) {
    return ((mx - viewXMin) / (viewXMax - viewXMin)) * canvasWidth;
  }
  function toScreenY(my) {
    return canvasHeight - ((my - viewYMin) / (viewYMax - viewYMin)) * canvasHeight;
  }
  function fromScreenX(sx) {
    return viewXMin + (sx / canvasWidth) * (viewXMax - viewXMin);
  }
  function fromScreenY(sy) {
    return viewYMin + ((canvasHeight - sy) / canvasHeight) * (viewYMax - viewYMin);
  }

  // ─── Nice tick algorithm ───────────────────────────────────────────
  function niceNumber(range, round) {
    const exp = Math.floor(Math.log10(range));
    const frac = range / Math.pow(10, exp);
    let nice;
    if (round) {
      if (frac < 1.5) nice = 1;
      else if (frac < 3) nice = 2;
      else if (frac < 7) nice = 5;
      else nice = 10;
    } else {
      if (frac <= 1) nice = 1;
      else if (frac <= 2) nice = 2;
      else if (frac <= 5) nice = 5;
      else nice = 10;
    }
    return nice * Math.pow(10, exp);
  }

  function getTicks(min, max, maxTicks = 8) {
    if (max - min < 1e-12) return [min];
    const range = niceNumber(max - min, false);
    const step = niceNumber(range / (maxTicks - 1), true);
    const nmin = Math.floor(min / step) * step;
    const nmax = Math.ceil(max / step) * step;
    const ticks = [];
    for (let v = nmin; v <= nmax + step * 0.5; v += step) {
      ticks.push(v);
    }
    return ticks;
  }

  function formatTick(value) {
    if (value === 0) return '0';
    const abs = Math.abs(value);
    if (abs >= 1e15 || (abs < 1e-10 && abs > 0)) {
      return value.toExponential(3);
    }
    if (Number.isInteger(value) && abs < 1e15) {
      return String(value);
    }
    const s = value.toPrecision(8);
    return s.replace(/(\.\d*?)0+$/, '$1').replace(/\.$/, '');
  }

  // ─── Adaptive sampling engine ──────────────────────────────────────
  /**
   * Muestrea f(x) en [xA, xB] con subdivisión recursiva donde la
   * curvatura supera la tolerancia (en unidades matemáticas).
   *
   * Retorna un array de segmentos continuos (cada segmento es Array<{x,y}>).
   * Las discontinuidades producen segmentos separados.
   */
  function adaptiveSample(expr, xA, xB, tol, maxDepth = 8) {
    const segments = [];   // Array<Array<{x:number, y:number}>>
    let current = [];

    /**
     * Subdivide recursivamente el intervalo [a,b] con valores conocidos fa,fb.
     */
    function subdivide(a, b, fa, fb, depth) {
      if (depth > maxDepth) {
        if (Number.isFinite(fb)) current.push({ x: b, y: fb });
        return;
      }

      const mid = (a + b) / 2;
      let fmid;
      try { fmid = evaluateFunc(expr, mid); } catch { fmid = NaN; }

      // Discontinuidad detectada (NaN / Infinity)
      if (!Number.isFinite(fmid)) {
        if (current.length > 0) {
          segments.push(current);
          current = [];
        }
        if (Number.isFinite(fb)) {
          current.push({ x: b, y: fb });
        }
        return;
      }

      // ¿El punto medio se desvía significativamente de la línea recta?
      const linearMid = (fa + fb) / 2;
      const deviation = Math.abs(fmid - linearMid);

      if (deviation < tol) {
        // Segmento suficientemente recto: aceptar
        if (Number.isFinite(fb)) current.push({ x: b, y: fb });
      } else {
        // Subdividir: mitad izquierda y mitad derecha
        subdivide(a, mid, fa, fmid, depth + 1);
        subdivide(mid, b, fmid, fb, depth + 1);
      }
    }

    // ── Puntos de muestreo iniciales ──
    const N = 200;
    const dx = (xB - xA) / N;

    let prevX = xA;
    let prevY;
    try { prevY = evaluateFunc(expr, xA); } catch { prevY = NaN; }

    if (Number.isFinite(prevY)) {
      current.push({ x: xA, y: prevY });
    }

    for (let i = 1; i <= N; i++) {
      const xi = xA + i * dx;
      let yi;
      try { yi = evaluateFunc(expr, xi); } catch { yi = NaN; }

      const prevFinite = Number.isFinite(prevY);
      const currFinite = Number.isFinite(yi);

      if (prevFinite && currFinite) {
        // Ambos finitos: refinar el segmento [prevX, xi]
        subdivide(prevX, xi, prevY, yi, 0);
      } else if (prevFinite && !currFinite) {
        // Salida del dominio válido: cerrar segmento actual
        if (current.length > 0) {
          segments.push(current);
          current = [];
        }
      } else if (!prevFinite && currFinite) {
        // Entrada al dominio válido: iniciar nuevo segmento
        current = [{ x: xi, y: yi }];
      }
      // else: ambos no finitos, continuar

      prevX = xi;
      prevY = yi;
    }

    // Último segmento pendiente
    if (current.length > 0) {
      segments.push(current);
    }

    return segments;
  }

  // ─── Canvas drawing ────────────────────────────────────────────────
  let drawPending = false;

  function scheduleDraw() {
    if (drawPending || !canvasEl) return;
    drawPending = true;
    requestAnimationFrame(() => {
      drawPending = false;
      if (!canvasEl) return;
      draw();
    });
  }

  function draw() {
    const ctx = canvasEl.getContext('2d');
    const dpr = window.devicePixelRatio || 1;

    // High-DPI canvas
    canvasEl.width = canvasWidth * dpr;
    canvasEl.height = canvasHeight * dpr;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const W = canvasWidth;
    const H = canvasHeight;

    // Clear
    ctx.clearRect(0, 0, W, H);

    // Background
    ctx.fillStyle = isDark ? '#1a1a2e' : '#fafbfc';
    ctx.fillRect(0, 0, W, H);

    drawGrid(ctx, W, H);
    drawAxes(ctx, W, H);
    drawCurves(ctx, W, H);
    drawLegend(ctx, W, H);
  }

  function drawGrid(ctx, W, H) {
    const xTicks = getTicks(viewXMin, viewXMax, 10);
    const yTicks = getTicks(viewYMin, viewYMax, 8);

    ctx.strokeStyle = isDark ? 'rgba(255,255,255,0.07)' : 'rgba(0,0,0,0.06)';
    ctx.lineWidth = 0.5;
    ctx.setLineDash([4, 4]);

    for (const tx of xTicks) {
      const sx = toScreenX(tx);
      if (sx < 0 || sx > W) continue;
      ctx.beginPath();
      ctx.moveTo(sx, 0);
      ctx.lineTo(sx, H);
      ctx.stroke();
    }

    for (const ty of yTicks) {
      const sy = toScreenY(ty);
      if (sy < 0 || sy > H) continue;
      ctx.beginPath();
      ctx.moveTo(0, sy);
      ctx.lineTo(W, sy);
      ctx.stroke();
    }

    ctx.setLineDash([]);
  }

  function drawAxes(ctx, W, H) {
    const zeroX = toScreenX(0);
    const zeroY = toScreenY(0);

    const axisColor = isDark ? '#aaa' : '#555';
    ctx.strokeStyle = axisColor;
    ctx.lineWidth = 1.5;
    ctx.fillStyle = isDark ? '#ddd' : '#333';
    ctx.font = '10px sans-serif';

    // ── Eje X ──
    if (zeroY >= -60 && zeroY <= H + 60) {
      const y = Math.max(0, Math.min(H, zeroY));
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(W, y);
      ctx.stroke();

      // Flecha
      ctx.beginPath();
      ctx.moveTo(W - 8, y - 4);
      ctx.lineTo(W, y);
      ctx.lineTo(W - 8, y + 4);
      ctx.stroke();

      // Etiquetas
      const xTicks = getTicks(viewXMin, viewXMax, 10);
      ctx.textAlign = 'center';
      ctx.textBaseline = 'top';
      for (const tx of xTicks) {
        if (tx === 0) continue;
        const sx = toScreenX(tx);
        if (sx < 5 || sx > W - 5) continue;
        ctx.fillText(formatTick(tx), sx, y + 6);
      }
    }

    // ── Eje Y ──
    if (zeroX >= -60 && zeroX <= W + 60) {
      const x = Math.max(0, Math.min(W, zeroX));
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, H);
      ctx.stroke();

      // Flecha
      ctx.beginPath();
      ctx.moveTo(x - 4, 8);
      ctx.lineTo(x, 0);
      ctx.lineTo(x + 4, 8);
      ctx.stroke();

      // Etiquetas
      const yTicks = getTicks(viewYMin, viewYMax, 8);
      ctx.textAlign = 'right';
      ctx.textBaseline = 'middle';
      for (const ty of yTicks) {
        if (ty === 0) continue;
        const sy = toScreenY(ty);
        if (sy < 5 || sy > H - 5) continue;
        ctx.fillText(formatTick(ty), x - 6, sy);
      }
    }

    // Origen "0"
    if (zeroX >= 2 && zeroX <= W - 2 && zeroY >= 2 && zeroY <= H - 2) {
      ctx.textAlign = 'right';
      ctx.textBaseline = 'top';
      ctx.fillText('0', zeroX - 4, zeroY + 4);
    }
  }

  function drawCurves(ctx, W, H) {
    // Tolerancia: ~1 píxel en el eje Y
    const tol = Math.max((viewYMax - viewYMin) / H, 1e-14);

    for (const fn of functions) {
      const { expr, color } = fn;
      if (!expr || !expr.trim()) continue;

      const segments = adaptiveSample(expr, viewXMin, viewXMax, tol, 8);

      ctx.save();
      ctx.strokeStyle = color || '#3498db';
      ctx.lineWidth = 2;
      ctx.lineJoin = 'round';
      ctx.lineCap = 'round';

      for (const seg of segments) {
        if (seg.length === 0) continue;

        // Dibujar el segmento como una polilínea continua
        ctx.beginPath();
        const first = seg[0];
        ctx.moveTo(toScreenX(first.x), toScreenY(first.y));

        for (let i = 1; i < seg.length; i++) {
          const pt = seg[i];
          ctx.lineTo(toScreenX(pt.x), toScreenY(pt.y));
        }
        ctx.stroke();
      }

      ctx.restore();
    }
  }

  /** Dibuja un rectángulo redondeado (fallback si ctx.roundRect no existe) */
  function roundRect(ctx, x, y, w, h, r) {
    if (typeof ctx.roundRect === 'function') {
      ctx.beginPath();
      ctx.roundRect(x, y, w, h, r);
      return;
    }
    // Fallback manual
    ctx.beginPath();
    ctx.moveTo(x + r, y);
    ctx.lineTo(x + w - r, y);
    ctx.quadraticCurveTo(x + w, y, x + w, y + r);
    ctx.lineTo(x + w, y + h - r);
    ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h);
    ctx.lineTo(x + r, y + h);
    ctx.quadraticCurveTo(x, y + h, x, y + h - r);
    ctx.lineTo(x, y + r);
    ctx.quadraticCurveTo(x, y, x + r, y);
    ctx.closePath();
  }

  function drawLegend(ctx, W, H) {
    if (functions.length === 0) return;

    const padX = 12;
    const padY = 8;
    const lineLen = 18;
    const rowH = 22;
    const fontSize = 11;

    ctx.font = `${fontSize}px sans-serif`;

    // Calcular ancho máximo de etiqueta
    let maxW = 0;
    for (const fn of functions) {
      const label = fn.label || fn.expr || '';
      const w = ctx.measureText(label).width;
      if (w > maxW) maxW = w;
    }

    const bgW = maxW + lineLen + padX * 2 + 10;
    const bgH = functions.length * rowH + padY * 2;
    const bgX = W - bgW - 10;
    const bgY = 10;

    // Fondo semitransparente
    ctx.fillStyle = isDark ? 'rgba(10,10,30,0.75)' : 'rgba(255,255,255,0.85)';
    ctx.strokeStyle = isDark ? 'rgba(255,255,255,0.12)' : 'rgba(0,0,0,0.12)';
    ctx.lineWidth = 1;
    roundRect(ctx, bgX, bgY, bgW, bgH, 6);
    ctx.fill();
    ctx.stroke();

    // Items
    for (let i = 0; i < functions.length; i++) {
      const fn = functions[i];
      const yOff = bgY + padY + i * rowH + fontSize - 2;

      // Línea de color
      ctx.strokeStyle = fn.color || '#3498db';
      ctx.lineWidth = 2.5;
      ctx.lineCap = 'round';
      ctx.beginPath();
      ctx.moveTo(bgX + padX, yOff);
      ctx.lineTo(bgX + padX + lineLen, yOff);
      ctx.stroke();

      // Etiqueta
      ctx.fillStyle = isDark ? '#eee' : '#222';
      ctx.textAlign = 'left';
      ctx.textBaseline = 'middle';
      ctx.fillText(fn.label || fn.expr || '', bgX + padX + lineLen + 6, yOff);
    }
  }

  // ─── React to state changes & draw ─────────────────────────────────
  $effect(() => {
    // canvasEl se lee en la condición (tracked) — se necesita para
    // que el efecto se ejecute cuando el canvas se monta vía bind:this.
    if (!canvasEl) return;

    // Leer todas las dependencias reactivas para que el efecto
    // se dispare ante cualquier cambio de estado que afecte al dibujo.
    const _fns = functions;
    const _vxMin = viewXMin;
    const _vxMax = viewXMax;
    const _vyMin = viewYMin;
    const _vyMax = viewYMax;
    const _w = canvasWidth;
    const _h = canvasHeight;
    const _dark = isDark;

    scheduleDraw();
  });

  // ─── Event handlers ────────────────────────────────────────────────

  /** Zoom con la rueda del ratón centrado en la posición del cursor */
  function handleWheel(e) {
    e.preventDefault();

    const rect = canvasEl.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    // No hacer zoom si el cursor está fuera del área visible
    if (mx < 0 || my < 0 || mx > canvasWidth || my > canvasHeight) return;

    const mathX = fromScreenX(mx);
    const mathY = fromScreenY(my);

    const zoomFactor = e.deltaY < 0 ? 0.85 : 1.15;

    const newRangeX = (viewXMax - viewXMin) * zoomFactor;
    const newRangeY = (viewYMax - viewYMin) * zoomFactor;

    // Rango mínimo para evitar división por cero
    const MIN_RANGE = 1e-10;
    const clampedRX = Math.max(newRangeX, MIN_RANGE);
    const clampedRY = Math.max(newRangeY, MIN_RANGE);

    // Mantener la posición del cursor fija en el espacio matemático
    const ratioX = (mathX - viewXMin) / (viewXMax - viewXMin);
    const ratioY = (mathY - viewYMin) / (viewYMax - viewYMin);

    viewXMin = mathX - ratioX * clampedRX;
    viewXMax = viewXMin + clampedRX;
    viewYMin = mathY - ratioY * clampedRY;
    viewYMax = viewYMin + clampedRY;

    if (onViewportChange) {
      onViewportChange({ xMin: viewXMin, xMax: viewXMax, yMin: viewYMin, yMax: viewYMax });
    }
  }

  /** Inicia el paneo (botón izquierdo) */
  function handleMouseDown(e) {
    if (e.button !== 0) return;
    isPanning = true;
    panStartX = e.offsetX;
    panStartY = e.offsetY;
    panViewXMin = viewXMin;
    panViewXMax = viewXMax;
    panViewYMin = viewYMin;
    panViewYMax = viewYMax;
    if (canvasEl) canvasEl.style.cursor = 'grabbing';
  }

  /** Actualiza paneo o tooltip según estado */
  function handleMouseMove(e) {
    const rect = canvasEl.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    if (isPanning) {
      // ── Paneo ──
      const dxPx = mx - panStartX;
      const dyPx = my - panStartY;

      const mathDX = (dxPx / canvasWidth) * (panViewXMax - panViewXMin);
      const mathDY = -(dyPx / canvasHeight) * (panViewYMax - panViewYMin);

      viewXMin = panViewXMin - mathDX;
      viewXMax = panViewXMax - mathDX;
      viewYMin = panViewYMin - mathDY;
      viewYMax = panViewYMax - mathDY;

      if (onViewportChange) {
        onViewportChange({ xMin: viewXMin, xMax: viewXMax, yMin: viewYMin, yMax: viewYMax });
      }
      return;
    }

    // ── Tooltip ──
    const mathX = fromScreenX(mx);
    const mathY = fromScreenY(my);

    let text = `(x: ${formatTick(mathX)},  y: ${formatTick(mathY)})`;
    let nearest = null;
    let nearestDist = 14; // pixeles

    for (const fn of functions) {
      const { expr, color, label } = fn;
      if (!expr || !expr.trim()) continue;
      try {
        const fy = evaluateFunc(expr, mathX);
        if (!Number.isFinite(fy)) continue;
        const screenFY = toScreenY(fy);
        const dist = Math.abs(screenFY - my);
        if (dist < nearestDist) {
          nearestDist = dist;
          nearest = {
            label: label || expr,
            color: color || '#3498db',
            value: fy
          };
        }
      } catch { /* skip */ }
    }

    if (nearest) {
      text = `${nearest.label}\n(x: ${formatTick(mathX)},  f(x): ${formatTick(nearest.value)})`;
      tooltip = { x: mx, y: my, text, color: nearest.color };
    } else {
      tooltip = { x: mx, y: my, text, color: null };
    }
  }

  function handleMouseUp(e) {
    if (isPanning) {
      isPanning = false;
      if (canvasEl) canvasEl.style.cursor = 'default';
    }
  }

  function handleMouseLeave() {
    if (isPanning) {
      isPanning = false;
      if (canvasEl) canvasEl.style.cursor = 'default';
    }
    tooltip = null;
  }
</script>

<div
  bind:this={containerEl}
  class="relative w-full h-full min-w-[280px] min-h-[200px] overflow-hidden rounded-lg
         border border-gray-300 dark:border-gray-700 select-none"
  tabindex="0"
  role="img"
  aria-label="Graph of mathematical functions"
>
  <canvas
    bind:this={canvasEl}
    class="block w-full h-full"
    style="width: {canvasWidth}px; height: {canvasHeight}px; cursor: default;"
    onwheel|nonpassive={handleWheel}
    onmousedown={handleMouseDown}
    onmousemove={handleMouseMove}
    onmouseup={handleMouseUp}
    onmouseleave={handleMouseLeave}
  />

  {#if tooltip}
    <div
      class="absolute pointer-events-none px-2.5 py-1.5 rounded text-xs font-mono whitespace-pre
             leading-relaxed z-10 border shadow-lg
             bg-gray-900/85 dark:bg-gray-900/85 text-white border-white/20"
      style="left: {Math.min(Math.max(tooltip.x + 14, 0), canvasWidth - 200)}px;
             top: {Math.max(tooltip.y - 48, 4)}px;"
    >
      {#if tooltip.color}
        <span
          class="inline-block w-3 h-3 rounded-full mr-1.5 align-middle"
          style="background-color: {tooltip.color};"
        ></span>
      {/if}
      {tooltip.text}
    </div>
  {/if}
</div>
