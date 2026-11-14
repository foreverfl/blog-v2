import Providers from "@/components/Providers";
import { useAuth } from "@/lib/context/auth-context";
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
 * One button per category.json item the viewer may see: "all" items always,
 * "login" items once the login check has finished and the viewer is logged in.
 *
 * @param lan - "ko" | "ja" | "en"
 */
function AppHomeButtons({ lan }: { lan: string }) {
  const { isReady, isLoggedIn } = useAuth();
  const nameKey = `name_${lan}` as "name_ko" | "name_ja" | "name_en";
  const canSeeLoginButtons = isReady && isLoggedIn;

  const appHomeButtons = classifications.flatMap((classification) =>
    classification.categories
      .filter(
        (category) =>
          category.appHome === "all" ||
          (category.appHome === "login" && canSeeLoginButtons),
      )
      .map((category) => ({
        href: `/${lan}/${classification.link}/${category.link}`,
        label: `${category.emoji} ${category[nameKey]}`,
      })),
  );

  return (
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
  );
}

/**
 * The first screen of the installed app, inside the shared Providers tree.
 *
 * @param lan - "ko" | "ja" | "en"
 */
export default function AppHomeIsland({ lan }: { lan: string }) {
  return (
    <Providers>
      <AppHomeButtons lan={lan} />
    </Providers>
  );
}
