import { strictEqual } from "node:assert";
import { test } from "vitest";
import { listUrl } from "./list-url";

test("a picked language is appended as lang", () => {
  strictEqual(
    listUrl({ offset: 0, lang: "Rust" }),
    "/github/overall?limit=100&offset=0&lang=Rust",
  );
});

test("an empty language adds no lang", () => {
  strictEqual(
    listUrl({ offset: 0, lang: "" }),
    "/github/overall?limit=100&offset=0",
  );
});

test("a language with symbols is url-encoded", () => {
  strictEqual(
    listUrl({ offset: 0, lang: "C++" }),
    "/github/overall?limit=100&offset=0&lang=C%2B%2B",
  );
});
