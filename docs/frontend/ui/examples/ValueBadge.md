# ValueBadge

Componente para exibir valores numéricos (saldo, variação). Suporta destaque positivo com glow.

Props
- value: string | number
- positive: boolean (aplica glow em Electric Volt)

Uso
```tsx
import ValueBadge from './react/ValueBadge';

<ValueBadge value="R$ 1.234,56" positive />
```

Observações
- A classe `.value-glow` aplica a borda sutil em Electric Volt.
