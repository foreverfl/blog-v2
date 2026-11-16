import RepoModal from "@/components/github/RepoModal";
import { listUrl } from "@/components/github/list-url";
import LoadMoreSentinel from "@/components/molecules/LoadMoreSentinel";
import { RUST_API } from "@/lib/api-base";
import "@/lib/i18n";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

export type RepoRow = {
  rank: number;
  full_name: string;
  url: string;
  description: string | null;
  description_ko: string | null;
  description_ja: string | null;
  stars: number;
  language: string | null;
  topics: string[];
  repo_created_at: string;
  stars_today: number | null;
  kind: string | null;
  languages: string[];
};

// One tab until the trending collection lands.
const TABS = ["overall"];

type LanguageCount = { name: string; count: number };
type KindCount = { name: string; count: number };

/**
 * Reads one page of the latest day's overall ranking from the rust api.
 *
 * @param offset - rows already loaded
 * @param kind - "ai-llm", or "" for every kind
 * @param lang - "Rust", or "" for every language
 * @returns the page's repos by rank; [] past the end or when the api has no day yet (404)
 */
async function fetchOverall(
  offset: number,
  kind: string,
  lang: string,
): Promise<RepoRow[]> {
  const response = await fetch(
    `${RUST_API}${listUrl({ offset, kind, lang })}`,
  );
  if (response.status === 404) return [];
  if (!response.ok) throw new Error(`GET /github/overall: ${response.status}`);
  const body: { date: string; repos: RepoRow[] } = await response.json();
  return body.repos;
}

/**
 * Reads the latest day's languages for the filter, most repos first.
 *
 * @returns [{ name: "TypeScript", count: 21 }, ...]; [] when the api has no day yet (404)
 */
async function fetchLanguages(): Promise<LanguageCount[]> {
  const response = await fetch(`${RUST_API}/github/languages`);
  if (response.status === 404) return [];
  if (!response.ok)
    throw new Error(`GET /github/languages: ${response.status}`);
  const body: { date: string; languages: LanguageCount[] } =
    await response.json();
  return body.languages;
}

/**
 * Reads the classified kinds for the filter, most repos first.
 *
 * @returns [{ name: "ai-llm", count: 12 }, ...]; [] before anything is classified
 */
async function fetchKinds(): Promise<KindCount[]> {
  const response = await fetch(`${RUST_API}/github/kinds`);
  if (!response.ok) throw new Error(`GET /github/kinds: ${response.status}`);
  const body: { kinds: KindCount[] } = await response.json();
  return body.kinds;
}

const CELL =
  "border-b border-gray-700 px-3 py-2 text-left dark:border-gray-600";

/**
 * Today's GitHub ranking: one tab per category, a one-line row per repo, 100 more rows each time the table end scrolls into view.
 *
 * @param props - { lan: "ko" | "en" | "ja" from the URL }
 * @returns the tab row and the ranking table
 */
export default function GithubRank({ lan }: { lan: string }) {
  const { t, i18n } = useTranslation();
  useEffect(() => {
    i18n.changeLanguage(lan);
  }, [lan, i18n]);
  const [tab, setTab] = useState(TABS[0]);
  const [selected, setSelected] = useState<RepoRow | null>(null);
  const [kind, setKind] = useState("");
  const [lang, setLang] = useState("");
  const { data: kinds = [] } = useQuery({
    queryKey: ["github", "kinds"],
    queryFn: fetchKinds,
  });
  const { data: languages = [] } = useQuery({
    queryKey: ["github", "languages"],
    queryFn: fetchLanguages,
  });
  const {
    data: pages,
    isSuccess,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteQuery({
    queryKey: ["github", tab, kind, lang],
    queryFn: ({ pageParam }) => fetchOverall(pageParam, kind, lang),
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) =>
      lastPage.length === 0 ? undefined : allPages.flat().length,
  });
  const repos = pages?.pages.flat() ?? [];

  return (
    <div className="mx-auto max-w-5xl px-4 py-6">
      <div className="mb-4 flex gap-2 text-sm">
        {TABS.map((key) => (
          <button
            key={key}
            type="button"
            onClick={() => setTab(key)}
            className={
              key === tab
                ? "rounded-md bg-gray-700 px-3 py-1 text-white dark:bg-slate-50 dark:text-black"
                : "rounded-md border border-gray-400 px-3 py-1 dark:border-gray-600"
            }
          >
            {key}
          </button>
        ))}
        <select
          value={kind}
          onChange={(event) => setKind(event.target.value)}
          aria-label={t("github_col_kind")}
          className="ml-auto rounded-md border border-gray-400 bg-transparent px-2 py-1 dark:border-gray-600"
        >
          <option value="">{t("github_kind_all")}</option>
          {kinds.map((kindCount) => (
            <option key={kindCount.name} value={kindCount.name}>
              {kindCount.name} ({kindCount.count})
            </option>
          ))}
        </select>
        <select
          value={lang}
          onChange={(event) => setLang(event.target.value)}
          aria-label={t("github_col_language")}
          className="rounded-md border border-gray-400 bg-transparent px-2 py-1 dark:border-gray-600"
        >
          <option value="">{t("github_lang_all")}</option>
          {languages.map((language) => (
            <option key={language.name} value={language.name}>
              {language.name} ({language.count})
            </option>
          ))}
        </select>
      </div>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr>
              <th className={CELL}>{t("github_col_rank")}</th>
              <th className={CELL}>{t("github_col_name")}</th>
              <th className={CELL}>{t("github_col_stars")}</th>
              <th className={CELL}>{t("github_col_language")}</th>
              <th className={CELL}>{t("github_col_kind")}</th>
            </tr>
          </thead>
          <tbody>
            {repos.map((repo) => (
              <tr
                key={repo.full_name}
                onClick={() => setSelected(repo)}
                className="cursor-pointer hover:bg-gray-100 dark:hover:bg-neutral-800"
              >
                <td className={CELL}>{repo.rank}</td>
                <td className={CELL}>{repo.full_name}</td>
                <td className={CELL}>{repo.stars.toLocaleString("en-US")}</td>
                <td className={CELL}>{repo.language ?? "—"}</td>
                <td className={CELL}>{repo.kind ?? "—"}</td>
              </tr>
            ))}
          </tbody>
        </table>
        {hasNextPage && !isFetchingNextPage && (
          <LoadMoreSentinel onHit={fetchNextPage} />
        )}
        {isSuccess && repos.length === 0 && (
          <p className="py-6 text-center text-sm">{t("github_empty")}</p>
        )}
      </div>
      <RepoModal repo={selected} onClose={() => setSelected(null)} />
    </div>
  );
}
