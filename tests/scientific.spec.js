import { test, expect } from '@playwright/test';

/**
 * Pruebas de funciones científicas.
 * Requieren expandir el panel científico para acceder a sin, cos, ln, etc.
 */

test.describe('Funciones científicas', () => {

  /** Expande el panel científico si no lo está */
  async function expandScientific(page) {
    const toggle = page.locator('.toggle-btn');
    const text = await toggle.textContent();
    if (text?.includes('▼')) {
      await toggle.click();
    }
  }

  /** Helper: clickea un botón del keypad por su texto exacto */
  async function clickKey(page, text) {
    await page.locator('.keypad').getByRole('button', { name: text, exact: true }).click();
  }

  /** Helper: clickea un botón científico por su texto exacto (regex) */
  async function clickSci(page, text) {
    const escaped = text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    await page.locator('.sci-btn').filter({ hasText: new RegExp(`^${escaped}$`) }).click();
  }

  test('sin(0) = 0', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    await clickSci(page, 'sin');
    await clickKey(page, '0');
    await clickSci(page, ')');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('0');
  });

  test('cos(0) = 1', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    await clickSci(page, 'cos');
    await clickKey(page, '0');
    await clickSci(page, ')');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('1');
  });

  test('sqrt(9) = 3', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    // Botón '√' (U+221A SQUARE ROOT)
    await clickSci(page, '√');
    await clickKey(page, '9');
    await clickSci(page, ')');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('3');
  });

  test('ln(e) ≈ 1', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    await clickSci(page, 'ln');
    // Usar regex exacto para 'e' (evitar conflicto con 'EXP' que contiene 'e')
    await page.locator('.sci-btn').filter({ hasText: /^e$/ }).click();
    await clickSci(page, ')');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('1');
  });

  test('pi muestra el valor de π', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    // Botón 'π'
    await clickSci(page, 'π');
    await clickKey(page, '=');

    // Math.PI = 3.141592653589793
    const resultText = await page.locator('.result').textContent();
    const resultNum = parseFloat(resultText);
    expect(resultNum).toBeCloseTo(Math.PI, 10);
  });

  test('abs(-5) = 5', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    // Botón '|x|' envía 'abs('
    await clickSci(page, '|x|');
    await clickKey(page, '5');
    // Botón '±' para negar el número
    await clickKey(page, '±');
    await clickSci(page, ')');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('5');
  });

  test('5! = 120', async ({ page }) => {
    await page.goto('/');
    await expandScientific(page);

    await clickKey(page, '5');
    // Botón 'n!' envía '!'
    await clickSci(page, 'n!');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('120');
  });

});
