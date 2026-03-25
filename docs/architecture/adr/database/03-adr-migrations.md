# ADR: Estratégia de Migrations e Inicialização do Banco

Data: 2026-03-25
Status: proposed

Contexto

O projeto define o Gateway em Rust como único responsável pelo acesso ao MongoDB (Axum + Mongo adapter). A inicialização atual usava um job Python (`mongo-init`) para criar índices e seeds, o que introduz dependência externa e desalinhamento com a stack.

Decisão

Reescrever a lógica de migrations/initialization em Rust, fornecendo um binário de "migrations" ou "db-init" que:

- Execute criação de índices, validações e sementes necessárias.
- Tenha comportamento retry/wait para aguardar o Mongo pronto.
- Seja empacotado via Docker (multi-stage) e usado como job `depends_on`/`command` no `docker-compose` quando necessário.

Consequências

- Alinhamento da stack (mesma linguagem da infra principal).
- Menor surface area de runtime (sem pip installs em startup).
- Melhor integração com as práticas de logging e error handling do gateway.

Alternativas consideradas

- Manter o python init: mais rápido para protótipos, mas cria dependência duplicada.
- Usar `/docker-entrypoint-initdb.d`: funciona apenas para scripts simples.

Plano de migração

1. Implementar binário Rust `db-init` que use `mongodb` crate.
2. Criar Dockerfile multistage para `db-init` e atualizar `docker-compose` para suportar job one-off.
3. Migrar lógica do atual script Python (se existir) para Rust e testar localmente.

