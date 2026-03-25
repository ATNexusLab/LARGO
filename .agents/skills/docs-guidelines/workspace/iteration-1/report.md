# Relatório de mudanças sugeridas — docs/architecture

Gerado pela skill docs-guidelines com base nos evals.

## Mapeamento proposto

- docs/architecture/adr-db.md -> docs/architecture/adr/database/01-adr-db.md
- docs/architecture/adr-gemini-model-selection.md -> docs/architecture/adr/ai/02-adr-gemini-model-selection.md
- docs/architecture/adr-migrations.md -> docs/architecture/adr/database/03-adr-migrations.md
- docs/architecture/architecture.md -> docs/architecture/adr/architecture/04-architecture.md

## Instruções de uso

1. Revise os destinos e nomes sugeridos.
2. Faça backup/commit ou stash das mudanças locais antes de aplicar.
3. Execute `.agents/skills/docs-guidelines/workspace/iteration-1/commands.sh` para aplicar `git mv` (o script ignora fontes ausentes).
4. Verifique com `git status`, `git diff --staged` e rode o verificador de links.
5. Separe as mudanças em PRs por domínio quando necessário (ex.: `adr/database`, `adr/ai`).

## Checklist de revisão

- [ ] Confirmar que todos os ADRs foram categorizados corretamente.
- [ ] Atualizar `docs/index.md` e índices locais.
- [ ] Atualizar referências relativas nos arquivos movidos.
- [ ] Rodar testes de link-checker.

---

(Arquivo gerado automaticamente pelo Copilot CLI)