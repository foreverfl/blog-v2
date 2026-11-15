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
  name_ko: string;
  name_ja: string;
  name_en: string;
  link: string;
  categories: AppHomeCategory[];
}

const classifications = categoryJson as AppHomeClassification[];

/**
 * One titled section per category.json classification, each holding the
 * buttons the viewer may see; a section with no visible button is not drawn.
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

  const sections = classifications
    .map((classification) => ({
      link: classification.link,
      name: classification[nameKey],
      buttons: classification.categories
        .filter(
          (category) => category.appHome && canSeeAppHome[category.appHome],
        )
        .map((category) => ({
          href:
            category.href ?? `/${lan}/${classification.link}/${category.link}`,
          emoji: category.emoji,
          name: category[nameKey],
        })),
    }))
    .filter((section) => section.buttons.length > 0);

  return (
    <main className="mx-auto flex min-h-screen max-w-md flex-col gap-8 px-4 pt-24">
      {sections.map((section) => (
        <section key={section.link} className="flex flex-col gap-3">
          <h2 className="text-sm font-semibold text-neutral-500">
            {section.name}
          </h2>
          <div className="grid grid-cols-4 gap-4">
            {section.buttons.map((button) => (
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
          </div>
        </section>
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
