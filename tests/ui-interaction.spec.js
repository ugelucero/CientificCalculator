import { test, expect } from '@playwright/test';

/**
 * Pruebas de interacción con la UI mediante clics en botones.
 * Verifica limpiar, negar número y cambiar modo angular.
 */

test.describe('Interacción UI', () => {

  /** Helper: clickea un botón del keypad por su texto exacto */
  async function clickKey(page, text) {
    await page.locator('.keypad').getByRole('button', { name: text, exact: true }).click();
  }

  test('Botón C limpia el display', async ({ page }) => {
    await page.goto('/');

    // Escribir una expresión
    await clickKey(page, '5');
    await clickKey(page, '+');
    await clickKey(page, '3');

    await expect(page.locator('.expression')).not.toHaveText('\u00A0');

    // Click en botón 'C'
    await clickKey(page, 'C');

    // El display debe estar limpio
    await expect(page.locator('.expression')).toHaveText('\u00A0');
    await expect(page.locator('.result')).toHaveText('\u00A0');
  });

  test('Botón ± niega el número actual', async ({ page }) => {
    await page.goto('/');

    // Escribir 5
    await clickKey(page, '5');

    // Click en ± para negar
    await clickKey(page, '±');

    // Evaluar para ver el resultado
    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('-5');
  });

  test('Botón ± deshace la negación', async ({ page }) => {
    await page.goto('/');

    // Escribir 5 y negar dos veces
    await clickKey(page, '5');
    await clickKey(page, '±');
    await clickKey(page, '±');

    await clickKey(page, '=');

    await expect(page.locator('.result')).toHaveText('5');
  });

  test('Clic en DEG/RAD cambia el modo angular', async ({ page }) => {
    await page.goto('/');

    // Por defecto, DEG está activo. Verificar que el badge muestre DEG
    await expect(page.locator('.angle-badge')).toHaveText('DEG');

    // Click en RAD
    await page.locator('.angle-group').getByRole('button', { name: 'RAD', exact: true }).click();

    // El badge debe cambiar a RAD
    await expect(page.locator('.angle-badge')).toHaveText('RAD');

    // Click en GRAD
    await page.locator('.angle-group').getByRole('button', { name: 'GRAD', exact: true }).click();

    await expect(page.locator('.angle-badge')).toHaveText('GRAD');

    // Volver a DEG
    await page.locator('.angle-group').getByRole('button', { name: 'DEG', exact: true }).click();

    await expect(page.locator('.angle-badge')).toHaveText('DEG');
  });

});
