# 🧮 Calculadora Científica Multiplataforma — Plan de Desarrollo

---

## 1. 🛠️ Análisis de Tecnologías

| Tecnología | Portabilidad | Rendimiento | Madurez UI | Curva de Aprendizaje | Veredicto |
|---|---|---|---|---|---|
| **C++ + Qt 6** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Alta | ✅ Clásica, nativa real, ideal para desktop |
| **Rust + egui** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | Muy alta | Bueno pero egui es limitado en widgets científicos |
| **Tauri (Rust + Web)** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Media | Excelente balance, UI flexible con HTML/CSS |
| **Flutter** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | Media | Bueno para mobile+desktop, ecosistema Dart |
| **Electron** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | Baja | Pesado, consumo alto de RAM |
| **.NET MAUI / Avalonia** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | Media | Menor madurez en Linux |

### 🏆 Recomendación: **Tauri (Rust + Svelte/React)**

**Justificación:**

- **Portabilidad total**: Windows, macOS, Linux compilando el mismo código Rust. Binarios nativos pequeños (~5-15 MB).
- **Motor de cálculo en Rust**: Precisión numérica, rendimiento seguro sin GC, librerías matemáticas maduras (`nalgebra`, `rug` para aritmética de precisión arbitraria).
- **UI web ligera**: Svelte o React + Canvas/SVG para gráficos. Rápida de iterar.
- **No requiere runtime externo**: A diferencia de Electron, Tauri usa el WebView nativo del SO.
- **Ecosistema**: Tauri 2.x estable, plugins oficiales para sistema de archivos, menús nativos, etc.

**Stack propuesto:**
```
Frontend:  Svelte 4 + TailwindCSS + D3.js/Chart.js (gráficos) + MathJax (renderizado de fórmulas)
Backend:   Rust + Tauri 2.x
Librerías:  nalgebra (álgebra lineal), rug (precisión arbitraria), evalexpr (parser de expresiones)
Testing:   Rust tests + Playwright (e2e UI)
CI/CD:     GitHub Actions → builds para Win/Mac/Linux
```

---

## 2. 📋 Funcionalidades a Desarrollar

### Módulo A — Núcleo de Cálculo

| ID | Funcionalidad | Prioridad |
|---|---|---|
| A1 | Parser de expresiones matemáticas (infijo, precedencia) | P0 |
| A2 | Operaciones aritméticas básicas (+, -, *, /, %, ^) | P0 |
| A3 | Soporte para números negativos y paréntesis anidados | P0 |
| A4 | Precisión decimal configurable (32/64/128 bits o arbitraria con `rug`) | P1 |
| A5 | Manejo de errores (división por cero, sqrt negativo en reales, overflow) | P0 |

### Módulo B — Funciones Científicas

| ID | Funcionalidad | Prioridad |
|---|---|---|
| B1 | Trigonométricas: sin, cos, tan, asin, acos, atan | P0 |
| B2 | Trigonométricas hiperbólicas: sinh, cosh, tanh | P1 |
| B3 | Logaritmos: log, ln, log2, log10 | P0 |
| B4 | Exponenciales: e^x, x^y, raíz cuadrada, raíz cúbica, raíz n-ésima | P0 |
| B5 | Factorial, permutaciones, combinaciones | P0 |
| B6 | Valor absoluto, signo, redondeo (floor, ceil, round) | P1 |
| B7 | Funciones estadísticas: sum, mean, median, stddev, variance | P2 |
| B8 | MCD, MCM, factorización prima | P2 |

### Módulo C — Constantes y Conversiones

| ID | Funcionalidad | Prioridad |
|---|---|---|
| C1 | Constantes: π, e, φ (áureo), constantes físicas | P1 |
| C2 | Conversión de unidades (longitud, masa, temperatura, etc.) | P1 |
| C3 | Conversión de ángulos: DEG ↔ RAD ↔ GRAD | P0 |

### Módulo D — Modos Especiales

| ID | Funcionalidad | Prioridad |
|---|---|---|
| D1 | Modo Programador: HEX, DEC, OCT, BIN + operaciones bitwise | P2 |
| D2 | Modo Estadístico: introducción de series de datos, regresión lineal | P2 |
| D3 | Números complejos (representación a+bi y polar) | P2 |
| D4 | Modo RPN (Reverse Polish Notation) | P3 |
| D5 | Modo fracciones (exactas con racionales) | P3 |

### Módulo E — Álgebra y Cálculo

| ID | Funcionalidad | Prioridad |
|---|---|---|
| E1 | Matrices: suma, resta, multiplicación, determinante, inversa | P2 |
| E2 | Sistemas de ecuaciones lineales | P3 |
| E3 | Derivadas simbólicas simples | P3 |
| E4 | Integrales definidas numéricas (método de Simpson) | P3 |
| E5 | Resolución de ecuaciones no lineales (Newton-Raphson) | P3 |

### Módulo F — Gráficos

| ID | Funcionalidad | Prioridad |
|---|---|---|
| F1 | Gráficos 2D de funciones f(x) | P2 |
| F2 | Zoom, desplazamiento, trazado de múltiples curvas | P2 |
| F3 | Exportación de gráficos (PNG, SVG) | P3 |
| F4 | Gráficos 3D de superficies | P3 |

### Módulo G — Interfaz de Usuario

| ID | Funcionalidad | Prioridad |
|---|---|---|
| G1 | Layout de calculadora con display y teclado | P0 |
| G2 | Soporte completo de teclado físico | P0 |
| G3 | Historial de operaciones (scrollable, clickable) | P1 |
| G4 | Memoria: M+, M-, MR, MC | P1 |
| G5 | Temas (claro/oscuro/personalizado) | P1 |
| G6 | Modo compacto / extendido | P3 |
| G7 | Internacionalización (i18n) | P3 |
| G8 | Atajos de teclado configurables | P3 |

### Módulo H — Sistema y Plataforma

| ID | Funcionalidad | Prioridad |
|---|---|---|
| H1 | Instaladores nativos (MSI, DMG, AppImage/deb) | P0 |
| H2 | Auto-actualización | P3 |
| H3 | Persistencia de configuración y estado | P1 |
| H4 | Exportación/importación de historial (CSV, JSON) | P3 |

---

## 3. 🗂️ Fases de Desarrollo

### 🔷 Fase 0 — Setup y Arquitectura (Semana 1)

| Paso | Descripción | Entregable |
|---|---|---|
| 0.1 | Inicializar proyecto Tauri 2.x con Svelte | Scaffolding funcional |
| 0.2 | Configurar ESLint, Prettier, Rustfmt, Clippy | Linting listo |
| 0.3 | Configurar CI/CD (GitHub Actions) compilando para Win/Mac/Linux | Pipelines verdes |
| 0.4 | Definir protocolo de comunicación Rust ↔ Frontend (comandos Tauri) | Documento de API interna |
| 0.5 | Crear estructura del parser de expresiones en Rust (tokenizer → AST → evaluador) | Core funcional con tests |
| 0.6 | Tests unitarios del motor de cálculo básico | Cobertura >80% |

### 🔷 Fase 1 — MVP: Calculadora Científica Básica (Semanas 2-3)

| Paso | Descripción | Entregable |
|---|---|---|
| 1.1 | UI base: display + teclado numérico + operadores | Interfaz funcional |
| 1.2 | Conectar UI con backend Rust para operaciones básicas | 2+2=4 funciona |
| 1.3 | Implementar todas las funciones científicas (trigonometría, logs, etc.) | Backend completo |
| 1.4 | Botones para funciones científicas en UI (layout de 2 paneles) | UI científica |
| 1.5 | Modo DEG/RAD/GRAD | Selector funcional |
| 1.6 | Soporte de teclado físico | Navegación sin ratón |
| 1.7 | Manejo de errores y display de mensajes (Error, NaN, ∞) | UX robusta |
| 1.8 | Tests E2E con Playwright | Suite de regresión |

### 🔷 Fase 2 — Memoria, Historial y Experiencia (Semanas 4-5)

| Paso | Descripción | Entregable |
|---|---|---|
| 2.1 | Sistema de memoria (M+, M-, MR, MC) con persistencia | Memoria funcional |
| 2.2 | Historial de operaciones con scroll y re-clic | Historial interactivo |
| 2.3 | Constantes predefinidas (π, e, φ) con botones dedicados | Acceso rápido |
| 2.4 | Temas claro/oscuro con Tailwind | Themes switcheable |
| 2.5 | Conversión de unidades (backend Rust + UI dropdown) | Conversor integrado |
| 2.6 | Ajustes de precisión decimal | Preferencias guardadas |

### 🔷 Fase 3 — Modos Avanzados (Semanas 6-8)

| Paso | Descripción | Entregable |
|---|---|---|
| 3.1 | Modo Programador: HEX/DEC/OCT/BIN + operaciones bitwise | Vista programador |
| 3.2 | Números complejos: parser ampliado y representación a+bi | Operaciones complejas |
| 3.3 | Matrices: UI de entrada de matrices + operaciones | Álgebra matricial |
| 3.4 | Modo Estadístico: entrada de datos, frecuencias, regresión | Análisis estadístico |
| 3.5 | Resolución numérica de ecuaciones (Newton-Raphson) | Solver básico |

### 🔷 Fase 4 — Gráficos (Semanas 9-10)

| Paso | Descripción | Entregable |
|---|---|---|
| 4.1 | Motor de gráficos 2D con Canvas/SVG (D3.js o similar) | Plotter funcional |
| 4.2 | Evaluación de funciones para plotting (muestreo adaptativo) | Curvas suaves |
| 4.3 | Controles: zoom, pan, tooltip de coordenadas | Interactividad |
| 4.4 | Múltiples curvas simultáneas con colores | Gráficos compuestos |
| 4.5 | Exportación PNG/SVG | Botón exportar |

### 🔷 Fase 5 — Pulido, Empaquetado y Release (Semanas 11-12)

| Paso | Descripción | Entregable |
|---|---|---|
| 5.1 | Build y testing en Windows, macOS, Linux | Binarios nativos |
| 5.2 | Instaladores: MSI (Win), DMG (Mac), AppImage/deb (Linux) | Instaladores |
| 5.3 | Documentación de usuario | README + wiki |
| 5.4 | i18n (inglés + español inicialmente) | Multi-idioma |
| 5.5 | Performance profiling y optimización | <100ms en operaciones |
| 5.6 | Release v1.0 en GitHub Releases | Tag público |

---

## 4. 👥 Asignación de Roles para Delegación

### 🧑‍💻 Junior — Tareas acotadas, bien definidas, baja complejidad

| Fase | Tareas a delegar |
|---|---|
| 0 | Configurar ESLint/Prettier/Rustfmt, crear estructura de carpetas |
| 1 | Implementar botones de UI básicos (maquetado Svelte + Tailwind), conectar eventos click |
| 2 | Añadir botones de constantes (π, e), crear selector de temas en UI |
| 3 | Maquetar UI de modo programador (vista de bits y botones HEX/DEC/OCT/BIN) |
| 4 | Diseñar CSS del panel de gráficos, exportación PNG/SVG |
| 5 | Escribir documentación de usuario, crear archivos de traducción i18n, preparar assets de instaladores |

### 🧑‍🔬 Senior — Implementación principal, lógica no trivial

| Fase | Tareas a delegar |
|---|---|
| 0 | Implementar el parser de expresiones en Rust (tokenizer, shunting-yard, evaluador AST) |
| 1 | Implementar funciones científicas en Rust (trigonometría, logaritmos, exponenciales, factorial) |
| 2 | Sistema de historial y memoria con persistencia en Rust |
| 3 | Motor de números complejos, parser de matrices, modo estadístico completo |
| 4 | Evaluador de funciones para gráficos (muestreo adaptativo con `nalgebra`) |
| 5 | Configuración de builds multiplataforma y CI/CD |

### 🏗️ Arquitecto — Planificación compleja, validación, desbloqueo

| Fase | Tareas a delegar |
|---|---|
| 0 | Diseño de arquitectura completa, definición de API Rust-Frontend |
| 1 | Revisión del parser (validar manejo de edge cases, precedencia de operadores) |
| 2 | Evaluar estrategia de persistencia (¿archivos locales? ¿SQLite? ¿config JSON?) |
| 3 | Diseñar protocolo de comunicación para gráficos (streaming de puntos vs. evaluación batch) |
| 4 | Revisión de rendimiento del plotter (estrategia de muestreo, resolución dinámica) |
| 5 | Auditoría de seguridad y revisión final de arquitectura antes de release |

---

## 📊 Resumen Visual

```
Fase 0: Setup        ████░░░░░░░░░░  (1 semana)   [Arquitecto + Senior]
Fase 1: MVP          ████████░░░░░░  (2 semanas)  [Senior + Junior]
Fase 2: UX           ████████████░░  (2 semanas)  [Senior + Junior]
Fase 3: Avanzado     ██████████████  (3 semanas)  [Senior + Junior + Arquitecto]
Fase 4: Gráficos     ██████████████  (2 semanas)  [Senior + Junior]
Fase 5: Release      ██████████████  (2 semanas)  [Senior + Junior + Arquitecto]
═══════════════════════════════════════════════════════════════
Total estimado: 12 semanas (3 meses) con 3-4 agentes en paralelo
```
