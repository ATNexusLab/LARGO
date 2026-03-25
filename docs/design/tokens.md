# Design Tokens — Identidade Visual (canônico)

Visão geral
- Este arquivo é gerado a partir de `docs/design/tokens.json` (fonte canônica) e serve como referência humana para os tokens usados no frontend.

Paleta
- --color-bg: #09090b  /* Obsidian Black */
- --color-surface: #18181b /* Zinc Night */
- --color-primary: #d9f99d /* Electric Volt */
- --color-secondary: #38bdf8 /* Steel Blue */
- --color-text: #fafafa /* Pure Zinc */
- --color-muted: #71717a /* Slate Gray */

Cores semânticas
- --color-success: #16A34A
- --color-danger: #DC2626
- --color-warning: #F59E0B
- --color-info: #0EA5A4

Tipografia
- --font-sans: 'Azonix', 'Inter', system-ui
- Observação: Azonix já está disponível em `docs/design/azonix` (fonte local). Preferir carregar localmente no frontend e garantir licenciamento.

Espaçamento (base: 4px)
- spacing-1: 4px
- spacing-2: 8px
- spacing-3: 12px
- spacing-4: 16px
- spacing-5: 24px
- spacing-6: 32px

Radius & Sombra
- radius-sm: 4px
- radius-md: 8px
- radius-lg: 12px
- shadow-sm: 0 1px 2px rgba(0,0,0,0.6)
- shadow-md: 0 4px 12px rgba(0,0,0,0.64)

Exemplo de tokens (JSON)
{
  "color": {
    "bg": "#09090b",
    "surface": "#18181b",
    "primary": "#d9f99d",
    "secondary": "#38bdf8",
    "text": "#fafafa",
    "muted": "#71717a",
    "success": "#16A34A",
    "danger": "#DC2626",
    "warning": "#F59E0B",
    "info": "#0EA5A4"
  },
  "font": {
    "sans": "Azonix, Inter, system-ui"
  },
  "spacing": {
    "1": "4px",
    "2": "8px",
    "3": "12px",
    "4": "16px",
    "5": "24px",
    "6": "32px"
  },
  "radius": {
    "sm": "4px",
    "md": "8px",
    "lg": "12px"
  },
  "shadow": {
    "sm": "0 1px 2px rgba(0,0,0,0.6)",
    "md": "0 4px 12px rgba(0,0,0,0.64)"
  }
}

Exemplo CSS variables
:root {
  --color-bg: #09090b;
  --color-surface: #18181b;
  --color-primary: #d9f99d;
  --color-secondary: #38bdf8;
  --color-text: #fafafa;
  --color-muted: #71717a;
  --font-sans: 'Azonix', 'Inter', system-ui;
  --radius-md: 8px;
}

Snippet Tailwind (tailwind.config.js)
module.exports = {
  theme: {
    extend: {
      colors: {
        bg: 'var(--color-bg)',
        primary: 'var(--color-primary)',
        secondary: 'var(--color-secondary)'
      },
      fontFamily: {
        sans: ['Azonix', 'Inter', 'system-ui']
      },
      spacing: {
        '1': '4px',
        '2': '8px',
        '3': '12px',
        '4': '16px'
      }
    }
  }
}

Observações
- Este arquivo é gerado a partir de `docs/design/tokens.json`. Para atualizar os tokens, edite `tokens.json` e solicite regeneração (ou automatize via script).
- Arquivos gerados: `docs/ui/styles.css` (CSS variables e utilitários) e `docs/design/palette.md` (contexto e regras de aplicação).
