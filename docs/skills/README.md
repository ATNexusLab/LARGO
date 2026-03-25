# Skills

Mapeamento de todas as skills disponíveis no projeto LARGO. Fonte canônica: `skills-lock.json`.

## Skills Instaladas

### `docs-guidelines`
- **Propósito:** Gera recomendações e checklist para reorganizar `docs/` (ADRs, índices, TOON).
- **Quando usar:** Ao reorganizar docs, antes de abrir PRs que movem/renomeiam documentos, ao documentar schemas TOON.
- **Fonte:** local (criada com `skill-creator`)

### `multi-stage-dockerfile`
- **Propósito:** Cria Dockerfiles multi-stage otimizados para qualquer linguagem/framework.
- **Quando usar:** Ao criar ou otimizar Dockerfiles de gateway (Rust), ai-worker (Python) ou web (React/Bun).

### `shadcn`
- **Propósito:** Gerencia componentes shadcn/ui — adicionar, buscar, corrigir, estilizar.
- **Quando usar:** Ao adicionar componentes shadcn ao frontend React/Bun, ou ao trabalhar com `components.json`.

### `skill-creator`
- **Propósito:** Cria novas skills, modifica existentes e mede performance.
- **Quando usar:** Quando nenhuma skill disponível cobre o domínio da tarefa.

### `ui-ux-pro-max`
- **Propósito:** Design intelligence para UI/UX web e mobile. Paletas, tipografia, layouts, acessibilidade.
- **Quando usar:** Decisões visuais, validação de paleta/tokens, implementação de componentes com identidade visual.

### `vercel-react-best-practices`
- **Propósito:** Padrões de performance React/Next.js do Vercel Engineering.
- **Quando usar:** Ao escrever, revisar ou refatorar componentes React ou data fetching no frontend.

---

## Skills Planejadas (não instaladas)

As skills abaixo estão listadas nas instruções do Copilot como futuras. Usar `skill-creator` para criá-las quando necessário:

| Skill                    | Domínio                                                       |
| ------------------------ | ------------------------------------------------------------- |
| `axum-patterns`          | Rotas, middlewares, extractors no Axum (Rust)                 |
| `mongodb-patterns`       | Queries, agregações, índices, transações no MongoDB           |
| `ai-worker-patterns`     | Fluxo OCR Vision API → Gemini → JSON estruturado              |
| `toon-prompts`           | Encoding TOON para prompts Gemini, schemas por entidade       |
| `docker-compose-setup`   | Criação e manutenção do docker-compose e Dockerfiles          |
| `react-dashboard`        | Páginas, rotas React, layouts do dashboard                    |
| `security-best-practices`| Checklist de segurança da aplicação                           |
| `rust-error-handling`    | Padrões de erro com thiserror/anyhow no Rust                  |

---

## Como usar uma skill

```
@skill-name [descrição do que precisa]
```

Para validar se uma skill está instalada antes de usar:
```
cat skills-lock.json
```

