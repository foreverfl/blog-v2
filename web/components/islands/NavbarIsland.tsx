import BackButton from "@/components/atom/BackButton";
import Providers from "@/components/Providers";
import Navbar from "@/components/organism/navbar/Navbar";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

// Navbar island: needs the shared Providers tree (auth for Profile,
// loading for SetLanguage, theme for SetMode).
// Devtools live only here: this island persists across navigations, and the queryClient is shared.
export default function NavbarIsland() {
  return (
    <Providers>
      <Navbar />
      <BackButton />
      {import.meta.env.DEV && <ReactQueryDevtools initialIsOpen={false} />}
    </Providers>
  );
}
