# ADR 0011 — Atualização automática por plataforma

- **Status:** aceito (pedido do mantenedor)
- **Data:** 2026-11-03
- **Issues:** #149, #150, #151, #152

## Contexto
O mantenedor quer que o app **baixe as atualizações sozinho ao abrir**, recorrendo à loja só quando for necessário. Cada plataforma tem regras próprias de instalação, assinatura e permissão. Uma atualização mal verificada é um vetor de ataque grave: quem controla o update controla a máquina.

## Decisão

### Regras gerais
1. **Toda atualização é assinada.** O app recusa e registra no log qualquer pacote com assinatura inválida. A chave privada fica **só** no environment `release` do GitHub, que exige aprovação do mantenedor (SECURITY.md, #115).
2. **Nada trava a UI.** A verificação roda em segundo plano depois que a interface abre. O download só começa com rede disponível e o app nunca reinicia sozinho no meio de uma reprodução.
3. **Canais:** `stable` (padrão) e `beta`, seguindo o ADR 0007. O usuário escolhe nas configurações.
4. **Pode ser desligada** nas configurações e por `HOMEDECK_UPDATES=0`. A verificação é uma conexão com o GitHub, e o `PRIVACY.md` passa a dizer isso.
5. **Compatibilidade entre aparelhos:** como cada aparelho da casa pode estar numa versão, o protocolo é versionado (`/api/v1`). Uma versão nova conversa com o protocolo atual e o anterior (ADR 0007).

### Por plataforma
| Instalação | Como atualiza |
|---|---|
| **AppImage** | `tauri-plugin-updater`: ao abrir, consulta o `latest.json` da release, baixa em segundo plano, confere a assinatura e mostra um toast em vidro "Atualização pronta · Reiniciar". Se o usuário não reiniciar, aplica na próxima abertura |
| `.deb`, `.rpm`, AUR, Flathub | **Não autoatualiza**: quem atualiza é o gerenciador de pacotes. O app só avisa que há versão nova e mostra o comando |
| **Android (APK do GitHub)** | Baixa o APK assinado e abre o instalador do Android, que exige confirmação do usuário (`REQUEST_INSTALL_PACKAGES`, com explicação) |
| **Android (Play Store)** | API de *in-app updates* da Play: modo flexível por padrão e imediato só para correção crítica |
| **Samsung TV (Tizen)** | Pela Samsung Apps. Em modo desenvolvedor, o app avisa e mostra como reinstalar o `.wgt` |

O app descobre como foi instalado assim:
- `APPIMAGE` definido → AppImage;
- binário em `/usr/bin` → pacote do sistema;
- no Android, `installerPackageName`: Play ou outro.

### Integração com a inicialização resiliente (ADR 0010)
Se a versão nova quebrar ao iniciar em **todos** os modos de compatibilidade e a versão anterior ainda estiver no disco, o supervisor volta pra ela e registra no relatório. Isso só vale pro AppImage, que mantém o arquivo anterior até a nova versão abrir com sucesso.

## Alternativas consideradas
- **Só pelas lojas:** o Linux não tem loja universal, e o APK do GitHub e o modo desenvolvedor da TV ficariam sem atualização.
- **Autoatualizar `.deb` e `.rpm` substituindo o binário:** brigaria com o gerenciador de pacotes e exigiria root.
- **Updater próprio:** o `tauri-plugin-updater` já resolve assinatura e substituição com segurança.

## Consequências
- O pipeline de release precisa gerar o `latest.json` e as assinaturas (#9, #115).
- Uma chave perdida impede updates automáticos para quem já instalou. O backup offline da chave é obrigatório (repo privado, `docs/SECRETS.md`).
- Mais uma conexão de rede, documentada e desligável.
