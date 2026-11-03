/**
 * Design tokens from docs/design/GUIA-DE-MARCA.md (source of truth).
 * Monochrome only: hierarchy comes from white opacity, never from colour.
 */
export const color = {
  bg: "#0B0B0D",
  bgDeep: "#08080A",
  surface: "#16161A",
  surface2: "#1A1A1E",
  ink: "#F4F4F5",
  ink2: "rgba(244,244,245,.62)",
  ink3: "rgba(244,244,245,.38)",
  accent: "#FFFFFF",
  onAccent: "#0C0C0E",
  /** Glass fallback for weak TV GPUs (light mode). */
  glassLight: "rgba(30,30,34,.85)",
} as const;

export const radius = {
  card: "20px",
  window: "28px",
  pill: "999px",
  node: "50%",
} as const;

export const blur = {
  chip: "14px",
  card: "18px",
  window: "34px",
} as const;

export const font = {
  family: "Outfit, Inter, system-ui, sans-serif",
  weight: { light: 300, regular: 400, medium: 500, semibold: 600 },
  /** Minimum body size on a 1080p TV viewed from the sofa. */
  tvMinSizePx: 24,
} as const;

export const focus = {
  outline: "2.5px solid #fff",
  outlineOffset: "3px",
  glow: "0 0 40px rgba(255,255,255,.28)",
  scale: 1.03,
} as const;
