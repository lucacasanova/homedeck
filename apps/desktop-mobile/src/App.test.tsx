import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { init } from "@noriginmedia/norigin-spatial-navigation-core";
import { App, TABS } from "./App";

init({});

describe("App shell", () => {
  it("renders every section tab, with Início selected", () => {
    render(<App />);
    for (const t of TABS) expect(screen.getByRole("tab", { name: t })).toBeTruthy();
    expect(screen.getByRole("tab", { name: "Início" }).getAttribute("aria-selected")).toBe("true");
  });

  it("falls back to the web label outside Tauri", async () => {
    render(<App />);
    expect(await screen.findByText("homedeck (web)")).toBeTruthy();
  });
});
