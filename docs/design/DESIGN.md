# Design

Interface de console (10-foot UI) com painéis de **vidro translúcido** por cima de um **papel de parede escolhido em cada dispositivo**.

## Direção visual

- **Fundo:** imagem em tela cheia, com desfoque e escurecimento leves, escolhida por dispositivo. Opções: imagem local, cor sólida, gradiente ou "dinâmico" (capa/miniatura do item em foco, com transição suave).
- **Vidro:** painéis e cards semitransparentes com `backdrop-filter: blur()`, borda de 1 px clara e sombra suave. O conteúdo de trás aparece sem atrapalhar a leitura.
- **Cores com tinta:** cada categoria tem um tom próprio, aplicado como tinta translúcida no vidro (vídeos, músicas, imagens, dispositivos).
- **Destaque:** um único verde-limão pra ação principal, foco e estado ativo.
- **Formas:** cantos bem arredondados, navegação em pílula, avatares redondos pra dispositivos.
- **Tipografia:** sans geométrica, títulos grandes e leves, números em destaque.
- **Feedback:** toasts em pílula na parte de baixo, com ação de desfazer.

## Tokens iniciais

| Token | Valor |
|---|---|
| `--bg-overlay` | `rgba(10, 12, 14, 0.45)` sobre o papel de parede |
| `--glass` | `rgba(255, 255, 255, 0.08)` |
| `--glass-strong` | `rgba(255, 255, 255, 0.14)` |
| `--glass-border` | `rgba(255, 255, 255, 0.12)` |
| `--blur` | `24px` (painéis), `12px` (cards) |
| `--accent` | `#d4f75a` (verde-limão) |
| `--on-accent` | `#141a05` |
| `--tint-video` | `rgba(90, 130, 170, 0.35)` (azul) |
| `--tint-music` | `rgba(140, 160, 60, 0.35)` (oliva) |
| `--tint-image` | `rgba(170, 80, 60, 0.35)` (terracota) |
| `--tint-device` | `rgba(160, 160, 160, 0.30)` (cinza) |
| `--radius-lg` | `28px` (painéis) |
| `--radius-md` | `18px` (cards) |
| `--radius-pill` | `999px` |
| `--text` | `rgba(255, 255, 255, 0.92)` |
| `--text-dim` | `rgba(255, 255, 255, 0.60)` |

Os valores finais saem da issue de design system e podem mudar.

## Layout

```
┌─────────────────────────────────────────────────────────────┐
│ ◉ homedeck     ( Início  Vídeos  Músicas  Imagens  Dispositivos )   ⚙ │
│                                                             │
│  Continuar assistindo                                       │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│  │ vidro +  │ │          │ │          │ │          │        │
│  │ miniatura│ │          │ │          │ │          │        │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │
│  Neste dispositivo · Na rede (PC-sala, TV-quarto)           │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐                     │
│  └──────────┘ └──────────┘ └──────────┘                     │
│                    ( ✓ Tocando na TV-sala   Desfazer )      │
└─────────────────────────────────────────────────────────────┘
          papel de parede do dispositivo ao fundo
```

## Foco (controle remoto e gamepad)

- O item em foco cresce cerca de 6%, ganha borda `--accent` de 3 px e um brilho externo.
- O foco nunca some. Ao trocar de seção, ele vai pro último item visitado.
- Contraste mínimo de 4.5:1 do texto sobre o vidro. Se o papel de parede for claro demais, o `--bg-overlay` escurece automaticamente.

## Desempenho

`backdrop-filter` é caro em TV e em hardware fraco. Existem dois modos:

| Modo | Quando | Como |
|---|---|---|
| Completo | PC, celular recente | blur real |
| Leve | Tizen, Android TV, ou ativado pelo usuário | sem blur; vidro = cor translúcida mais opaca sobre o papel de parede já pré-desfocado (gerado uma vez e guardado em cache) |

O app detecta a plataforma e escolhe o padrão. O usuário pode trocar nas configurações.

## Referências

Inspiração: dashboards com glassmorphism escuro, cards coloridos translúcidos e navegação em pílula. Nada de copiar marcas ou produtos específicos.
