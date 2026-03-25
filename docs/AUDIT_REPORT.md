# Auditoria: pasta docs/ — 2026-03-25

Resumo
- Varredura completa dos arquivos em `docs/` (~40 arquivos).
- Objetivos: garantir relevância do conteúdo, confirmar que não há código-fonte em `docs/`, e checar conformidade de design com a skill `ui-ux-pro-max`.

Escopo & Metodologia
- Foram listados e lidos: índices, ADRs, prompts TOON, tokens (JSON), exemplos de UI, assets (imagens, fontes) e arquivos binários.
- Verificou-se estrutura, convenções de nomes de ADRs, duplicidades e arquivos que aparentam ser código.

Principais achados
1. ADRs
   - ADRs estão organizadas em `docs/architecture/adr/<dominio>/` com prefixos numéricos (ex.: `01-adr-db.md`). Padrão seguido — OK.
2. AI / TOON
   - `docs/ai/prompts.md` documenta TOON e exemplos. Conteúdo apropriado.
3. Tokens & CSS
   - `docs/design/tokens.json` é a fonte canônica de tokens.
   - Existe duplicação: variáveis CSS em `docs/frontend/ui/styles.css` repetem o mesmo conteúdo (risco de dessincronização).
4. Componentes & Exemplos
   - `docs/frontend/ui/examples/*` contém snippets e .md com exemplos — isto é documentação (OK).
5. Assets binários
   - `docs/design/azonix/Azonix.otf` (fonte) e `docs/design/assets/logo.png` (logo) são binários no repositório. Verificar licença e tamanho; considerar Git LFS.
6. Inconsistência de índices
   - Mistura de `index.md` e `README.md` entre áreas (ex.: `docs/ai/index.md`, `docs/database/index.md`). `docs/skills` possui `index.md` e `README.md` (duplicação a ser resolvida).
7. Arquivos que parecem código
   - `docs/frontend/ui/styles.css` é código CSS: se for apenas exemplo, mantê-lo; se for artefato de produção, mover para o repositório frontend.
8. Verificação de links
   - Não foi rodado um verificador automático de links; mover arquivos exigirá atualização de referências relativas (ex.: `docs/index.md` e links entre áreas).

Mapeamento sugerido (ações de baixo risco)
- `docs/frontend/ui/styles.css` -> `docs/design/generated/ui-styles.css` (centralizar gerados)
- `docs/design/azonix/Azonix.otf` -> `docs/design/assets/fonts/Azonix.otf` (padronizar pasta de assets)
- `docs/ai/index.md` -> `docs/ai/README.md` (quando README não existir)
- `docs/database/index.md` -> `docs/database/README.md` (quando README não existir)
- `docs/agents/index.md` -> `docs/agents/README.md` (quando README não existir)
- `docs/frontend/web/index.md` -> `docs/frontend/web/README.md`
- `docs/skills/index.md` -> `docs/skills/index.source.md` (preservar para merge manual com `README.md`)

Riscos / validações necessárias
- Confirmar licença da fonte Azonix antes de mantê-la no VCS (ou usar mirror/asset server).
- Validar se `styles.css` é código de produção; em caso afirmativo mover para frontend (preserva histórico) e deixar apenas documentação e exemplos em `docs/`.
- Sincronizar `tokens.json` com qualquer CSS gerado; preferir `tokens.json` como fonte da verdade e automatizar a geração do CSS.
- Atualizar links relativos após qualquer `git mv`.

Recomendações prioritárias
1. Checar licença/uso das fontes e assets binários. (ALTA)
2. Centralizar tokens em `docs/design/tokens.json` e criar script para gerar CSS em `docs/design/generated/` (ALTA).
3. Padronizar índices por área (usar `README.md` como padrão) e atualizar `docs/index.md`. (MÉDIO)
4. Mover arquivos de código (ex.: CSS de produção) para o repositório correspondente (frontend). (MÉDIO)
5. Adicionar verificador de links à CI (`lychee` ou `markdown-link-check`). (ALTA)
6. Usar Git LFS ou host de assets para fontes/PNG grandes. (MÉDIO)

Artefatos gerados (neste repositório)
- `docs/AUDIT_REPORT.md`  — relatório detalhado (este arquivo).
- `docs/AUDIT_COMMANDS.sh` — script com comandos `git mv` sugeridos (executar apenas após revisão).
- `docs/AUDIT_CHECKLIST.md` — checklist de PR e lista de links/ações a atualizar.

Próximos passos sugeridos
1. Revisar os artefatos gerados e validar pontos de licença.
2. Autorizar execução dos comandos sugeridos em um branch dedicado.
3. Abrir PRs pequenos, um por domínio/área, rodar verificador de links e revisar manualmente quaisquer merges de `index.md`/`README.md`.

Observação final
- As mudanças sugeridas preservam histórico via `git mv`. Não aplicar comandos sem revisão; alguns `git mv` podem sobrescrever arquivos com o mesmo nome — revisar caso a caso.
