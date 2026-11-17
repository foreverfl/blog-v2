export const PAGE_SIZE = 100;

/**
 * Builds the path of one page of the overall ranking, without the api base.
 *
 * @param page - { offset: rows already loaded, lang: "Rust" or "" / undefined for every language }
 * @returns e.g. "/github/overall?limit=100&offset=200&lang=Rust"
 */
export function listUrl({
  offset,
  lang,
}: {
  offset: number;
  lang?: string;
}): string {
  const path = `/github/overall?limit=${PAGE_SIZE}&offset=${offset}`;
  if (!lang) return path;
  return `${path}&lang=${encodeURIComponent(lang)}`;
}
