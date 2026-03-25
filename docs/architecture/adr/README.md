# ADRs — Architecture Decision Records

Registro de decisões arquiteturais do projeto LARGO. Cada ADR documenta o contexto, a decisão tomada e suas consequências.

## Índice

| ID   | Arquivo                                                    | Título                          | Status    |
| ---- | ---------------------------------------------------------- | ------------------------------- | --------- |
| 01   | [database/01-adr-db.md](database/01-adr-db.md)             | Banco de dados: MongoDB         | Aceito    |
| 02   | [ai/02-adr-gemini-model-selection.md](ai/02-adr-gemini-model-selection.md) | Seleção do modelo Gemini | Aceito |
| 03   | [database/03-adr-migrations.md](database/03-adr-migrations.md) | Estratégia de migrações DB  | Proposto  |
| 04   | [architecture/04-architecture.md](architecture/04-architecture.md) | Arquitetura macro do sistema | Aceito |

## Convenções

- **Aceito** — decisão implementada ou aprovada para implementação.
- **Proposto** — decisão em discussão, ainda não aceita. Não implementar antes da aceitação formal.
- **Depreciado** — substituído por ADR mais recente.
- **Supersedido** — ver ADR substituto indicado no arquivo.

## Como adicionar um ADR

1. Criar o arquivo no subdiretório correspondente ao domínio (`ai/`, `architecture/`, `database/`, etc.).
2. Nomear com `NN-adr-titulo.md` (sequencial no domínio).
3. Atualizar este índice.
4. Referenciar o ADR relevante em `docs/` quando aplicável.
