import { strictEqual } from "node:assert";
import { test } from "vitest";
import { pickDescription } from "./description";

test("ko shows the korean translation", () => {
  strictEqual(
    pickDescription("ko", { description_ko: "가", description: "a" }),
    "가",
  );
});

test("ko without a translation falls back to the original", () => {
  strictEqual(
    pickDescription("ko", { description_ko: null, description: "a" }),
    "a",
  );
});

test("en shows the original even when translations exist", () => {
  strictEqual(
    pickDescription("en", {
      description_ko: "가",
      description_ja: "あ",
      description: "a",
    }),
    "a",
  );
});
