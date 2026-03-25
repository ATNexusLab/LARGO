---
name: docs-guidelines
description: "Gera recomendações e checklist para reorganizar a pasta docs/ (ADRs, índices, TOON)."
user-invocable: true
compatibility: "1.0"
metadata:
  author: Copilot
  tags: [docs, adr, audit, toon, checklist]
---

# Skill: docs-guidelines

Resumo

Esta skill gera recomendações e um checklist operacional para reorganizar a pasta `docs/` do projeto. O objetivo é produzir instruções acionáveis (mapeamento de arquivos, comandos git sugeridos, checklist de PR) que permitam reestruturar a documentação com baixo risco e PRs menores.

Quando usar

- Ao reorganizar ADRs e índices da documentação.
- Antes de abrir PRs que movem/renomeiam documentos.
- Ao documentar prompts TOON ou schemas na pasta `docs/ai`.

Entrada esperada

- Caminho raiz da documentação (ex.: `docs/`) ou área específica (ex.: `docs/architecture`).
- Contexto opcional: prioridades de domínio (backend, infra, ai, web).

Saída esperada

- Relatório Markdown com: mapa de arquivos -> destino sugerido;
- Comandos `git mv` sugeridos (preservam histórico);
- Checklist passo-a-passo para preparar PRs pequenos;
- Lista de links que precisam ser atualizados.

Checklist (passos sugeridos)

1. Auditar: listar todos os arquivos em `docs/` e identificar ADRs e READMEs.
2. Mapear destinos por domínio: `docs/architecture/adr/<dominio>/`.
3. Aplicar convenção de nomes para ADRs: `NN-descricao-curta.md` (NN = ordinal).
4. Criar/atualizar README por área: `docs/<area>/README.md`.
5. Atualizar `docs/index.md` com links para os índices locais.
6. Gerar comandos `git mv` sugeridos e instruções para PRs.
7. Rodar verificador de links e ajustar referências relativas.
8. Recomendar divisão de alterações em PRs por domínio/área.

Convenções recomendadas

- ADRs: `docs/architecture/adr/<dominio>/NN-descricao.md`.
- Índices locais: `docs/<area>/index.md` ou `README.md`.
- Prompts TOON: documentar em `docs/ai/prompts.md` com schema e exemplos.
- PRs: prefira PRs pequenos e centrados em um domínio.

Exemplos de prompts

- "Auditar `docs/architecture` e sugerir estrutura de ADRs e comandos `git mv`."
- "Gerar README para `docs/ai` com sumário e exemplos de TOON."

Evals

Ver `evals/evals.json` para casos de teste que validam o mapeamento e o checklist.

Limitações

- A skill gera recomendações; por segurança não aplica mudanças no repositório por padrão. Integração automática (ex.: aplicar `git mv`) exige autorização explícita e revisão adicional.

Outputs de exemplo

- `report.md` (mapping de arquivos e destinos)
- `commands.sh` (lista de `git mv` sugeridos)
- `docs/index.md` (diff sugerido)

Manutenção

- Autor: Copilot
- Revisões futuras: manter histórico no PR e atualizar `docs/index.md` ao final de cada reorganização.
