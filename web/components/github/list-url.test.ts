import { expect, test } from "vitest";
import { listUrl } from "./list-url";

test("first page starts at offset 0", () => {
  expect(listUrl({ offset: 0 })).toBe("/github/overall?limit=100&offset=0");
});

test("later page carries its offset", () => {
  expect(listUrl({ offset: 200 })).toBe("/github/overall?limit=100&offset=200");
});
