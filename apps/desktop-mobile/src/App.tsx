import { useEffect, useState } from "react";
import { FocusContext, useFocusable } from "@noriginmedia/norigin-spatial-navigation-react";
import { Scene } from "./Scene";
import { Logo } from "./Logo";
import { appInfo, toggleFullscreen } from "./native";

export const TABS = ["Início", "Vídeos", "Músicas", "Imagens", "Dispositivos"] as const;
export type Tab = (typeof TABS)[number];

function TabButton({
  label,
  active,
  onSelect,
}: {
  label: Tab;
  active: boolean;
  onSelect: () => void;
}) {
  const { ref, focused } = useFocusable({ onEnterPress: onSelect, onFocus: onSelect });
  return (
    <button
      ref={ref}
      type="button"
      role="tab"
      aria-selected={active}
      className={`tab${active ? " glass solid" : ""}${focused ? " is-focused" : ""}`}
      onClick={onSelect}
    >
      {label}
    </button>
  );
}

function NavBar({ active, onSelect }: { active: Tab; onSelect: (t: Tab) => void }) {
  const { ref, focusKey, focusSelf } = useFocusable({
    focusKey: "NAV",
    saveLastFocusedChild: true,
    trackChildren: true,
  });

  useEffect(() => {
    focusSelf();
  }, [focusSelf]);

  return (
    <FocusContext.Provider value={focusKey}>
      <nav ref={ref} className="navbar glass" role="tablist" aria-label="Seções">
        {TABS.map((t) => (
          <TabButton key={t} label={t} active={t === active} onSelect={() => onSelect(t)} />
        ))}
      </nav>
    </FocusContext.Provider>
  );
}

function EmptyState({ tab }: { tab: Tab }) {
  const { ref, focused } = useFocusable();
  return (
    <section className="panel glass" aria-live="polite">
      <p className="overline">{tab}</p>
      <h1 className="headline">
        Nada por aqui <strong>ainda.</strong>
      </h1>
      <p className="lead">
        Em breve: adicione uma pasta de vídeos, músicas ou imagens e assista daqui, com o controle.
      </p>
      <button ref={ref} type="button" className={`cta glass solid${focused ? " is-focused" : ""}`}>
        Adicionar pasta
      </button>
    </section>
  );
}

export function App() {
  const [tab, setTab] = useState<Tab>("Início");
  const [label, setLabel] = useState("homedeck");

  useEffect(() => {
    appInfo().then(setLabel, () => undefined);
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "F11") {
        e.preventDefault();
        void toggleFullscreen();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  return (
    <div className="app">
      <Scene />
      <header className="topbar">
        <Logo className="logo" />
        <NavBar active={tab} onSelect={setTab} />
      </header>
      <main className="content">
        <EmptyState tab={tab} />
      </main>
      <footer className="footer">
        <span>{label}</span>
        <span>Setas para navegar · Enter para escolher · F11 tela cheia</span>
      </footer>
    </div>
  );
}
