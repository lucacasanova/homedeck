import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { init } from "@noriginmedia/norigin-spatial-navigation-core";
import "@fontsource/outfit/300.css";
import "@fontsource/outfit/400.css";
import "@fontsource/outfit/500.css";
import "@fontsource/outfit/600.css";
import "@homedeck/ui/tokens.css";
import "./styles.css";
import { App } from "./App";

// Remote control, gamepad D-pad and keyboard arrows all drive focus.
init({ shouldFocusDOMNode: true });

const root = document.getElementById("root");
if (root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
