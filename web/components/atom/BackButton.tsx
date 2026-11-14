import { useClientPathname } from "@/lib/hooks/useClientPathname";

/**
 * Floating back button for the app installed on an iPhone, which has no back
 * control of its own. Drawn nowhere else, and not on the app home page.
 */
export default function BackButton() {
  const pathname = useClientPathname();
  const pathSegments = pathname.split("/").filter(Boolean);
  const isAppHome = pathSegments.length === 2 && pathSegments[1] === "app";

  // navigator.standalone exists only in iOS Safari, true only in the installed app.
  const isInstalledOnIphone =
    (navigator as Navigator & { standalone?: boolean }).standalone === true;

  if (!isInstalledOnIphone || isAppHome) return null;

  return (
    <button
      type="button"
      aria-label="Back"
      onClick={() => history.back()}
      className="fixed left-4 z-40 flex h-12 w-12 items-center justify-center rounded-full bg-neutral-800/50 text-2xl text-white backdrop-blur"
      style={{ bottom: "calc(1rem + env(safe-area-inset-bottom))" }}
    >
      ←
    </button>
  );
}
