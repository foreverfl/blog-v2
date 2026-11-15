"use client";

import { useEffect, useState } from "react";

// Replacement for next's usePathname in a client-only island.
// Islands kept with transition:persist outlive a navigation, so re-read on astro:page-load.
export function useClientPathname(): string {
  const [pathname, setPathname] = useState(() =>
    typeof window !== "undefined" ? window.location.pathname : "/",
  );

  useEffect(() => {
    const readPathname = () => setPathname(window.location.pathname);
    document.addEventListener("astro:page-load", readPathname);
    return () => document.removeEventListener("astro:page-load", readPathname);
  }, []);

  return pathname;
}
