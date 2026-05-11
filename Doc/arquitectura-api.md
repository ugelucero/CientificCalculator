# 🧮 Arquitectura de Comunicación Rust ↔ Frontend

**Proyecto:** Calculadora Científica Multiplataforma  
**Framework:** Tauri 2.x (Rust backend + Svelte frontend)  
**Documento:** Protocolo IPC y estructura del proyecto  
**Versión:** 1.0.0 — Fase 0, Paso 0.4

---

## 1. Visión General

```
┌──────────────────────────────────────────────────────────────┐
│                     SVELTE FRONTEND                          │
│                                                              │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌──────────────┐   │
│  │ Display  │  │ Keypad   │  │ History │  │ ModeSelector │   │
│  └────┬─────┘  └────┬─────┘  └────┬────┘  └──────┬──────┘   │
│       │             │             │               │          │
│       └──────┬──────┘─────────────┘───────────────┘          │
│              │                                                │
│     ┌────────▼────────┐                                      │
│     │  Svelte Stores  │  (estado reactivo)                   │
│     │  expressionStore│                                      │
│     │  resultStore    │                                      │
│     │  historyStore   │                                      │
│     │  memoryStore    │                                      │
│     │  settingsStore  │                                      │
│     └────────┬────────┘                                      │
│              │ invoca comandos Tauri                         │
└──────────────┼───────────────────────────────────────────────┘
               │  ═══════════════════════════
               │     TAURI IPC (invoke)     │  Serialización:
               │  ═══════════════════════════  Serde JSON ↔ Rust
┌──────────────┼───────────────────────────────────────────────┐
│              ▼                                                │
│     ┌─────────────────┐                                      │
│     │  Command Layer  │  (commands/)                         │
│     │  evaluate.rs    │  Recibe, deserializa, delega         │
│     │  convert.rs     │                                      │
│     │  history.rs     │                                      │
│     │  memory.rs       │                                      │
│     └────────┬────────┘                                      │
│              │ llama                                          │
│     ┌────────▼────────┐                                      │
│     │  Parser Engine  │  (parser/)                           │
│     │  tokenizer →    │                                      │
│     │  ast → evaluator│                                      │
│     └────────┬────────┘                                      │
│              │ usa                                            │
│     ┌────────▼────────┐                                      │
│     │  Math Engine    │  (math/)                              │
│     │  trig, stats,  │                                      │
│     │  matrix, complex│                                      │
│     └────────┬────────┘                                      │
│              │ produce                                        │
│     ┌────────▼────────┐                                      │
│     │  Models         │  (models/)                           │
│     │  tipos, errores │  Serde → JSON → Frontend            │
│     └─────────────────┘                                      │
│                                                              │
│                     RUST BACKEND                             │
└──────────────────────────────────────────────────────────────┘
```

---

## 2. Comandos Tauri (IPC)

Todos los comandos se exponen mediante `#[tauri::command]` y se invocan desde el frontend con `invoke('command_name', { args })`. Tauri 2.x serializa los argumentos como JSON y los deserializa en Rust vía Serde.

### 2.1. Evaluación de Expresiones

| Comando | Firma Rust | Descripción |
|---|---|---|
| `evaluate_expression` | `fn evaluate_expression(expr: String, mode: CalcMode) -> Result<ExpressionResult, CalcError>` | Núcleo de la calculadora. Parsea y evalúa la expresión matemática en el modo indicado. |

**Ejemplo de invocación desde Svelte:**

```javascript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('evaluate_expression', {
  expr: 'sin(45) + log(100)',
  mode: 'Scientific'   // se serializa como string enum
});
// result = { result: "1.7071067812", display: "sin(45°) + log₁₀(100)", format: "Decimal" }
```

**Notas de diseño:**
- El `mode` afecta qué funciones y operadores están disponibles y cómo se interpretan ciertos valores (ej: bases numéricas en modo Programador).
- Si el `mode` no soporta la operación (ej: matrices en modo Standard), se devuelve `CalcError` con `ErrorKind::InvalidMode`.
- El campo `display` del resultado contiene la expresión formateada para mostrar al usuario (con símbolos: π, √, ÷, etc.).

### 2.2. Conversión de Unidades

| Comando | Firma Rust | Descripción |
|---|---|---|
| `convert_units` | `fn convert_units(value: f64, from: Unit, to: Unit) -> Result<f64, CalcError>` | Convierte un valor entre unidades compatibles. |

**Ejemplo:**

```javascript
const km = await invoke('convert_units', {
  value: 100,
  from: { category: 'Length', name: 'Kilometer' },
  to:   { category: 'Length', name: 'Mile' }
});
// km = 62.13712...
```

**Restricciones:**
- `from` y `to` deben pertenecer a la misma `UnitCategory`. Si no, retorna `CalcError` con `ErrorKind::DomainError` y un mensaje descriptivo.
- Temperatura se maneja como caso especial (las fórmulas no son lineales).

### 2.3. Historial

| Comando | Firma Rust | Descripción |
|---|---|---|
| `get_history` | `fn get_history() -> Vec<HistoryEntry>` | Devuelve todas las entradas del historial ordenadas por timestamp descendente. |
| `clear_history` | `fn clear_history()` | Elimina todo el historial. |

**Notas de diseño:**
- El historial se persiste automáticamente en un archivo local (`app_data/history.json`). El frontend NO gestiona la persistencia, solo invoca estos comandos.
- `evaluate_expression` inserta automáticamente una entrada en el historial si la evaluación fue exitosa. No se necesita un comando `add_to_history` separado.
- Futuras extensiones: `remove_history_entry(id)`, `export_history(format)`.

### 2.4. Memoria

| Comando | Firma Rust | Descripción |
|---|---|---|
| `memory_store` | `fn memory_store(value: f64)` | Almacena un valor en la memoria (sobrescribe el anterior). |
| `memory_recall` | `fn memory_recall() -> Option<f64>` | Recupera el valor almacenado. `None` si la memoria está vacía. |
| `memory_clear` | `fn memory_clear()` | Borra la memoria. |
| `memory_add` | `fn memory_add(value: f64)` | Suma un valor al contenido actual de la memoria. |
| `memory_subtract` | `fn memory_subtract(value: f64)` | Resta un valor al contenido actual de la memoria. |

**Notas de diseño:**
- La memoria también se persiste en `app_data/memory.json`.
- Si se invoca `memory_add` o `memory_subtract` sin valor previo en memoria, se asume 0 como base.
- Al invocarse `memory_recall` sobre memoria vacía, se devuelve `null` en JSON (mapeo natural de `Option<f64>`).

### 2.5. Configuración de Sesión

| Comando | Firma Rust | Descripción |
|---|---|---|
| `set_angle_mode` | `fn set_angle_mode(mode: AngleMode)` | Establece el modo angular para funciones trigonométricas. Afecta todas las evaluaciones posteriores. |
| `set_precision` | `fn set_precision(digits: u8)` | Configura el número de dígitos decimales para el display de resultados. Rango válido: 0–15. |

**Notas de diseño:**
- Estas configuraciones son **stateful del lado de Rust**. Se almacenan en el `AppState` de Tauri (gestionado con `tauri::State`). El frontend NO mantiene el estado de ángulo ni precisión; lo delega al backend.
- Persisten entre sesiones (se guardan en `app_data/settings.json`).
- Futuras extensiones: `get_settings() -> Settings`, `set_settings(settings: Settings)`.

### 2.6. Comandos de Futuras Fases (placeholders)

Estos comandos se implementarán en fases posteriores. Se documentan aquí para预留ar el espacio de nombres:

| Comando | Firma (prevista) | Fase |
|---|---|---|
| `plot_function` | `fn plot_function(expr: String, x_range: (f64, f64), samples: u32) -> Result<Vec<(f64, f64)>, CalcError>` | Fase 4 |
| `evaluate_matrix` | `fn evaluate_matrix(op: MatrixOp, a: Matrix, b: Option<Matrix>) -> Result<Matrix, CalcError>` | Fase 3 |
| `solve_equation` | `fn solve_equation(expr: String, variable: String, initial_guess: f64) -> Result<f64, CalcError>` | Fase 3 |
| `get_statistics` | `fn get_statistics(data: Vec<f64>) -> Result<StatsResult, CalcError>` | Fase 3 |

---

## 3. Tipos de Datos Compartidos

Todos los tipos comparten las mismas estructuras entre Rust (backend) y TypeScript/Svelte (frontend) a través de serialización Serde con formato JSON. Para el frontend se generarán tipos TypeScript automáticamente con `ts-rs` o se mantendrán manualmente en `src/lib/types.ts`.

### 3.1. Enums

```
CalcMode
├── Standard        — Operaciones aritméticas básicas
├── Scientific      — Funciones trigonométricas, logaritmos, exponenciales
├── Programmer       — Bases numéricas (BIN/OCT/DEC/HEX), operaciones bitwise
├── Statistics       — Funciones estadísticas: media, desviación, regresión
├── Complex          — Números complejos: (a + bi), representación polar
└── Matrix           — Operaciones con matrices: suma, multiplicación, determinante
```

**Definición Rust (esquemática):**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum CalcMode {
    Standard,
    Scientific,
    Programmer,
    Statistics,
    Complex,
    Matrix,
}
```

> **Convención:** `rename_all = "PascalCase"` garantiza que en JSON se transmita `"Scientific"` (no `"scientific"` ni `"SCIENTIFIC"`). Todos los enums siguen esta convención.

```
AngleMode
├── Deg              — Grados sexagesimales (360° = vuelta completa)
├── Rad              — Radianes (2π = vuelta completa)
└── Grad             — Grados centesimales (400ᵍ = vuelta completa)
```

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum AngleMode {
    Deg,
    Rad,
    Grad,
}
```

```
FormatType
├── Decimal          — Notación decimal: 3.14159265
├── Scientific       — Notación científica: 3.14159e0
├── Fraction         — Fracción exacta: 22/7
├── Hex              — Hexadecimal: 0xFF
├── Oct              — Octal: 0o377
├── Bin              — Binario: 0b11111111
└── Complex          — Notación compleja: 3+4i
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum FormatType {
    Decimal,
    Scientific,
    Fraction,
    Hex,
    Oct,
    Bin,
    Complex,
}
```

```
ErrorKind
├── DivisionByZero    — División por cero
├── DomainError       — Argumento fuera del dominio (ej: sqrt(-1) en reales)
├── ParseError        — La expresión no se pudo parsear
├── Overflow          — Desbordamiento numérico
├── InvalidMode       — Operación no soportada en el modo actual
├── DimensionMismatch — Dimensiones incompatibles (matrices, vectores)
├── UndefinedVariable — Variable no definida en el contexto
└── InternalError     — Error interno inesperado (catch-all)
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum ErrorKind {
    DivisionByZero,
    DomainError,
    ParseError,
    Overflow,
    InvalidMode,
    DimensionMismatch,
    UndefinedVariable,
    InternalError,
}
```

### 3.2. Structs

#### ExpressionResult

Resultado principal de toda evaluación. Contiene tres campos diseñados para cubrir las necesidades de presentación del frontend.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpressionResult {
    /// Valor numérico del resultado como string para evitar pérdida de precisión.
    /// Ej: "3.14159265358979", "0xFF", "3+4i", "[[1,2],[3,4]]"
    result: String,

    /// Expresión formateada para mostrar al usuario con símbolos amigables.
    /// Ej: "sin(45°) + log₁₀(100)"  (en vez de "sin(45)+log(100)")
    display: String,

    /// Formato del resultado para que el frontend sepa cómo renderizarlo.
    /// Ej: si es Hex → el frontend muestra un badge "HEX" y permite copiar en hexadecimal.
    format: FormatType,
}
```

**Ejemplo JSON de respuesta:**

```json
{
  "result": "1.7071067812",
  "display": "sin(45°) + log₁₀(100)",
  "format": "Decimal"
}
```

```json
{
  "result": "0xFF",
  "display": "255",
  "format": "Hex"
}
```

#### CalcError

Error estructurado que permite al frontend mostrar mensajes específicos y posiblemente resaltar la posición del error en la expresión.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CalcError {
    /// Clasificación del error para que el frontend pueda reaccionar diferenciadamente.
    kind: ErrorKind,

    /// Mensaje legible para el usuario, ya localizado (por defecto en inglés).
    message: String,

    /// Posición (0-indexed) del carácter donde se detectó el error en la expresión original.
    /// Permite al frontend subrayar o resaltar el token problemático.
    /// None si la posición no aplica (ej: Overflow).
    position: Option<usize>,
}
```

**Ejemplo JSON de error:**

```json
{
  "kind": "ParseError",
  "message": "Unexpected token ')' at position 5",
  "position": 5
}
```

```json
{
  "kind": "DivisionByZero",
  "message": "Division by zero is undefined",
  "position": null
}
```

#### Unit y UnitCategory

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Unit {
    /// Categoría a la que pertenece la unidad (ej: "Length", "Temperature").
    category: UnitCategory,

    /// Nombre de la unidad (ej: "Kilometer", "Mile", "Celsius").
    name: String,

    /// Símbolo abreviado (ej: "km", "mi", "°C").
    symbol: String,

    /// Factor de conversión relativo a la unidad base de la categoría.
    /// Para temperatura se usa un esquema especial (offset + factor).
    to_base_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum UnitCategory {
    Length,
    Mass,
    Time,
    Temperature,
    Speed,
    Area,
    Volume,
    Energy,
    Pressure,
    Data,
    Angle,
}
```

**Comando auxiliar (fase 2):** Se añadirá `get_units_by_category(category: UnitCategory) -> Vec<Unit>` para que el frontend pueda llenar los dropdowns dinámicamente.

#### HistoryEntry

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryEntry {
    /// Identificador único de la entrada (UUID v4).
    id: String,

    /// Expresión tal como la ingresó el usuario (sin formatear).
    expression: String,

    /// Resultado formateado (value del campo `result` de ExpressionResult).
    result: String,

    /// Timestamp ISO 8601 (ej: "2026-05-12T14:30:00Z").
    timestamp: String,

    /// Modo en que se evaluó la expresión.
    mode: CalcMode,
}
```

**Ejemplo JSON:**

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "expression": "sin(45)+log(100)",
  "result": "1.7071067812",
  "timestamp": "2026-05-12T14:30:00Z",
  "mode": "Scientific"
}
```

#### Matrix

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Matrix {
    /// Número de filas.
    rows: usize,

    /// Número de columnas.
    cols: usize,

    /// Datos en orden row-major (fila tras fila).
    /// Longitud siempre = rows × cols.
    data: Vec<f64>,
}
```

**Ejemplo JSON (matriz 2×3):**

```json
{
  "rows": 2,
  "cols": 3,
  "data": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
}
```

> Matriz representada:
> ```
> ⎡ 1  2  3 ⎤
> ⎣ 4  5  6 ⎦
> ```

#### Complex

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Complex {
    /// Parte real.
    real: f64,

    /// Parte imaginaria (coeficiente de i).
    imag: f64,
}
```

**Ejemplo JSON:**

```json
{ "real": 3.0, "imag": -4.0 }
```

> Representa el número complejo `3 - 4i`.

### 3.3. Estado de la Aplicación (AppState en Rust)

El backend mantiene un estado mutable compartido que los comandos leen y modifican. Se inyecta en Tauri como `tauri::State<AppState>`.

```rust
struct AppState {
    /// Último resultado evaluado (para la tecla ANS).
    last_result: Option<ExpressionResult>,

    /// Modo angular actual para funciones trigonométricas.
    angle_mode: AngleMode,

    /// Precisión decimal para display (0–15).
    precision: u8,

    /// Memoria de la calculadora.
    memory: Option<f64>,

    /// Historial de evaluaciones.
    history: Vec<HistoryEntry>,

    /// Variables definidas por el usuario (ej: x=5).
    variables: HashMap<String, f64>,
}
```

> **Seguridad:** En Tauri 2.x, `AppState` se envuelve con `Mutex` (o `RwLock` si se anticipa alta concurrencia de lectura). Para una calculadora de usuario único, `Mutex<AppState>` es suficiente.

**Inicialización:**

```rust
// En main.rs —(esquemático, no código final)
let state = AppState::load_from_disk()  // Lee settings.json + history.json + memory.json
    .unwrap_or_else(|_| AppState::default());

tauri::Builder::default()
    .manage(Mutex::new(state))
    .invoke_handler(tauri::generate_handler![
        evaluate_expression, convert_units,
        get_history, clear_history,
        memory_store, memory_recall, memory_clear, memory_add, memory_subtract,
        set_angle_mode, set_precision,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

---

## 4. Flujo de Datos Detallado

### 4.1. Flujo principal: Evaluación de expresión

```
Usuario pulsa teclas en Keypad
          │
          ▼
┌──────────────────────┐
│  Svelte frontend     │
│                      │
│  1. Keypad.svelte    │  Cada botón actualiza expressionStore
│     emite evento     │  (ej: agregar dígito, operador, función)
│                      │
│  2. expressionStore  │  Contiene la cadena sin evaluar
│     = "sin(45)+2"   │
│                      │
│  3. Al pulsar "="    │
│     se invoca:       │
└──────────┬───────────┘
           │
           │  invoke('evaluate_expression', {
           │    expr: "sin(45)+2",
           │    mode: "Scientific"
           │  })
           │
           ▼
┌──────────────────────┐
│  Rust Command Layer  │
│                      │
│  evaluate.rs:        │
│  1. Leer AppState    │  (angle_mode, precision, variables)
│  2. Delegar al parser│
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  Parser Engine       │
│                      │
│  tokenizer.rs:       │  "sin(45)+2" → [Func("sin"), LParen, Num(45), RParen, Add, Num(2)]
│                      │
│  ast.rs:             │  tokens → AST = Add(FuncCall("sin", Num(45)), Num(2))
│                      │
│  evaluator.rs:       │  AST + angle_mode=Deg → resultado
│     sin(45°) = 0.7071...
│     0.7071... + 2 = 2.7071...
└──────────┬───────────┘
           │
           │  Result<ExpressionResult, CalcError>
           │
           ▼
┌──────────────────────┐
│  Devolución al FE    │
│                      │
│  Éxito:              │
│  {                    │
│    "result": "2.7071",│
│    "display": "sin(45°)+2"
│    "format": "Decimal"
│  }                    │
│                      │
│  Error:               │
│  {                    │
│    "kind": "ParseError",
│    "message": "Unexpected token...",
│    "position": 7
│  }                    │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  Svelte Frontend     │
│                      │
│  4. resultStore      │  Se actualiza con el resultado
│     = "2.7071"      │
│                      │
│  5. Display.svelte   │  Muestra resultado de forma reactiva
│     actualiza vista  │
│                      │
│  6. historyStore     │  Se actualiza (el historial se
│     add entry        │  obtiene con get_history() de Rust,
│                      │  o el backend ya lo insertó)
└──────────────────────┘
```

### 4.2. Flujo de memoria

```
Usuario pulsa "M+"
        │
        ▼
  Frontend: invoke('memory_add', { value: currentResult })
        │
        ▼
  Rust: 1. Lee valor actual de AppState.memory (o 0 si None)
        2. memory = memory + value
        3. Guarda en disk (memory.json)
        4. Retorna void (éxito) o CalcError (fallo)
        │
        ▼
  Frontend: M-badge se ilumina indicando "memoria ocupada"

Usuario pulsa "MR"
        │
        ▼
  Frontend: invoke('memory_recall')
        │
        ▼
  Rust: Retorna Option<f64> → JSON: 42.5 | null
        │
        ▼
  Frontend: Inserta el valor en la expresión actual
```

### 4.3. Flujo de conversiones de unidades

```
Usuario selecciona "100 km → millas"
        │
        ▼
  Frontend: invoke('convert_units', {
    value: 100,
    from: { category: "Length", name: "Kilometer", symbol: "km", to_base_factor: 1000 },
    to:   { category: "Length", name: "Mile", symbol: "mi", to_base_factor: 1609.344 }
  })
        │
        ▼
  Rust:  result = value × from.to_base_factor / to.to_base_factor
         result = 100 × 1000 / 1609.344 = 62.13712...
        │
        ▼
  Frontend: Muestra "≈ 62.14 mi"
```

> **Nota sobre temperatura:** La conversión Celsius → Fahrenheit no es lineal simple. El backend detecta que `UnitCategory::Temperature` y aplica la fórmula adecuada en lugar del factor directo.

### 4.4. Secuencia de inicialización

```
App inicia
    │
    ├─► Rust: AppState::load_from_disk()
    │   ├── Lee settings.json  → angle_mode, precision
    │   ├── Lee memory.json     → valor en memoria
    │   └── Lee history.json    → historial previo
    │
    ├─► Rust: tauri::Builder::default()
    │   .manage(Mutex::new(app_state))
    │   .invoke_handler(...)
    │   .run(...)
    │
    └─► Svelte: Monta App.svelte
        ├── invoke('get_history')    → popula historyStore
        ├── invoke('memory_recall')   → actualiza indicador de memoria
        └── Lee configuración del backend (o usa valores por defecto)
            → angleModeStore, precisionStore
```

---

## 5. Estructura de Directorios del Proyecto

```
CientificCalculator/
│
├── src-tauri/                          # ← Rust backend (Tauri)
│   ├── Cargo.toml                       # Dependencias: serde, serde_json, tauri, uuid, etc.
│   ├── tauri.conf.json                  # Configuración de ventana, permisos, plugins
│   ├── build.rs                         # Build script de Tauri
│   ├── icons/                           # Íconos de la app
│   │
│   └── src/
│       ├── main.rs                      # Entry point: inicializa AppState, registra comandos
│       │
│       ├── commands/                    # Handlers de comandos Tauri (IPC)
│       │   ├── mod.rs                   # Re-exporta todos los submódulos
│       │   ├── evaluate.rs              # evaluate_expression
│       │   ├── convert.rs               # convert_units
│       │   ├── history.rs               # get_history, clear_history
│       │   └── memory.rs                # memory_store, memory_recall, memory_clear,
│       │                                # memory_add, memory_subtract
│       │
│       ├── parser/                      # Motor de parsing de expresiones
│       │   ├── mod.rs                   # Función principal: parse(expr) → Result<AST, CalcError>
│       │   ├── tokenizer.rs             # Lexer: String → Vec<Token>
│       │   ├── ast.rs                   # Definición del AST y estructuras intermedias
│       │   └── evaluator.rs             # Evaluador: AST + Context → Result<f64, CalcError>
│       │
│       ├── math/                        # Motor matemático (funciones puras)
│       │   ├── mod.rs                   # Re-exporta funciones matemáticas
│       │   ├── trig.rs                  # Funciones trigonométricas (respetan angle_mode)
│       │   ├── stats.rs                 # Funciones estadísticas
│       │   ├── matrix.rs                # Operaciones con matrices
│       │   └── complex.rs               # Operaciones con números complejos
│       │
│       ├── models/                      # Tipos compartidos (Serde-serializables)
│       │   ├── mod.rs                   # Re-exporta tipos
│       │   ├── types.rs                 # CalcMode, AngleMode, FormatType, ExpressionResult,
│       │   │                            # Unit, UnitCategory, HistoryEntry, Matrix, Complex
│       │   └── errors.rs                # CalcError, ErrorKind
│       │
│       └── persistence/                # Persistencia (fase 2+)
│           ├── mod.rs                   # Traits de persistencia
│           ├── settings.rs              # Lectura/escritura de configuración
│           ├── history_file.rs          # Historial en JSON
│           └── memory_file.rs           # Memoria en JSON
│
├── src/                                 # ← Svelte frontend
│   ├── app.html                         # Template HTML raíz
│   ├── app.css                          # Estilos globales + Tailwind
│   │
│   ├── lib/
│   │   ├── components/                  # Componentes Svelte
│   │   │   ├── Display.svelte           # Pantalla de la calculadora (expresión + resultado)
│   │   │   ├── Keypad.svelte            # Teclado numérico y operadores básicos
│   │   │   ├── ScientificKeys.svelte    # Panel de funciones científicas (se expande/colapsa)
│   │   │   ├── History.svelte            # Panel lateral de historial
│   │   │   ├── ModeSelector.svelte       # Selector de modo (Standard/Scientific/Programmer/...)
│   │   │   ├── MemoryIndicator.svelte   # Indicador de estado de memoria (M)
│   │   │   └── AngleModeToggle.svelte   # Toggle DEG/RAD/GRAD
│   │   │
│   │   ├── stores/                      # Svelte stores (estado reactivo)
│   │   │   ├── expression.ts            # Store de la expresión actual
│   │   │   ├── result.ts               # Store del último resultado
│   │   │   ├── history.ts               # Store del historial (sincronizado con Rust)
│   │   │   ├── memory.ts               # Store de estado de memoria
│   │   │   ├── settings.ts             # Stores de configuración (modo, ángulo, precisión)
│   │   │   └── errors.ts               # Store de errores para UI
│   │   │
│   │   ├── utils/                       # Funciones helper
│   │   │   ├── formatters.ts            # Formateo de números para display
│   │   │   ├── tauri-bridge.ts          # Wrapper de invoke() con tipado y manejo de errores
│   │   │   └── key-bindings.ts          # Mapeo de teclas del teclado físico
│   │   │
│   │   └── types.ts                     # Tipos TypeScript espejo de los Rust models
│   │                                    # (CalcMode, AngleMode, ExpressionResult, CalcError, etc.)
│   │
│   ├── App.svelte                       # Componente raíz
│   └── main.js                          # Entry point de Svelte
│
├── Doc/                                 # ← Documentación
│   ├── plan-desarrollo.md               # Plan general del proyecto
│   └── arquitectura-api.md              # ← Este documento
│
├── tests/                               # ← Tests E2E (Playwright)
│   ├── basic.spec.ts                    # Tests de operaciones básicas
│   ├── scientific.spec.ts               # Tests de funciones científicas
│   ├── memory.spec.ts                   # Tests de memoria
│   └── history.spec.ts                  # Tests de historial
│
├── static/                              # Assets estáticos (íconos, fuentes)
│   └── icons/
│
├── package.json                         # Dependencias del frontend
├── svelte.config.js                     # Configuración de Svelte
├── vite.config.ts                       # Configuración de Vite (bundler)
├── tsconfig.json                        # Configuración de TypeScript
└── README.md                            # Descripción general del proyecto
```

---

## 6. Contrato de Serialización JSON

Para garantizar compatibilidad exacta entre Rust y TypeScript, se establecen las siguientes reglas de serialización:

### 6.1. Reglas generales

| Regla | Ejemplo | Configuración Serde |
|---|---|---|
| Enums en **PascalCase** | `"Scientific"`, `"Deg"`, `"Hex"` | `#[serde(rename_all = "PascalCase")]` |
| Structs en **camelCase** | `"result"`, `"display"`, `"format"` | `#[serde(rename_all = "camelCase")]` |
| `Option<T>` → `T` o `null` | `position: null` | Default de Serde |
| `Vec<T>` → array JSON | `[...]` | Default de Serde |
| Errores → `Result<T, CalcError>` → Tauri maneja serializando `Ok` como valor y `Err` como error de invoke | | `#[tauri::command]` maneja `Result` automáticamente |

### 6.2. Tipos TypeScript espejo

El frontend mantiene un archivo `src/lib/types.ts` con tipos que espejan exactamente los de Rust:

```typescript
// CalcMode
type CalcMode = 'Standard' | 'Scientific' | 'Programmer' | 'Statistics' | 'Complex' | 'Matrix';

// AngleMode
type AngleMode = 'Deg' | 'Rad' | 'Grad';

// FormatType
type FormatType = 'Decimal' | 'Scientific' | 'Fraction' | 'Hex' | 'Oct' | 'Bin' | 'Complex';

// ExpressionResult
interface ExpressionResult {
  result: string;
  display: string;
  format: FormatType;
}

// CalcError
interface CalcError {
  kind: ErrorKind;
  message: string;
  position: number | null;
}

// ErrorKind
type ErrorKind = 'DivisionByZero' | 'DomainError' | 'ParseError' | 'Overflow'
               | 'InvalidMode' | 'DimensionMismatch' | 'UndefinedVariable' | 'InternalError';

// HistoryEntry
interface HistoryEntry {
  id: string;
  expression: string;
  result: string;
  timestamp: string;
  mode: CalcMode;
}

// Unit
interface Unit {
  category: UnitCategory;
  name: string;
  symbol: string;
  toBaseFactor: number;
}

// UnitCategory
type UnitCategory = 'Length' | 'Mass' | 'Time' | 'Temperature' | 'Speed'
                  | 'Area' | 'Volume' | 'Energy' | 'Pressure' | 'Data' | 'Angle';

// Matrix
interface Matrix {
  rows: number;
  cols: number;
  data: number[];
}

// Complex
interface Complex {
  real: number;
  imag: number;
}
```

### 6.3. Wrapper de invoke tipado

Para evitar errores de tipeo en las llamadas IPC, se crea un wrapper en `src/lib/utils/tauri-bridge.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type {
  CalcMode, AngleMode, ExpressionResult, CalcError,
  HistoryEntry, Unit
} from '$lib/types';

// Tipo helper para resultados de invoke
type InvokeResult<T> = Promise<T>;

export const api = {
  evaluateExpression: (expr: string, mode: CalcMode): InvokeResult<ExpressionResult> =>
    invoke('evaluate_expression', { expr, mode }),

  convertUnits: (value: number, from: Unit, to: Unit): InvokeResult<number> =>
    invoke('convert_units', { value, from, to }),

  getHistory: (): InvokeResult<HistoryEntry[]> =>
    invoke('get_history'),

  clearHistory: (): InvokeResult<void> =>
    invoke('clear_history'),

  memoryStore: (value: number): InvokeResult<void> =>
    invoke('memory_store', { value }),

  memoryRecall: (): InvokeResult<number | null> =>
    invoke('memory_recall'),

  memoryClear: (): InvokeResult<void> =>
    invoke('memory_clear'),

  memoryAdd: (value: number): InvokeResult<void> =>
    invoke('memory_add', { value }),

  memorySubtract: (value: number): InvokeResult<void> =>
    invoke('memory_subtract', { value }),

  setAngleMode: (mode: AngleMode): InvokeResult<void> =>
    invoke('set_angle_mode', { mode }),

  setPrecision: (digits: number): InvokeResult<void> =>
    invoke('set_precision', { digits }),
};
```

> **Nota:** Los nombres de funciones en TypeScript se convierten a **camelCase** (`evaluateExpression`), mientras que los comandos Tauri se registran en **snake_case** (`evaluate_expression`). Tauri hace este mapeo automáticamente.

---

## 7. Manejo de Errores

### 7.1. En Rust (backend)

Todos los comandos retornan `Result<T, CalcError>`. Los errores se originan en tres capas:

```
┌─────────────────────────────────────────────────────────┐
│ Capa 1: PARSER                                         │
│ tokenizer / ast                                        │
│   Error: ParseError, posición en la expresión           │
│   Ej: "2+*3" → ParseError en posición 2                │
├─────────────────────────────────────────────────────────┤
│ Capa 2: EVALUADOR                                      │
│ evaluator                                             │
│   Error: DivisionByZero, DomainError, Overflow         │
│   Ej: "1/0" → DivisionByZero                           │
│   Ej: "sqrt(-1)" en modo Standard → DomainError        │
│   Ej: "2^10000" → Overflow                            │
├─────────────────────────────────────────────────────────┤
│ Capa 3: COMMAND HANDLER                                │
│ commands/*.rs                                          │
│   Error: InvalidMode, DimensionMismatch                │
│   Ej: Operación de matrices en modo Standard           │
│   También: errores de persistencia (InternalError)     │
└─────────────────────────────────────────────────────────┘
```

Cada capa convierte sus errores internos a `CalcError` con `position` cuando aplica. Los command handlers siempre devuelven `Result<T, CalcError>`.

### 7.2. En Svelte (frontend)

El wrapper `tauri-bridge.ts` captura errores IPC y los convierte en `CalcError` tipado:

```typescript
try {
  const result = await api.evaluateExpression(expr, mode);
  resultStore.set(result);
} catch (error) {
  // Tauri serializa CalcError como objeto JSON en el error
  const calcError = error as CalcError;
  errorStore.set(calcError);

  // Mostrar error en el display:
  // - ParseError: resaltar el token en la posición indicada
  // - DivisionByZero: mostrar "No se puede dividir por cero"
  // - DomainError: mostrar mensaje específico del dominio
  // - etc.
}
```

El componente `Display.svelte` interpreta `ErrorKind` para decidir el formato visual del mensaje de error:

| `ErrorKind` | Comportamiento en UI |
|---|---|
| `DivisionByZero` | Display rojo: "No se puede dividir por cero" |
| `DomainError` | Display rojo: mensaje específico (ej: "Raíz cuadrada de número negativo") |
| `ParseError` | Display rojo + subrayado del token en la posición `position` |
| `Overflow` | Display rojo: "Resultado demasiado grande" |
| `InvalidMode` | Display rojo: "Operación no disponible en modo [modo]" |
| `DimensionMismatch` | Display rojo: "Dimensiones incompatibles" |

---

## 8. Dependencias Rust (Cargo.toml)

Las dependencias clave del backend:

| Crate | Versión | Propósito |
|---|---|---|
| `tauri` | 2.x | Framework principal |
| `serde` | 1.x | Serialización/deserialización |
| `serde_json` | 1.x | Formato JSON para IPC y persistencia |
| `uuid` | 1.x | IDs únicos para HistoryEntry |
| `chrono` | 0.4 | Timestamps en historial |
| `rug` | 1.x (opcional) | Aritmética de precisión arbitraria |
| `nalgebra` | 0.33 (opcional) | Álgebra lineal para matrices |
| `thiserror` | 2.x | Derivación de errores custom |

> **Nota:** `rug` y `nalgebra` se marcan como opcionales y se activan con features de Cargo cuando se implementen los modos de precisión arbitraria y matrices respectivamente.

---

## 9. Convenciones y Decisiones de Diseño

### 9.1. Nomenclatura

| Contexto | Convención | Ejemplo |
|---|---|---|
| Comandos Tauri (Rust fn) | `snake_case` | `evaluate_expression` |
| Comandos Tauri (invoke JS) | `snake_case` | `invoke('evaluate_expression', ...)` |
| Funciones wrapper (TS) | `camelCase` | `api.evaluateExpression(...)` |
| Enums Rust | `PascalCase` | `CalcMode::Scientific` |
| Enums JSON | `PascalCase` | `"Scientific"` |
| Structs Rust | `PascalCase` | `ExpressionResult` |
| Campos de structs JSON | `camelCase` | `"toBaseFactor"` |
| Archivos Rust | `snake_case` | `evaluate.rs` |
| Componentes Svelte | `PascalCase` | `Display.svelte` |
| Stores Svelte | `camelCase` | `expressionStore` |
| Tipos TypeScript | `PascalCase` | `interface ExpressionResult` |

### 9.2. Decisiones clave

1. **Estado en Rust, no en Svelte:** El backend es la única fuente de verdad para el modo angular, la precisión y el historial. El frontend refleja lo que Rust informa. Esto evita desincronización y simplifica la lógica de UI.

2. **Historial gestionado por Rust:** `evaluate_expression` inserta automáticamente en el historial. El front solo hace `get_history()` para renderizar. No necesita sincronizar manualmente.

3. **Precisión como string:** `ExpressionResult.result` es siempre un `String`, nunca un `f64`. Razón: evita pérdida de precisión en serialización JSON, permite representar resultados no numéricos (complejos, matrices), y permite notaciones como `0xFF`.

4. **Serialización snake_case para comandos, camelCase para datos:** Tauri 2.x convierte automáticamente los nombres de parámetros de comandos (snake_case en Rust → snake_case en JS). Los campos de structs de datos siguen `camelCase` por convención JSON.

5. **Persistencia simple (JSON en disco):** Para la fase inicial se usan archivos JSON en el directorio de datos de la app. No se introduce SQLite hasta que el historial crezca significativamente (Criterio: >1000 entradas, o cuando se necesite búsqueda).

6. **Manejo de errores estructurado:** Todo error pasa por `CalcError` con `ErrorKind` clasificado. Nunca se usa `panic!` ni `unwrap()` en handlers de comandos. Todo error se propaga como `Result`.

7. **Modo de ángulo como estado global:** `set_angle_mode` modifica un estado en `AppState`. Todas las evaluaciones trigonométricas posteriores lo respetan. No se pasa el ángulo en cada evaluación para simplificar la interfaz.

---

## 10. Diagrama de Dependencias entre Módulos Rust

```
                    main.rs
                      │
                      │  registra comandos y AppState
                      ▼
               ┌──────────────┐
               │  commands/   │
               │  (IPC layer) │
               └──────┬───────┘
                      │
          ┌───────────┼───────────┐
          │           │           │
          ▼           ▼           ▼
    ┌──────────┐ ┌─────────┐ ┌─────────┐
    │evaluate  │ │convert  │ │history  │
    │.rs       │ │.rs      │ │memory   │
    └────┬─────┘ └────┬────┘ └────┬────┘
         │             │           │
         │             │           │
         ▼             ▼           ▼
    ┌──────────┐ ┌──────────┐ ┌──────────┐
    │ parser/  │ │  math/   │ │persistence│
    │          │ │          │ │  (JSON)    │
    │ tokenizer│ │ trig.rs  │ │            │
    │ ast.rs   │ │ stats.rs │ │ settings.rs│
    │ evaluator│ │ matrix.rs│ │ history.rs │
    └────┬─────┘ │ complex  │ │ memory.rs  │
         │       └────┬─────┘ └─────┬──────┘
         │            │              │
         └────────────┼──────────────┘
                      │
                      ▼
               ┌──────────────┐
               │   models/    │
               │              │
               │ types.rs     │  ← Tipos compartidos (CalcMode, ExpressionResult, etc.)
               │ errors.rs    │  ← CalcError, ErrorKind
               └──────────────┘
```

**Regla de dependencia:** Los módulos dependen siempre hacia abajo. `models/` no depende de nadie. `parser/` y `math/` dependen solo de `models/`. `commands/` depende de `parser/`, `math/`, y `models/`. `main.rs` depende de `commands/`.

---

## 11. Checklist de Implementación (para el Senior)

- [ ] Crear `src-tauri/src/models/types.rs` con todos los enums y structs Serde
- [ ] Crear `src-tauri/src/models/errors.rs` con CalcError y ErrorKind
- [ ] Crear `src-tauri/src/commands/` con stubs que retornan `Result<T, CalcError>`
- [ ] Implementar `AppState` con `Mutex<AppState>` en `main.rs`
- [ ] Registrar todos los comandos en `tauri::Builder`
- [ ] Implementar `parser/tokenizer.rs` con tests unitarios
- [ ] Implementar `parser/ast.rs` con tests unitarios
- [ ] Implementar `parser/evaluator.rs` con tests unitarios
- [ ] Implementar `math/trig.rs` (respetando angle_mode de AppState)
- [ ] Crear `src/lib/types.ts` espejando los tipos Rust
- [ ] Crear `src/lib/utils/tauri-bridge.ts` con el wrapper de invoke tipado
- [ ] Escribir tests E2E mínimos (evaluate "2+2" = "4")

---

*Documento generado en Fase 0, Paso 0.4. Se actualizará conforme avancen las fases de desarrollo.*
