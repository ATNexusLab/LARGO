# Backlog Ativo — LARGO

- [ ] Task 2 — [AI] Foundation do ai-worker
  - **Agente responsável:** `backend`
  - **Dependências:** `Task 1.4`
  - **Status:** pending
  - **Objetivo:** introduzir o serviço FastAPI já acoplado a contratos estáveis do gateway.
  - **Critérios de aceite:**
    - Dado a foundation do gateway estabilizada, quando a task terminar, então o ai-worker deve expor contrato explícito compatível com o gateway.

- [ ] Task 3 — [FEAT] Foundation do frontend para `tasks`
  - **Agente responsável:** `frontend`
  - **Dependências:** `Task 1.4`
  - **Status:** pending
  - **Objetivo:** criar a primeira surface real consumindo o contrato estabilizado de `tasks`.
  - **Critérios de aceite:**
    - Dado o contrato de `tasks` estabilizado, quando a task terminar, então o frontend deve consumir o fluxo inicial sem depender de OCR/Gemini.

- [ ] Task 4 — [INFRA] Fechar docker-compose end-to-end
  - **Agente responsável:** `devops`
  - **Dependências:** `Task 2`, `Task 3`
  - **Status:** pending
  - **Objetivo:** consolidar a execução completa do sistema no compose.
  - **Critérios de aceite:**
    - Dado gateway, ai-worker, frontend e Mongo prontos, quando a task terminar, então o compose deve refletir a topologia real e permitir o fluxo mínimo fim a fim.

## Notas operacionais

- A retomada segue a preferência [[prefer-production-ready-over-mvp]].
- Fora da `Task 1`: `ai-worker`, frontend, OCR/Gemini/TOON e fluxos além de `POST /tasks` + healthcheck.
