# UI Kit — Componentes Base (inicial)

Objetivo
- Fornecer snippets e exemplos para componentes base usando tokens e Tailwind/shadcn/ui.

Botões
- Primary (filled): `class="bg-primary text-white rounded-md px-4 py-2 hover:bg-primary-600"`
- Secondary (outline): `class="border border-neutral-300 text-neutral-900 rounded-md px-4 py-2 bg-transparent"`
- Ghost: `class="text-primary bg-transparent px-3 py-1"`

Exemplo React (Tailwind + shadcn)
<Button className="bg-primary text-white rounded-md px-4 py-2">Salvar</Button>

Inputs
- Estilo base: `class="w-full border border-neutral-300 rounded-md px-3 py-2 bg-white placeholder:text-neutral-400"`
- Estados: focus (outline-primary), error (border-danger + helper text)

Card
- Estrutura: `class="bg-white rounded-md shadow-md p-4"`
- Use para agrupamento de informações e listas resumidas.

Navbar / Header
- Altura: 64px; padding horizontal padrão (spacing-4).
- Colocar logo à esquerda, ações à direita; usar contraste suficiente entre bg e texto.

Formulários
- Labels: font-size 14px, margin-bottom spacing-2
- Group spacing consistent: uso de spacing-4 entre campos

Tokens em uso
- Preferir classes utilitárias que referenciem tokens (ex.: `bg-[var(--color-primary)]`).

Próximos passos (após escolha da fonte)
- Gerar variantes de componentes com a tipografia final aplicada.
- Criar exemplos práticos (formulário de cadastro, dashboard simples, lista de tarefas).
