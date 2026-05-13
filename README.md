# 🧮 CientificCalculator

Calculadora científica multiplataforma construida con Tauri 2.x + Rust + Svelte 5.

## ✨ Características

- **Modos**: Standard, Scientific, Programmer, Complex, Matrix, Statistics, Graph, Solver
- **Operaciones**: aritméticas, trigonométricas, logarítmicas, exponenciales, factorial, etc.
- **Modo Programador**: bases HEX/DEC/OCT/BIN, operaciones bitwise (AND, OR, XOR, NOT, SHL, SHR)
- **Números Complejos**: operaciones a+bi, polar, conj, arg, real, imag
- **Matrices**: suma, resta, multiplicación, determinante, inversa, transpuesta
- **Estadístico**: media, mediana, moda, desviación estándar, regresión lineal, cuartiles
- **Gráficos 2D**: zoom, pan, tooltip, múltiples curvas simultáneas
- **Solver**: Newton-Raphson con derivada automática
- **Conversión de unidades**: 10 categorías, 71 unidades
- **Historial**: persistente entre sesiones
- **Memoria**: M+, M-, MR, MC
- **Temas**: claro/oscuro
- **Atajos de teclado**: números, operadores, Enter, Escape, etc.

## 🚀 Instalación

### Linux
```bash
# Debian/Ubuntu
sudo dpkg -i CientificCalculator_0.1.0_amd64.deb

# Fedora/RHEL
sudo rpm -i CientificCalculator-0.1.0-1.x86_64.rpm

# O ejecutar directamente
./app
```

### Windows
Descarga el instalador MSI desde [Releases](https://github.com/ugelucero/CientificCalculator/releases).

### macOS
Descarga el DMG desde [Releases](https://github.com/ugelucero/CientificCalculator/releases).

## 🛠️ Compilar desde código

```bash
# Requisitos: Rust 1.70+, Node 18+, dependencias del sistema
sudo apt install libdbus-1-dev libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev

git clone https://github.com/ugelucero/CientificCalculator.git
cd CientificCalculator
npm install
npm run tauri build
```

## 📖 Uso básico

| Tecla | Acción |
|-------|--------|
| `0-9` | Dígitos |
| `+`, `-`, `*`, `/` | Operaciones básicas |
| `Enter` o `=` | Evaluar |
| `Escape` | Limpiar |
| `Backspace` | Borrar último token |

## 🏗️ Arquitectura

```
CientificCalculator/
├── src/                # Frontend Svelte 5
│   └── lib/
│       ├── components/ # 10 componentes UI
│       └── utils/      # tauri-bridge, formatters
├── src-tauri/          # Backend Rust
│   └── src/
│       ├── commands/   # Comandos Tauri (evaluate, convert, memory, history, solver)
│       ├── math/       # Módulos matemáticos (trig, complex, matrix, stats, solver, convert)
│       ├── parser/     # Parser Shunting Yard (tokenizer, AST, evaluador)
│       ├── models/     # Tipos de datos compartidos
│       └── persistence/ # Persistencia de estado
└── Doc/                # Documentación técnica
```

## 📦 Dependencias técnicas

- **Frontend**: Svelte 5, TailwindCSS, Vite
- **Backend**: Rust, Tauri 2.x
- **Librerías Rust**: serde, chrono, uuid, nalgebra

## 📄 Licencia

MIT
