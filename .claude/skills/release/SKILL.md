---
name: release
description: Prepara e publica uma release do homedeck seguindo o ADR 0007 (SemVer único, release-please, canais nightly/beta/stable). Use quando o usuário pedir "lançar versão", "release", "publicar", "gerar beta".
---

# Release

Siga o ADR 0007 (`docs/adr/0007-versionamento-e-releases.md`).

1. **Pré-checagem**
   - `main` com CI verde: `gh run list --branch main --limit 3`.
   - Nenhuma issue `priority:p0` aberta no milestone que a release fecha.
   - Nenhum advisory de segurança aberto sem correção.
2. **Versão:** o release-please abre o PR "chore(main): release X.Y.Z". Confira:
   - o número segue os commits (`fix` → patch, `feat` → minor; major só depois da 1.0);
   - o `CHANGELOG.md` está legível pra usuário final. Ajuste o texto no próprio PR se precisar.
3. **Beta:** pra testar antes, peça ao usuário pra criar a tag `vX.Y.Z-beta.N` (pré-release).
4. **Publicar:** o merge do PR de release é feito pelo **usuário**. Nunca faça você mesmo.
5. **Depois do merge**, confira na release do GitHub:
   - artefatos de todas as plataformas do MVP (AppImage, APK, `.wgt`);
   - `SHA256SUMS` + assinatura + attestation.
   - AUR: atualize o `pkgver` do pacote `homedeck-bin`.
6. Feche o milestone se a release completa ele, e rode a skill `sync`.
7. Use a skill `announce` pra publicar a novidade no Discussions e linke o anúncio nas notas da release.
