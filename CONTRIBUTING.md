# Como contribuir

Obrigado pelo interesse! Este guia mostra o caminho de uma ideia até o merge.

## Antes de começar

- Leia o [Código de Conduta](CODE_OF_CONDUCT.md).
- Dúvida → [Discussions](https://github.com/lucacasanova/homedeck/discussions). Falha de segurança → [SECURITY.md](SECURITY.md).
- Veja a [visão](docs/planning/VISION.md), a [arquitetura](docs/planning/ARCHITECTURE.md) e o [design](docs/design/DESIGN.md).

## Ciclo de uma issue

```
Discussão (opcional) → Issue → Backlog → Pronto pra começar → Em andamento → Em revisão → Feito
```

- O [board](https://github.com/users/lucacasanova/projects/2) mostra o estado de tudo.
- Pra começar, prefira issues com [`good first issue`](https://github.com/lucacasanova/homedeck/labels/good%20first%20issue) ou [`help wanted`](https://github.com/lucacasanova/homedeck/labels/help%20wanted), em "Pronto pra começar".
- **Comente na issue antes de começar**, pra evitar duas pessoas no mesmo trabalho.
- Issues `pós-mvp` são bem-vindas pra discussão, mas o código delas só entra depois do MVP, salvo combinado com o mantenedor.

## Fluxo de código

1. Faça um fork e crie uma branch `tipo/N-descricao` (ex.: `feat/22-music-queue`).
2. Use commits em [Conventional Commits](https://www.conventionalcommits.org/pt-br/), em inglês: `feat(player): add shuffle`.
3. **Assine cada commit** com `git commit -s` ([DCO](https://developercertificate.org/)). Isso adiciona `Signed-off-by` e declara que você tem direito de contribuir aquele código.
4. Abra o PR com o template, título em Conventional Commits e `Closes #N`.
5. O CI precisa passar e um mantenedor revisa. O merge é por squash.

## Regras do projeto

- UI precisa funcionar só com setas + OK + voltar.
- Nunca apagar arquivo do usuário: usar quarentena.
- Endpoint novo exige autenticação e entra no `ARCHITECTURE.md`.
- Decisão de arquitetura → ADR em `docs/adr/`.
- **Somente jogos originais de lojas oficiais** ([ADR 0006](docs/adr/0006-somente-jogos-originais.md)). PRs com ROMs, cracks ou contorno de DRM são recusados.

## Contribuições com IA

Ferramentas de IA (Claude Code, Copilot etc.) são permitidas:

- **Você é responsável** pelo código: precisa entender, testar e revisar tudo o que enviar.
- Indique no PR que usou IA, e use `Co-Authored-By` quando a ferramenta oferecer.
- Este repositório inclui configuração pro Claude Code (`CLAUDE.md`, `.claude/skills`, `.claude/agents`). Usá-la ajuda a seguir o processo automaticamente.

## Licença

Ao contribuir, você concorda que seu código será licenciado sob [GPL-3.0-or-later](LICENSE).
