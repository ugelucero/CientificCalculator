import { test, expect } from '@playwright/test';

/**
 * Pruebas de interacción mediante teclado físico.
 * Verifica que las teclas numéricas, operadores y acciones
 * (Enter, Escape, Backspace) funcionan correctamente.
 *
 * Nota: backspace() elimina el último TOKEN completo (no carácter por carácter).
 *       Por ejemplo, "123" se borra entero con un solo Backspace.
 */

test.describe('Interacción por teclado', () => {

  test('Presionar 1 + 2 Enter → resultado 3', async ({ page }) => {
    await page.goto('/');

    await page.keyboard.press('1');
    await page.keyboard.press('+');
    await page.keyboard.press('2');
    await page.keyboard.press('Enter');

    await expect(page.locator('.result')).toHaveText('3');
  });

  test('Escape limpia el display', async ({ page }) => {
    await page.goto('/');

    // Escribir una expresión
    await page.keyboard.press('1');
    await page.keyboard.press('+');
    await page.keyboard.press('2');

    await expect(page.locator('.expression')).not.toHaveText('\u00A0');

    // Presionar Escape para limpiar
    await page.keyboard.press('Escape');

    // El display de expresión debe estar vacío (muestra \u00A0)
    await expect(page.locator('.expression')).toHaveText('\u00A0');
    // El resultado también debe estar vacío
    await expect(page.locator('.result')).toHaveText('\u00A0');
  });

  test('Backspace borra el último token', async ({ page }) => {
    await page.goto('/');

    // Un solo dígito → Backspace lo elimina
    await page.keyboard.press('5');
    await expect(page.locator('.expression')).toHaveText('5');

    await page.keyboard.press('Backspace');
    await expect(page.locator('.expression')).toHaveText('\u00A0');

    // "123" es un solo token numérico → Backspace lo elimina entero
    await page.keyboard.press('1');
    await page.keyboard.press('2');
    await page.keyboard.press('3');
    await expect(page.locator('.expression')).toHaveText('123');

    await page.keyboard.press('Backspace');
    await expect(page.locator('.expression')).toHaveText('\u00A0');

    // "1 + 2" con operador: cada token se borra individualmente
    await page.keyboard.press('1');
    await page.keyboard.press('+');
    await page.keyboard.press('2');
    const exprWithOp = await page.locator('.expression').textContent();
    expect(exprWithOp.length).toBeGreaterThan(2);

    // Primer Backspace → elimina el "2"
    await page.keyboard.press('Backspace');
    const afterFirstBack = await page.locator('.expression').textContent();
    expect(afterFirstBack.length).toBeLessThan(exprWithOp.length);

    // Segundo Backspace → elimina el operador '+'
    await page.keyboard.press('Backspace');
    const afterSecondBack = await page.locator('.expression').textContent();
    expect(afterSecondBack.length).toBeLessThan(afterFirstBack.length);

    // Tercer Backspace → elimina el "1"
    await page.keyboard.press('Backspace');
    await expect(page.locator('.expression')).toHaveText('\u00A0');
  });

});
