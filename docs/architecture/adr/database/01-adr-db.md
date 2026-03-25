# ADR 0001 — Escolha do MongoDB para persistência

Status: Aceito
Data: 2026-03-23

Contexto
--------
LARGO precisa de um banco de dados que suporte documentos semi-estruturados (NF-e, itens de nota, notas pessoais) e consultas flexíveis (filtros por data, CNPJ, usuário, status de tarefas). A equipe prioriza desenvolvimento local com orquestração via Docker Compose e operação simples para usuários finais.

Decisão
-------
Adotar MongoDB (linha 6.0.x) como banco de dados principal do projeto.

Justificativa
-------------
- Document model facilita armazenar notas fiscais com estruturas variáveis (itens, impostos, metadados).
- Suporte nativo a índices secundários e índices compostos para consultas analíticas.
- Boa compatibilidade com drivers Rust/Python e operação em containers Docker.

Consequências
-------------
- Definir JSON Schema validators para collections críticas (`expenses`, `tasks`, `users`).
- Criar índices obrigatórios para consultas comuns (ex.: `expenses.date`, `tasks.status`, `users.email` único).
- Automatizar criação de coleções/validators/indexes via script de inicialização (executado dentro do contexto Docker — `docker compose run --rm mongo-init`).

Alternativas consideradas
-------------------------
- PostgreSQL+JSONB: mais forte em consistência e joins; descartado por complexidade na modelagem de NF-e e pela necessidade de iterações rápidas de esquema.

Notas operacionais
------------------
- Versão do MongoDB deve ser fixada no `docker-compose.yml` (evitar `latest`).
- Backups: usar `mongodump` e restauração com `mongorestore`; documentado em `docs/database/setup.md`.

Revisões
--------
- (2026-03-23) Documento inicial — decisão tomada para Sprint 1.
