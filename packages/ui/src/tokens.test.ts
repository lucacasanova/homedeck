import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { color, radius } from "./tokens";

const css = readFileSync(new URL("./tokens.css", import.meta.url), "utf8");

/** "rgba(244, 244, 245, 0.62)" and "rgba(244,244,245,.62)" compare equal. */
const normalize = (v: string): string =>
  v
    .toLowerCase()
    .replace(/\s+/g, "")
    .replace(/(^|[^0-9])\.(\d)/g, "$10.$2");

function cssVar(name: string): string | undefined {
  const match = new RegExp(`--${name}:\\s*([^;]+);`).exec(css);
  return match?.[1] === undefined ? undefined : normalize(match[1]);
}

function channels(hex: string): number[] {
  return [1, 3, 5].map((i) => Number.parseInt(hex.slice(i, i + 2), 16));
}

describe("design tokens", () => {
  it("palette is monochrome (no accent colour)", () => {
    for (const hex of [
      color.bg,
      color.bgDeep,
      color.surface,
      color.surface2,
      color.ink,
      color.accent,
    ]) {
      const [r = 0, g = 0, b = 0] = channels(hex);
      expect(Math.max(r, g, b) - Math.min(r, g, b)).toBeLessThanOrEqual(4);
    }
  });

  it("tokens.css matches tokens.ts", () => {
    expect(cssVar("bg")).toBe(normalize(color.bg));
    expect(cssVar("surface")).toBe(normalize(color.surface));
    expect(cssVar("ink")).toBe(normalize(color.ink));
    expect(cssVar("ink-2")).toBe(normalize(color.ink2));
    expect(cssVar("ink-3")).toBe(normalize(color.ink3));
    expect(cssVar("accent")).toBe(normalize(color.accent));
    expect(cssVar("glass-light")).toBe(normalize(color.glassLight));
    expect(cssVar("radius-card")).toBe(radius.card);
  });
});
