import Providers from "@/components/Providers";
import Footer from "@/components/organism/Footer";

// Footer island: the bug report modal needs the shared Providers tree (toast).
export default function FooterIsland() {
  return (
    <Providers>
      <Footer />
    </Providers>
  );
}
