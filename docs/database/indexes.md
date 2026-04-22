Índices obrigatórios e recomendações

Objetivo: garantir desempenho nas consultas mais comuns e unicidade onde necessário.

Coleção: expenses
- idx_expenses_date: { date: -1 } — consultas por intervalo de data e ordenação recente
- idx_expenses_cnpj: { cnpj: 1 } — filtros por estabelecimento/CNPJ
- idx_expenses_user_date: { user_id: 1, date: -1 } — consultas do usuário por período

Coleção: tasks
- Foundation Task 1: nenhum índice secundário adicional é obrigatório.
- O índice `_id` padrão do Mongo é suficiente para o primeiro fluxo `POST /tasks`, que é apenas de criação.
- Índices como `{ status: 1 }`, `{ due_date: 1 }` ou `{ user_id: 1 }` ficam adiados até existirem read paths e autenticação correspondentes.

Coleção: users
- uniq_users_email: { email: 1 } (unique) — garante unicidade de login

Observações e práticas
- Criar índices compostos quando consultas frequentes envolvem múltiplos filtros (ex.: user_id + date).
- Monitorar cardinalidade e remover índices não utilizados.
- Declarar índices com nomes explícitos facilita verificação em scripts e testes.
- Todos os índices que forem necessários devem ser criados pelo `db-init` para manter a infra idempotente.
