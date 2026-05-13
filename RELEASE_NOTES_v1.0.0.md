# 🧮 CientificCalculator v1.0.0

Calculadora científica multiplataforma construida con Tauri 2.x + Rust + Svelte 5.

## ✨ Características

### Modos de cálculo
- **Standard** — Operaciones aritméticas básicas
- **Scientific** — Trigonometría, logaritmos, exponenciales, factorial, raíces
- **Programmer** — Bases HEX/DEC/OCT/BIN, operaciones bitwise (AND, OR, XOR, NOT, SHL, SHR)
- **Complex** — Números complejos (a+bi, polar, conj, arg, real, imag)
- **Matrix** — Matrices 1×1 a 5×5, suma, resta, multiplicación, determinante, inversa, transpuesta
- **Statistics** — Media, mediana, moda, desviación estándar, varianza, cuartiles, regresión lineal
- **Solver** — Newton-Raphson con derivada automática
- **Graph** — Gráficos 2D con zoom, pan, tooltip y múltiples curvas

### Funcionalidades adicionales
- ✅ Conversión de unidades (10 categorías, 71 unidades)
- ✅ Memoria (M+, M-, MR, MC) con persistencia a disco
- ✅ Historial de operaciones (100 entradas)
- ✅ Temas claro/oscuro
- ✅ Soporte de teclado físico
- ✅ Atajos: Enter(=), Escape(C), Backspace(⌫)
- ✅ Evaluador fallback en navegador (desarrollo web)

## 📥 Descargas

| Plataforma | Formato | Tamaño |
|------------|---------|--------|
| Linux (Debian/Ubuntu) | [.deb](https://github.com/ugelucero/CientificCalculator/releases/download/v1.0.0/CientificCalculator_1.0.0_amd64.deb) | 3 MB |
| Linux (Fedora/RHEL) | [.rpm](https://github.com/ugelucero/CientificCalculator/releases/download/v1.0.0/CientificCalculator-1.0.0-1.x86_64.rpm) | 3 MB |
| Linux (AppImage) | [.AppImage](https://github.com/ugelucero/CientificCalculator/releases/download/v1.0.0/CientificCalculator_1.0.0_amd64.AppImage) | 74 MB |
| Windows | .msi (próximamente) | — |
| macOS | .dmg (próximamente) | — |

## 🛠️ Compilar desde código

```bash
git clone https://github.com/ugelucero/CientificCalculator.git
cd CientificCalculator
npm install
npm run tauri build
```

## 📦 Dependencias del sistema (Linux)

```bash
sudo apt install libdbus-1-dev libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev
```
