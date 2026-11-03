// Official logos from the brand kit — never redraw them (docs/design/GUIA-DE-MARCA.md).
import logoUrl from "../../../assets/brand/logo/homedeck-logo-horizontal-white.svg";

/** Horizontal logo (symbol + name): the default per the brand guide. */
export function Logo({ className }: { className?: string }) {
  return <img className={className} src={logoUrl} alt="homedeck" />;
}
