const messages = {
  es: {
    'precision': 'Precisión',
    'history': 'Historial',
    'clear': 'Limpiar',
    'memory': 'Memoria',
    'converter': 'Conversor',
    'theme_light': 'Claro',
    'theme_dark': 'Oscuro',
    'error_empty': 'Expresión vacía',
    'error_nan': 'Resultado no es un número',
    'mode_standard': 'Std',
    'mode_scientific': 'Sci',
    'mode_programmer': 'Prog',
    'mode_complex': 'Cmplx',
    'mode_matrix': 'Mat',
    'mode_statistics': 'Stat',
    'mode_solver': 'Solv',
    'mode_graph': 'Graph',
  }
};

let currentLang = 'es';
export function t(key) {
  return messages[currentLang]?.[key] || key;
}
export function setLang(lang) {
  if (messages[lang]) currentLang = lang;
}
export function getLang() { return currentLang; }
