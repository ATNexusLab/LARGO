# Paleta escolhida — Obsidian Black + Electric Volt

Resumo
- Fundo (Base): Obsidian Black — #09090b
- Superfície (Cards): Zinc Night — #18181b
- Destaque (Primary): Electric Volt — #d9f99d
- Ação (Secondary): Steel Blue — #38bdf8
- Texto (Heading): Pure Zinc — #fafafa
- Texto (Mudo): Slate Gray — #71717a

Hierarquia visual (recomendações)
- Valores Monetários (BRL): o valor principal deve usar `--color-text` (Pure Zinc). Para saldos/valores positivos aplicar a classe `.value-glow` que adiciona uma borda sutil em Electric Volt para indicar "saúde" do sistema.
- Upload de NF-e (estado `processing_ai`): substituir spinner azul por uma barra de progresso `progress-electric` com animação de pulso (`.pulse`) em Electric Volt, indicando processamento em tempo real.
- Gráficos e tendências: usar Electric Volt para a série principal (linha/área) e Steel Blue para séries secundárias; evitar múltiplas cores vibrantes simultâneas.

Exemplos de aplicação (HTML/CSS)

Card com destaque positivo:

```html
<div class="card value-glow">
  <div class="card-body">
    <h3 class="text-muted">Saldo</h3>
    <div class="value">R$ 1.234,56</div>
  </div>
</div>
```

Barra de progresso (processing_ai):

```html
<div class="progress progress-electric" role="progressbar" aria-valuenow="45" aria-valuemin="0" aria-valuemax="100">
  <div class="progress-value" style="width:45%"></div>
</div>
```

Notas
- Testar contraste (WCAG AA) para todas as combinações. Pure Zinc sobre Obsidian Black tem contraste confortável.
- Ao enviar o logo (SVG/PNG) para `docs/design/assets/`, atualizo os exemplos e variações (negativo, mono).
