# crates/core — lógica do nó (Rust)

Leia antes o `CLAUDE.md` da raiz.

## Papel
Tudo que não é UI: biblioteca, metadados, banco local, descoberta, pareamento, contas, streaming e protocolo. O casco Tauri (`apps/desktop-mobile/src-tauri`) só expõe comandos que chamam este crate.

## Regras
- **Sem Tauri aqui:** o core precisa compilar e ser testado sem webview, porque vai rodar também como nó sem UI. Dependência de Tauri fica no casco.
- **`unsafe` proibido** (`unsafe_code = "deny"` no workspace). Exceção só com ADR.
- **Erros tipados:** nada de `unwrap()` ou `expect()` fora de teste (clippy avisa). Erros de rede ou arquivo viram variantes de erro, não `panic`.
- **Segurança (ADR 0004 e 0005):** todo acesso a mídia passa por pareamento **e** login. Caminho vindo da rede é normalizado e confinado às pastas da biblioteca (sem `..` e sem symlink saindo delas).
- **Nunca apagar arquivo do usuário:** remoção vai pra quarentena.
- **Operação longa** (scan, hash, transcode) é cancelável e não bloqueia a thread de quem chamou.
- **Telemetria (ADR 0008):** só eventos do catálogo `docs/privacy/TELEMETRY.md`.
- **Dependência nova:** licença compatível com GPL e crates.io apenas (`deny.toml`); `cargo deny check` precisa passar.

## Testar
```bash
cargo test -p homedeck-core
cargo clippy -p homedeck-core --all-targets -- -D warnings
```
