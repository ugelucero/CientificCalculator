# Release Checklist

## Prerelease
- [ ] Compilar para Linux: `npm run tauri build`
- [ ] Compilar para Windows (desde Windows)
- [ ] Compilar para macOS (desde macOS)
- [ ] Verificar que .deb, .rpm, .msi, .dmg se generan correctamente

## Testing
- [ ] Probar todos los modos: Std, Sci, Prog, Cmplx, Mat, Stat, Solv, Graph
- [ ] Probar conversión de unidades
- [ ] Probar historial y memoria
- [ ] Probar tema claro/oscuro
- [ ] Probar atajos de teclado
- [ ] Verificar que los tests E2E pasan: `npx playwright test`

## Release
- [ ] Actualizar versión en `package.json` y `src-tauri/Cargo.toml`
- [ ] Crear tag: `git tag v1.0.0 && git push origin v1.0.0`
- [ ] Crear Release en GitHub con los binarios adjuntos
- [ ] Escribir release notes describiendo cambios
