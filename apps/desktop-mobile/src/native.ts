/** Thin wrappers over Tauri so the UI also runs in a plain browser (dev, tests, Tizen later). */
const inTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function appInfo(): Promise<string> {
  if (!inTauri()) return "homedeck (web)";
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("app_info");
}

export async function toggleFullscreen(): Promise<void> {
  if (!inTauri()) return;
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  const win = getCurrentWindow();
  await win.setFullscreen(!(await win.isFullscreen()));
}
