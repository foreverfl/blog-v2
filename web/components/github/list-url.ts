export const PAGE_SIZE = 100;

/**
 * Builds the path of one page of the overall ranking, without the api base.
 *
 * @param page - { offset: rows already loaded, kind: "ai-llm" or "" / undefined for every kind, lang: "Rust" or "" / undefined for every language }
 * @returns e.g. "/github/overall?limit=100&offset=200&kind=ai-llm&lang=Rust"
 */
export function listUrl({
  offset,
  kind,
  lang,
}: {
  offset: number;
  kind?: string;
  lang?: string;
}): string {
  let path = `/github/overall?limit=${PAGE_SIZE}&offset=${offset}`;
  if (kind) path += `&kind=${encodeURIComponent(kind)}`;
  if (lang) path += `&lang=${encodeURIComponent(lang)}`;
  return path;
}
