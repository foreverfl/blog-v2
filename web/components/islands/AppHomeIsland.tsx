import Providers from "@/components/Providers";
import { useAuth } from "@/lib/context/auth-context";
import categoryJson from "@/public/category.json";

/**
 * One item of category.json, as the app home reads it.
 * appHome "all": everyone sees its button. "member": only when logged in.
 * "admin": only the site owner. No appHome: the item has no button.
 */
interface AppHomeCategory {
  name_ko: string;
  name_ja: string;
  name_en: string;
  emoji: string;
  link: string;
  appHome?: "all" | "member" | "admin";
  href?: string;
}

interface AppHomeClassification {
  link: string;
  categories: AppHomeCategory[];
}

const classifications = categoryJson as AppHomeClassification[];

/**
 * One button per category.json item the viewer may see: "all" items always,
 * "member" and "admin" items once the login check has finished and says so.
 * An item with its own href goes there instead of /lan/classification/item.
 *
 * @param lan - "ko" | "ja" | "en"
 */
function AppHomeButtons({ lan }: { lan: string }) {
  const { isReady, isLoggedIn, isAdmin } = useAuth();
  const nameKey = `name_${lan}` as "name_ko" | "name_ja" | "name_en";
  const canSeeAppHome = {
    all: true,
    member: isReady && isLoggedIn,
    admin: isReady && isAdmin,
  };

  const appHomeButtons = classifications.flatMap((classification) =>
    classification.categories
      .filter((category) => category.appHome && canSeeAppHome[category.appHome])
      .map((category) => ({
        href:
          category.href ?? `/${lan}/${classification.link}/${category.link}`,
        emoji: category.emoji,
        name: category[nameKey],
      })),
  );

  return (
    <main className="mx-auto grid min-h-screen max-w-md grid-cols-4 content-start gap-4 px-4 pt-24">
      {appHomeButtons.map((button) => (
        <a
          key={button.href}
          href={button.href}
          className="flex flex-col items-center gap-1"
        >
          <span className="flex aspect-square w-full items-center justify-center rounded-2xl bg-neutral-200 text-3xl dark:bg-neutral-800">
            {button.emoji}
          </span>
          <span className="w-full truncate text-center text-xs">
            {button.name}
          </span>
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
