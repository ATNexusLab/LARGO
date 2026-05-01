Schema e exemplos de documentos — Banco: `largo`

Observação de escopo da branch
------------------------------
- A foundation operacional inicial cobre apenas o recurso `tasks` necessário para `POST /tasks`.
- `expenses` e `users` seguem documentados como alvo do produto, mas não entram no smoke path obrigatório da Task 1.

Collections principais
---------------------
1) expenses (gastos / NF-e)
- Campos principais
  - _id: ObjectId
  - user_id: ObjectId (opcional) — referência para usuário local
  - total: number
  - items: array of objects [{ description, quantity, unit_price, total }]
  - date: date
  - cnpj: string
  - estabelecimento: string
  - raw_text: string (texto extraído via OCR)
  - meta: object (qualquer metadado adicional)

Exemplo (documento):
{
  "_id": "ObjectId(..)",
  "user_id": "ObjectId(..)",
  "total": 150.5,
  "items": [ { "description": "Arroz 5kg", "quantity": 1, "unit_price": 20.0, "total": 20.0 } ],
  "date": {"$date": "2026-03-23T12:34:00Z"},
  "cnpj": "12.345.678/0001-99",
  "estabelecimento": "Mercado Exemplo",
  "raw_text": "...texto extraído...",
  "meta": {"ocr_confidence": 0.93}
}

JSON Schema (validator) — aplicado via script de inicialização (ver `docs/database/setup.md`):

expenses_validator = {
  "$jsonSchema": {
    "bsonType": "object",
    "required": ["total", "items", "date", "cnpj"],
    "properties": {
      "total": {"bsonType": ["double", "int", "long", "decimal"]},
      "items": {"bsonType": "array"},
      "date": {"bsonType": "date"},
      "cnpj": {"bsonType": "string"},
      "estabelecimento": {"bsonType": "string"},
      "user_id": {"bsonType": "objectId"}
    }
  }
}

2) tasks (foundation Task 1)
- Campos mínimos da foundation
  - _id: ObjectId
  - title: string
  - status: enum: ['pending','in_progress','done','blocked']
  - created_at: date

- Regras da foundation
  - `title` é obrigatório
  - `status` é persistido pelo servidor e nasce como `pending` em `POST /tasks`
  - `created_at` é persistido pelo servidor
  - `description`, `due_date` e `user_id` ficam fora do contrato mínimo inicial

Exemplo (documento):
{
  "title": "Revisar recibos do mês",
  "status": "pending",
  "created_at": {"$date": "2026-03-23T10:00:00Z"}
}

JSON Schema (validator mínimo da foundation):

tasks_validator = {
  "$jsonSchema": {
    "bsonType": "object",
    "required": ["title", "status", "created_at"],
    "properties": {
      "title": {
        "bsonType": "string",
        "minLength": 1,
        "maxLength": 120
      },
      "status": {
        "enum": ["pending", "in_progress", "done", "blocked"]
      },
      "created_at": {"bsonType": "date"}
    }
  }
}

3) users (usuários)
- Campos principais
  - _id: ObjectId
  - email: string (único)
  - password_hash: string
  - roles: array (ex: ['admin'])
  - created_at: date

Observação
----------
Schemas no banco permanecem deliberadamente mínimos para permitir evolução controlada. A validação de contrato HTTP deve ser mais estrita no gateway do que no Mongo.
