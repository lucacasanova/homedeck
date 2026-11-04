# ADR 0010 — Inicialização resiliente, modos de compatibilidade e relatório de falha

- **Status:** aceito (pedido do mantenedor)
- **Data:** 2026-11-03
- **Issues:** #158, #128

## Contexto
O AppImage do CI quebrou com `SIGSEGV` no Manjaro (Wayland, Intel + NVIDIA) dentro do driver EGL Wayland da NVIDIA (#158). Esse tipo de falha:
- acontece **antes** da interface existir;
- mata o processo inteiro, então não dá pra capturar de dentro;
- depende da combinação de distro, sessão (Wayland ou X11), GPU e driver.

O homedeck precisa abrir em qualquer combinação razoável de sistema e driver. Quando não abrir, precisa deixar um diagnóstico útil, respeitando a privacidade (ADR 0008).

## Decisão

### Supervisor (desktop Linux)
1. O binário roda primeiro como **supervisor**, que é leve, não usa WebKit nem GPU, e inicia a interface como **processo filho**.
2. O filho sinaliza "pronto" quando a página carrega, imprimindo um **token aleatório de uso único** na própria saída padrão. Só o supervisor lê esse pipe, então nenhum outro processo consegue falsificar o sinal. Não há arquivo marcador em `/tmp`.
3. Se o filho morrer **antes de ficar pronto**:
   - o supervisor grava um relatório;
   - tenta o próximo **modo de compatibilidade**.
4. O modo que funcionar fica salvo e passa a ser o primeiro tentado nas próximas aberturas.

| Modo | Variáveis |
|---|---|
| `default` | nenhuma |
| `no-dmabuf` | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
| `mesa-egl` | `__EGL_VENDOR_LIBRARY_FILENAMES=<50_mesa.json>` (força Intel/AMD via Mesa em vez da NVIDIA) |
| `x11` | `GDK_BACKEND=x11` + `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
| `software` | `LIBGL_ALWAYS_SOFTWARE=1` + `WEBKIT_DISABLE_COMPOSITING_MODE=1` + `WEBKIT_DISABLE_DMABUF_RENDERER=1` |

### Relatório local (sempre, sem rede)
Fica em `~/.local/share/io.github.lucacasanova.homedeck/crash/<data>.txt` e contém:
- versão do app e formato do pacote (AppImage ou não);
- distro e kernel;
- sessão e desktop;
- GPUs e versão do driver NVIDIA;
- vendors EGL disponíveis;
- modo tentado e sinal ou código de saída;
- as últimas linhas da saída de erro;
- o fim do log do app.

Antes de gravar, o relatório é **redigido**:
- a pasta pessoal (inclusive o caminho real, se for um symlink) vira `~`;
- o nome de usuário e o hostname viram `<usuário>` e `<host>`;
- `/run/user/<uid>` é anonimizado;
- caminhos pessoais (`/home`, `/var/home`, `/media`, `/run/media`, `/mnt`, `~/`) viram `<caminho>`, mantendo só a extensão, que ajuda a diagnosticar codec.

Caminhos de sistema, como `/usr/lib/...`, são mantidos.

Arquivos com permissão `0600`, em pastas `0700`. Sem `HOME`, nada é gravado (nunca cai em `/tmp`). O marcador `pending` guarda só o nome do arquivo.

### Consentimento
- **Algum modo funcionou:** o app abre e mostra que houve falha na última abertura, oferecendo ver ou enviar o relatório (#128).
- **Nenhum modo funcionou:** o supervisor mostra uma **caixa de diálogo nativa GTK**, que não usa WebKit nem GPU, com as opções:
  - **Abrir pasta**;
  - **Reportar no GitHub**: abre a pasta e uma issue nova **sem nenhum dado na URL**, com instruções pra anexar o arquivo. O usuário escolhe o que envia;
  - **Fechar**.
- Nada é enviado sem um clique explícito.

### Linha de comando
- `homedeck --diagnostics`: imprime o relatório do ambiente.
- `homedeck --compat <modo>`: força um modo.
- `homedeck --no-guard`: roda sem o supervisor.
- Teste de recuperação: `HOMEDECK_TEST_CRASH_MODES`, que **só existe** em builds com `--features test-hooks` e nunca em releases.

### Sinais
SIGTERM, SIGINT e SIGHUP são repassados ao filho (via `nix`). Depois de 5 s, o filho é encerrado à força. A flag de processo filho vai por argumento, não por variável de ambiente, então programas abertos pelo app não a herdam.

### Causa raiz do caso NVIDIA (#158)
O `linuxdeploy` embute a `libwayland` do Ubuntu do CI no AppImage, e ela é incompatível com o `egl-wayland` do driver NVIDIA do sistema. O build define `LINUXDEPLOY_EXCLUDED_LIBRARIES="libwayland-*"`, que o Tauri repassa ao `linuxdeploy`, e o `scripts/check-appimage.sh` falha o CI se alguma `libwayland` voltar a ser embutida. A `libwayland` é estável e o sistema sempre tem uma, porque o GTK3 depende dela. Testado no Manjaro com Intel + NVIDIA: o AppImage abre direto no modo `default`, tanto em Wayland quanto em X11.

### Janela que não carrega
Interface que não sinaliza "pronto" em **20 s** conta como falha: o filho é encerrado e o próximo modo é tentado. Isso cobre o caso de janela aberta, mas cinza e vazia, depois de uma falha de WebKit/EGL.

## Alternativas consideradas
- **Só documentar variáveis de ambiente:** o usuário comum não chega até elas.
- **Script `AppRun` no AppImage:** cobre só o AppImage, não o `.deb` nem o AUR.
- **Capturar o crash de dentro (sinal ou minidump):** possível depois com crashpad/minidump, mas não resolve a retomada automática.

## Consequências
- Um processo a mais em execução, com custo desprezível.
- Falhas de driver viram uma inicialização um pouco mais lenta, em vez de app quebrado.
- Windows e macOS entram no supervisor quando essas plataformas chegarem (M8).
