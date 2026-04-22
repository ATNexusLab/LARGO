# Architecture Note 01 — Foundation inicial de `tasks`

Data: 2026-03-25
Status: Aceita para Task 1

## Contexto

A foundation inicial do recovery precisa fechar quatro ambiguidades antes de qualquer implementação:

1. o contrato mínimo de `POST /tasks`;
2. os campos mínimos do recurso `tasks`;
3. a postura inicial de autenticação;
4. o que conta como skeleton/smoke path de `db-init`.

O escopo desta nota é intencionalmente restrito ao primeiro fluxo persistido no Mongo. `ai-worker`, frontend, OCR/Gemini/TOON e demais rotas ficam fora.

## Decisões

### 1. Contrato mínimo de `POST /tasks`

`POST /tasks` é o primeiro fluxo HTTP real da foundation e existe apenas para criar uma tarefa local no Mongo.

#### Request

- Método: `POST`
- Path: `/tasks`
- `Content-Type`: `application/json`
- Corpo mínimo aceito:

```json
{
  "title": "Revisar recibos do mês"
}
```

#### Regras de validação de entrada

- `title` é obrigatório;
- `title` deve ser string;
- `title` deve ser persistido sem espaços nas pontas;
- após trim, `title` deve ter entre 1 e 120 caracteres;
- o payload não aceita campos extras na foundation.

#### Persistência mínima esperada

O gateway persiste um documento `tasks` com os seguintes campos mínimos:

```json
{
  "_id": "ObjectId",
  "title": "Revisar recibos do mês",
  "status": "pending",
  "created_at": "ISODate"
}
```

- `_id` é gerado pelo Mongo;
- `status` é controlado pelo servidor e sempre nasce como `pending`;
- `created_at` é controlado pelo servidor.

#### Response de sucesso

- Status: `201 Created`
- Corpo:

```json
{
  "id": "67e2b5e7b9d5c2c7d4c8b001",
  "title": "Revisar recibos do mês",
  "status": "pending",
  "created_at": "2026-03-25T14:30:00Z"
}
```

#### Falhas mínimas explícitas

**400 Bad Request**

Usado para JSON malformado ou `Content-Type` incompatível.

```json
{
  "error": {
    "code": "BAD_REQUEST",
    "message": "request body must be valid JSON"
  }
}
```

**422 Unprocessable Entity**

Usado para violação do contrato de entrada (`title` ausente, vazio, não string, maior que 120, ou presença de campos extras).

```json
{
  "error": {
    "code": "TASK_VALIDATION_FAILED",
    "message": "title is required and must contain between 1 and 120 characters",
    "details": [
      {
        "field": "title",
        "issue": "required"
      }
    ]
  }
}
```

**503 Service Unavailable**

Usado para falha mínima de persistência: Mongo indisponível, write não reconhecido, ou banco não preparado para aceitar o documento.

```json
{
  "error": {
    "code": "TASK_PERSISTENCE_FAILED",
    "message": "task could not be persisted"
  }
}
```

### 2. Campos mínimos do recurso `tasks`

Campos obrigatórios da foundation:

- `id` / `_id`
- `title`
- `status`
- `created_at`

Campos explicitamente fora da foundation:

- `description`
- `due_date`
- `updated_at`
- `user_id`
- qualquer metadado de AI

### 3. Postura inicial de autenticação

A foundation inicial de `POST /tasks` opera **sem autenticação de aplicação**.

Isto significa:

- nenhuma credencial é exigida para `POST /tasks` na Task 1;
- não existe vínculo com `users`;
- o gateway não deriva identidade de `Authorization` header nessa foundation;
- o fluxo é tratado como **single-tenant local/bootstrap**, adequado apenas ao ambiente de desenvolvimento/recovery atual.

Limite explícito:

- esta exceção vale apenas para a foundation inicial;
- JWT continua sendo o alvo arquitetural para quando a capability de identidade existir.

### 4. `db-init`: skeleton/smoke path

Para a Task 1, o significado operacional é:

#### Skeleton

Existe um caminho one-shot chamado `db-init` com contrato operacional claro:

- conecta no Mongo usando configuração de ambiente;
- espera o banco ficar pronto;
- garante a coleção `tasks`;
- aplica o validator mínimo;
- garante o conjunto mínimo de índices da foundation;
- retorna sucesso/erro de forma observável.

#### Smoke path

O smoke path está satisfeito quando, em ambiente limpo:

1. `db-init` roda com sucesso;
2. `db-init` roda novamente com sucesso;
3. após isso, um documento compatível com o contrato de `POST /tasks` pode ser inserido;
4. um documento incompatível com os campos mínimos é rejeitado.

## Alternativas rejeitadas

### Exigir JWT já na foundation

**Rejeitada** porque não existe capability de identidade pronta nesta branch. Forçar JWT agora criaria um contrato artificial, dependente de usuários ainda inexistentes, e atrasaria o primeiro caminho persistido.

### Aceitar `description` e `due_date` já no primeiro contrato

**Rejeitada** porque esses campos ainda não sustentam nenhum fluxo obrigatório da Task 1. Incluir agora aumentaria superfície de validação, schema e testes sem retorno imediato.

### Permitir payload aberto com campos arbitrários

**Rejeitada** porque a foundation precisa de um contrato pequeno, testável e estável. Evolução futura deve ser aditiva e intencional.

### Criar índices secundários especulativos já na Task 1

**Rejeitada** porque ainda não existe read path que justifique custo de manutenção adicional. O conjunto mínimo de índices da foundation permanece o estritamente necessário ao fluxo de criação.
