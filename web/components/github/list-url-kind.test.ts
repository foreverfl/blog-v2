import { strictEqual } from "node:assert";
import { test } from "vitest";
import { listUrl } from "./list-url";

test("a picked kind is appended as ?kind=", () => {
  strictEqual(
    listUrl({ offset: 0, kind: "ai-llm" }),
    "/github/overall?limit=100&offset=0&kind=ai-llm",
  );
});

test("kind comes before lang when both are picked", () => {
  strictEqual(
    listUrl({ offset: 0, kind: "ai-llm", lang: "Rust" }),
    "/github/overall?limit=100&offset=0&kind=ai-llm&lang=Rust",
  );
});

test("empty kind and lang append nothing", () => {
  strictEqual(
    listUrl({ offset: 0, kind: "", lang: "" }),
    "/github/overall?limit=100&offset=0",
  );
});
