/** Thin wrappers over Tauri so the UI also runs in a plain browser (dev, tests, Tizen later). */
const inTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function appInfo(): Promise<string> {
  if (!inTauri()) return "homedeck (web)";
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("app_info");
}

/**
 * Sends uncaught UI errors to the native rolling log file, so a report from a
 * user's machine includes what broke in the interface. No-op in a browser.
 */
export async function forwardErrorsToLog(): Promise<void> {
  if (!inTauri()) return;
  const { error } = await import("@tauri-apps/plugin-log");
  window.addEventListener("error", (e) => {
    void error(`[ui] ${e.message} at ${e.filename}:${e.lineno}:${e.colno}`);
  });
  window.addEventListener("unhandledrejection", (e) => {
    void error(`[ui] unhandled rejection: ${String(e.reason)}`);
  });
}

export async function toggleFullscreen(): Promise<void> {
  if (!inTauri()) return;
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  const win = getCurrentWindow();
  await win.setFullscreen(!(await win.isFullscreen()));
}
