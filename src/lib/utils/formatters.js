/**
 * formatters.js - Utilidades de formateo para el display.
 */
export function formatNumber(value, precision = 10) {
  if (value === null || value === undefined) return '';
  const num = Number(value);
  if (Number.isNaN(num)) return 'Error';
  if (!Number.isFinite(num)) return num > 0 ? '∞' : '−∞';
  return num.toLocaleString('en-US', {
    maximumFractionDigits: precision,
    useGrouping: false,
  });
}

export function formatScientific(value, precision = 10) {
  if (value === null || value === undefined) return '';
  const num = Number(value);
  if (Number.isNaN(num)) return 'Error';
  if (!Number.isFinite(num)) return num > 0 ? '∞' : '−∞';
  return num.toExponential(precision);
}
