# ADR 03 — Estratégia de inicialização e migrações do banco

Status: Aceito
Data: 2026-03-25

## Contexto

Na branch `feat/recovery-foundation-start`, o único componente operacional versionado é o MongoDB via `docker-compose.yml`. A foundation inicial precisa entregar um caminho confiável para persistir `POST /tasks` ponta a ponta, sem reintroduzir dependências paralelas de runtime nem esconder falhas de infraestrutura dentro do startup do gateway.

Também já foi consolidado que:

- a capability inicial é `tasks`;
- o primeiro fluxo HTTP obrigatório é `POST /tasks`;
- não existe código recuperável para `gateway/`, `ai-worker` ou `web/`;
- a retomada seguirá por bootstrap incremental.

## Decisão

Adotar um binário/job one-shot em Rust chamado `db-init` como mecanismo autoritativo de inicialização estrutural do MongoDB.

Para a foundation da Task 1, `db-init` deve:

- aguardar o Mongo ficar disponível com retry e timeout explícitos;
- garantir de forma idempotente a existência da coleção `tasks`;
- aplicar o validator mínimo da coleção `tasks`;
- garantir o conjunto mínimo de índices definido para a foundation;
- encerrar com código `0` em sucesso e código diferente de `0` em falha operacional.

Para evitar acoplamento frágil entre bootstrap e runtime:

- `db-init` **não** deve ficar embutido no startup normal do gateway;
- o gateway pode assumir que o banco já foi inicializado pelo caminho operacional definido;
- seeds de negócio ficam fora do escopo da foundation;
- `expenses` e `users` não fazem parte do smoke path obrigatório da Task 1.

## Consequências

- A inicialização do banco fica alinhada com a stack principal de backend (Rust), reduzindo drift operacional.
- Falhas de preparação do Mongo ficam visíveis como falhas do job `db-init`, em vez de se manifestarem como erros indiretos durante requests.
- O contrato de `POST /tasks` passa a depender de um passo de bootstrap explícito e observável.
- A foundation pode evoluir coleção por coleção, começando por `tasks`, sem bloquear por escopo maior de banco.

## Definition of Done operacional para a foundation

Para a Task 1, considera-se que o skeleton/smoke path de `db-init` está fechado quando:

1. Em uma base vazia, uma execução de `db-init` cria/prepara a coleção `tasks` com validator e índices mínimos esperados.
2. Uma segunda execução não falha e não duplica artefatos (idempotência).
3. Após a execução, uma inserção compatível com o contrato mínimo de `POST /tasks` é aceita.
4. Após a execução, uma inserção incompatível com o schema mínimo é rejeitada pelo banco.

## Alternativas consideradas

### 1. Manter inicialização em Python

**Prós**
- Mais rápida para prototipação imediata.

**Contras**
- Duplica stack operacional sem necessidade.
- Aumenta surface area de build/runtime.
- Introduz drift entre contratos documentados no backend e setup do banco.

**Decisão:** rejeitada.

### 2. Usar `docker-entrypoint-initdb.d`

**Prós**
- Simples para scripts estáticos.

**Contras**
- Ruim para evolução incremental idempotente.
- Limitada para lógica de retry/wait e observabilidade.
- Acopla preparação do schema ao ciclo de vida do container do Mongo.

**Decisão:** rejeitada.

### 3. Executar migrations automaticamente no startup do gateway

**Prós**
- Menos um passo operacional aparente.

**Contras**
- Mistura concerns de runtime HTTP com preparação de infraestrutura.
- Pode gerar condições de corrida em múltiplas instâncias.
- Dificulta diagnosticar se a falha está no serviço ou no banco.

**Decisão:** rejeitada.

## Próximos passos autorizados por este ADR

1. Implementar o binário Rust `db-init`.
2. Empacotar `db-init` como execução one-shot no ambiente local.
3. Materializar apenas o schema mínimo de `tasks` necessário para `POST /tasks`.
