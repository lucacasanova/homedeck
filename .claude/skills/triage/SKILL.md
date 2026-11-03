---
name: triage
description: Faz a triagem de issues novas do homedeck — labels, milestone, épico, board e duplicatas. Use quando o usuário pedir "triagem", "organizar issues", "tem issue nova", ou quando surgir uma ideia/bug que precisa virar issue.
---

# Triagem

1. Liste as issues sem triagem: `gh issue list --search "no:milestone is:open"` e as sem `type:*`.
2. Pra cada uma:
   - **Duplicata?** `gh search issues --repo lucacasanova/homedeck "<termos>"`. Se for, comente o link e feche com `--reason "not planned"`.
   - **Labels:** um `type:*`, os `area:*` e `platform:*` aplicáveis e um `priority:p0–p2`.
   - **Escopo:** é do MVP (M0–M5)? Se não for, label `pós-mvp` e o milestone M6–M11 certo. **Nunca feche uma ideia por estar fora do MVP.**
   - **Viola política?** Jogos não oficiais, contorno de DRM: comente citando o ADR 0006 e feche como `not planned`.
   - **Épico:** vincule como sub-issue (`gh api -X POST repos/lucacasanova/homedeck/issues/<épico>/sub_issues -F sub_issue_id=<id>`).
   - **Board:** `gh project item-add 2 --owner lucacasanova --url <url>`. Status Backlog, ou "Pronto pra começar" se for p0 do milestone atual sem bloqueio.
   - **Pronto quando:** se faltar critério claro, proponha um em comentário.
   - **Pra iniciantes:** se for pequena e isolada, adicione `good first issue`.
3. Responda com uma tabela: issue, labels, milestone e ação tomada.
