Índices obrigatórios e recomendações

Objetivo: garantir desempenho nas consultas mais comuns e unicidade onde necessário.

Coleção: expenses
- idx_expenses_date: { date: -1 } — consultas por intervalo de data e ordenação recente
- idx_expenses_cnpj: { cnpj: 1 } — filtros por estabelecimento/CNPJ
- idx_expenses_user_date: { user_id: 1, date: -1 } — consultas do usuário por período

Coleção: tasks
- idx_tasks_status: { status: 1 } — listagem por estado (pending/in_progress/done)
- idx_tasks_due_date: { due_date: 1 } — busca por prazo
- idx_tasks_user: { user_id: 1 } — tarefas do usuário

Coleção: users
- uniq_users_email: { email: 1 } (unique) — garante unicidade de login

Observações e práticas
- Criar índices compostos quando consultas frequentes envolvem múltiplos filtros (ex.: user_id + date).
- Monitorar cardinalidade e remover índices não utilizados.
- Declarar índices com nomes explícitos facilita verificação em scripts e testes.
- Todos os índices devem ser criados pelo script de inicialização para manter infra idempotente.
