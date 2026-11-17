export type RepoDescriptions = {
  description: string | null;
  description_ko?: string | null;
  description_ja?: string | null;
};

/**
 * Picks the description to show first: the translation for lan, falling back to the original.
 *
 * @param lan - "ko" | "en" | "ja"
 * @param row - the original description and its translations
 * @returns description_ko for ko, description_ja for ja, otherwise (or when that is null) description
 */
export function pickDescription(
  lan: string,
  row: RepoDescriptions,
): string | null {
  if (lan === "ko") return row.description_ko ?? row.description;
  if (lan === "ja") return row.description_ja ?? row.description;
  return row.description;
}
