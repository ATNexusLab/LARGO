```Tsx
import React from 'react';

type Props = { value: string | number; positive?: boolean; className?: string };

export default function ValueBadge({ value, positive = false, className = '' }: Props) {
  return (
    <div className={`inline-value ${positive ? 'glow value-glow' : ''} ${className}`}>
      <span className="value">{value}</span>
    </div>
  );
}
```