# Checklist de auditoria / PR — docs/

Antes de executar mudanças
- [ ] Revisar `docs/AUDIT_REPORT.md` e confirmar as ações listadas.
- [ ] Validar licença de `docs/design/assets/*` (Azonix otf, logo). Registrar licença ou remover/hostear externamente.
- [ ] Fazer backup do branch atual e criar branch de trabalho: `git checkout -b docs/reorg/audit`.
- [ ] Rodar o script sugerido localmente: `bash docs/AUDIT_COMMANDS.sh` (revisar saídas).
- [ ] Rodar verificador de links: `npx markdown-link-check "docs/**/*.md" -c .mlcconfig` ou `lychee`.
- [ ] Sincronizar `docs/design/tokens.json` com qualquer CSS gerado. Preferir gerar CSS a partir do JSON.

Checklist para o PR (preencher antes de pedir revisão)
- [ ] PR focado em uma área (ex.: `design/`, `ai/`, `frontend/`).
- [ ] Todos os `git mv` revisados e sem sobrescrita inesperada.
- [ ] Verificação automática de links passou.
- [ ] Arquivos binários documentados (licença, origem) e, se necessário, movidos para LFS.
- [ ] `docs/index.md` e links locais atualizados conforme os moves.
- [ ] Documentar no PR o porquê de cada mudança e criar referência cruzada com ADRs quando aplicável.
- [ ] Incluir no commit message o trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`

Lista inicial de links que provavelmente precisam ser atualizados
- `docs/index.md` -> checar referências para: `./ai/index.md`, `./database/index.md`, `./agents/index.md`, `./frontend/web/index.md`, `./skills/index.md`.
- Checar todos os arquivos que mencionam `docs/design/azonix/` ou `docs/design/assets/logo.png` e ajustar para `docs/design/assets/fonts/` e `docs/design/assets/` respectivamente.

Notas sobre conformidade com ui-ux-pro-max
- Tokens: `docs/design/tokens.json` existe — deve ser a fonte de verdade. Gerar CSS a partir dele e usar variáveis CSS no conteúdo de exemplo.
- Paleta: `docs/design/palette.md` e `docs/design/tokens.md` seguem a paleta proposta (Obsidian Black / Electric Volt). Confirmar contraste WCAG AA para textos/CTAs.
- Fontes: garantir fallback em `font-family` e confirmar licenciamento de `Azonix` antes de inclusão.
- Componentes: exemplos em `docs/frontend/ui/examples/` devem usar tokens (variáveis) e classes Tailwind que referenciam tokens.

Sugestão de tools para CI
- Lychee (link checker) — https://github.com/lycheeverse/lychee
- markdown-link-check — https://www.npmjs.com/package/markdown-link-check
- script de geração (python/node) para converter `tokens.json` -> `docs/design/generated/ui-styles.css`

Última recomendação
- Fazer PRs pequenos, por domínio. Revisões manuais são essenciais (especialmente para merges de `index.md` x `README.md`).
