import Providers from "@/components/Providers";
import categoryJson from "@/public/category.json";

/**
 * One item of category.json, as the app home reads it.
 * appHome "all": everyone sees its button. "login": only when logged in.
 * No appHome: the item has no button.
 */
interface AppHomeCategory {
  name_ko: string;
  name_ja: string;
  name_en: string;
  emoji: string;
  link: string;
  appHome?: "all" | "login";
}

interface AppHomeClassification {
  link: string;
  categories: AppHomeCategory[];
}

const classifications = categoryJson as AppHomeClassification[];

/**
 * The first screen of the installed app: one button per category.json item
 * whose appHome is "all".
 *
 * @param lan - "ko" | "ja" | "en"
 */
export default function AppHomeIsland({ lan }: { lan: string }) {
  const nameKey = `name_${lan}` as "name_ko" | "name_ja" | "name_en";

  const appHomeButtons = classifications.flatMap((classification) =>
    classification.categories
      .filter((category) => category.appHome === "all")
      .map((category) => ({
        href: `/${lan}/${classification.link}/${category.link}`,
        label: `${category.emoji} ${category[nameKey]}`,
      })),
  );

  return (
    <Providers>
      <main className="mx-auto grid min-h-screen max-w-md grid-cols-2 content-start gap-4 px-4 pt-24">
        {appHomeButtons.map((button) => (
          <a
            key={button.href}
            href={button.href}
            className="rounded-2xl border border-neutral-300 p-6 text-center text-lg dark:border-neutral-700"
          >
            {button.label}
          </a>
        ))}
      </main>
    </Providers>
  );
}
