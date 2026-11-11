export const PAGE_SIZE = 100;

/**
 * Path of one page of the overall ranking.
 *
 * @param page - { offset: rows already loaded }
 * @returns "/github/overall?limit=100&offset=<offset>", without the api base
 */
export function listUrl({ offset }: { offset: number }): string {
  return `/github/overall?limit=${PAGE_SIZE}&offset=${offset}`;
}
