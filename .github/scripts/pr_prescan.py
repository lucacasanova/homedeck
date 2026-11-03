#!/usr/bin/env python3
"""Security pre-scan for pull requests.

Runs from the BASE branch (pull_request_target) and only reads the PR diff
through the GitHub API. It never checks out or executes code from the PR.

Outputs:
  - a sticky PR comment summarising findings
  - labels: external-contribution, sensitive-paths, needs-security-review
  - exit code 1 when a critical finding exists (fails the required check)
"""
import json
import os
import re
import sys
import urllib.request

API = os.environ.get("GITHUB_API_URL", "https://api.github.com")
REPO = os.environ["GITHUB_REPOSITORY"]
TOKEN = os.environ["GITHUB_TOKEN"]
PR = int(os.environ["PR_NUMBER"])
IS_FORK = os.environ.get("PR_IS_FORK") == "true"
MARKER = "<!-- homedeck-prescan -->"


def api(method, path, body=None):
    req = urllib.request.Request(
        f"{API}{path}",
        method=method,
        data=json.dumps(body).encode() if body is not None else None,
        headers={
            "Authorization": f"Bearer {TOKEN}",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    with urllib.request.urlopen(req) as r:
        data = r.read()
        return json.loads(data) if data else None


def pr_files():
    files, page = [], 1
    while True:
        batch = api("GET", f"/repos/{REPO}/pulls/{PR}/files?per_page=100&page={page}")
        files += batch
        if len(batch) < 100 or page >= 30:
            return files
        page += 1


# ---------------------------------------------------------------- rules
HIDDEN_CHARS = re.compile("[%s-%s%s-%s%s-%s%s%s]" % tuple(map(chr, (0x202A, 0x202E, 0x2066, 0x2069, 0x200B, 0x200D, 0x2060, 0xFEFF))))
PIPE_TO_SHELL = re.compile(r"(curl|wget)[^\n|]*\|\s*(sudo\s+)?(ba|z)?sh\b")
LONG_BLOB = re.compile(r"[A-Za-z0-9+/=]{300,}")
URL = re.compile(r"https?://([a-zA-Z0-9.-]+)")
RISKY_API = re.compile(
    r"\b(std::process::Command|Command::new|child_process|execSync|spawnSync|"
    r"\beval\s*\(|new Function\s*\(|unsafe\s*\{|libc::|dlopen|LoadLibrary|"
    r"reqwest::|ureq::|hyper::Client|XMLHttpRequest|navigator\.sendBeacon|"
    r"WebSocket\s*\(|fetch\s*\(|setuid|chmod\s+\+x|base64\s+-d|atob\s*\()"
)
TELEMETRY_API = re.compile(r"\b(aptabase|track_event|trackEvent|telemetry::|sentry|glitchtip)\b", re.I)
INSTALL_HOOKS = re.compile(r'"(preinstall|install|postinstall|prepare|prepublish)"\s*:')

BINARY_EXT = re.compile(r"\.(exe|dll|so|dylib|bin|jar|class|wasm|apk|aab|ipa|deb|rpm|msi|pyc|o|a|node)$", re.I)
SENSITIVE_PATHS = [
    (re.compile(r"^\.github/"), "workflows/configuração do GitHub"),
    (re.compile(r"^\.claude/|(^|/)CLAUDE\.md$"), "configuração/regras do Claude"),
    (re.compile(r"(^|/)CODEOWNERS$|^SECURITY\.md$|^GOVERNANCE\.md$"), "governança/segurança"),
    (re.compile(r"(^|/)build\.rs$|(^|/)Makefile$|\.sh$|(^|/)Dockerfile"), "script de build/execução"),
    (re.compile(r"(^|/)(Cargo\.toml|package\.json|pnpm-workspace\.yaml)$"), "manifesto de dependências"),
    (re.compile(r"(^|/)(Cargo\.lock|pnpm-lock\.yaml|package-lock\.json|yarn\.lock)$"), "lockfile"),
    (re.compile(r"^docs/privacy/|^PRIVACY\.md$"), "privacidade/telemetria"),
    (re.compile(r"^assets/brand/fonte/.*\.(js|py|sh|html)$"), "scripts do kit de marca"),
    (re.compile(r"^release-please-config\.json$|^\.release-please-manifest\.json$"), "configuração de release"),
]
TRUSTED_HOSTS = (
    "github.com", "githubusercontent.com", "shields.io", "img.shields.io", "contrib.rocks",
    "star-history.com", "api.star-history.com", "conventionalcommits.org", "developercertificate.org",
    "claude.com", "anthropic.com", "tauri.app", "rust-lang.org", "docs.rs", "crates.io",
    "npmjs.com", "developer.android.com", "developer.samsung.com", "docs.tizen.org",
    "googleapis.com", "w3.org", "mozilla.org", "example.com", "localhost", "127.0.0.1",
)


def added_lines(patch):
    """Yield (new_line_number, text) for lines added in a unified diff."""
    line = 0
    for raw in (patch or "").splitlines():
        if raw.startswith("@@"):
            m = re.search(r"\+(\d+)", raw)
            line = int(m.group(1)) - 1 if m else 0
        elif raw.startswith("+") and not raw.startswith("+++"):
            line += 1
            yield line, raw[1:]
        elif not raw.startswith("-"):
            line += 1


def scan(files):
    critical, review, sensitive, info = [], [], set(), []
    touched_catalog = any(f["filename"] == "docs/privacy/TELEMETRY.md" for f in files)

    for f in files:
        name, status = f["filename"], f["status"]
        is_asset = name.startswith("assets/")
        is_doc = name.endswith(".md")

        for rx, why in SENSITIVE_PATHS:
            if rx.search(name):
                sensitive.add(why)
                review.append((name, None, f"altera {why}"))

        if BINARY_EXT.search(name) and status != "removed":
            critical.append((name, None, "arquivo binário/executável adicionado"))
        if f.get("changes", 0) > 0 and "patch" not in f and status != "removed" and not is_asset:
            review.append((name, None, "diff não disponível (binário ou muito grande): revisar manualmente"))
        if status == "renamed" and f.get("previous_filename", "").startswith((".github/", ".claude/")):
            review.append((name, None, f"renomeado a partir de {f['previous_filename']}"))

        for ln, text in added_lines(f.get("patch")):
            if HIDDEN_CHARS.search(text):
                critical.append((name, ln, "caractere invisível/bidirecional (possível Trojan Source)"))
            if PIPE_TO_SHELL.search(text):
                critical.append((name, ln, "download executado direto pelo shell (pipe para sh/bash)"))
            if not is_asset and LONG_BLOB.search(text):
                critical.append((name, ln, "bloco longo codificado (base64/ofuscado)"))
            if not is_doc and RISKY_API.search(text):
                review.append((name, ln, f"API sensível: `{RISKY_API.search(text).group(0).strip()}`"))
            if INSTALL_HOOKS.search(text) and name.endswith("package.json"):
                critical.append((name, ln, "script de instalação (pre/postinstall) adicionado"))
            if TELEMETRY_API.search(text) and not is_doc and not touched_catalog:
                review.append((name, ln, "código de telemetria sem atualizar docs/privacy/TELEMETRY.md (ADR 0008)"))
            for host in URL.findall(text):
                h = host.lower()
                if not any(h == t or h.endswith("." + t) for t in TRUSTED_HOSTS):
                    review.append((name, ln, f"domínio externo novo: `{h}`"))

    if IS_FORK:
        info.append("PR vindo de fork (contribuição externa).")
    return critical, review, sorted(sensitive), info


def render(critical, review, sensitive, info, nfiles):
    def rows(items):
        out = []
        seen = set()
        for name, ln, why in items:
            key = (name, ln, why)
            if key in seen:
                continue
            seen.add(key)
            loc = f"`{name}:{ln}`" if ln else f"`{name}`"
            out.append(f"| {loc} | {why} |")
            if len(out) >= 40:
                out.append("| … | lista truncada |")
                break
        return "\n".join(out)

    if critical:
        verdict = "🔴 **Bloqueado:** achados críticos. O check falha até serem removidos ou explicados."
    elif review or sensitive:
        verdict = "🟡 **Revisão de segurança necessária** antes do merge."
    else:
        verdict = "🟢 Nenhum sinal de risco encontrado pelo pré-scan."

    body = [MARKER, "## 🛡️ Pré-scan de segurança", "", verdict, "", f"Arquivos analisados: {nfiles}."]
    if info:
        body += [""] + [f"- {i}" for i in info]
    if critical:
        body += ["", "### Críticos", "| Onde | O quê |", "|---|---|", rows(critical)]
    if sensitive:
        body += ["", "### Áreas sensíveis alteradas", ""] + [f"- {s}" for s in sensitive]
    if review:
        body += ["", "### Pontos para revisar", "| Onde | O quê |", "|---|---|", rows(review)]
    body += [
        "",
        "<sub>Análise estática do diff, sem executar código do PR. Não substitui a revisão humana: "
        "o mantenedor revisa com a skill `review-contribution`. Falso positivo? Explique no PR.</sub>",
    ]
    return "\n".join(body)


def upsert_comment(body):
    comments = api("GET", f"/repos/{REPO}/issues/{PR}/comments?per_page=100")
    mine = [c for c in comments if MARKER in (c.get("body") or "")]
    if mine:
        api("PATCH", f"/repos/{REPO}/issues/comments/{mine[0]['id']}", {"body": body})
    else:
        api("POST", f"/repos/{REPO}/issues/{PR}/comments", {"body": body})


def main():
    files = pr_files()
    critical, review, sensitive, info = scan(files)
    upsert_comment(render(critical, review, sensitive, info, len(files)))

    labels = []
    if IS_FORK:
        labels.append("external-contribution")
    if sensitive:
        labels.append("sensitive-paths")
    if critical or review:
        labels.append("needs-security-review")
    if labels:
        api("POST", f"/repos/{REPO}/issues/{PR}/labels", {"labels": labels})

    print(f"critical={len(critical)} review={len(review)} sensitive={len(sensitive)}")
    for name, ln, why in critical:
        loc = f"file={name},line={ln}" if ln else f"file={name}"
        print(f"::error {loc}::{why}")
    sys.exit(1 if critical else 0)


if __name__ == "__main__":
    main()
