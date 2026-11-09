import { expect, test } from "@playwright/test";

test("the Korean home page opens with a title", async ({ page }) => {
  const response = await page.goto("/ko");

  expect(response?.status()).toBe(200);
  await expect(page).not.toHaveTitle("");
});
