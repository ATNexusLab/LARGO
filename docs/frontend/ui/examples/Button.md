# Button

Componente Button (exemplo) — variantes: primary, secondary, ghost.

Propriedades principais
- variant: 'primary' | 'secondary' | 'ghost'
- Suporta props nativas de button (onClick, disabled, etc.)

Uso
```tsx
import Button from './react/Button';

<Button variant="primary">Salvar</Button>
```

Observações
- Usa `var(--color-primary)` e `var(--color-bg)` para estilos. Consulte `docs/design/tokens.json` para os valores canônicos.
