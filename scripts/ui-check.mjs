#!/usr/bin/env node
// Visual + navigation smoke test of the UI in headless Chrome (no extension needed).
//
//   pnpm ui:check                       build, serve, screenshot, test arrow navigation
//   pnpm ui:check --url http://...      use a running server instead
//   pnpm ui:check --size 1280x720 --keys ArrowRight,ArrowDown
//
// Output goes to .ui-check/ (git-ignored): home.png, after-keys.png and report.json.
import { spawn, execSync } from "node:child_process";
import { mkdirSync, writeFileSync, existsSync } from "node:fs";

const args = Object.fromEntries(
  process.argv
    .slice(2)
    .reduce(
      (acc, a, i, all) => (a.startsWith("--") ? [...acc, [a.slice(2), all[i + 1]]] : acc),
      [],
    ),
);
const [width, height] = (args.size ?? "1920x1080").split("x").map(Number);
const keys = (args.keys ?? "ArrowRight,ArrowRight,ArrowLeft,ArrowDown").split(",");
const outDir = ".ui-check";
mkdirSync(outDir, { recursive: true });

const browser = ["google-chrome-stable", "chromium", "google-chrome"].find((b) => {
  try {
    execSync(`command -v ${b}`, { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
});
if (!browser) {
  console.error("Chrome/Chromium não encontrado (veja pnpm env:check).");
  process.exit(1);
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const children = [];
const cleanup = () => children.forEach((c) => c.kill());
process.on("exit", cleanup);

let url = args.url;
if (!url) {
  if (!existsSync("apps/desktop-mobile/dist/index.html") || args.build !== "false") {
    execSync("pnpm --filter @homedeck/app build", { stdio: "inherit" });
  }
  const preview = spawn(
    "pnpm",
    ["--filter", "@homedeck/app", "exec", "vite", "preview", "--port", "4174", "--strictPort"],
    { stdio: "ignore" },
  );
  children.push(preview);
  url = "http://localhost:4174";
  for (let i = 0; i < 50; i++) {
    try {
      await fetch(url);
      break;
    } catch {
      await sleep(200);
    }
  }
}

const port = 9334;
children.push(
  spawn(
    browser,
    [
      "--headless=new",
      "--disable-gpu",
      "--hide-scrollbars",
      `--remote-debugging-port=${port}`,
      `--window-size=${width},${height}`,
      `--user-data-dir=${outDir}/profile`,
      url,
    ],
    { stdio: "ignore" },
  ),
);

let target;
for (let i = 0; i < 50 && !target; i++) {
  await sleep(200);
  try {
    target = (await (await fetch(`http://127.0.0.1:${port}/json`)).json()).find(
      (t) => t.type === "page",
    );
  } catch {
    /* not ready */
  }
}
const ws = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((r) => ws.addEventListener("open", r));
let id = 0;
const pending = new Map();
ws.addEventListener("message", (e) => {
  const m = JSON.parse(e.data);
  pending.get(m.id)?.(m);
  pending.delete(m.id);
});
const cdp = (method, params = {}) =>
  new Promise((r) => {
    pending.set(++id, r);
    ws.send(JSON.stringify({ id, method, params }));
  });
const evaluate = async (expression) =>
  (await cdp("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true })).result
    ?.result?.value;
const shot = async (file) =>
  writeFileSync(
    `${outDir}/${file}`,
    Buffer.from((await cdp("Page.captureScreenshot", { format: "png" })).result.data, "base64"),
  );

await sleep(1500);
const state = () =>
  evaluate(`({
  selected: document.querySelector('[aria-selected="true"]')?.textContent ?? null,
  focused: document.querySelector('.is-focused')?.textContent ?? null,
  errors: window.__uiErrors ?? []
})`);
await evaluate(
  `window.__uiErrors = []; addEventListener('error', e => window.__uiErrors.push(String(e.message)));`,
);
await shot("home.png");

const steps = [{ key: "(início)", ...(await state()) }];
const codes = { ArrowRight: 39, ArrowLeft: 37, ArrowUp: 38, ArrowDown: 40, Enter: 13 };
for (const key of keys) {
  await evaluate(
    `dispatchEvent(new KeyboardEvent('keydown', { key: '${key}', keyCode: ${codes[key] ?? 0}, bubbles: true }))`,
  );
  await sleep(200);
  steps.push({ key, ...(await state()) });
}
await shot("after-keys.png");

const report = { url, size: `${width}x${height}`, steps };
writeFileSync(`${outDir}/report.json`, JSON.stringify(report, null, 2));
console.table(
  steps.map(({ key, selected, focused }) => ({ tecla: key, aba: selected, foco: focused })),
);
console.log(`\nPrints e relatório em ${outDir}/`);
ws.close();
cleanup();
process.exit(steps.some((s) => s.errors?.length) ? 1 : 0);
