import { strictEqual } from "node:assert";
import { test } from "vitest";
import { listUrl } from "./list-url";

test("first page starts at offset 0", () => {
  strictEqual(listUrl({ offset: 0 }), "/github/overall?limit=100&offset=0");
});

test("later page carries the rows already loaded as offset", () => {
  strictEqual(listUrl({ offset: 200 }), "/github/overall?limit=100&offset=200");
});
