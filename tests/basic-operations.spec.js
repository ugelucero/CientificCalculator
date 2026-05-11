import { test, expect } from '@playwright/test';

/**
 * Pruebas de operaciones aritméticas básicas.
 * Se evalúan mediante clics en botones (o teclas para ^).
 */

test.describe('Operaciones básicas', () => {

  /** Helper: clickea un botón del keypad por su texto exacto */
  async function clickKey(page, text) {
    await page.locator('.keypad').getByRole('button', { name: text, exact: true }).click();
  }

  test('2 + 3 = 5', async ({ page }) => {
    await page.goto('/');

    await clickKey(page, '2');
    await clickKey(page, '+');
    await clickKey(page, '3');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('5');
  });

  test('10 - 4 = 6', async ({ page }) => {
    await page.goto('/');

    await clickKey(page, '1');
    await clickKey(page, '0');
    // Botón '−' (U+2212 MINUS SIGN) en el teclado
    await page.locator('.keypad').locator('button').filter({ hasText: '−' }).click();
    await clickKey(page, '4');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('6');
  });

  test('3 × 7 = 21', async ({ page }) => {
    await page.goto('/');

    await clickKey(page, '3');
    // Botón '×' (U+00D7 MULTIPLICATION SIGN)
    await page.locator('.keypad').locator('button').filter({ hasText: '×' }).click();
    await clickKey(page, '7');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('21');
  });

  test('20 ÷ 4 = 5', async ({ page }) => {
    await page.goto('/');

    await clickKey(page, '2');
    await clickKey(page, '0');
    // Botón '÷' (U+00F7 DIVISION SIGN)
    await page.locator('.keypad').locator('button').filter({ hasText: '÷' }).click();
    await clickKey(page, '4');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('5');
  });

  test('2^3 = 8 (usando teclado)', async ({ page }) => {
    await page.goto('/');

    // ^ no está en el teclado básico; usamos teclas físicas
    await page.keyboard.press('2');
    await page.keyboard.press('^');
    await page.keyboard.press('3');
    await page.keyboard.press('Enter');

    await expect(page.locator('.result')).toHaveText('8');
  });

  test('10 % 3 = 1', async ({ page }) => {
    await page.goto('/');

    await clickKey(page, '1');
    await clickKey(page, '0');
    await clickKey(page, '%');
    await clickKey(page, '3');
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('1');
  });

});
