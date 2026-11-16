export const PAGE_SIZE = 100;

/**
 * Builds the path of one page of the overall ranking, without the api base.
 *
 * @param page - { offset: rows already loaded }
 * @returns e.g. "/github/overall?limit=100&offset=200"
 */
export function listUrl({ offset }: { offset: number }): string {
  return `/github/overall?limit=${PAGE_SIZE}&offset=${offset}`;
}
