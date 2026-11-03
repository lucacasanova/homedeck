---
name: announce
description: Escreve e publica anúncios do homedeck na categoria Announcements do GitHub Discussions — milestone concluído, release publicada, decisão de arquitetura importante, mudança que afeta usuários, chamada de testadores (ex. POC na TV) ou mudança na política de privacidade/telemetria. Use quando o usuário pedir "anunciar", "post de novidade", "comunicar a comunidade", ou quando as skills sync/release detectarem um desses eventos.
---

# Anúncios

## Quando anunciar
| Evento | Gatilho |
|---|---|
| Milestone concluído | skill `sync` detecta milestone fechado |
| Release | skill `release` depois que a tag foi publicada |
| ADR aceito que muda a direção do projeto | merge de ADR com status `aceito` |
| Chamada de testadores | POC precisa de dispositivo real (TV Samsung, Android TV...) |
| Privacidade/telemetria | qualquer mudança em `PRIVACY.md` ou `docs/privacy/TELEMETRY.md` (obrigatório) |
| Mudança que quebra algo | protocolo novo, migração de dados, plataforma descontinuada |

Não anuncie tarefas internas, refatorações ou issues individuais.

## Como escrever
- PT-BR, linguagem simples, pra usuário final, não pra dev.
- Título curto com emoji: `🎉 M1 concluído: player no Linux`, `🚀 homedeck 0.2.0`, `🙋 Procuramos testadores com TV Samsung`.
- Estrutura:
  1. Uma frase dizendo o que aconteceu.
  2. **O que muda pra você** (bullets).
  3. Links: release, milestone, ADR ou issues.
  4. **Próximos passos**: o próximo milestone.
  5. Como participar: Discussions, issue ou `good first issue`.
- Nunca inclua dados de telemetria, números brutos de uso ou nada do repo privado `homedeck-ops`.

## Fluxo
1. Escreva o rascunho e mostre ao usuário.
2. Publique quando o usuário aprovar, ou direto se ele já tiver pedido a publicação explicitamente nesta tarefa:

```bash
Q=$(gh api graphql -f query='{repository(owner:"lucacasanova",name:"homedeck"){id discussionCategories(first:10){nodes{id name}}}}')
RID=$(jq -r .data.repository.id <<<"$Q")
CID=$(jq -r '.data.repository.discussionCategories.nodes[]|select(.name=="Announcements")|.id' <<<"$Q")
gh api graphql \
  -f query='mutation($r:ID!,$c:ID!,$t:String!,$b:String!){createDiscussion(input:{repositoryId:$r,categoryId:$c,title:$t,body:$b}){discussion{url}}}' \
  -f r="$RID" -f c="$CID" -f t="<título>" -f b="$(cat <arquivo-do-rascunho>)" \
  --jq .data.createDiscussion.discussion.url
```

3. Entregue a URL ao usuário. Se for release ou milestone, adicione o link do anúncio nas notas da release.
4. Fixar um anúncio (no máximo 4 fixados) só existe na interface do GitHub: use o navegador (Playwright) se o usuário pedir.
