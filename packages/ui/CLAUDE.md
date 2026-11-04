# packages/ui — UI compartilhada (React)

Leia antes o `CLAUDE.md` da raiz e `docs/design/GUIA-DE-MARCA.md` (fonte da verdade da marca).

## Papel
Tokens de design e componentes usados pelo app desktop/mobile e, depois, pelo app Tizen.

## Regras
- **Tokens em dois lugares sincronizados:** `src/tokens.ts` e `src/tokens.css`. O teste `tokens.test.ts` garante que batem e que **não existe cor de destaque**. Se mudar um, mude o outro.
- **Monocromático:** hierarquia se faz com opacidade do branco, nunca com cor nova.
- **Navegável por controle:** componente interativo usa `useFocusable` (Norigin) e mostra o foco com `.is-focused` (outline branco, brilho e escala 1.03).
- **Modo leve:** vidro com `backdrop-filter` precisa funcionar também na classe `.light` (TV com GPU fraca), com fundo `--glass-light`.
- **Sem dependência de Tauri:** este pacote roda em qualquer navegador, inclusive na WebView da Tizen.
- **Leve:** cada KB pesa na TV. Prefira CSS a bibliotecas e não traga framework de CSS.

## Testar
```bash
pnpm --filter @homedeck/ui test
pnpm --filter @homedeck/ui typecheck
```
