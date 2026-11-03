# Riscos e POCs

Riscos que podem mudar a arquitetura. Validar **antes** de investir na fase correspondente.

| ID | Risco | Impacto | Como validar | Plano B |
|---|---|---|---|---|
| R1 | App web Tizen sem UDP/multicast → sem mDNS | TV não acha os nós sozinha | POC: tentar descoberta na TV real | Varredura HTTP da sub-rede numa porta fixa; código de pareamento manual; app Tizen .NET |
| R2 | WebView Tizen/iOS rejeita TLS autoassinado | Sem conexão segura | POC: `fetch` HTTPS com cert próprio | Criptografia na camada de app sobre HTTP; CA local instalada no pareamento |
| R3 | iOS não serve arquivos em segundo plano | iPhone não funciona como servidor | Documentado pela Apple | iOS só como cliente/controle |
| R4 | Gamepad API instável na Tizen (por modelo/ano) | Controle Xbox não funciona na TV | POC na TV real com controle pareado | Usar só o controle remoto da TV na UI; game streaming via Moonlight Tizen |
| R5 | Codecs que a TV não toca (HEVC 10-bit, AV1, legendas embutidas) | Vídeo não abre | Matriz de testes por modelo | Transcode HLS no nó de origem |
| R6 | Tauri 2 mobile/Android TV imaturo | Retrabalho no casco | Build hello-world com D-pad | Casco nativo Kotlin com WebView |
| R7 | Certificação Samsung demorada ou recusada | Atraso no lançamento na TV | Ler os requisitos cedo | Distribuir `.wgt` pra modo desenvolvedor |
| R8 | Anti-cheat bloqueia controle virtual | Alguns jogos não funcionam | Testar títulos populares | Documentar a lista de incompatíveis |

## Custos conhecidos

| Item | Custo |
|---|---|
| Apple Developer | US$ 99/ano (precisa de Mac pra compilar) |
| Google Play | US$ 25 (uma vez) |
| Samsung Seller Office | gratuito |
