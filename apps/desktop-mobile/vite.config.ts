/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env["TAURI_DEV_HOST"];

// https://v2.tauri.app/start/frontend/vite/
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host ?? false,
    ...(host ? { hmr: { protocol: "ws", host, port: 1421 } } : {}),
    watch: { ignored: ["**/src-tauri/**"] },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    // Conservative target so the same bundle can later run on older TV webviews.
    target: "es2020",
    sourcemap: !!process.env["TAURI_ENV_DEBUG"],
  },
  test: {
    environment: "jsdom",
  },
});
